use fltk::{input::*, output::*, window::*};

fn main() {
    let mut wind = Window::new().set(100, 100, 400, 300, "Hello from rust");
    let inp = Input::new().set(140, 100, 120, 60, "");
    let mut out = Output::new().set(140, 200, 120, 60, "");
    fl::register_callback(&inp, &mut || println!("{:?}", fl::event()));
    out.set_value("Hello!");
    out.set_color(Color::FrameDefault);
    wind.end();
    wind.show();
    fl::run();
}
