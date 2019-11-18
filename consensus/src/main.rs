#![feature(proc_macro_hygiene, decl_macro)]
use std::sync::{Mutex,Arc};
use std::rc::Rc;
mod lib;
use lib::bft_node::Btf_Node;
use lib::bft_message::{Bft_Prepare_Message, Bft_Message, Bft_Commit_Message, Bft_PrePrepare_Message, Bft_Replay};
use lib::default_tcp_communication::Default_TCP_Communication;
use lib::threadpool::ThreadPool;
use lib::default_tcp_communication;
use std::thread;
extern crate rustc_serialize;
use rustc_serialize::json;
use rustc_serialize::json::DecodeResult;
use std::sync::mpsc::Receiver;
use std::boxed::Box;


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

fn main() {
//    // start new node
//    rocket::ignite().mount("/", routes![receiveMsg,prePrepare,receivePrepare,receiveCommit]).launch();

    // start node
    let port = "8787";
    let mut node:Btf_Node = Btf_Node::start_node("", port);

    // start a thread to handler msg rount
    let node_mutex: Arc<Mutex<Btf_Node>> = Arc::new(Mutex::new(node));

    // start a
    let receiver = Default_TCP_Communication::startListen("127.0.0.1", port);

    while(true) {
        let msg_result = receiver.recv();
        if msg_result.is_ok() {

            let msg = msg_result.unwrap();
            if(msg.command.as_str() == "quit") {
                println!("receive quit command");
                break;
            }

            let mutex = Arc::clone(&node_mutex);
            let mut i = 0;
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
                        node.receiveClientMsg(node_msg);
                    }




                }else if(msg.command.as_str() == "prePrepare") {
                    println!("receive prePrepare command");
                    let node_msg_result:DecodeResult<Bft_PrePrepare_Message> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        println!("parse prePrepare msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_msg = node_msg_result.unwrap();
                        node.doPrepare(node_msg);

                    }

                } else if(msg.command.as_str() == "receivePrepare") {
                    println!("receive receiveMsg command");
                    let node_msg_result:DecodeResult<Bft_Prepare_Message> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        println!("parse client msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_msg = node_msg_result.unwrap();
                        node.receivePrepare(node_msg);
                    }

                }else if(msg.command.as_str() == "receiveCommit") {
                    println!("receive receiveCommit command");
                    let node_msg_result:DecodeResult<Bft_Commit_Message> = json::decode(&msg.payload);
                    if(!node_msg_result.is_ok()) {
                        println!("parse receiveCommit msg json error {}", node_msg_result.err().unwrap());
                    } else {
                        let node_msg = node_msg_result.unwrap();
                        node.receiveCommit(node_msg);
                    }


                } else {
                    println!("receive unknow command");
                }


            });

            i += 1;

        }
    }


}