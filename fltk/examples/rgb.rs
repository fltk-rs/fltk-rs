use fltk::{prelude::*, *};
use std::rc::Rc;
use std::cell::RefCell;

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

    let i = Rc::from(RefCell::from(3));
    let i_c = i.clone();
    
    frame.draw(move |f| {
        let mut image = unsafe { image::RgbImage::new2(&fb, 128, 128, -3, 128 * 3).unwrap() };
        // image.scale(f.width(), f.height(), false, true);
        image.draw(f.x(), f.y(), f.width(), f.height());
        // unsafe { draw::draw_image2(&fb, 0, 0, 128, 128, -3, -128 * 3); }
    });

    while app.wait() {
        // *i.borrow_mut() += 3;
        // wind.redraw();
        // app::sleep(1.);
    }
}
