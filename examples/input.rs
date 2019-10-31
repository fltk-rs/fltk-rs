use fltk::{input::*, output::*, window::*};

fn main() {
    let mut wind = Window::new().set(100, 100, 400, 300, "Hello from rust");
    let _inp = Input::new().set(140, 100, 120, 60, "");
    let mut out = Output::new().set(140, 200, 120, 60, "");
    out.set_value("Hello!");
    wind.end();
    wind.show();
    fl::run();
}
