
use std::fs::File;
use std::io::prelude::*;
use crate::parser;


pub fn main() {

    // println!("{:#?}", config);
    let mut config: ConfigFile = ConfigFile::new(String::from("config.toml"));
    let mut tasks = config.read()["tasks"].as_array_mut().unwrap();
    let mut tasklist: TaskList = TaskList::new();

    let task: Task = Task::new(255, String::from("Task 1"));

    tasklist.add(task.to_toml());
    tasklist.add(task.to_toml());

    config.write(&tasklist.tasks);

    println!{"{:#?}", tasklist.tasks};

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: u8,
    task: String,
    priority: u8,
    date: String, // NOTE: Future implementation
}

#[derive(Debug)]
pub struct TaskList {
    tasks: Vec<toml::Value>,
}

pub struct ConfigFile {
    filename: String,
}

impl Task {
    pub fn new(id: u8, task: String) -> Self {
        Self {
            id,
            task,
            priority: 1,
            date : String::from("NULL"),
        }
    }

    pub fn to_toml(&self) -> toml::Value {
        let toml_string: String = toml::to_string(&self).unwrap();
        toml_string.parse::<toml::Value>().unwrap()
    }
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    pub fn add(&mut self, task: toml::Value) {
        self.tasks.push(task);
    }

    pub fn save(&self){
        /* Save in configuration file */
        todo!();
    }
}

impl ConfigFile {
    pub fn new(filename: String) -> Self {
        Self { 
            filename,
        }
    }

    pub fn read(&self) -> toml::Value {
        let mut content = String::new();
        let mut file = File::open(&self.filename).expect("Unable to open file");
        file.read_to_string(&mut content).expect("Could not read file");
        
        content.parse::<toml::Value>().unwrap()
    }

    pub fn write(&self, config: &Vec<toml::Value>) {
        let mut file = File::create(&self.filename).expect("Unable to create file");
        
        let mut table = toml::Table::new();
        table.insert(String::from("tasks"), toml::Value::Array(config.to_vec()));
        file.write(toml::to_string_pretty(&table).unwrap().as_bytes()).expect("Unable to write file");

    }
}

