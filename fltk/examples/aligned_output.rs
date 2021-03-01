use fltk::*;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
struct RtlInput {
    g: group::Group,
    f: frame::Frame,
    i: input::Input,
}

impl RtlInput {
    pub fn new(x: i32, y: i32, w: i32, h: i32, title: &str) -> Self {
        let mut g = group::Group::new(x, y, w, h, title);
        let mut f = frame::Frame::new(x, y, w, h, "");
        let mut i = input::Input::new(x, y, w, h, "");
        g.end();
        g.set_frame(FrameType::NoBox);
        g.set_align(Align::Right);
        f.set_align(Align::Right | Align::Inside);
        f.set_color(Color::White);
        f.set_frame(FrameType::DownBox);
        i.hide();
        let mut i_c = i.clone();
        let mut f_c = f.clone();
        g.handle(|ev| match ev {
            Event::Focus => {
                if !i_c.shown() {
                    f_c.hide();
                }
                true
            },
            Event::Unfocus => {
                true
            },
            _ => false,
        });
        RtlInput {
            g,
            f,
            i,
        }
    }
}

impl Deref for RtlInput {
    type Target = input::Input;

    fn deref(&self) -> &Self::Target {
        &self.i
    }
}

impl DerefMut for RtlInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.i
    }
}

fn main() {
    let app = app::App::default();
    let mut win = window::Window::new(100, 100, 400, 300, "");
    let mut inp = RtlInput::new(80, 100, 200, 40, " أدخل الاسم");
    win.end();
    win.show();
    app.run().unwrap();
}
