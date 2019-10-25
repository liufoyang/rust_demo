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

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref NODEMUTEX: Mutex<Btf_Node> = {

        let mut node:Btf_Node = Btf_Node::start_node("", "7878");
        let mut mutex: Mutex<Btf_Node> = Mutex::new(node);
        return mutex;
    };
}

#[post("/receiveMsg", data = "<bft_msg>")]
fn receiveMsg(bft_msg:Json<Bft_Message>) -> &'static str {
    let mut node = NODEMUTEX.lock().unwrap();
    node.receiveClientMsg(bft_msg.into_inner());
    return "receive msg";
}

#[post("/prePrepare", data = "<bft_msg>")]
fn prePrepare(bft_msg:Json<Bft_PrePrepare_Message>) -> &'static str {
    let mut node = NODEMUTEX.lock().unwrap();
    node.doPrepare(bft_msg.into_inner());
    return "receive pre prepare msg";
}

#[post("/receivePrepare", data = "<bft_msg>")]
fn receivePrepare(bft_msg:Json<Bft_Prepare_Message>) -> &'static str {
    let mut node = NODEMUTEX.lock().unwrap();
    node.receivePrepare(bft_msg.into_inner());
    return "receive prepare msg";
}

#[post("/receiveCommit", data = "<bft_msg>")]
fn receiveCommit(bft_msg:Json<Bft_Commit_Message>) -> &'static str {
    let mut node = NODEMUTEX.lock().unwrap();
    node.receiveCommit(bft_msg.into_inner());
    return "receive commit msg";
}

fn main() {
    // start new node
    let mut node = NODEMUTEX.lock().unwrap();
    rocket::ignite().mount("/", routes![receiveMsg,prePrepare,receivePrepare,receiveCommit]).launch();

}