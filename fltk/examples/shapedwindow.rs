use fltk::{prelude::*, *};

pub struct ShapedWindow {
    frm: frame::Frame,
    wind: window::Window,
}

impl ShapedWindow {
    pub fn default() -> Self {
        let shape = prep_surface();

        let mut wind = window::Window::default().with_size(400, 400);
        let frm = frame::Frame::default().size_of_parent();
        wind.end();
        wind.set_shape(Some(shape));
        let mut x = 0;
        let mut y = 0;
        wind.handle(move |w, ev| match ev {
            enums::Event::Push => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                true
            }
            enums::Event::Drag => {
                w.set_pos(app::event_x_root() - x, app::event_y_root() - y);
                true
            }
            _ => false,
        });
        Self { wind, frm }
    }
    pub fn set_image(&mut self, image: Option<image::RgbImage>) {
        self.frm.set_image(image);
    }
    pub fn show(&mut self) {
        self.wind.show();
    }
    pub fn set_cursor_image(&mut self, image: image::RgbImage) {
        self.wind.set_cursor_image(image, 0, 0);
    }
}

fn main() {
    let mut pattern: Vec<u8> = vec![0_u8; 500 * 500 * 3];
    for (iter, pixel) in pattern.chunks_exact_mut(3).enumerate() {
        let x = iter % 500;
        let y = iter / 500;
        let (red, green, blue) = utils::hex2rgb((x ^ y) as u32);
        pixel.copy_from_slice(&[red, green, blue]);
    }
    let pattern = image::RgbImage::new(&pattern, 500, 500, enums::ColorDepth::Rgb8).unwrap();
    let app = app::App::default();
    let mut win = ShapedWindow::default();
    win.set_image(Some(pattern.clone()));
    win.set_cursor_image(pattern);
    win.show();
    app.run().unwrap();
}

fn prep_surface() -> image::RgbImage {
    let surf = surface::ImageSurface::new(400, 400, false);
    surface::ImageSurface::push_current(&surf);
    draw::set_draw_color(enums::Color::Black);
    draw::draw_rectf(-1, -1, 402, 402);
    draw::set_draw_color(enums::Color::White);
    draw::draw_pie(0, 0, 400, 400, 0., 360.);
    let img = surf.image().unwrap();
    surface::ImageSurface::pop_current();
    img
}
