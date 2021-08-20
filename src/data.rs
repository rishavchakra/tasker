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
// Boards contain columns, which contain items
// Easier to organize but harder to work with

// Or, use a simpler organization
// Tasks are stored in a top-level list
// Tasks store the category and board they are under
// Boards are also stored in the top-level and contain category names

/**
TaskData structs contain all the data in the JSON file.

*/
#[derive(Serialize, Deserialize)]
pub struct TaskData {
    boards: Vec<Board>,
    tasks: Vec<Task>,
}

/**
Boards contain lists of columns of task items
name, description, and column names
*/
#[derive(Serialize, Deserialize)]
pub struct Board {
    name: String,
    description: String,
    columns: Vec<String>,
}

/**
Tasks are individual items in the board that contain the task's information
name, description, completion state, board index, column index
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    name: String,
    description: String,
    state: bool,
    board: usize,
    column: usize
}

pub fn add_board(_name: String) {
    // TODO: this
}

pub fn add_column(_name: String) {
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

/**
Returns a list of names of all the boards
*/
pub fn get_board_names() -> Vec<String> {
    let data = open_json();
    data.boards.iter().map(|board| board.name.clone()).collect::<Vec<String>>()
}

/**
Returns a list of columns in a board with a given name
*/
pub fn get_column_names(board_name: &String) -> Option<Vec<String>> {
    let data = open_json();
    for board in &data.boards {
        if &board.name == board_name {
            return Some(board.columns.clone());
        }
    }
    None
}

/**
Returns a list of columns in a board with a given board index
*/
pub fn get_column_names_ind(board: usize) -> Option<Vec<String>> {
    let data = open_json();
    if board > data.boards.len() {
        return None;
    }
    Some(data.boards[board].columns.clone())
}

/**
Returns a list of tasks in a given board/column pair
*/
pub fn get_task_names(board_name: &String, column_name: &String) -> Vec<String> {
    let data = open_json();
    let boards = get_board_names();
    let columns = get_column_names(&board_name).unwrap();
    let mut task_names = Vec::new();
    for task in &data.tasks {
        if &boards[task.board] == board_name && &columns[task.column] == column_name {
            task_names.push(task.name.clone());
        }
    }
    task_names
}

/**
Returns a list of tasks in a given board/column pair by their indices
*/
pub fn get_task_names_ind(board: usize, column: usize) -> Vec<String> {
    let data = open_json();
    let mut task_names = Vec::new();
    for task in &data.tasks {
        if board == task.board && column == task.column {
            task_names.push(task.name.clone());
        }
    }
    task_names
}

/**
Returns a list of tasks in a given board/column pair
*/
pub fn get_tasks(board_name: &String, column_name: &String) -> Vec<Task> {
    let data = open_json();
    let boards = get_board_names();
    let columns = get_column_names(&board_name).unwrap();
    let mut tasks = Vec::new();
    for task in &data.tasks {
        if &boards[task.board] == board_name && &columns[task.column] == column_name {
            tasks.push(task.clone());
        }
    }
    tasks
}

/**
Returns a list of tasks in a given board/column pair by their indices
*/
pub fn get_tasks_ind(board: usize, column: usize) -> Vec<Task> {
    let data = open_json();
    let mut tasks = Vec::new();
    for task in &data.tasks {
        if board == task.board && column == task.column {
            tasks.push(task.clone());
        }
    }
    tasks
}

/**
Opens the json file and returns the data as its data struct representation
*/
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
