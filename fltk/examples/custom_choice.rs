#![allow(dead_code)]

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
        but.set_color(Color::White);
        but.handle(|b, ev| match ev {
            Event::Enter => {
                b.set_color(Color::Blue);
                b.top_window().unwrap().redraw();
                true
            }
            Event::Leave => {
                b.set_color(Color::White);
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
    idx: RefCell<i32>,
}

impl MyPopup {
    pub fn new(choices: &[&str]) -> Self {
        let val = Rc::from(RefCell::from(String::from("")));
        let idx = RefCell::from(0);
        let mut win = window::Window::default().with_size(120, choices.len() as i32 * 25);
        win.set_color(Color::White);
        win.set_frame(FrameType::BorderBox);
        let mut pack = group::Pack::new(1, 1, win.w() - 2, win.h() - 2, None);
        win.set_border(false);
        win.end();
        for (i, choice) in choices.iter().enumerate() {
            let mut but = PopupButton::new(choice);
            but.clear_visible_focus();
            but.set_callback({
                let mut win = win.clone();
                let val = val.clone();
                let idx = idx.clone();
                move |b| {
                    *val.borrow_mut() = b.label();
                    *idx.borrow_mut() = i as i32;
                    win.hide();
                }
            });
            pack.add(&*but);
        }
        pack.auto_layout();
        Self { win, val, idx }
    }
    pub fn popup(&mut self, x: i32, y: i32) -> (String, i32) {
        self.win.show();
        self.win.handle(|w, ev| match ev {
            Event::Unfocus => {
                w.hide();
                true
            }
            _ => false,
        });
        self.win.force_position(true);
        self.win.set_pos(x, y);
        while self.win.shown() {
            app::wait();
        }
        (self.val.borrow().to_string(), *self.idx.borrow())
    }
}

struct MyChoice {
    grp: group::Group,
    frame: frame::Frame,
    btn: button::Button,
    choices: Rc<RefCell<Vec<&'static str>>>,
}

impl MyChoice {
    pub fn new<S: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: S) -> Self {
        let grp = group::Group::new(x, y, w, h, label).with_align(Align::Left);
        let mut frame = frame::Frame::new(x, y, w - w / 4, h, None);
        frame.set_frame(FrameType::DownBox);
        frame.set_color(Color::BackGround2);
        let mut btn = button::Button::new(x + w - w / 4, y, w / 4, h, "@2>");
        btn.clear_visible_focus();
        grp.end();
        let choices = Rc::from(RefCell::from(vec![]));
        btn.set_callback({
            let c = choices.clone();
            let mut f = frame.clone();
            let btn_win = btn.window().unwrap();
            move |b| {
                let mut menu = MyPopup::new(&c.borrow());
                let s = menu.popup(b.x() + btn_win.x() - f.w(), b.y() + btn_win.y() + b.h());
                f.set_label(&s.0);
            }
        });
        Self {
            grp,
            frame,
            btn,
            choices,
        }
    }

    pub fn add_choices(&mut self, choices: &[&'static str]) {
        *self.choices.borrow_mut() = choices.to_vec();
    }

    pub fn button(&mut self) -> &mut button::Button {
        &mut self.btn
    }

    pub fn frame(&mut self) -> &mut frame::Frame {
        &mut self.frame
    }

    pub fn group(&mut self) -> &mut group::Group {
        &mut self.grp
    }

    pub fn set_current_choice(&mut self, idx: i32) {
        self.frame.set_label(self.choices.borrow()[idx as usize])
    }

    pub fn choice(&self) -> String {
        self.frame.label()
    }

    pub fn value(&self) -> i32 {
        let choice = self.choice();
        if let Some(val) = self.choices.borrow().iter().position(|x| x == &choice) {
            val as _
        } else {
            -1
        }
    }
}

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let mut choice = MyChoice::new(160, 200, 100, 30, None);
    choice.add_choices(&["choice1", "choice2", "choice3"]);
    choice.set_current_choice(1);
    choice.button().set_frame(FrameType::BorderBox);
    choice.frame().set_frame(FrameType::BorderBox);
    win.end();
    win.show();
    app.run().unwrap();
}
