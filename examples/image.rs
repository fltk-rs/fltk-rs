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
//     let rgb = RgbImage::new(&img.to_bytes(), x as i32, y as i32, 3);
//
//     frame.set_image(&rgb);
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
//     frame.set_image(&jpg);
//
//     wind.show();
//
//     app.run().unwrap();
// }

// // This is an example showing using the inbuilt image functions

use fltk::{app::*, frame::*, image::*, window::*};
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::default().set_scheme(AppScheme::Gleam);
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 300, "");

    let mut image = SharedImage::load(&PathBuf::from("screenshots/calc.jpg"))?;
    image.scale(200, 200, true, true);

    frame.set_image(&image);

    wind.make_resizable(true);
    wind.show();

    app.run()?;
    Ok(())
}
