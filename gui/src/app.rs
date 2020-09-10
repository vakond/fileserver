//! App.

use gtk::*;

const NAME: &str = "FileServer";

pub struct App {
    window: Window,
}

impl App {
    pub fn new() -> Self {
        gtk::init().expect("failed to initialize GTK application");

        let window = Window::new(WindowType::Toplevel);

        window.set_title(NAME);
        window.set_wmclass(NAME, NAME);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App { window }
    }

    pub fn run(&self) {
        self.window.show_all();
        gtk::main();
    }
}
