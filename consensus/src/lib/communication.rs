use bft_message::*;
use bft_node::*;
///bft通信模块接口
///
pub trait  BftCommunication {
    fn startListen(&self, node:Btf_Node);

    fn sendMessage<T: Serialize >(&self, address:&str, port:&str, data:T);
}