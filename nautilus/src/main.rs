use std::env;

#[macro_use] extern crate rocket;

mod api;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 {
        if args[1] == "api" {
            api::main();
            return;
        } else if args[1] == "worker" {
            println!("Worker not implemented yet");
            return;
        }
    }
    
    println!("");
    println!("Please pass an argument whether to start the api or the worker");
    println!("i.e. `cargo run -- api` or `cargo run -- worker`");
    println!("");
}
