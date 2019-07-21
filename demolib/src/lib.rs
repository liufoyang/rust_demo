#[cfg(test)]
mod demo_mod {
    mod waiter {
        fn add_order(){}
        fn set_table() {}
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn call_demo_mod_create() {
    crate::demo_mod::waiter::add_order();
}
