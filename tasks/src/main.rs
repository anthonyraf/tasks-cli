#![allow(unused)]

mod parser;
mod core;
use crate::core::{ConfigFile, Task, TaskList};


use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate serde;

use serde::{Serialize, Deserialize};

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
    let mut db = ConfigFile::new("tasks.toml".to_string());
    let mut tasklist = TaskList::new();
    let args = parser::parse_args();

    match &args.action {
        parser::Action::Add(add) => {
            tasklist.add(Task::new(1, add.task.to_owned(), add.priority.to_owned()).to_toml());
            db.write(&tasklist.tasks);
        }
        _ => println!("undefined")
    }


}

