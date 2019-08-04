use std::env;
use std::process;

use lib;
use lib::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // --snip--
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });;


    let result = lib::run(config).unwrap_or_else(|err| {
        println!("grep  error for: {}", err);
        process::exit(1);
    });
}
//
//fn parse_config(args: &[String]) -> (&str, &str) {
//    let query = &args[1];
//    let filename = &args[2];
//
//    (query, filename)
//}
