#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use bft_node::*;

fn main() {
    // start new node
    let node = btf_node::start_node("", 7878);
    rocket::ignite().mount("/", routes![index]).launch();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

