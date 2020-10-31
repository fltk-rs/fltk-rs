use fltk::*;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::new(100, 100, 400, 400, "Hello from rust");
    let mut frame = frame::Frame::new(0, 0, 400, 400, "");
    frame.draw2(|f| {
        let mut image = {
            let v: Vec<u32> = (0..128 * 128)
                .map(|i| {
                    let x = i % 128;
                    let y = i / 128;
                    x ^ y
                })
                .collect();
            let v: Vec<u8> = v.into_iter().map(|i| i as u8).collect();
            image::RgbImage::new(&v, 128, 128, 1).unwrap()
        };
        image.scale(400, 400, false, true);
        image.draw(f.x(), f.y(), f.width(), f.height());
    });
    wind.make_resizable(true);
    wind.end();
    wind.show();
    app.run().unwrap();
}
