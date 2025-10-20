use fltk::{app, frame::Frame, image::SharedImage, prelude::*, window::Window};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::default().with_size(400, 300);
    let mut frame = Frame::default_fill();

    let mut image = SharedImage::load("screenshots/calc.jpg")?;
    
    frame.set_image(Some(image));
    frame.image().unwrap().scale(200, 200, true, true);

    // // To remove an image
    // frame.set_image(None::<SharedImage>);

    wind.end();
    wind.make_resizable(true);
    wind.show();

    app.run()?;
    Ok(())
}
