use gtk::prelude::*;

pub fn build_ui(app: &gtk::Application) {
    // let glade_src = include_str!("../layouts/main.glade");
    // let builder = gtk::Builder::from_string(glade_src);
    // let window: gtk::Window = builder.get_object("main_ApplicationWindow").unwrap();

    let window = gtk::ApplicationWindow::new(app);
    window.set_application(Some(app));
    window.set_title("Tasker");
    window.set_default_size(600, 400);

    // The notebook holds the tabs for multiple different boards
    let notebook = gtk::Notebook::new();

    // Child widget of the notebook, contains multiple different categories
    let scrolled_window = gtk::ScrolledWindow::new::
        <gtk::Adjustment, gtk::Adjustment>(
        gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT
    );

    let _tab1_frame = gtk::Frame::new(None);
    notebook.add(&scrolled_window);

    let tab1_hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    // Placeholder items
    for i in 0..10 {
        let button_text = format!("Button {}", i);
        let pl_button = gtk::Button::with_label(&button_text);
        tab1_hbox.pack_start(&pl_button, false, false, 0);
    }

    scrolled_window.add(&tab1_hbox);

    window.add(&notebook);
    window.show_all();
}