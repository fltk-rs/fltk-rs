use fltk::{enums::ColorDepth, prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 400);
    let mut frame = frame::Frame::default_fill();
    wind.make_resizable(true);
    wind.end();
    wind.show();
    frame.draw(move |f| {
        let mut fb: Vec<u8> = vec![0u8; (f.w() * f.h() * 4) as usize];
        for (iter, pixel) in fb.chunks_exact_mut(4).enumerate() {
            let x = iter % f.w() as usize;
            let y = iter / f.w() as usize;
            let (red, green, blue) = utils::hex2rgb((x ^ y) as u32);
            pixel.copy_from_slice(&[red, green, blue, 255]);
        }
        let mut image = image::RgbImage::new(&fb, f.w(), f.h(), ColorDepth::Rgba8)
            .unwrap()
            .to_srgb_image()
            .unwrap()
            .blur(50)
            .unwrap()
            .convert(ColorDepth::Rgb8)
            .unwrap();
        image.draw(f.x(), f.y(), f.width(), f.height());
    });

    app.run().unwrap();
}
