mod data;
mod gui;

use gio::prelude::*;

fn main() {
    let app = gtk::Application::new(
        Some("com.rishavc.tasker_gui"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Failed to initialize GTK");

    app.connect_activate(gui::build_ui);

    app.run(&std::env::args().collect::<Vec<_>>());
}

mod tests {
    
    #[test]
    fn test_json_boardnames() {
        use super::data;
        let board_names = data::get_board_names();
        assert_eq!(board_names, vec!["Board0", "Board1"]);
    }
    
    #[test]
    fn test_json_columnnames() {
        use super::data;
        let board_names = data::get_board_names();
        let cat_name1 = data::get_column_names(&board_names[0]).unwrap();
        assert_eq!(cat_name1, vec!["Board 0 column 0", "Board 0 column 1"]);
        let cat_name2 = data::get_column_names(&board_names[1]).unwrap();
        assert_eq!(cat_name2, vec!["Board 1 column 0", "Board 1 column 1"]);
    }
    
    #[test]
    fn test_json_tasknames() {
        use super::data;
        // let task_names = data::get_task_names(&String::from("Board0"), &String::from("Category0"));
        let task_names = data::get_task_names_ind(0, 0);
        assert_eq!(task_names, vec!["Task name", "Another task name"]);
    }
}
