// This module contains code for retrieving and saving data

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

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

struct _Board {
    name: String,
    description: String,
    categories: Vec<_Category>
}

struct _Category {
    name: String,
    items: Vec<Item>
}

struct Item {
    name: String,
    description: String,
    state: bool
}

pub fn add_board(_name: String) {
    // TODO: this
}

pub fn add_category(_name: String) {
    // TODO: also this
}

pub fn add_item(_name: String, _description: String) {
    // TODO: this too
    let item_to_add = Item {
        name: _name,
        description: _description,
        state: false
    };
}

pub fn open_json() {
    let mut data_file = File::open("tasker_data.json")
        .expect("ERROR: Could not find the data json file");
    let mut data_str = String::new();
    data_file.read_to_string(&mut data_str)
        .expect("ERROR: Could not read data file into string");
    
    println!("{}", data_str);
}