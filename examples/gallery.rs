use fltk::{frame::*, window::*};

fn main() {
    let mut wind = Window::new().set(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new().set(20, 10, 360, 260, "Hello World!");
    frame.set_label_size(30);

    wind.end();
    wind.show();
    fl::run();
}
