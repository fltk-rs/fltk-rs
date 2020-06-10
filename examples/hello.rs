use fltk::{app::*, frame::*, image::*, window::*};
use std::path::PathBuf;

fn main() {
    let app = App::default().set_scheme(AppScheme::Gleam);
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 300, "");

    let mut image = SvgImage::load(&PathBuf::from("screenshots/RustLogo.svg")).unwrap();
    image.scale(200, 200, true, true);
    frame.set_image(&image);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}