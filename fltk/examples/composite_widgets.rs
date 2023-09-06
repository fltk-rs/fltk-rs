use fltk::{prelude::*, *};
use std::ops::{Deref, DerefMut};

struct MyButton {
    grp: group::Group,
}

impl MyButton {
    pub fn new(w: i32, h: i32) -> MyButton {
        let mut grp = group::Group::new(0, 0, w, h, None);
        grp.set_frame(enums::FrameType::RFlatBox);
        grp.set_color(enums::Color::from_u32(0x01579b));
        grp.set_align(enums::Align::Center);
        let mut btn = button::Button::new(grp.x() + 420, grp.y() + 35, 30, 25, "@1+");
        btn.set_frame(enums::FrameType::OFlatFrame);
        btn.set_color(enums::Color::from_u32(0xf49da9));
        btn.set_callback(move |b| b.parent().unwrap().hide());
        grp.end();
        grp.handle(|g, ev| match ev {
            enums::Event::Push => {
                g.do_callback();
                true
            }
            _ => false,
        });
        MyButton { grp }
    }
}

impl Deref for MyButton {
    type Target = group::Group;
    fn deref(&self) -> &Self::Target {
        &self.grp
    }
}
impl DerefMut for MyButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grp
    }
}

fn main() {
    let app = app::App::default();
    app::set_visible_focus(false);
    let mut win = window::Window::default().with_size(500, 400);
    win.make_resizable(true);
    win.set_color(enums::Color::Black);
    let mut col = group::Flex::default_fill().column();
    col.set_margin(20);
    col.set_pad(10);

    for i in 0..3 {
        let label = format!("Button {}", i + 1);
        let mut but = MyButton::new(500, 100);
        but.set_label(&label);
        but.set_callback(move |_| println!("{label}"));
    }

    col.end();
    win.end();
    win.show();
    app.run().unwrap();
}
