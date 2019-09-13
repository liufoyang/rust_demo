use bft_message::*;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
extern crate serde;
extern crate serde_json;
use serde_json;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use reqwest;

#[derive(Serialize, Deserialize)]
pub struct Btf_Node_Simple {
    node_id:u64,
    pub address:String,
    pub port:String,
    pub public_key: String,
}

pub struct Btf_Node{
    base:bft_simple,
    status:String,
    view_num:u64,
    node_list:Vec<Btf_Node_Simple>,
    msg_cache:HashMap<u64, Bft_Message>,
    prepare_cache: HashMap<u64, Vec<Bft_Prepare_Message>>,
    commit_cache: HashMap<u64, Vec<Bft_Commit_Message>>,
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
            private_key: "private".to_string(),
        };

        return node;
    }

    pub fn get_node_base(&self) -> &Btf_Node_Simple {
        return &(self.base);
    }

    pub fn doPrepare(& mut self, msg:Bft_PrePrepare_Message) {
        // check is the primary message, check the digest by primary pub key;
        if msg.get_view_num() != self.view_num {
            return;
        }

        // check if have before,if not put msg to msg_cache
        let mut source_msg_option:Option<Bft_Message> = Option::None;
        if self.msg_cache.contains_key(& msg.get_sequence_num()) {
            // have receive this msg num before, check if the same msg
            let receive_msg = self.msg_cache.get(& msg.get_sequence_num()).unwrap();
            if receive_msg.get_id() == msg.get_client_msg().get_id(){
                // the same
                source_msg_option = Some(receive_msg.clone());

                // find the pre cache
                for prepare_msg in self.prepare_cache.get(& msg.get_sequence_num()).unwrap() {
                    if prepare_msg.get_node_id() == self.get_node_base().node_id {
                        // broadcast again
                        broadcastMsg(prepare_msg, "prepare");
                        break;
                    }
                }

            } else {
                // not same msg for the same num
                return;
            }


        } else {
            self.msg_cache.insert(msg.get_view_num(),  msg.get_client_msg().clone());
            source_msg_option = Some(msg.get_client_msg().clone());

            // do digest by this node
            let digest = & self.get_node_base().public_key;
            // new the prepare msg for this node

            // check pass add to prepare cache;
            let prepare_msg = Bft_Prepare_Message::new(self.view_num, msg.get_sequence_num(), digest.as_str(), self.get_node_base().node_id);
            if self.prepare_cache.contains_key(& msg.get_sequence_num()) {
                self.prepare_cache.get(& msg.get_sequence_num()).unwrap().push(prepare_msg);
            } else {
                let mut prepare_vec = Vec::new();
                prepare_vec.push(prepare_msg);
                self.prepare_cache.insert(msg.get_sequence_num().clone(), prepare_vec);
            }

            // send the prepare msg
            broadcastMsg(&prepare_msg, "prepare");
        }
    }

    /// receivePrepare msg from other node,
    /// check msg is valid, if yes ,put to precache list
    ///
    pub fn receivePrepare(&mut self, msg:Bft_Prepare_Message) {

        if msg.get_view_num() != self.view_num {
            return;
        }

        // check desigt
        let client_source_msg = msg.get_client_msg();
        let mut source_msg_option:Option<Bft_Message> = Option::None;

        if self.msg_cache.contains_key(& msg.get_sequence_num()) {
            // have receive this msg num before, check if the same msg
            let receive_msg = self.msg_cache.get(& msg.get_sequence_num()).unwrap();
            if receive_msg.get_id() == msg.get_client_msg().get_id(){
                // the same
                source_msg_option = Some(receive_msg.clone());
            }
        } else {
            self.msg_cache.insert(msg.get_view_num(),  msg.get_client_msg().clone());
            source_msg_option = Some(msg.get_client_msg().clone());
        }

        // have receive pre prepare msg in this node but not same msg, return error;
        if  source_msg_option.is_some()
            && source_msg_option.unwrap().get_id() != client_source_msg.get_id() {
            return;
        }
        // check the desigt
        let mut node_option:Option<Btf_Node_Simple>  = Option::None;
        for simple in & self.node_list {
            if simple.node_id == msg.get_node_id() {
                node_option = Some(simple.clone());
            }
        }

        // not know node, not process its prepare,
        if node_option.is_none() {
            return;
        }

        let simple_node = node_option.unwrap();

        // check design fail
        if simple_node.public_key != msg.get_public_key() {
            return;
        }

        // check pass put the prepare msg to cache
        if self.prepare_cache.contains_key(& msg.get_sequence_num()) {
            self.prepare_cache.get(& msg.get_sequence_num()).unwrap().push(msg);
        } else {
            let mut prepare_vec = Vec::new();
            prepare_vec.push(msg);
            self.prepare_cache.insert(msg.get_sequence_num().clone(), prepare_vec);
        }

        // check if need to do commit;
        checkIfCommit(&msg.get_sequence_num());

    }

    fn checkIfCommit(&self, _sequence_num:& u64) {
        if self.prepare_cache.len() <= 0 {
            return;
        }
        let min_pass_count = self.node_list.len()*2/3 + 1;
        let prepare_msg_list_option = self.prepare_cache.get(_sequence_num);
        if prepare_msg_list_option.is_none() {
            return
        }

        let prepare_list = prepare_msg_list_option.unwrap();

        // have enough prepare msg
        if prepare_list.len()>= min_pass_count {
            // new commit msg and broadcast the msg _view_num:u32, _sequence_num:u32, _digest:HashValue, _node_id:u32
            let commit_msg:Bft_Commit_Message = Bft_Commit_Message::new(self.view_num, _sequence_num.clone(), self.get_node_base().public_key.as_str(), self.get_node_base().node_id);



            // put msg to log file
            if self.commit_cache.contains_key(_sequence_num) {
                self.commit_cache.get(_sequence_num).unwrap().push(commit_msg);
            } else {
                let mut commit_msg_list = Vec::new();
                commit_msg_list.push(commit_msg);
            }

            // broadcast the msg to other
            broadcastMsg(&commit_msg, "commit");
        }
    }

    pub fn receiveCommit(& mut self, msg:Bft_Commit_Message) {

        if msg.get_view_num() != self.view_num {
            return;
        }

        if self.msg_cache.contains_key(& msg.get_sequence_num()) {
            // have receive this msg num before, check if the same msg
            let receive_msg = self.msg_cache.get(& msg.get_sequence_num()).unwrap();
            if receive_msg.get_id() == msg.get_client_msg().get_id(){
                // the same
                source_msg_option = Some(receive_msg.clone());
            }
        } else {
            self.msg_cache.insert(msg.get_view_num(),  msg.get_client_msg().clone());
            source_msg_option = Some(msg.get_client_msg().clone());
        }

        // have receive pre prepare msg in this node but not same msg, return error;
        if  source_msg_option.is_some()
            && source_msg_option.unwrap().get_id() != client_source_msg.get_id() {
            return;
        }

        let source_msg = source_msg_option.upwrap();

        // check the desigt
        // check the desigt
        let mut node_option:Option<Btf_Node_Simple>  = Option::None;
        for simple in & self.node_list {
            if simple.node_id == msg.get_node_id() {
                node_option = Some(simple.clone());
            }
        }

        // not know node, not process its prepare,
        if node_option.is_none() {
            return;
        }

        let simple_node = node_option.unwrap();

        // check design fail
        if simple_node.public_key != msg.get_public_key() {
            return;
        }

        // check pass add to prepare cache;
        if self.commit_cache.contains_key(&msg.get_sequence_num()) {
            self.commit_cache.get(&msg.get_sequence_num()).unwrap().push(msg);
        } else {
            let mut commit_msg_list = Vec::new();
            commit_msg_list.push(msg);
            self.commit_cache.insert(msg.get_sequence_num(), commit_msg_list);
        }

        doReplay(msg.get_sequence_num());

    }

    pub fn doReplay(&self, _sequence_num:u64) {
        if !self.commit_cache.contains_key(&_sequence_num) {
            return;
        }

        // never receive primary node pre prepare, not replay
        if !self.msg_cache.contains_key(&_sequence_num) {
            return;
        }
        let msg = self.msg_cache.get(&_sequence_num).unwrap();
        /// commit mes count > 2f+1 then pass and view not change commit local;
        ///  commit mes count > f+1 then pass and view have changed commit at this node view;
        let min_pass_count = self.node_list.le()/3 + 1;
        let commit_msg_list = self.commit_cache.get(&_sequence_num).unwrap();
        if commit_msg_list.len()>= min_pass_count {
            // new replay msg and send to client _view_num:u32, _payload: &str, _node_id:&str, _source_msg:Bft_Message
            let replay_msg:Bft_Replay = Bft_Replay::new(self.view_num, "succes process", self.get_node_base().node_id.clone(), msg.clone());
            broadcastMsg(replay_msg, "replay");
        }
    }

    /// start new node, connect the bft network
    pub fn start_node(_address:&str, _port:u64) -> Btf_Node{

        // send request for primary
        let node_isntance:Btf_Node;
        if _address.len() > 0 {
            // the bft network primary not null, is not the first node,send init msg to
            let url = String::from("https://").push_str(_address + ":" + _port);
            let body = reqwest::get(url)?
                .text()?;

            // body is {view_num:num, node_num:xxx, node_list:[{node_num:xxx, address:xxx, port:xxx, public_key:xxx}]};
            let node_value::Value = serde_json::from_str(body)?;

            let view_num = node_value["view_num"].as_u64().unwrap();
            let node_num = node_value["node_num"].as_u64().unwrap();
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
            let ip = "127.0.0.1";
            node_isntance = Btf_Node::new(view_num, simple_vec, ip, "8087",node_num);

            return node_isntance;
        } else {
            // 没有其他节点，这个就是第一个节点，第一个视图
            let port = _port;
            let view_num = 1;
            let node_list = Vec::new();
            let ip = "127.0.0.1";
            let node_id = 1;
            node_isntance = Btf_Node::new(view_num, simple_vec, ip, "8087",node_num);

            return node_isntance;

        }

        // broadcast
        let listener = TcpListener::bind("127.0.0.1" + _port).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream);
        }

        let node = Btf_Node::new(view_num, node_list, ip, port,node_id);
    }

    /// send message to all other node
    ///
    fn broadcastMsg<T: Serialize >(&self, data:& T, command:&str) {

        for node in &(self.node_list) {
            let mut url = String::from("https://").push_str(_address + ":" + _port);
            url.push("/" + command);

            let mut res = reqwest::Client::new()
                .post(url.to_str())
                .json(data)
                .send()
                .unwrap();
            if res.status() !=200 {

            }
        }
    }




}