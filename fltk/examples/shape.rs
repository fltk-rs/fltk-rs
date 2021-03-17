use fltk::{app, frame::*, image::*, window::*};

fn main() {
    let mut fb: Vec<u8> = vec![0u8; 400 * 400 * 4];
    for (iter, pixel) in fb.chunks_exact_mut(4).enumerate() {
        let x = (iter % 400 as usize) as i16;
        let y = (iter / 400 as usize) as i16;
        let d = {
            let xd = x as i32 - 200 as i32;
            let yd = y as i32 - 200 as i32;
            ((xd.pow(2) + yd.pow(2)) as f64).sqrt().powi(2)
        };
        let inside_the_circle = d < (200 as f64).powi(2);

        let rgba = if inside_the_circle {
            [0x00, 0x00, 0x00, 0xff] // non-transparent
        } else {
            [0x00, 0x00, 0x00, 0x00] // transparent
        };

        pixel.copy_from_slice(&rgba);
    }
    let mut image = RgbImage::new(&fb, 400, 400, ColorDepth::Rgba8).unwrap();
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 400);
    
    wind.end();
    wind.show();
    unsafe { wind.set_shape(Some(image)); }

    app.run().unwrap();
}
