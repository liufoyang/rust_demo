use std::sync::Arc;
use std::time::Duration;
use std::thread;

fn main() {

    let list:Vec<i32> = Vec::new();
    let list_arc = Arc::new(list);
    println!("Hello, world!");

    let mut list_1 = list_arc.clone();
    let thread_1 = thread::Builder::new().name("thread_1".to_string()).spawn(move|| {
        let millis_100 = Duration::from_millis(200);
        for i in 0..32 {
            println!("list size {}", list_1.len());
            thread::sleep(millis_100);
        }

    }).unwrap();

    let mut list_2 = list_arc.clone();
    let thread_2 = thread::Builder::new().name("thread_2".to_string()).spawn(move|| {
        let millis_100 = Duration::from_millis(200);
        for i in 0..32 {
            //list_2.push(i);
            println!("list size {}", list_2.len());
            thread::sleep(millis_100);
        }

    }).unwrap();

//    let mut list_3 = list_arc.clone();
//    list_3.push(2);
    thread_1.join();
    thread_2.join();
}
