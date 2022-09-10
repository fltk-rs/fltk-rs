use fltk::{
    app,
    button::Button,
    enums::{Align, Color, Event, FrameType, Key, Shortcut},
    frame::Frame,
    group::{Pack, PackType},
    prelude::*,
    window::Window,
};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Ops {
    None,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    CE,
    C,
    Back,
}

#[derive(Debug, Copy, Clone)]
enum Message {
    Number(i32),
    Op(Ops),
    Dot,
}

struct MyButton {
    btn: Button,
}

impl MyButton {
    pub fn new(title: &'static str) -> MyButton {
        let mut b = MyButton {
            btn: Button::new(0, 0, 100, 0, title),
        };
        b.set_label_size(24);
        b.set_frame(FrameType::FlatBox);
        match title {
            "CE" => {
                b.set_color(Color::from_hex(0xd50000));
                b.set_shortcut(Shortcut::None | Key::Delete);
            }
            "x" | "/" | "+" | "-" | "=" | "C" | "@<-" => {
                b.set_color(Color::from_hex(0xffee58));
                b.set_label_color(Color::Black);
                let shortcut = if title == "x" {
                    '*'
                } else {
                    title.chars().next().unwrap()
                };
                b.set_shortcut(Shortcut::None | shortcut);
                if shortcut == '@' {
                    b.set_shortcut(Shortcut::None | Key::BackSpace);
                }
                if shortcut == '=' {
                    b.set_shortcut(Shortcut::None | Key::Enter);
                }
            }
            _ => {
                if title == "0" {
                    b.resize(0, 0, 100 * 2, 0);
                }
                b.set_label_color(Color::White);
                b.set_selection_color(Color::from_hex(0x1b1b1b));
                b.set_shortcut(Shortcut::None | title.chars().next().unwrap());
                b.handle(move |b, ev| match ev {
                    Event::Enter => {
                        b.set_color(Color::from_hex(0x2b2b2b));
                        b.redraw();
                        true
                    }
                    Event::Leave => {
                        b.set_color(Color::from_hex(0x424242));
                        b.redraw();
                        true
                    }
                    _ => false,
                });
            }
        }
        b
    }
}

impl Deref for MyButton {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.btn
    }
}

impl DerefMut for MyButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.btn
    }
}

fn main() {
    let app = app::App::default();
    app::set_visible_focus(false);
    app::background(0x42, 0x42, 0x42);

    let win_w = 400;
    let win_h = 500;
    let but_row = 160;

    let mut operation = Ops::None;
    let mut txt = String::from("0");
    let mut old_val = String::from("0");
    let mut new_val: String;

    let mut wind = Window::default()
        .with_label("FLTK Calc")
        .with_size(win_w, win_h)
        .center_screen();

    let mut out = Frame::new(0, 0, win_w, 160, "").with_align(Align::Right | Align::Inside);
    out.set_color(Color::from_hex(0x1b1b1b));
    out.set_frame(FrameType::FlatBox);
    out.set_label_color(Color::White);
    out.set_label_size(36);
    out.set_label("0");

    let vpack = Pack::new(0, but_row, win_w, win_h - 170, "");

    let mut hpack = Pack::new(0, 0, win_w, 68, "");
    let but_ce = MyButton::new("CE");
    let but_c = MyButton::new("C");
    let but_back = MyButton::new("@<-");
    let but_div = MyButton::new("/");
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0, 0, win_w, 68, "");
    let mut but7 = MyButton::new("7");
    let mut but8 = MyButton::new("8");
    let mut but9 = MyButton::new("9");
    let but_mul = MyButton::new("x");
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0, 0, win_w, 68, "");
    let mut but4 = MyButton::new("4");
    let mut but5 = MyButton::new("5");
    let mut but6 = MyButton::new("6");
    let but_sub = MyButton::new("-");
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0, 0, win_w, 68, "");
    let mut but1 = MyButton::new("1");
    let mut but2 = MyButton::new("2");
    let mut but3 = MyButton::new("3");
    let but_add = MyButton::new("+");
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0, 0, win_w, 68, "");
    let mut but_dot = MyButton::new(".");
    let mut but0 = MyButton::new("0");
    let but_eq = MyButton::new("=");
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    vpack.end();

    wind.make_resizable(false);
    wind.end();
    wind.show();

    app::set_focus(&*but1);
    app::get_system_colors();

    let but_vec = vec![
        &mut but1, &mut but2, &mut but3, &mut but4, &mut but5, &mut but6, &mut but7, &mut but8,
        &mut but9, &mut but0,
    ];

    let but_op_vec = vec![
        but_add, but_sub, but_mul, but_div, but_c, but_ce, but_back, but_eq,
    ];

    let (s, r) = app::channel::<Message>();

    for but in but_vec {
        let label = but.label();
        but.emit(s, Message::Number(label.parse().unwrap()));
    }

    for mut but in but_op_vec {
        let op = match but.label().as_str() {
            "+" => Ops::Add,
            "-" => Ops::Sub,
            "x" => Ops::Mul,
            "/" => Ops::Div,
            "=" => Ops::Eq,
            "CE" => Ops::CE,
            "C" => Ops::C,
            "@<-" => Ops::Back,
            _ => Ops::None,
        };
        but.emit(s, Message::Op(op));
    }

    but_dot.emit(s, Message::Dot);

    while app.wait() {
        if let Some(val) = r.recv() {
            match val {
                Message::Number(num) => {
                    if out.label() == "0" {
                        txt.clear();
                    }
                    txt.push_str(&num.to_string());
                    out.set_label(txt.as_str());
                }
                Message::Dot => {
                    if operation == Ops::Eq {
                        txt.clear();
                        operation = Ops::None;
                        out.set_label("0.");
                        txt.push_str("0.");
                    }
                    if !txt.contains('.') {
                        txt.push('.');
                        out.set_label(txt.as_str());
                    }
                }
                Message::Op(op) => match op {
                    Ops::Add | Ops::Sub | Ops::Div | Ops::Mul => {
                        old_val.clear();
                        old_val.push_str(&out.label());
                        operation = op;
                        out.set_label("0");
                    }
                    Ops::Back => {
                        let val = out.label();
                        txt.pop();
                        if val.len() > 1 {
                            out.set_label(txt.as_str());
                        } else {
                            out.set_label("0");
                        }
                    }
                    Ops::CE => {
                        txt.clear();
                        old_val.clear();
                        txt.push('0');
                        out.set_label(txt.as_str());
                    }
                    Ops::C => {
                        txt.clear();
                        txt.push('0');
                        out.set_label(txt.as_str());
                    }
                    Ops::Eq => {
                        new_val = out.label();
                        let old: f64 = old_val.parse().unwrap();
                        let new: f64 = new_val.parse().unwrap();
                        let val = match operation {
                            Ops::Div => old / new,
                            Ops::Mul => old * new,
                            Ops::Add => old + new,
                            Ops::Sub => old - new,
                            _ => new,
                        };
                        operation = Ops::None;
                        txt = String::from("0");
                        out.set_label(&val.to_string());
                    }
                    _ => (),
                },
            }
        }
    }
}
