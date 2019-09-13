/// client source msg to bft node
/// the primiry receive the node and begin to prepare phase
extern crate serde;
extern crate serde_json;
use serde_json;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Bft_Message {
    id:String,
    client_id:String,
    payload:String,
    timestamp:u64
}

impl Bft_Message {
    pub fn new(_payload: &str, _client_id:&str) ->Bft_Message{
        let msg =  Bft_Message{
            payload:_payload.to_string(),
            client_id: _client_id.to_string(),
            id: "1232345".to_string(),
            timestamp:100000
        };
        return msg;
    }

    pub fn sign(&self) -> HashValue{
        source_msg_bytes = [];
        let digest = Bft_Message_Bytes(source_msg_bytes).hash();
        return digest;
    }

    pub fn get_id(&self) -> &str {
        return self.id.as_str()
    }
}

///  use to contain the byte[] of Bft_Message
///  do the hash for Bft_Message

pub struct Bft_Message_Bytes<'a>(pub &'a [u8]);

impl<'a> CryptoHash for Bft_Message_Bytes<'a> {
    type Hasher = RawTransactionHasher;

    fn hash(&self) -> HashValue {
        let mut state = Self::Hasher::default();
        state.write(self.0);
        state.finish()
    }
}

/// the replay to client from Bft node
///  every Bft node non fault will send the repay;
#[derive(Serialize, Deserialize)]
pub struct Bft_Replay{
    view_num:u64,
    timestamp:u64,
    client_id:String,
    node_id:u64,
    payload:String
}

impl Bft_Replay {
    pub fn new(_view_num:u64, _payload: &str, _node_id:u64, _source_msg:Bft_Message)->Bft_Replay {
        let replay = Bft_Replay {
            view_num:_view_num,
            timestamp:_source_msg.timestamp,
            client_id:_source_msg.client_id,
            node_id:_node_id,
            payload:_payload.to_string()
        };
        return replay;
    }
}
#[derive(Serialize, Deserialize)]
pub struct Bft_PrePrepare_Message {
    view_num:u64,
    sequence_num:u64,
    msg_digest:HashValue,
    client_msg:Bft_Message,
}

impl Bft_PrePrepare_Message {
    pub fn new(_view_num:u64, _sequence_num: u64,_source_msg:Bft_Message)->Bft_PrePrepare_Message {


        let replay = Bft_prepare_Message {
            view_num:_view_num,
            sequence_num:_sequence_num,
            msg_digest:_source_msg.sign(),
            client_msg:_source_msg,
        };
        return replay;
    }

    pub fn get_view_num(&self) -> u64 {
        return self.view_num.clone();
    }

    pub fn get_sequence_num(&self) -> u64 {
        return self.sequence_num.clone();
    }

    pub fn get_client_msg(&self) -> &Bft_Message {
        return &(self.client_msg);
    }

    pub fn get_msg_digest(&self) -> &HashValue {
        return &(self.msg_digest);
    }

}
#[derive(Serialize, Deserialize)]
pub struct Bft_Prepare_Message {
    view_num:u64,
    sequence_num:u64,
    msg_digest:String,
    node_id:u64
}
#[derive(Serialize, Deserialize)]
impl Bft_Prepare_Message {
    pub fn new(_view_num:u64, _sequence_num:u64, _digest:&str, _node_id:u64 ) ->Bft_Prepare_Message {
        let msg = Bft_Prepare_Message {
            view_num:_view_num,
            sequence_num:_sequence_num,
            msg_digest:_digest.to_string(),
            node_id:_node_id
        };
        return msg;
    }

    pub fn get_view_num(&self) -> u64 {
        return self.view_num.clone();
    }

    pub fn get_sequence_num(&self) -> u64 {
        return self.sequence_num.clone();
    }

    pub fn get_client_msg(&self) -> &Bft_Message {
        return &(self.client_msg);
    }

    pub fn get_msg_digest(&self) -> &String {
        return &(self.msg_digest);
    }

    pub fn get_node_id(&self) -> u64 {
        return self.node_id.clone();
    }
}
#[derive(Serialize, Deserialize)]
pub struct Bft_Commit_Message {
    view_num:u64,
    sequence_num:u64,
    msg_digest:String,
    node_id:u64
}
#[derive(Serialize, Deserialize)]
impl Bft_Commit_Message {
    pub fn new(_view_num:u64, _sequence_num:u64, _digest:&str, _node_id:u64 ) ->Bft_Commit_Message {
        let msg = Bft_Commit_Message {
            view_num:_view_num,
            sequence_num:_sequence_num,
            msg_digest:_digest.to_string(),
            node_id:_node_id
        };
        return msg;
    }
    pub fn get_view_num(&self) -> u64 {
        return self.view_num.clone();
    }

    pub fn get_sequence_num(&self) -> u64 {
        return self.sequence_num.clone();
    }

    pub fn get_client_msg(&self) -> &Bft_Message {
        return &(self.client_msg);
    }

    pub fn get_msg_digest(&self) -> &HashValue {
        return &(self.msg_digest);
    }
}