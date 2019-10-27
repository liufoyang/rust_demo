#![feature(proc_macro_hygiene, decl_macro)]
use std::sync::{Mutex,Arc};
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
    static ref NODEMUTEX: Arc<Mutex<Btf_Node>> = {

        let mut node:Btf_Node = Btf_Node::start_node("", "8000");
        let mut mutex: Arc<Mutex<Btf_Node>> = Arc::new(Mutex::new(node));
        return mutex;
    };
}

#[post("/receiveMsg", data = "<bft_msg>")]
fn receiveMsg(bft_msg:Json<Bft_Message>) -> &'static str {
    let mutex = Arc::clone(&NODEMUTEX);
    let mut node = mutex.lock().unwrap();
    node.receiveClientMsg(bft_msg.into_inner());
    return "receive msg";
}

#[post("/prePrepare", data = "<bft_msg>")]
fn prePrepare(bft_msg:Json<Bft_PrePrepare_Message>) -> &'static str {
    let mutex = Arc::clone(&NODEMUTEX);
    let mut node = mutex.lock().unwrap();
    node.doPrepare(bft_msg.into_inner());
    return "receive pre prepare msg";
}

#[post("/receivePrepare", data = "<bft_msg>")]
fn receivePrepare(bft_msg:Json<Bft_Prepare_Message>) -> &'static str {
    let mutex = Arc::clone(&NODEMUTEX);
    let mut node = mutex.lock().unwrap();
    node.receivePrepare(bft_msg.into_inner());
    return "receive prepare msg";
}

#[post("/receiveCommit", data = "<bft_msg>")]
fn receiveCommit(bft_msg:Json<Bft_Commit_Message>) -> &'static str {
    let mutex = Arc::clone(&NODEMUTEX);
    let mut node = mutex.lock().unwrap();
    node.receiveCommit(bft_msg.into_inner());
    return "receive commit msg";
}

fn main() {
    // start new node
    rocket::ignite().mount("/", routes![receiveMsg,prePrepare,receivePrepare,receiveCommit]).launch();

}