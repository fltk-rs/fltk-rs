use fltk::{prelude::*, *};

const WAIT_CURSOR: &[&str] = &[
    "13 18 3 1",
    "  c None",
    ". c #FFFFFF",
    "B c #000000",
    "  ........   ",
    "  .BBBBBB.   ",
    "  .BBBBBB.   ",
    "  .BBBBBB.   ",
    "  .BBBBBB.   ",
    " .B......B.  ",
    ".B....B...B. ",
    ".B....B...B. ",
    ".B....B...BB.",
    ".B.BBBB...BB.",
    ".B........B. ",
    ".B........B. ",
    " .B......B.  ",
    "  .BBBBBB.   ",
    "  .BBBBBB.   ",
    "  .BBBBBB.   ",
    "  .BBBBBB.   ",
    "  ........   ",
];

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
        wind.handle({
            let mut x = 0;
            let mut y = 0;
            move |w, ev| match ev {
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
            }
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
    let cursor = image::Pixmap::new(WAIT_CURSOR).unwrap();
    image::RgbImage::from_pixmap(&cursor)
}
