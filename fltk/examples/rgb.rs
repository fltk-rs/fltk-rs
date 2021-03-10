use fltk::*;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 400);
    let mut frame = frame::Frame::default().size_of(&wind);
    frame.draw2(move |f| {
        let mut fb: Vec<u8> = vec![0u8; 128 * 128 * 3];
        for (iter, pixel) in fb.chunks_exact_mut(3).enumerate() {
            let x = iter % 128;
            let y = iter / 128;
            let (red, green, blue) = utils::hex2rgb((x ^ y) as u32);
            pixel.copy_from_slice(&[red, green, blue]);
        }
        let mut image = image::RgbImage::new(&fb, 128, 128, ColorDepth::Rgb8).unwrap();
        image.scale(f.width(), f.height(), false, true);
        image.draw(f.x(), f.y(), f.width(), f.height());
    });
    wind.make_resizable(true);
    wind.end();
    wind.show();
    app.run().unwrap();
}
