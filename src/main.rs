mod data;   

use gio::prelude::*;
use gtk::prelude::*;

fn main() {
    data::open_json();
    let app = gtk::Application::new(
        Some("com.rishavc.tasker_gui"),
        gio::ApplicationFlags::FLAGS_NONE,
    ).expect("Failed to initialize GTK");

    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Tasker");
        window.set_default_size(400, 240);
        window.show_all();
    });

    app.run(&std::env::args().collect::<Vec<_>>());
}
