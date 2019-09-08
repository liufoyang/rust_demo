use bft_message::*;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
extern crate serde;
extern crate serde_json;
use serde_json;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use reqwest;

#[derive(Serialize, Deserialize)]
pub struct Btf_Node_Simple {
    node_id:u64,
    address:String,
    port:String,
    public_key: String,
}

pub struct Btf_Node{
    base:bft_simple,
    status:String,
    view_num:u64,
    node_list:Vec<Btf_Node_Simple>,
    msg_cache:Vec<(u64, Bft_Message)>,
    prepare_cache: Vec<(u64, Vec<Bft_Prepare_Message>)>,
    commit_cache: Vec<(u64, Vec<Bft_Commit_Message>)>,
    private_key: String,
}

impl Btf_Node {

    fn new(_view_num:u64, _node_list:Vec<Btf_Node_Simple>, _ip:&str, _port:&str,_node_id:u64) -> Btf_Node{
        let bft_simple = Btf_Node_Simple{
            node_id:_node_id,
            address:_ip.to_string(),
            port: _port.to_string(),
            public_key:"".to_string()
        };
        let node = Btf_Node{
            base:bft_simple,
            status:"new".to_string(),
            view_num:_view_num,
            node_list:_node_list,
            msg_cache:Vec::new(),
            prepare_cache:Vec::new(),
            commit_cache:Vec::new(),
            private_key: Ed25519PrivateKey,
        };

        return node;
    }

    fn doPrepare(&self, msg:Bft_PrePrepare_Message) {
        // check is the primary message, check the digest by primary pub key;

        // do digest by this node
        let digest;
        // new the prepare msg for this node

        let prepaireMsg = Bft_Prepare_Message::new(self.view_num, msg.get_sequence_num(), digest, self.node_id );

        // broadcast the msg to other
    }

    fn doCommit(&self, _sequence_num:u64) {
        if self.prepare_cache.len() <= 0 {
            return;
        }
        let min_pass_count = self.node_list.le()*2/3 + 1;
        for prepare_item(num, prepare_msg_list) in self.prepare_cache {
            if num!= _sequence_num {
                continue;
            }

            if prepare_msg_list.len()>= min_pass_count {
                // new commit msg and broadcast the msg
            }
        }
    }


    fn doReplay(&self, _sequence_num:u64) {
        if self.commit_cache.len() <= 0 {
            return;
        }
        /// commit mes count > 2f+1 then pass and view not change commit local;
        ///  commit mes count > f+1 then pass and view have changed commit at this node view;
        let min_pass_count = self.node_list.le()/3 + 1;
        for prepare_item(num, prepare_msg_list) in self.commit_cache {
            if num!= _sequence_num {
                continue;
            }

            if prepare_msg_list.len()>= min_pass_count {
                // new replay msg and send to client
            }
        }
    }

    /// start new node, connect the bft network
    pub fn start_node(_address:&str, _port:u64) -> Btf_Node{

        // send request for primary
        if _address.len() > 0 {
            let url = String::from("https://").push_str(_address + ":" + _port);
            let body = reqwest::get(url)?
                .text()?;

            // body is {view_num:num, node_num:xxx, node_list:[{node_num:xxx, address:xxx, port:xxx, public_key:xxx}]};
            let node_value::Value = serde_json::from_str(body)?;

            let mut simple_vec:Vec<Btf_Node_Simple> = Vec::new();
            let mut simple_list:Vec<Value> = node_value["node_list"];
            for one_simple in & simple_list {
                let simple = Btf_Node_Simple{
                    node_id: one_simple["node_num"].as_u64().unwrap(),
                    address:one_simple["address"].as_str().to_string(),
                    port:one_simple["port"].as_str().to_string(),
                    public_key: one_simple["public_key"].as_str().to_string()
                };
                simple_vec.push(simple);
            }

        }
        let port = _port;
        let view_num = 1;
        let node_list = Vec::new();
        let ip = "127.0.0.1";
        let node_id = 1;
        if _address.len()>0 {
            // the bft network primary not null, is not the first node,send init msg to

        } else {

        }

        // broadcast
        let listener = TcpListener::bind("127.0.0.1" + _port).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream);
        }

        let node = Btf_Node::new(view_num, node_list, ip, port,node_id);
    }


}