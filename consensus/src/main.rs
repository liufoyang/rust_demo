#![feature(proc_macro_hygiene, decl_macro)]
use std::sync::Mutex;
use std::rc::Rc;
mod lib;
use lib::bft_node::Btf_Node;
use lib::bft_message::{Bft_Prepare_Message, Bft_Message, Bft_Commit_Message, Bft_PrePrepare_Message, Bft_Replay};

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

static  mut bft_node_option:Mutex<Option<Btf_Node>> = Mutex::new(None);
fn main() {
    // start new node
    let mut node = Btf_Node::start_node("", "7878");
    rocket::ignite().mount("/", routes![receiveMsg,prePrepare,receivePrepare,receiveCommit]).manage(node).launch();
    unsafe {bft_node_option = Mutex::new(Some(node))}
}

#[post("/receiveMsg", data = "<bft_msg>")]
fn receiveMsg(mut state:State<Btf_Node>, bft_msg:Json<Bft_Message>) -> &'static str {
    state.receiveClientMsg(bft_msg.into_inner());
    return "receive msg";
}

#[post("/prePrepare", data = "<bft_msg>")]
fn prePrepare(mut state:State<Btf_Node>, bft_msg:Json<Bft_PrePrepare_Message>) -> &'static str {
    state.doPrepare(bft_msg.into_inner());
    return "receive pre prepare msg";
}

#[post("/receivePrepare", data = "<bft_msg>")]
fn receivePrepare(mut state:State<Btf_Node>, bft_msg:Json<Bft_Prepare_Message>) -> &'static str {
    state.receivePrepare(bft_msg.into_inner());
    return "receive prepare msg";
}

#[post("/receiveCommit", data = "<bft_msg>")]
fn receiveCommit(mut state:State<Btf_Node>, bft_msg:Json<Bft_Commit_Message>) -> &'static str {
    state.receiveCommit(bft_msg.into_inner());
    return "receive commit msg";
}