use fltk::{prelude::*, *};

pub struct ShapedWindow {
    wind: window::Window,
}

impl ShapedWindow {
    pub fn default() -> Self {
        let shape = prep_shape();

        let mut wind = window::Window::default().with_size(400, 400);
        wind.set_color(enums::Color::White);
        let mut but = button::Button::default()
            .with_label("Button")
            .with_size(80, 80)
            .center_of_parent();
        but.set_frame(enums::FrameType::OFlatFrame);
        but.set_color(enums::Color::Cyan);
        but.clear_visible_focus();
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
        Self { wind }
    }
    pub fn set_cursor(&mut self, img: image::RgbImage) {
        self.wind.set_cursor_image(img, 0, 0);
    }
    pub fn show(&mut self) {
        self.wind.show();
    }
}

fn main() {
    let app = app::App::default();
    let mut win = ShapedWindow::default();
    win.show();
    // Called after showing the window
    win.set_cursor(prep_cursor());
    app.run().unwrap();
}

fn prep_shape() -> image::RgbImage {
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

fn prep_cursor() -> image::RgbImage {
    let mut fb: Vec<u8> = vec![0u8; 20 * 20 * 4];
    for (iter, pixel) in fb.chunks_exact_mut(4).enumerate() {
        let x = iter % 20;
        let y = iter / 20;
        if x > 5 && x < 15 && y > 5 && y < 15 {
            pixel.copy_from_slice(&[255, 0, 0, 255]);
        } else {
            pixel.copy_from_slice(&[0, 0, 0, 0]);
        }
    }
    let image = image::RgbImage::new(&fb, 20, 20, enums::ColorDepth::Rgba8).unwrap();
    image
}
