use fltk::{app, button::*, draw, frame::*, window::*};
use std::ops::{Deref, DerefMut};

struct FlatButton {
    button: Button,
}

impl FlatButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> FlatButton {
        let mut x = FlatButton {
            button: Button::new(x, y, w, h, label),
        };
        x.draw();
        x
    }
    fn draw(&mut self) {
        let b = self.button.clone();
        self.button.draw(Box::new(move || {
            draw::set_draw_color(Color::from_u32(0xFFC300));
            draw::draw_rectf(b.x(), b.y(), b.width(), b.height());
            draw::set_draw_color(Color::Black);
            draw::draw_text_angled(-15, &b.label(), b.x() + 2, b.y() + 15);
        }));
    }
}

impl Deref for FlatButton {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.button
    }
}

impl DerefMut for FlatButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.button
    }
}

fn main() {
    let app = app::App::default().set_scheme(app::AppScheme::Gtk);

    let mut wind = Window::new(200, 100, 400, 300, "Custom Widget");
    wind.set_color(Color::from_u32(0x6D4C41));
    let mut my_wid = FlatButton::new(160, 200, 80, 40, "Increment");
    let mut frame = Frame::new(0, 0, 400, 300, "0");

    wind.end();
    wind.show();

    my_wid.set_callback(Box::new(move || {
        frame.set_label(&(frame.label().parse::<i32>().unwrap() + 1).to_string())
    }));

    app.run().unwrap();
}
