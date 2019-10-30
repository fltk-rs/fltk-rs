use fltk::{input::*, window::*};

fn main() {
    let mut wind = Window::new().set(100, 100, 400, 300, "Hello from rust");
    let _inp = Input::new().set(140, 120, 120, 60, "");
    wind.end();
    wind.show();
    fl::run();
}
