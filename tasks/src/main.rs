#![allow(unused)]

mod core;
mod parser;

// #[macro_use]
// extern crate serde_derive;
// extern crate serde;

// use serde::{Serialize, Deserialize};

/*
fn main() {
    let args = parse_args();
    // match &args.action {
    //     parser::Action::Add(add) => println!("Task : {}", add.task),
    //     _ => println!("{:?}", args.action)
    // }
    println!("{:?}", args.action);
}
*/

fn main() {
    core::main();
}