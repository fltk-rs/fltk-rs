use fltk::*;
use fltk::prelude::*;

fn callback() {
    println!("TICK");
    app::repeat_timeout(1.0, Box::new(callback));
}

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::new(100, 100, 400, 300, "");
    app::add_timeout(1.0, Box::new(callback));
    wind.end();
    wind.show();
    app.run().unwrap();
}