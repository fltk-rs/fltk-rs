use fltk::*;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Clone)]
struct MyOutput {
    f: frame::Frame,
    val: Rc<RefCell<String>>,
}

impl MyOutput {
    pub fn new(x: i32, y: i32, width: i32, height: i32, align: Align) -> Self {
        let mut o = MyOutput {
            f: frame::Frame::new(x, y, width, height, ""),
            val: Rc::from(RefCell::from(String::from(""))),
        };
        let v = o.val.clone();
        o.f.draw(Box::new(move || {
            draw::push_clip(x, y, width, height);
            draw::draw_box(FrameType::DownBox, x, y, width, height, Color::White);
            draw::set_draw_color(Color::Black);
            draw::draw_text2(&v.borrow(), x, y, width, height, align);
            draw::pop_clip();
        }));
        o
    }
    pub fn set_value(&mut self, val: &str) {
        *self.val.borrow_mut() = String::from(val);
        self.f.redraw();
    }
}

impl Deref for MyOutput {
    type Target = frame::Frame;

    fn deref(&self) -> &Self::Target {
        &self.f
    }
}

impl DerefMut for MyOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.f
    }
}

fn main() {
    let app = app::App::default();
    let mut win = window::Window::new(100, 100, 400, 300, "");
    let mut out = MyOutput::new(20, 100, 360, 40, Align::Center);
    let mut but = button::Button::new(160, 200, 80, 40, "Click Me!");
    win.end();
    win.show();
    but.set_callback(Box::new(move || {
        out.set_value("Clicked!");
    }));
    app.run().unwrap();
}
