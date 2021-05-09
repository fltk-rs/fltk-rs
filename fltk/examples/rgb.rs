use fltk::{prelude::*, *};

fn main() {
    let mut fb: Vec<u8> = vec![0u8; 128 * 128 * 3];
    for (iter, pixel) in fb.chunks_exact_mut(3).enumerate() {
        let x = iter % 128;
        let y = iter / 128;
        let (red, green, blue) = utils::hex2rgb((x ^ y) as u32);
        pixel.copy_from_slice(&[red, green, blue]);
    }

    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 400);
    let mut frame = frame::Frame::default().size_of(&wind);
    wind.make_resizable(true);
    wind.end();
    wind.show();
    let mut image = image::RgbImage::new(&fb, 128, 128, enums::ColorDepth::Rgb8).unwrap();
    frame.draw(move |f| {
        image.scale(f.width(), f.height(), false, true);
        image.draw(f.x(), f.y(), f.width(), f.height());
    });

    app.run().unwrap();
}
