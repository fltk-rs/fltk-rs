use fltk::*;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::new(100, 100, 400, 400, "Hello from rust");
    let mut frame = frame::Frame::new(0, 0, 400, 400, "");
    frame.draw2(|f| {
        let mut image = {
            let t: Vec<u32> = (0..128 * 128 * 3)
                .map(|i| {
                    let x = i % 128;
                    let y = i / 128;
                    x ^ y
                })
                .collect();
            let mut v: Vec<u8> = vec![];
            for elem in t {
                let (r, g, b) = utils::hex2rgb(elem);
                v.push(r);
                v.push(g);
                v.push(b);
            }
            image::RgbImage::new(&v, 128, 128, 3).unwrap()
        };
        image.scale(400, 400, false, true);
        image.draw(f.x(), f.y(), f.width(), f.height());
    });
    wind.make_resizable(true);
    wind.end();
    wind.show();
    app.run().unwrap();
}
