// This module contains code for retrieving and saving data

// use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

/**
TaskData structs contain all the data in the JSON file.

*/
#[derive(Serialize, Deserialize, Clone)]
pub struct MasterData {
    boards: Vec<Board>
}

/**
Boards contain lists of columns of task items
name, description, and column names
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct Board {
    name: String,
    description: String,
    sections: Vec<Section>,
    parent: Box<MasterData>
}

impl Board {
    pub fn update(&mut self, name: String, description: String) {
        self.name = name;
        self.description = description;
    }
}
/**
Sections contain tasks
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct Section {
    name: String,
    description: String,
    tasks: Vec<Task>,
    parent: Box<Board>
}

impl Section {
    pub fn update(&mut self, name: String, description: String) {
        self.name = name;
        self.description = description;
    }
}

/**
Tasks are individual items in the board that contain the task's information
name, description, completion state
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    name: String,
    description: String,
    state: bool,
    parent: Box<Section>
}

impl Task {
    pub fn update(&mut self, name: String, description: String) {
        self.name = name;
        self.description = description;
    }

    pub fn move_to_section(&mut self, destination: &mut Section) {
        
    }
}

/**
Opens the json file and returns the data as its data struct representation
*/
fn read_json() -> MasterData {
    let mut data_file =
        File::open("tasker_data.json").expect("ERROR: Could not find the data json file");
    let mut data_str = String::new();
    data_file
        .read_to_string(&mut data_str)
        .expect("ERROR: Could not read data file into string");
    let json_obj: MasterData =
        serde_json::from_str(&data_str).expect("ERROR: Could not parse JSON data");
    json_obj
}


fn save_json(data: MasterData) {

}