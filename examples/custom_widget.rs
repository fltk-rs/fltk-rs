use fltk::{app, draw, frame::*, widget::*, window::*};
use std::ops::{Deref, DerefMut};

struct FlatButton {
    wid: Widget,
}

impl FlatButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> FlatButton {
        let mut x = FlatButton {
            wid: Widget::new(x, y, w, h, label),
        };
        x.draw();
        x.handle();
        x
    }

    // Overrides the draw function
    fn draw(&mut self) {
        let b = self.wid.clone();
        self.wid.draw(Box::new(move || {
            draw::draw_box(
                FrameType::GtkDownBox,
                b.x(),
                b.y(),
                b.width(),
                b.height(),
                Color::from_u32(0xFFC300),
            );
            draw::set_draw_color(Color::Black);
            draw::draw_text_angled(-15, &b.label(), b.x() + 2, b.y() + 15);
        }));
    }

    // Overrides the handle function.
    // Notice the do_callback which allows the set_callback method to work
    fn handle(&mut self) {
        let mut wid = self.wid.clone();
        self.wid.handle(Box::new(move |ev| match ev {
            Event::Push => {
                wid.do_callback();
                true
            }
            _ => false,
        }));
    }
}

impl Deref for FlatButton {
    type Target = Widget;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for FlatButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}

fn main() {
    let app = app::App::default().set_scheme(app::AppScheme::Gtk);

    let mut wind = Window::new(200, 100, 400, 300, "Custom Widget");
    wind.set_color(Color::from_u32(0x6D4C41));
    let mut but = FlatButton::new(160, 200, 80, 40, "Increment");
    let mut frame = Frame::new(0, 0, 400, 300, "0");
    frame.set_label_color(Color::from_u32(0xFFC300));

    wind.end();
    wind.show();

    but.set_callback(Box::new(move || {
        frame.set_label(&(frame.label().parse::<i32>().unwrap() + 1).to_string())
    }));

    app.run().unwrap();
}
