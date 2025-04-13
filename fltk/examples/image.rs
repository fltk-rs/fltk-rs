use fltk::{app, frame::Frame, image::JpegImage, prelude::*, window::Window};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::default().with_size(400, 300);
    let mut frame = Frame::default_fill();

    {
        let mut image = JpegImage::load("screenshots/calc.jpg")?;
        frame.set_image(Some(image.clone()));
        image.scale(200, 200, true, true);
    }

    // // To remove an image
    // frame.set_image(None::<SharedImage>);

    wind.end();
    wind.make_resizable(true);
    wind.show();

    app.run()?;
    Ok(())
}
