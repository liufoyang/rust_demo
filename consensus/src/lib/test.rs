use std::sync::Mutex;
use std::rc::Rc;
use crate::Btf_Node;
use crate::Bft_Message;
use crate::Bft_PrePrepare_Message;
use crate::Bft_Prepare_Message;
use crate::Bft_Commit_Message;

#[test]
fn bft_node_create() {
    let mut node:Btf_Node = Btf_Node::start_node("", "7878");
}

#[test]
fn receiveClientMsg() {
    let mut node:Btf_Node = Btf_Node::start_node("", "7878");
    let clientMsg = Bft_Message::new("hello world", "bft_client0001");
    node.receiveClientMsg(clientMsg);
}

#[test]
fn doPrepare() {
    let mut node:Btf_Node = Btf_Node::start_node("", "7878");
    let clientMsg = Bft_Message::new("hello world", "bft_client0001");
    let prePrepareMsg:Bft_PrePrepare_Message = Bft_PrePrepare_Message::new(1, 1, clientMsg);
    node.doPrepare(prePrepareMsg);
}

#[test]
fn receivePrepare() {
    let mut node:Btf_Node = Btf_Node::start_node("", "7878");
    let prepareMsg:Bft_Prepare_Message = Bft_Prepare_Message::new(1, 1, "sign", 11);

    node.receivePrepare(prepareMsg);
}

#[test]
fn receiveCommit() {
    let mut node:Btf_Node = Btf_Node::start_node("", "7878");
    let commit_msg:Bft_Commit_Message = Bft_Commit_Message::new(1, 1, "public", 11);
    node.receiveCommit(commit_msg);
}
