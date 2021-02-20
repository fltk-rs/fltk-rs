use fltk::*;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::new(100, 100, 400, 400, "Hello from rust");
    let mut frame = frame::Frame::new(0, 0, 400, 400, "");
    frame.draw2(|f| {
        let mut image = {
            let mut v: Vec<u8> = vec![0u8; 128 * 128 * 3];
            for (i, pixel) in v.chunks_exact_mut(3).enumerate() {
                let x = i % 128;
                let y = i / 128;
                let (r, g, b) = utils::hex2rgb((x ^ y) as u32);
                pixel.copy_from_slice(&[r, g, b]);
            }
            image::RgbImage::new(&v, 128, 128, ColorDepth::Rgb8).unwrap()
        };
        image.scale(f.width(), f.height(), false, true);
        image.draw(f.x(), f.y(), f.width(), f.height());
    });
    wind.make_resizable(true);
    wind.end();
    wind.show();
    app.run().unwrap();
}
