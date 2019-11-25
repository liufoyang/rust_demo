#![feature(proc_macro_hygiene, decl_macro)]
use std::sync::{Mutex,Arc};
use std::rc::Rc;
mod lib;
use lib::bft_node::{Btf_Node_Simple,Btf_Node};
use lib::bft_message::{Bft_Prepare_Message, Bft_Message, Bft_Commit_Message, Bft_PrePrepare_Message, Bft_Replay, Bft_PrePrepare_Simple, Bft_Regist_Msg, Bft_Regist_Reply};
use lib::default_tcp_communication::Default_TCP_Communication;
use lib::threadpool::ThreadPool;
use lib::default_tcp_communication;
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


//#[macro_use]
//extern crate lazy_static;
//
//lazy_static! {
//    static ref NODEMUTEX: Arc<Mutex<Btf_Node>> = {
//
//        let mut node:Btf_Node = Btf_Node::start_node("", "8000");
//        let mutex: Arc<Mutex<Btf_Node>> = Arc::new(Mutex::new(node));
//        return mutex;
//    };
//}
//
//#[post("/receiveMsg", data = "<bft_msg>")]
//fn receiveMsg(bft_msg:Json<Bft_Message>) -> &'static str {
//    let mutex = Arc::clone(&NODEMUTEX);
//    let mut node = mutex.lock().unwrap();
//    node.receiveClientMsg(bft_msg.into_inner());
//    return "receive msg";
//}
//
//#[post("/prePrepare", data = "<bft_msg>")]
//fn prePrepare(bft_msg:Json<Bft_PrePrepare_Message>) -> &'static str {
//    let mutex = Arc::clone(&NODEMUTEX);
//    let mut node = mutex.lock().unwrap();
//    node.doPrepare(bft_msg.into_inner());
//    return "receive pre prepare msg";
//}
//
//#[post("/receivePrepare", data = "<bft_msg>")]
//fn receivePrepare(bft_msg:Json<Bft_Prepare_Message>) -> &'static str {
//    let mutex = Arc::clone(&NODEMUTEX);
//    let mut node = mutex.lock().unwrap();
//    node.receivePrepare(bft_msg.into_inner());
//    return "receive prepare msg";
//}
//
//#[post("/receiveCommit", data = "<bft_msg>")]
//fn receiveCommit(bft_msg:Json<Bft_Commit_Message>) -> &'static str {
//    let mutex = Arc::clone(&NODEMUTEX);
//    let mut node = mutex.lock().unwrap();
//    node.receiveCommit(bft_msg.into_inner());
//    return "receive commit msg";
//}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Clone)]
struct Node_Config {
    primaryIp: String,
    primaryPort: String,
    nodeIP: String,
    nodePort: String,
    pubKey: String,
    privateKey: String
}

fn main() {

    let mut config_file_name = "node_config.json";
    let mut arguments = Vec::new();
    for argument in env::args() {
        arguments.push(argument);
    }

    if arguments.len() > 1 {
        config_file_name = arguments[1].as_str();
        println!("config file {}", config_file_name);
    }


    let mut config_file = File::open(config_file_name).expect("Failed to open config file");
    let mut config_contents = String::new();
    config_file.read_to_string(&mut config_contents);

    let config_result:DecodeResult<Node_Config> = json::decode(&config_contents);

    if !config_result.is_ok(){
        panic!("config file content error");
    }

    let config:Node_Config = config_result.unwrap();

    // start node
    let mut node:Btf_Node = Btf_Node::start_node(config.primaryIp.as_str(), config.primaryPort.as_str(), config.nodeIP.as_str(), config.nodePort.as_str());

    // start a thread to handler msg rount
    let node_mutex: Arc<Mutex<Btf_Node>> = Arc::new(Mutex::new(node));

    let mut executor = Command_Executor::new(2);
    let executor_mutex:Arc<Mutex<Command_Executor>> = Arc::new(Mutex::new(executor));

    // start a
    let receiver = Default_TCP_Communication::startListen(config.nodeIP.as_str(), config.nodePort.as_str());

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
                        node.handler_expire(pre_timeout.sequence_num.clone());
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
        println!("error to start expire thread {:?}", expire_handler.err())
    }

    while(true) {
        let msg_result = receiver.recv();
        if msg_result.is_ok() {

            let (mut msg, mut stream) = *(msg_result.unwrap());
            if(msg.command.as_str() == "quit") {
                println!("receive quit command");
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
                    println!("receive client msg {}", msg.payload);
                    let payload = msg.payload.as_str();
                    //let clientMsg_encode:Bft_Message = json::decode(&encode_str).unwrap();
                    let node_msg_result:DecodeResult<Bft_Message> = json::decode(&payload);
                    if(!node_msg_result.is_ok()) {
                        println!("parse client msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_msg = node_msg_result.unwrap();

                        //
                        let mut executor = executor_sub.lock().unwrap();
                        let sequence_num =  node.receiveClientMsg(node_msg, &mut executor);
                        let mut logs_str = String::from("receiveMsg ");
                        logs_str.push_str(sequence_num.to_string().as_str());
                        logs_str.push_str(" ");
                        logs_str.push_str(payload);
                        executor.savelog(logs_str.as_str());

                        stream.write("get it".as_bytes());
                    }
                    println!("quit command");

                } else if(msg.command.as_str() == "prePrepare") {
                    println!("receive prePrepare command");
                    let node_msg_result:DecodeResult<Bft_PrePrepare_Message> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        println!("parse prePrepare msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_msg = node_msg_result.unwrap();
                        let(view_num, sequece_num) = node.doPrepare(node_msg);
                        let now = SystemTime::now();

                        let pre_timeout = Bft_PrePrepare_Simple {
                            view_num:view_num,
                            sequence_num:sequece_num,
                            time:now
                        };
                        let pre_sender = pre_sender_sub.lock().unwrap();
                        pre_sender.send(pre_timeout);
                    }

                } else if(msg.command.as_str() == "prepare") {
                    println!("receive prepare command");
                    let node_msg_result:DecodeResult<Bft_Prepare_Message> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        println!("parse client msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let mut executor = executor_sub.lock().unwrap();
                        let node_msg = node_msg_result.unwrap();
                        node.receivePrepare(node_msg, &mut *executor);
                    }

                }else if(msg.command.as_str() == "commit") {
                    println!("receive commit command");
                    let node_msg_result:DecodeResult<Bft_Commit_Message> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        println!("parse receiveCommit msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_msg = node_msg_result.unwrap();
                        node.receiveCommit(node_msg);
                    }


                } else if(msg.command.as_str() == "regist") {
                    println!("regist command");
                    let node_msg_result:DecodeResult<Bft_Regist_Msg> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        println!("parse regist msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_msg = node_msg_result.unwrap();
                        let regist_result = node.regist_node(node_msg);

                        let mut payload = json::encode(&regist_result).unwrap();
                        payload.push_str("\n");
                        println!("reply to new regist {}", payload);
                        stream.write_all(payload.as_bytes());
                        stream.flush();
                    }


                }else if(msg.command.as_str() == "newnode") {
                    println!("newnode command");
                    let node_msg_result:DecodeResult<Btf_Node_Simple> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        println!("parse newnode msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_sample = node_msg_result.unwrap();
                        node.receive_new_node(node_sample);
                    }


                }else {
                    println!("receive unknow command");
                    stream.write("unknow message format".as_bytes());
                }


            });

            i += 1;

        } else {

        }
    }

    //expire_handler.join();


}