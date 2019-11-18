use std::sync::Mutex;
use std::rc::Rc;
use crate::Btf_Node;
use crate::Bft_Message;
use crate::Bft_PrePrepare_Message;
use crate::Bft_Prepare_Message;
use crate::Bft_Commit_Message;
use crate::ThreadPool;
extern crate rustc_serialize;
use rustc_serialize::json::{self, ToJson, Json};

#[test]
fn bft_node_create() {
    let mut node:Btf_Node = Btf_Node::start_node("", "7878");
}

#[test]
fn receiveClientMsg() {
    let mut node:Btf_Node = Btf_Node::start_node("", "7878");
    let clientMsg = Bft_Message::new("hello world", "bft_client0001");
    let payload_result = json::encode(&clientMsg);

    if payload_result.is_ok() {
        let encode_str= payload_result.unwrap();
        println!("payload {}", encode_str);

        let clientMsg_encode:Bft_Message = json::decode(&encode_str).unwrap();
        println!("encode {}", clientMsg_encode.get_id());
    } else {
        println!("error {} ", payload_result.err().unwrap());
    }


    node.receiveClientMsg(clientMsg);
}

#[test]
fn doPrepare() {
    let mut node:Btf_Node = Btf_Node::start_node("", "7878");
    let clientMsg = Bft_Message::new("hello world", "bft_client0001");
    let prePrepareMsg:Bft_PrePrepare_Message = Bft_PrePrepare_Message::new(1, 1, clientMsg);
    let payload_result = json::encode(&prePrepareMsg);
    if payload_result.is_ok() {
        println!("payload {}", payload_result.unwrap());
    } else {
        println!("error {} ", payload_result.err().unwrap());
    }
    node.doPrepare(prePrepareMsg);
}

#[test]
fn receivePrepare() {
    println!("begin {}", "receivePrepare");
    let mut node:Btf_Node = Btf_Node::start_node("", "7878");
    let prepareMsg:Bft_Prepare_Message = Bft_Prepare_Message::new(1, 1, "sign", 11);
    let payload_result = json::encode(&prepareMsg);
    if payload_result.is_ok() {
        println!("payload {}", payload_result.unwrap());
    } else {
        println!("error {} ", payload_result.err().unwrap());
    }
    node.receivePrepare(prepareMsg);
}

#[test]
fn receiveCommit() {
    let mut node:Btf_Node = Btf_Node::start_node("", "7878");
    let commit_msg:Bft_Commit_Message = Bft_Commit_Message::new(1, 1, "public", 11);
    let payload_result = json::encode(&commit_msg);
    if payload_result.is_ok() {
        println!("payload {}", payload_result.unwrap());
    } else {
        println!("error {} ", payload_result.err().unwrap());
    }
    node.receiveCommit(commit_msg);
}

//#[test]
//fn threadPool() {
//    let threadpool = ThreadPool::new(2);
//
//    let mut count =0;
//
//    threadpool.execute(move|| {
//        count += 1;
//        println!("count:{}", count);
//    });
//
//}
