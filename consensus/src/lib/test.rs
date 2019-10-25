
#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::rc::Rc;
    use lib::bft_node::Btf_Node;
    use lib::bft_message::{Bft_Prepare_Message, Bft_Message, Bft_Commit_Message, Bft_PrePrepare_Message, Bft_Replay};

    #[test]
    fn bft_node_create() {
        let mut node:Btf_Node = Btf_Node::start_node("", "7878");

        assert!(!node);
    }
}
