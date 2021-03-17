use fltk::{app, frame::*, image::*, utils, window::*};

fn main() {
    let mut shape: Vec<u8> = vec![0u8; 400 * 400 * 4];
    for (iter, pixel) in shape.chunks_exact_mut(4).enumerate() {
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
    let shape = RgbImage::new(&shape, 400, 400, ColorDepth::Rgba8).unwrap();

    let mut pattern: Vec<u8> = vec![0u8; 500 * 500 * 3];
    for (iter, pixel) in pattern.chunks_exact_mut(3).enumerate() {
        let x = iter % 500;
        let y = iter / 500;
        let (red, green, blue) = utils::hex2rgb((x ^ y) as u32);
        pixel.copy_from_slice(&[red, green, blue]);
    }
    let pattern = RgbImage::new(&pattern, 500, 500, ColorDepth::Rgb8).unwrap();

    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 400);
    let mut frm = Frame::default().size_of_parent();
    wind.end();
    wind.show();
    wind.set_shape(Some(shape));
    frm.set_image(Some(pattern));

    app.run().unwrap();
}
