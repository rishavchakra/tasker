// This module contains code for retrieving and saving data

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

// At the top level, there is a list of boards
// Within each board, there is a list of Categories
// Within each category, there is a list of items
// Each item contains:
// 		name, description, state
// Maybe: JSON contains boards and items
// Items reference the category they are inside

// If not, more traditional organization
// Boards contain categories, which contain items
// Easier to organize but harder to work with

// Or, use a simpler organization
// Tasks are stored in a top-level list
// Tasks store the category and board they are under
// Boards are also stored in the top-level and contain category names

#[derive(Serialize, Deserialize)]
pub struct TaskData {
    boards: Vec<Board>,
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
struct Board {
    name: String,
    description: String,
    categories: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    name: String,
    description: String,
    state: bool,
    board: usize,
    category: usize
}

pub fn add_board(_name: String) {
    // TODO: this
}

pub fn add_category(_name: String) {
    // TODO: also this
}

pub fn add_task(_name: String, _description: String) {
    // TODO: this too
    // let task_to_add = Task {
    //     name: _name,
    //     description: _description,
    //     state: false,
    // };
}

pub fn get_board_names() -> Vec<String> {
    let data = open_json();
    data.boards.iter().map(|board| board.name.clone()).collect::<Vec<String>>()
}

pub fn get_category_names(board_name: &String) -> Option<Vec<String>> {
    let data = open_json();
    for board in &data.boards {
        if &board.name == board_name {
            return Some(board.categories.clone());
        }
    }
    None
}

pub fn get_category_names_ind(board: usize) -> Option<Vec<String>> {
    let data = open_json();
    if board > data.boards.len() {
        return None;
    }
    Some(data.boards[board].categories.clone())
}

pub fn get_task_names(board_name: &String, category_name: &String) -> Vec<String> {
    let data = open_json();
    let boards = get_board_names();
    let categories = get_category_names(&board_name).unwrap();
    let mut task_names = Vec::new();
    for task in &data.tasks {
        if &boards[task.board] == board_name && &categories[task.category] == category_name {
            task_names.push(task.name.clone());
        }
    }
    task_names
}

pub fn get_task_names_ind(board: usize, category: usize) -> Vec<String> {
    let data = open_json();
    let mut task_names = Vec::new();
    for task in &data.tasks {
        if board == task.board && category == task.category {
            task_names.push(task.name.clone());
        }
    }
    task_names
}

pub fn get_tasks(board_name: &String, category_name: &String) -> Vec<Task> {
    let data = open_json();
    let boards = get_board_names();
    let categories = get_category_names(&board_name).unwrap();
    let mut tasks = Vec::new();
    for task in &data.tasks {
        if &boards[task.board] == board_name && &categories[task.category] == category_name {
            tasks.push(task.clone());
        }
    }
    tasks
}

pub fn get_tasks_ind(board: usize, category: usize) -> Vec<Task> {
    let data = open_json();
    let mut tasks = Vec::new();
    for task in &data.tasks {
        if board == task.board && category == task.category {
            tasks.push(task.clone());
        }
    }
    tasks
}

fn open_json() -> TaskData {
    let mut data_file =
        File::open("tasker_data.json").expect("ERROR: Could not find the data json file");
    let mut data_str = String::new();
    data_file
        .read_to_string(&mut data_str)
        .expect("ERROR: Could not read data file into string");
    let json_obj: TaskData =
        serde_json::from_str(&data_str).expect("ERROR: Could not parse JSON data");
    json_obj
}
