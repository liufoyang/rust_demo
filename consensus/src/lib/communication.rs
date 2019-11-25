use std::sync::mpsc::Receiver;
///bft通信模块接口
///
///
pub trait  BftCommunication {
    fn startListen(&self, addr:&str, port:&str) ->Receiver<Box<BftCommunicationMsg>>;
}

pub struct BftCommunicationMsg {
    pub command:String,
    pub version:String,
    pub payload:String
}

impl BftCommunicationMsg {
    pub fn to_string(&self)->String {
        let mut msg_str = String::new();
        msg_str.push_str(self.command.as_str());
        msg_str.push_str(" ");
        msg_str.push_str(self.version.as_str());
        msg_str.push_str(" ");
        msg_str.push_str(self.payload.len().to_string().as_str());

        msg_str.push_str("\n");
        msg_str.push_str(self.payload.as_str());
        msg_str.push_str("\n");
        return msg_str;
    }
}