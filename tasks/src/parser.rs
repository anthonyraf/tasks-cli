

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Add(Add),
    Remove(Remove),
    List(List),
    Done(Done),
}

#[derive(Parser, Debug)]
#[clap(about="Add a task to the list")]
pub struct Add {
    #[arg(long, short , help="Priority of the task (1-3)")]
    pub priority: u8,

    #[arg(help="The task to add")]
    pub task: String,
}

#[derive(Parser, Debug)]
#[clap(about="Remove a task from the list")]
pub struct Remove {
    #[arg(help="The ID of task to remove")]
    pub id: u8,
}

#[derive(Parser, Debug)]
#[clap(about="List all tasks")]
pub struct List {
    #[arg(short, long, help="Show all tasks")]
    pub all: bool,

    #[arg(short, long, help="Show active tasks")]
    pub active: bool, 

    #[arg(short, long, help="Show done tasks")]
    pub done: bool,
}

#[derive(Parser, Debug)]
#[clap(about="Mark a task as done")]
pub struct Done {
    #[arg(help="The ID of task to mark as done")]
    pub id: u8,

    #[clap(long, short, help="Save the task to the done list", default_value_t=true)]
    pub save: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
