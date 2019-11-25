use super::bft_message::*;
use super::command_executor::Command_Executor;
//
use std::io::prelude::*;
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::u64;
use futures::{stream, Future};
use std::thread;
use std::sync::mpsc;
use std::time::SystemTime;
use std::time::Duration;
use super::communication;
extern crate rustc_serialize;
use rustc_serialize::json::{self, ToJson, Json};
use super::communication::{BftCommunication,BftCommunicationMsg};
use super::default_tcp_communication::Default_TCP_Communication;
use rustc_serialize::json::DecodeResult;


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Clone)]
pub struct Btf_Node_Simple {
    node_id:u64,
    pub address:String,
    pub port:String,
    pub public_key: String,
}

impl Btf_Node_Simple {
    pub fn get_node_id (&self) -> u64 {
        return self.node_id.clone();
    }

    pub fn new(_node_id:u64, _address:&str, _port:&str, _pub_key:&str) ->Btf_Node_Simple {
        let node = Btf_Node_Simple {
            node_id:_node_id,
            address:_address.to_string(),
            port:_port.to_string(),
            public_key: _pub_key.to_string(),
        };

        return node;

    }
}

pub struct Btf_Node{
    base:Btf_Node_Simple,
    status:String,
    view_num:u64,
    is_primary: bool,
    node_list: Vec<Btf_Node_Simple>,
    msg_cache:HashMap<u64, Bft_Message>,
    prepare_cache: HashMap<u64, Vec<Bft_Prepare_Message>>,
    commit_cache: HashMap<u64, Vec<Bft_Commit_Message>>,
    private_key: String,
    executor: Command_Executor,
}

impl Btf_Node {

    fn new(_view_num:u64, mut _node_list:Vec<Btf_Node_Simple>, _ip:&str, _port:&str,_node_id:u64, isPrimary:bool) -> Btf_Node{
        let bft_simple = Btf_Node_Simple{
            node_id:_node_id,
            address:_ip.to_string(),
            port: _port.to_string(),
            public_key:"".to_string()
        };

        // put self to the node list
        _node_list.push(bft_simple.clone());

        let mut command_executor = Command_Executor::new(2);
        let node = Btf_Node{
            base:bft_simple,
            status:"new".to_string(),
            view_num:_view_num,
            is_primary:isPrimary,
            node_list:_node_list,
            msg_cache:HashMap::new(),
            prepare_cache:HashMap::new(),
            commit_cache:HashMap::new(),
            private_key: "private".to_string(),
            executor:command_executor,

        };

        return node;
    }

    pub fn get_node_base(&self) -> &Btf_Node_Simple {
        return &(self.base);
    }

    pub fn receiveClientMsg(& mut self, msg:Bft_Message, executor:&mut Command_Executor) -> u64 {

        let view_num_temp:usize = 10;
        println!("begin process for");

        if self.is_primary {
            // is primary node send prepare
            let keys = self.msg_cache.keys();
            let mut num:u64 = 0;
            for key in keys {
                if &num <= key {
                    num = key + 1;
                }
            }

            // clone one msg save in self node;
            self.msg_cache.insert(num.clone(), msg.clone());
            let prePrepareMsg:Bft_PrePrepare_Message = Bft_PrePrepare_Message::new(self.view_num.clone(), num, msg);
            let payload = json::encode(&prePrepareMsg).unwrap();

            //self.executor.execute(payload.as_str());
            self.broadcastMsg(payload, "prePrepare");

            println!("process for primary");
            return num;

        } else {
            return 0;
        }
    }
    pub fn doPrepare(& mut self,  msg:Bft_PrePrepare_Message) -> (u64, u64){

        println!("begin doPrepare for");

        // check is the primary message, check the digest by primary pub key;
        if msg.get_view_num() != self.view_num {
            println!("doPrepare view num not same {}, {}", msg.get_view_num(), self.view_num);
            return (msg.get_view_num(), msg.get_sequence_num());
        }

        // check if have before,if not put msg to msg_cache
        //let mut source_msg_option:Option<&Bft_Message> = Option::None;
        if self.msg_cache.contains_key(& msg.get_sequence_num()) {
            println!("doPrepare have recevie the sequence num ");
            // have receive this msg num before, check if the same msg
            let receive_msg = self.msg_cache.get(& msg.get_sequence_num()).unwrap();
            if receive_msg.get_id() == msg.get_client_msg().get_id(){
                // the same
                //source_msg_option = Some(receive_msg);

                // find the pre cache
                let mut has_send= false;
                if self.prepare_cache.contains_key(& msg.get_sequence_num()) {
                    for prepare_msg in self.prepare_cache.get(& msg.get_sequence_num()).unwrap() {
                        if prepare_msg.get_node_id() == self.get_node_base().node_id {
                            // broadcast again
                            let payload = json::encode(&prepare_msg).unwrap();
                            self.broadcastMsg(payload, "prepare");
                            has_send = true;
                            break;
                        }
                    }
                }

                if !has_send {
                    let digest = & self.get_node_base().public_key;

                    // check pass add to prepare cache;
                    let prepare_msg = Bft_Prepare_Message::new(self.view_num, msg.get_sequence_num(), digest.as_str(), self.get_node_base().node_id);
                    let seq_num = msg.get_sequence_num();
                    if self.prepare_cache.contains_key(& seq_num) {
                        let list = self.prepare_cache.get_mut(& seq_num).unwrap();
                        list.push(prepare_msg.clone());
                    } else {
                        let mut prepare_vec = Vec::new();
                        prepare_vec.push(prepare_msg.clone());
                        self.prepare_cache.insert(seq_num, prepare_vec);
                    }

                    // send the prepare msg
                    let payload = json::encode(&prepare_msg).unwrap();
                    self.broadcastMsg(payload, "prepare");
                }

                let mut receive_msg2 = self.msg_cache.get_mut(& msg.get_sequence_num()).unwrap();
                receive_msg2.set_status(2);


            } else {
                // not same msg for the same num

            }
            return (msg.get_view_num(), msg.get_sequence_num());

        } else {
            println!("doPrepare new  sequence num msg");
            let mut client_msg = msg.get_client_msg().clone();
            client_msg.set_status(2);
            self.msg_cache.insert(msg.get_sequence_num(),  client_msg);
            //source_msg_option = Some(& msg.get_client_msg());

            // do digest by this node
            let digest = & self.get_node_base().public_key;
            // new the prepare msg for this node

            // check pass add to prepare cache;
            let prepare_msg = Bft_Prepare_Message::new(self.view_num, msg.get_sequence_num(), digest.as_str(), self.get_node_base().node_id);
            let seq_num = msg.get_sequence_num();
            if self.prepare_cache.contains_key(& seq_num) {
                let list = self.prepare_cache.get_mut(& seq_num).unwrap();
                list.push(prepare_msg.clone());
            } else {
                let mut prepare_vec = Vec::new();
                prepare_vec.push(prepare_msg.clone());
                self.prepare_cache.insert(seq_num, prepare_vec);
            }

            // send the prepare msg
            let payload = json::encode(&prepare_msg).unwrap();
            self.broadcastMsg(payload, "prepare");

            return (msg.get_view_num(), msg.get_sequence_num());
        }
    }

    /// receivePrepare msg from other node,
    /// check msg is valid, if yes ,put to precache list
    ///
    pub fn receivePrepare(&mut self, msg:Bft_Prepare_Message, mut executor:&mut Command_Executor) {

        println!("bein receivePrepare");
        if msg.get_view_num() != self.view_num {
            println!("receivePrepare view num not same {}, {}", msg.get_view_num(), self.view_num);
            return;
        }

        // check desigt
        // check the desigt
        let mut node_option:Option<Btf_Node_Simple>  = Option::None;
        for simple in & self.node_list {
            if simple.node_id == msg.get_node_id() {
                node_option = Some(simple.clone());
            }
        }

        // not know node, not process its prepare,
        if node_option.is_none() {
            println!("receivePrepare no illge node");
            return;
        }

        let simple_node = node_option.unwrap();

        // check design fail
//        if simple_node.public_key.as_str() != msg.get_msg_digest() {
//            return;
//        }

        // check pass put the prepare msg to cache
        let seq_num = msg.get_sequence_num();
        if self.prepare_cache.contains_key(& seq_num) {
            let list = self.prepare_cache.get_mut(& seq_num).unwrap();
            list.push(msg.clone());
        } else {
            let mut prepare_vec = Vec::new();
            prepare_vec.push(msg.clone());
            self.prepare_cache.insert(seq_num, prepare_vec);
        }

        // check if need to do commit;
        let has_commit =  self.checkIfCommit(&msg.get_sequence_num());


        if has_commit && self.msg_cache.contains_key(&msg.get_sequence_num()) {

            // new commit msg and broadcast the msg _view_num:u32, _sequence_num:u32, _digest:HashValue, _node_id:u32
            let commit_msg:Bft_Commit_Message = Bft_Commit_Message::new(self.view_num, seq_num.clone(), self.get_node_base().public_key.as_str(), self.get_node_base().node_id);


            println!("begin to commit {}", msg.get_sequence_num());
            // put msg to log file
            if self.commit_cache.contains_key(&seq_num) {
                let list = self.commit_cache.get_mut(&seq_num).unwrap();
                list.push(commit_msg.clone());
            } else {
                let mut commit_msg_list = Vec::new();
                commit_msg_list.push(commit_msg.clone());
                self.commit_cache.insert(seq_num.clone(), commit_msg_list);
            }

            let mut receive_msg = self.msg_cache.get_mut(& msg.get_sequence_num()).unwrap();

            if receive_msg.get_status() != 3 {
                // do commit
                let mut logs_str = String::from("commit ");
                logs_str.push_str(seq_num.to_string().as_str());
                logs_str.push_str(" ");
                logs_str.push_str(receive_msg.get_payload());
                executor.savelog(logs_str.as_str());

                // do command
                let payload = json::encode(&commit_msg).unwrap();
                receive_msg.set_status(3);

                let sourcePayLoad = receive_msg.get_payload();
                println!("commit msg {}", payload);
                // broadcast the msg to other
                self.broadcastMsg(payload, "commit");
            }


        }

    }

    fn checkIfCommit(& mut self, _sequence_num:& u64) ->bool {
        if !self.prepare_cache.contains_key(_sequence_num) {
            return false;
        }
        let min_pass_count = self.node_list.len()*2/3 + 1;
        let prepare_msg_list_option = self.prepare_cache.get(_sequence_num);
        if prepare_msg_list_option.is_none() {
            return false;
        }

        let prepare_list = prepare_msg_list_option.unwrap();

        // have enough prepare msg
        println!("enough prepare {} {}", prepare_list.len(), min_pass_count);
        return prepare_list.len()>= min_pass_count;
    }

    pub fn receiveCommit(& mut self, msg:Bft_Commit_Message) {

        let mut source_msg_option:Option<Bft_Message> = None;
        if self.msg_cache.contains_key(& msg.get_sequence_num()) {
            // have receive this msg num before, check if the same msg
            let receive_msg = self.msg_cache.get(& msg.get_sequence_num()).unwrap();
            source_msg_option = Some(receive_msg.clone());
        }

        // have receive pre prepare msg in this node but not same msg, return error;
        if  source_msg_option.is_some() {
            let source_msg = source_msg_option.unwrap();
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
        if simple_node.public_key.as_str() != msg.get_msg_digest() {
            return;
        }

        // check pass add to prepare cache;
        if self.commit_cache.contains_key(&msg.get_sequence_num()) {
            let list = self.commit_cache.get_mut(&msg.get_sequence_num()).unwrap();
            list.push(msg.clone());
        } else {
            let mut commit_msg_list = Vec::new();
            commit_msg_list.push(msg.clone());
            self.commit_cache.insert(msg.get_sequence_num(), commit_msg_list);
        }

        self.doReplay(msg.get_sequence_num());

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
        let min_pass_count = self.node_list.len()/3 + 1;
        let commit_msg_list = self.commit_cache.get(&_sequence_num).unwrap();
        println!("enough commit {} {}", commit_msg_list.len(), min_pass_count);
        if commit_msg_list.len()>= min_pass_count {
            // new replay msg and send to client _view_num:u32, _payload: &str, _node_id:&str, _source_msg:Bft_Message
            let replay_msg:Bft_Replay = Bft_Replay::new(self.view_num, "succes process", self.get_node_base().node_id.clone(), msg.clone());
            let payload = json::encode(&replay_msg).unwrap();
            self.broadcastMsg(payload, "replay");
        }
    }

    /// start new node, connect the bft network
    pub fn start_node(_primary_address:&str, _primary_port: &str, _ip:&str, _port:&str) -> Btf_Node{

        // send request for primary
        let mut node_isntance:Btf_Node;
        let mut simple_vec:Vec<Btf_Node_Simple> = Vec::new();
        let mut _view_num:u64 = 0;
        let mut _node_id:u64 = 1;
        if _primary_address.len() > 0 {
            // the bft network primary not null, is not the first node,send init msg to
            let regist_msg = Bft_Regist_Msg::new(_ip, _port, "pubKey" );
            let payload = json::encode(&regist_msg).unwrap();

            let send_result = Btf_Node::sendToPrimaryMsg(payload,"regist", _primary_address,_primary_port);
            if send_result.is_ok() {
                let mut result_str = send_result.unwrap();
                //result_str = "{\"node_list\":[{\"node_id\":1,\"address\":\"10.3.209.223\",\"port\":\"8780\",\"public_key\":\"\"}],\"view_num\":1,\"check_point_num\":0,\"node_id\":2}\n".to_string();//result_str.trim().to_string();
                result_str = result_str.trim().to_string();


                let node_msg_result:DecodeResult<Bft_Regist_Reply> = json::decode(&result_str);
                if node_msg_result.is_ok() {
                    let reply_msg = node_msg_result.unwrap();
                    _view_num = reply_msg.get_view_num();
                    simple_vec = reply_msg.get_node_ist();
                    _node_id = reply_msg.get_node_id();

                } else {
                    println!("regist reply msg error {} {}", node_msg_result.err().unwrap(), result_str);
                }
            }

            node_isntance = Btf_Node::new(_view_num, simple_vec, _ip, _port,_node_id, false);


        } else {
            // 没有其他节点，这个就是第一个节点，第一个视图
            let port = _port;
            let view_num = 1;
            let node_list = Vec::new();
            let ip = _ip;
            let node_id = 1;
            node_isntance = Btf_Node::new(view_num, node_list, ip, port,node_id, true);

        }

        return node_isntance;
    }

    pub fn handler_expire(&mut self, seq_num:u64) -> Option<Bft_View_Change_Message> {

        if !self.msg_cache.contains_key(&seq_num) {
            return Option::None;
        }

        let mut receive_msg = self.msg_cache.get_mut(&seq_num).unwrap();
        if receive_msg.get_status() != 1 {
            return Option::None;
        }

        let view_change_msg = Bft_View_Change_Message::new(self.view_num.clone(), seq_num, 0, "signed", self.get_node_base().get_node_id());
        let payload = json::encode(&view_change_msg).unwrap();
        self.broadcastMsg(payload, "viewchange");
        return Some(view_change_msg);

    }

    /// send message to all other node
    ///
    fn broadcastMsg (&self, data:String , command:&str) {

        println!("bengin to broadcase {}", self.node_list.len());
        let payload_str = data;
        for node in &(self.node_list) {

            // not send to self
            if node.node_id == self.get_node_base().node_id {
                continue;
            }
//            //build BftCommunicationMsg
            let communication_msg = BftCommunicationMsg{
                command:command.to_string(),
                version:"v0.1".to_string(),
                payload:payload_str.to_string()
            };

            Default_TCP_Communication::sendMessage(node.address.as_str(), node.port.as_str(), communication_msg);
            println!("send to node {}", node.address.as_str());

        }


    }

    fn sendToPrimaryMsg (data:String , command:&str, _primary_addr:&str, _port:&str) ->std::result::Result<String, &'static str>{

        println!("bengin to broadcase");
        let payload_str = data;
        let communication_msg = BftCommunicationMsg{
            command:command.to_string(),
            version:"v0.1".to_string(),
            payload:payload_str.to_string()
        };

        let send_result = Default_TCP_Communication::sendMessageWithReply(_primary_addr, _port, communication_msg);
        println!("send to primary node {}", command);

        return send_result;

    }

    pub fn regist_node(&mut self, msg:Bft_Regist_Msg) -> Bft_Regist_Reply {

        let mut node_list:Vec<Btf_Node_Simple> = Vec::new();

        let mut node_id:u64 = 0;
        for node in & self.node_list {
            if node_id < node.node_id {
                node_id = node.node_id.clone();
            }
            node_list.push(node.clone());
        }
        node_id +=1;
        let reply = Bft_Regist_Reply::new(node_list, self.view_num.clone(), 0, node_id.clone());

        // push the new slave node to list
        let bft_slave = Btf_Node_Simple{
            node_id:node_id,
            address:msg.address,
            port: msg.port,
            public_key:msg.public_key
        };

        // broadcase to node list  new regist node
        let payload = json::encode(&bft_slave).unwrap();
        self.broadcastMsg(payload, "newnode");

        self.node_list.push(bft_slave);
        return reply;
    }

    pub fn receive_new_node(&mut self, new_node:Btf_Node_Simple) {

        if self.is_primary {
            return;
        }
        println!("have a new node {}", new_node.node_id);
        self.node_list.push(new_node);
    }


}