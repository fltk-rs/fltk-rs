use fltk::{frame::*, window::*};

fn main() {
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let _frame = Frame::new(20, 10, 360, 260, "");

    wind.end();
    wind.show();
    fl::run();
}
