mod app;

extern crate gtk;
use app::App;

fn main() {
    let app = App::new();
    app.run();
}
