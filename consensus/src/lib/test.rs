use std::sync::Mutex;
use std::rc::Rc;
use crate::Btf_Node_Simple;
use crate::Btf_Node;
use crate::Bft_Message;
use crate::Bft_Regist_Reply;
use crate::Bft_PrePrepare_Message;
use crate::Bft_Prepare_Message;
use crate::Bft_Commit_Message;
use crate::Command_Executor;
use crate::ThreadPool;
extern crate rustc_serialize;
use rustc_serialize::json::DecodeResult;
use rustc_serialize::json::{self, ToJson, Json};

#[test]
fn bft_node_create() {
    let mut node:Btf_Node = Btf_Node::start_node("", "8787", "10.3.209.223", "8787");
}

#[test]
fn receiveClientMsg() {
    let mut node:Btf_Node = Btf_Node::start_node("", "8787", "10.3.209.223", "8787");
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

    let mut threadpool = Command_Executor::new(2);
    node.receiveClientMsg(clientMsg, &mut threadpool);
}

#[test]
fn doPrepare() {
    let mut node:Btf_Node = Btf_Node::start_node("", "8787", "10.3.209.223", "8787");
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
    let mut node:Btf_Node = Btf_Node::start_node("", "8787", "10.3.209.223", "8787");
    let prepareMsg:Bft_Prepare_Message = Bft_Prepare_Message::new(1, 1, "sign", 11);
    let payload_result = json::encode(&prepareMsg);
    if payload_result.is_ok() {
        println!("payload {}", payload_result.unwrap());
    } else {
        println!("error {} ", payload_result.err().unwrap());
    }
    let mut threadpool = Command_Executor::new(2);
    node.receivePrepare(prepareMsg, &mut threadpool);
}

#[test]
fn receiveCommit() {
    let mut node:Btf_Node = Btf_Node::start_node("", "8787", "10.3.209.223", "8787");
    let commit_msg:Bft_Commit_Message = Bft_Commit_Message::new(1, 1, "public", 11);
    let payload_result = json::encode(&commit_msg);
    if payload_result.is_ok() {
        println!("payload {}", payload_result.unwrap());
    } else {
        println!("error {} ", payload_result.err().unwrap());
    }
    node.receiveCommit(commit_msg);
}

#[test]
fn jsonChange() {
    let mut node_list:Vec<Btf_Node_Simple> = vec![];
    let simple_node = Btf_Node_Simple::new(1, "10.3.209.223","8780", "");
    node_list.push(simple_node);
    let replay_msg = Bft_Regist_Reply::new(node_list, 1, 0, 2);
    let payload_result = json::encode(&replay_msg);
    if payload_result.is_ok() {
        let payload = payload_result.unwrap();
        let node_msg_result:DecodeResult<Bft_Regist_Reply> = json::decode(&payload);

        println!("payload {}", payload);
        if !node_msg_result.is_ok() {
            println!("regist reply msg error {} {}", node_msg_result.err().unwrap(), payload);
        }
    }
}

#[test]
fn threadPool() {
    let threadpool = ThreadPool::new(2);

    let mut count =0;

    threadpool.execute(move|| {
        count += 1;
        println!("count:{}", count);
    });

}


//#[test]
//fn command_executor() {
//    let mut threadpool = Command_Executor::new(2);
//
//    let mut count =0;
//
//    threadpool.execute("just test for bft message");
//
//}
