use std::fs::File;
use std::io::prelude::*;
use crate::parser;

// Import serde derive


// pub fn main() {
//     let mut file = File::open("config.toml").expect("Unable to open file");
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).expect("Could not read file");

//     let mut config = contents.parse::<toml::Value>().unwrap();

//     config["informations"]["age"] = toml::Value::Integer(20);
//     println!("{:?}", config);
//     println!("{:#?}", config["informations"]["age"].as_integer().unwrap());
// }


pub fn main() {
    let mut tasklist: TaskList = TaskList::new();

    for i in 0..10 {
        let task = Task::new(i, format!("Task n°{}", i));
        tasklist.add(task);
    }

    // println!("{:#?}", tasklist.tasks);
    let mut config: toml::Value = ConfigFile::new(String::from("config.toml")).read();
    //let mut table = config.as_table_mut().unwrap();

    // let mut hobbies = table.get("informations").unwrap().clone().try_into::<toml::value::Table>().unwrap();
    let mut hobbies = config["informations"]["hobbies"].as_table_mut().unwrap(); // modify directly the config

    hobbies.insert("swim".to_string(), toml::Value::String("Swimming".to_string()));

    //config["informations"]["hobbies"] = toml::map::Map(hobbies);
    println!("{:?}", config["informations"]["hobbies"]);
}

#[derive(Debug)]
pub struct Task {
    id: u8,
    task: String,
    date: String, // NOTE: Future implementation
}

pub struct TaskList {
    tasks: Vec<Task>,
}

pub struct ConfigFile {
    filename: String,
}

impl Task {
    pub fn new(id: u8, task: String) -> Self {
        Self {
            id,
            task,
            date : String::from("NULL"),
        }
    }
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn write_to_file(&self){
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

    pub fn write(&self, config: toml::Value) {
        let mut file = File::create(&self.filename).expect("Unable to create file");
        file.write_all(toml::to_string(&config).unwrap().as_bytes()).expect("Unable to write to file");
    }
}

