use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        let hello_label = Label::new(Some("Hello World"));
        win.add(&hello_label);
            
        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();
}
