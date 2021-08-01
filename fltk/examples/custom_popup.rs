use fltk::{enums::*, prelude::*, *};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct PopupButton {
    but: button::Button,
}

impl PopupButton {
    pub fn new(label: &str) -> Self {
        let mut but = button::Button::default().with_label(label);
        but.set_frame(FrameType::FlatBox);
        but.handle(|b, ev| match ev {
            Event::Enter => {
                b.set_color(Color::Blue);
                b.top_window().unwrap().redraw();
                true
            }
            Event::Leave => {
                b.set_color(Color::BackGround);
                b.top_window().unwrap().redraw();
                true
            }
            _ => false,
        });
        Self { but }
    }
}

impl Deref for PopupButton {
    type Target = button::Button;

    fn deref(&self) -> &Self::Target {
        &self.but
    }
}

impl DerefMut for PopupButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.but
    }
}

pub struct MyPopup {
    win: window::Window,
    val: Rc<RefCell<String>>,
}

impl MyPopup {
    pub fn new(choices: &[&str]) -> Self {
        let val = Rc::from(RefCell::from(String::from("")));
        let mut win = window::Window::default().with_size(120, choices.len() as i32 * 25);
        let mut pack = group::Pack::default().size_of_parent();
        pack.set_frame(FrameType::ThinUpFrame);
        pack.set_color(Color::Black);
        win.set_border(false);
        win.make_modal(true);
        win.end();
        for choice in choices {
            let mut but = PopupButton::new(choice);
            but.clear_visible_focus();
            but.set_callback({
                let mut win = win.clone();
                let val = val.clone();
                move |b| {
                    *val.borrow_mut() = b.label();
                    win.hide();
                }
            });
            pack.add(&*but);
        }
        pack.auto_layout();
        Self { win, val }
    }
    pub fn popup(&mut self, x: i32, y: i32) -> String {
        self.win.show();
        self.win.set_pos(x, y);
        self.win.redraw();
        while self.win.shown() {
            app::wait();
        }
        self.val.borrow().to_string()
    }
}

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let mut frame = frame::Frame::default()
        .with_size(200, 100)
        .with_label("Right click me!")
        .center_of_parent();
    frame.set_frame(FrameType::BorderFrame);
    frame.set_color(Color::Red);
    win.end();
    win.show();
    let mut menu = MyPopup::new(&["1st item", "2nd item", &format!("{:?}", frame.frame())]);
    frame.handle(move |_, ev| match ev {
        Event::Push => {
            if app::event_mouse_button() == app::MouseButton::Right {
                println!("{}", menu.popup(app::event_x_root(), app::event_y_root()));
                true
            } else {
                false
            }
        }
        _ => false,
    });
    app.run().unwrap();
}
