use fltk::{app::*, frame::*, image::*, window::*};
use std::path::PathBuf;

fn main() {
    let app = App::default().set_scheme(AppScheme::Gleam);
    let mut wind = Window::new(100, 100, 800, 600, "Hello from rust");
    let mut frame = Frame::new(-100, -100, 300, 300, "");
    let image = SvgImage::new(PathBuf::from("screenshots/RustLogo.svg"));
    frame.set_image(image);
    wind.make_resizable(true);
    wind.show();
    app.run().unwrap();
}
