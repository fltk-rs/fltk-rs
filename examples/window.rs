use fltk::fl;
use fltk::window::Window;

fn main() {
    let mut wind = Window::new(1200, 600, "hello from rust");
    wind.end();
    wind.show();
    fl::run();
}
