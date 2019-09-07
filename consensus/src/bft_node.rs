use bft_message::*;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

pub struct btf_node{
    node_id:u32,
    status:String,
    view_num:u32,
    node_list:Vec<(String, String, u32, Ed25519PublicKey)>,
    msg_cache:Vec<(u32, Bft_Message)>,
    prepare_cache: Vec<(u32, Vec<Bft_Prepare_Message>)>,
    commit_cache: Vec<(u32, Vec<Bft_Commit_Message>)>,
    private_key: Ed25519PrivateKey,
    public_key: Ed25519PublicKey,
    ip:String,
    port:u32
}

impl btf_node {

    fn new(_view_num:u32, _node_list:Vec<(String, String, u32)>, _ip:&str, _port:u32,_node_id:u32) -> btf_node{
        btf_node{
            node_id:_node_id,
            status:"new".to_string(),
            view_num:_view_num,
            node_list:_node_list,
            msg_cache:Vec::new(),
            prepare_cache:Vec::new(),
            commit_cache:Vec::new(),
            private_key: Ed25519PrivateKey,
            public_key: Ed25519PublicKey,
            ip:_ip.to_string(),
            port:_port
        }
    }

    fn doPrepare(&self, msg:Bft_PrePrepare_Message) {
        // check is the primary message, check the digest by primary pub key;

        // do digest by this node
        let digest;
        // new the prepare msg for this node

        let prepaireMsg = Bft_Prepare_Message::new(self.view_num, msg.sequence_num, digest, self.node_id );

        // broadcast the msg to other
    }

    fn doCommit(&self, _sequence_num:u32) {
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


    fn doReplay(&self, _sequence_num:u32) {
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
    pub fn start_node(_address:&str, _port:u32) -> btf_node{

        //
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

        let node = btf_node::new(view_num, node_list, ip, port,node_id);
    }


}