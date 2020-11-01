// // This is an example using the image crate

// use fltk::{app::*, frame::*, image::RgbImage, window::*};
// use image::GenericImageView;
//
// fn main() {
//     let app = App::default();
//     let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
//     let mut frame = Frame::new(0, 0, 400, 200, "");
//
//     let img = image::open("./examples/image.jpg").unwrap();
//     let (x, y) = img.dimensions();
//     let rgb = RgbImage::new(&img.to_bytes(), x, y, 3).unwrap();
//
//     frame.set_image(Some(rgb));
//
//     wind.show();
//
//     app.run().unwrap();
// }

// // This is an example using the rust_embed crate

// use fltk::{app::*, frame::*, image::*, window::*};
// #[macro_use]
// extern crate rust_embed;
//
// #[derive(RustEmbed)]
// #[folder = "examples/"]
// struct Asset;
//
// fn main() {
//     let app = App::default();
//     let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
//     let mut frame = Frame::new(0, 0, 400, 200, "");
//
//     let img = Asset::get("image.jpg").unwrap();
//     let jpg = JpegImage::from_data(&img).unwrap();
//
//     frame.set_image(Some(jpg));
//
//     wind.show();
//
//     app.run().unwrap();
// }

// // This is an example showing using the inbuilt image functions

use fltk::{app::*, frame::*, image::*, window::*};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::default().with_scheme(Scheme::Gleam);
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 300, "");

    let mut image = SharedImage::load("screenshots/calc.jpg")?;
    image.scale(200, 200, true, true);

    frame.set_image(Some(image));

    // // To remove an image
    // frame.set_image(None::<SharedImage>);

    wind.make_resizable(true);
    wind.show();

    app.run()?;
    Ok(())
}
