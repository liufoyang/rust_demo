#![feature(proc_macro_hygiene, decl_macro)]
use std::sync::{Mutex,Arc};
use std::rc::Rc;
mod lib;
use lib::bft_node::{Btf_Node_Simple,Btf_Node};
use lib::bft_message::*;
use lib::default_tcp_communication::Default_TCP_Communication;
use lib::threadpool::ThreadPool;
use lib::default_tcp_communication;
use lib::bft_signtor;
use std::thread;
extern crate rustc_serialize;
use rustc_serialize::json;
use rustc_serialize::json::DecodeResult;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::boxed::Box;
use std::time::{Duration, SystemTime};
use lib::command_executor::Command_Executor;
use std::net::TcpStream;
use std::result::Result;
use std::fs::File;
use std::io::{Read,Write};
use std::env;
use std::collections::BTreeSet;
extern crate rustc_hex;
use rustc_hex::{FromHex,ToHex};
extern crate crypto;

extern crate flexi_logger;
use flexi_logger::{Logger, opt_format};
extern crate log;
use log::*;

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Clone)]
struct Node_Config {
    primaryIp: String,
    primaryPort: String,
    nodeIP: String,
    nodePort: String,
    pubKey: String,
    privateKey: String,
    business_file: String,
    checkpoint_file:String,
    log_record_file:String,
}

fn main() {

    Logger::with_str("info")
        .log_to_file()
        .directory("log_files")
        .format(opt_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    let mut config_file_name = "./config/node_config.json";
    let mut arguments = Vec::new();
    for argument in env::args() {
        arguments.push(argument);
    }

    if arguments.len() > 1 {
        config_file_name = arguments[1].as_str();
        info!("config file {}", config_file_name);
    }


    let mut config_file = File::open(config_file_name).expect("Failed to open config file");
    let mut config_contents = String::new();
    config_file.read_to_string(&mut config_contents);

    let config_result:DecodeResult<Node_Config> = json::decode(&config_contents);

    if !config_result.is_ok(){
        panic!("config file content error");
    }

    let mut config:Node_Config = config_result.unwrap();

    if config.business_file.len() == 0 {
        config.business_file = "businessfile.log".to_string();
    }

    if config.log_record_file.len() == 0 {
        config.log_record_file = "msg_record_file.log".to_string();
    }

    if config.checkpoint_file.len() == 0 {
        config.business_file = "datafile.log".to_string();
    }

    // start a comnunication and listener network
    let (communication, receiver) = Default_TCP_Communication::startListen(config.nodeIP.as_str(), config.nodePort.as_str());

    // start node
    let mut node:Btf_Node = Btf_Node::start_node(config.primaryIp.as_str(), config.primaryPort.as_str(), config.nodeIP.as_str(), config.nodePort.as_str(), communication);

    // start a thread to handler msg rount
    let node_mutex: Arc<Mutex<Btf_Node>> = Arc::new(Mutex::new(node));

    let mut executor = Command_Executor::new(config.log_record_file.as_str(), config.checkpoint_file.as_str(), config.business_file.as_str());
    let executor_mutex:Arc<Mutex<Command_Executor>> = Arc::new(Mutex::new(executor));

    // a thread check expire for prepare msg
    let (pre_sender, pre_receiver) = channel();
    let running:Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

    let pre_sender_mutex =  Arc::new(Mutex::new(pre_sender));

    let is_run_sub = Arc::clone(&running);
    let mutex_sub = Arc::clone(&node_mutex);
    let expire_handler = thread::Builder::new().name("expire_process".to_string()).spawn(move||
        {
            let mut pre_prepare_num_list:Vec<Bft_PrePrepare_Simple> = Vec::new();
            while(true) {
                let rece_result = pre_receiver.recv_timeout(Duration::from_secs(10));
                let time_out_second = Duration::from_secs(180);

                if rece_result.is_ok() {
                    pre_prepare_num_list.push(rece_result.unwrap());
                }

                if pre_prepare_num_list.len() > 0 {
                    let pre_timeout = pre_prepare_num_list.get(0).unwrap();
                    if pre_timeout.time.elapsed().unwrap() > time_out_second {
                        let mut node = mutex_sub.lock().unwrap();
                        node.handler_expire(pre_timeout.msg_sign.as_str());
                        pre_prepare_num_list.remove(0);
                    }
                }

                let is_run = is_run_sub.lock().unwrap();
                if ! *is_run {
                    break;
                }
            }

        });

    if !expire_handler.is_ok() {
        error!("error to start expire thread {:?}", expire_handler.err())
    }


    let is_run_checkpoint = Arc::clone(&running);
    let mutex_checkpoint = Arc::clone(&node_mutex);
    let executor_checkpoint = Arc::clone(&executor_mutex);
    let checkpoint_handler = thread::Builder::new().name("checkpoint_process".to_string()).spawn(move||
        {
            let minu_5 = Duration::from_secs(300);
            while(true) {

                thread::sleep(minu_5);

                let is_run = is_run_checkpoint.lock().unwrap();
                if ! *is_run {
                    break;
                } else {
                    let mut node = mutex_checkpoint.lock().unwrap();
                    let mut executor = executor_checkpoint.lock().unwrap();
                    node.save_checkpoin(&mut executor);

                }
            }

        });

    if !checkpoint_handler.is_ok() {
        error!("error to start expire thread {:?}", checkpoint_handler.err())
    }

    while(true) {
        let msg_result = receiver.recv();
        if msg_result.is_ok() {

            info!("receiver main msg");
            let msg = *(msg_result.unwrap());
            if(msg.command.as_str() == "quit") {
                info!("receive quit command");
                let is_run_arc = Arc::clone(&running);
                let mut is_run = is_run_arc.lock().unwrap();
                *is_run = false;
                break;
            }

            let mutex = Arc::clone(&node_mutex);
            let mut i = 0;

            let pre_sender_sub = Arc::clone(&pre_sender_mutex);
            let executor_sub = Arc::clone(&executor_mutex);
            thread::Builder::new().name(i.to_string()).spawn(move|| {

                let mut node = mutex.lock().unwrap();
                // handler and rount the process
                if(msg.command.as_str() == "receiveMsg") {
                    info!("receive client msg {}", msg.payload);
                    let payload = msg.payload.as_str();
                    //let clientMsg_encode:Bft_Message = json::decode(&encode_str).unwrap();
                    let node_msg_result:DecodeResult<Bft_Message> = json::decode(&payload);
                    if(!node_msg_result.is_ok()) {
                        warn!("parse client msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_msg = node_msg_result.unwrap();

                        //
                        let mut executor = executor_sub.lock().unwrap();
                        let msg_md5_sign =  node.receiveClientMsg(node_msg, &mut executor);
                        let mut logs_str = String::from("receiveMsg ");
                        logs_str.push_str(msg_md5_sign.as_str());
                        logs_str.push_str(" ");
                        logs_str.push_str(payload);
                        executor.savelog(logs_str.as_str());

                        // send timeout
                        let now = SystemTime::now();

                        let pre_timeout = Bft_PrePrepare_Simple {
                            view_num:0,
                            msg_sign:msg_md5_sign.to_string(),
                            time:now
                        };
                        let pre_sender = pre_sender_sub.lock().unwrap();
                        pre_sender.send(pre_timeout);
                    }
                    info!("quit command");

                } else if(msg.command.as_str() == "prePrepare") {
                    info!("receive prePrepare command");
                    let node_msg_result:DecodeResult<Bft_PrePrepare_Message> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        warn!("parse prePrepare msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_msg = node_msg_result.unwrap();
                        let result = node.doPrepare(node_msg);

                        if result.is_some() {
                            let (view_num, sequece_num) = result.unwrap();
                        }
                    }

                } else if(msg.command.as_str() == "prepare") {
                    info!("receive prepare command");
                    let node_msg_result:DecodeResult<Bft_Prepare_Message> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        warn!("parse client msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let mut executor = executor_sub.lock().unwrap();
                        let node_msg = node_msg_result.unwrap();
                        node.receivePrepare(node_msg, &mut *executor);
                    }

                }else if(msg.command.as_str() == "commit") {
                    info!("receive commit command");
                    let node_msg_result:DecodeResult<Bft_Commit_Message> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        warn!("parse receiveCommit msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_msg = node_msg_result.unwrap();
                        node.receiveCommit(node_msg);
                    }


                } else if(msg.command.as_str() == "regist") {
                    info!("regist command");
                    let node_msg_result:DecodeResult<Bft_Regist_Msg> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        warn!("parse regist msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_msg = node_msg_result.unwrap();
                        node.regist_node(node_msg, msg.id.as_str());
                    }


                }else if(msg.command.as_str() == "newnode") {
                    info!("newnode command");
                    let node_msg_result:DecodeResult<Btf_Node_Simple> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        warn!("parse newnode msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_sample = node_msg_result.unwrap();
                        node.receive_new_node(node_sample);
                    }


                }else if(msg.command.as_str() == "viewchange") {
                    info!("newnode command");
                    let node_msg_result:DecodeResult<Bft_View_Change_Message> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        warn!("parse newnode msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let view_change = node_msg_result.unwrap();
                        node.receiveViewChange(view_change);
                    }


                }else if(msg.command.as_str() == "newview") {
                    info!("newnode command");
                    let node_msg_result:DecodeResult<Bft_New_View_Message> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        warn!("parse newnode msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let new_view_msg = node_msg_result.unwrap();
                        node.receiveNewView(new_view_msg);
                    }


                }else if (msg.command.as_str() == "forword"){
                    info!("receive forword msg {}", msg.payload);
                    let payload = msg.payload.as_str();
                    //let clientMsg_encode:Bft_Message = json::decode(&encode_str).unwrap();
                    let node_msg_result:DecodeResult<Bft_Message> = json::decode(&payload);
                    if(!node_msg_result.is_ok()) {
                        warn!("parse client msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_msg = node_msg_result.unwrap();

                        //
                        let mut executor = executor_sub.lock().unwrap();
                        let msg_md5_sign =  node.receiveForwordMsg(node_msg, &mut executor);
                        let mut logs_str = String::from("receiveMsg ");
                        logs_str.push_str(msg_md5_sign.as_str());
                        logs_str.push_str(" ");
                        logs_str.push_str(payload);
                        executor.savelog(logs_str.as_str());

                        // send timeout
                        let now = SystemTime::now();

                        let pre_timeout = Bft_PrePrepare_Simple {
                            view_num:0,
                            msg_sign:msg_md5_sign.to_string(),
                            time:now
                        };
                        let pre_sender = pre_sender_sub.lock().unwrap();
                        pre_sender.send(pre_timeout);
                    }

                } else {
                    warn!("receive unknow command");
                }


            });

            i += 1;

        } else {

        }
    }

    //expire_handler.join();


}