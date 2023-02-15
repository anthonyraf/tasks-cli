use std::fs::File;
use std::io::prelude::*;
use std::panic;
use crate::parser;


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: u8,
    task: String,
    priority: u8,
    date: String, // NOTE: Future implementation
}

#[derive(Debug)]
pub struct TaskList {
    pub tasks: Vec<toml::Value>,
}

pub struct ConfigFile {
    filename: String,
}

impl Task {
    pub fn new(id: u8, task: String, priority: u8) -> Self {
        Self {
            id,
            task,
            priority,
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

    pub fn remove(&mut self, id: u32) {
        todo!();
    }

    pub fn update_index(&mut self) {
        todo!();
    }

    pub fn save(&self){
        /* Save in configuration file */
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

    pub fn write(&mut self, config: &Vec<toml::Value>) {
        let mut file_content = self.get("tasks");
        let mut file = File::create(&self.filename).expect("Unable to create file");
        

        let mut table = toml::Table::new();
        
        file_content.push(config[0].to_owned());
        println!("{:?}", file_content);
        table.insert(String::from("tasks"), toml::Value::Array(file_content));
        file.write(toml::to_string_pretty(&table).unwrap().as_bytes()).expect("Unable to write file");

    }

    pub fn get(&mut self, field_name: &str) -> Vec<toml::Value> {
        let content = &mut self.read();
        content[field_name].as_array_mut().unwrap().to_owned()

    }
}

