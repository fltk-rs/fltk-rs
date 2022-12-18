// Shows how to use SvgImage loaded from a file and loaded from data (&str)

const IMAGE2: &str = r#"<svg viewBox="0 0 100 100">
<circle fill="red" cx="50" cy="50" r="20"></circle>
</svg>"#;

use fltk::{app, enums::FrameType, frame::Frame, image::SvgImage, prelude::*, window::Window};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);

    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");

    let mut frame = Frame::default().with_size(360, 260).center_of(&wind);
    frame.set_frame(FrameType::EngravedBox);
    let mut image1 = SvgImage::load("screenshots/RustLogo.svg").unwrap();
    image1.scale(200, 200, true, true);
    frame.set_image(Some(image1));

    wind.make_resizable(true);
    wind.end();
    wind.show();
    wind.set_icon(Some(SvgImage::from_data(IMAGE2).unwrap()));

    app.run().unwrap();
}
