use fltk::*;

fn main() {
    let shape = prep_surface();

    let mut pattern: Vec<u8> = vec![0u8; 500 * 500 * 3];
    for (iter, pixel) in pattern.chunks_exact_mut(3).enumerate() {
        let x = iter % 500;
        let y = iter / 500;
        let (red, green, blue) = utils::hex2rgb((x ^ y) as u32);
        pixel.copy_from_slice(&[red, green, blue]);
    }
    let pattern = image::RgbImage::new(&pattern, 500, 500, ColorDepth::Rgb8).unwrap();

    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 400);
    let mut frm = frame::Frame::default().size_of_parent();
    wind.end();
    wind.show();
    wind.set_shape(Some(shape));
    frm.set_image(Some(pattern));

    app.run().unwrap();
}

fn prep_surface() -> image::RgbImage {
    let surf = surface::ImageSurface::new(400, 400, false);
    surface::ImageSurface::push_current(&surf);
    draw::set_draw_color(Color::Black);
    draw::draw_rectf(-1, -1, 402, 402);
    draw::set_draw_color(Color::White);
    draw::draw_pie(0, 0, 400, 400, 0., 360.);
    let img = surf.image().unwrap();
    surface::ImageSurface::pop_current();
    img
}
