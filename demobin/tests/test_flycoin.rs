extern crate rustDemo;

#[cfg(test)]
mod fly_coin_test {
    mod waiter {
        fn add_order(){ print!("add_order")}
        fn set_table() {print!("set_table")}
    }
    #[test]
    fn it_works() {
        flycoin::test_lfy();
    }
}