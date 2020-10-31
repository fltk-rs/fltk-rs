use fltk::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let app = app::App::default();
    let mut wind = window::DoubleWindow::new(100, 100, 400, 400, "Hello from rust");
    let mut frame = frame::Frame::new(0, 0, 400, 400, "");
    wind.end();
    wind.show();

    let v: Vec<u32> = (0..128 * 128)
        .map(|i| {
            let x = i % 128;
            let y = i / 128;
            x ^ y
        })
        .collect();
    let data: Vec<u8> = v.into_iter().map(|i| i as u8).collect();
    let data = Rc::from(RefCell::from(data));
    let data_c = data.clone();

    frame.draw2(move |f| {
        let mut image =
            unsafe { image::RgbImage::from_data(&data_c.borrow(), 128, 128, 1).unwrap() };
        image.scale(400, 400, false, true);
        image.draw(f.x(), f.y(), f.width(), f.height());
    });

    let mut i = 0;
    while app.wait() {
        if i < data.borrow().len() {
            data.borrow_mut()[i] = 255;
        }
        std::thread::sleep(std::time::Duration::from_millis(30));
        frame.redraw();
        i += 1;
    }
}
