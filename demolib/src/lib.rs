#[cfg(test)]

use flycoin;

mod demo_mod {
    mod waiter {
        fn add_order(){ print!("add_order")}
        fn set_table() {print!("set_table")}
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

