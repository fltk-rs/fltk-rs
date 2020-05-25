use fltk::{app, button::*, output::*, window::*};

#[derive(Debug, Copy, Clone, PartialEq)]
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

fn main() {
    let app = app::App::default().set_scheme(app::AppScheme::Gleam);
    let border = 20;
    let but_width = 90;
    let but_height = 60;
    let column1 = border;
    let column2 = but_width + border;
    let column3 = but_width * 2 + border;
    let column4 = but_width * 3 + border;
    let row1 = 180;
    let row2 = but_height + 180;
    let row3 = but_height * 2 + 180;
    let row4 = but_height * 3 + 180;
    let row5 = but_height * 4 + 180;

    let mut operation = Ops::None;
    let mut txt = String::from("0");
    let mut old_val = String::from("0");
    let mut new_val: String;

    let mut wind = Window::default()
        .with_label("FLTK Calc")
        .with_size(400, 500)
        .center_screen();
    wind.set_color(Color::Light2);

    let mut out = Output::new(border, border, 360, 140, "");
    out.set_text_size(30);
    out.set_value("0");

    let but_ce = Button::new(column1, row1, but_width, but_height, "CE");
    let but_c = Button::new(column2, row1, but_width, but_height, "C");
    let but_back = Button::new(column3, row1, but_width, but_height, "@<-");
    let but_div = Button::new(column4, row1, but_width, but_height, "/");
    let but_mul = Button::new(column4, row2, but_width, but_height, "x");
    let but_sub = Button::new(column4, row3, but_width, but_height, "-");
    let but_add = Button::new(column4, row4, but_width, but_height, "+");
    let but_eq = Button::new(column4, row5, but_width, but_height, "=");

    let mut but7 = Button::new(column1, row2, but_width, but_height, "7");
    let mut but8 = Button::new(column2, row2, but_width, but_height, "8");
    let mut but9 = Button::new(column3, row2, but_width, but_height, "9");
    let mut but4 = Button::new(column1, row3, but_width, but_height, "4");
    let mut but5 = Button::new(column2, row3, but_width, but_height, "5");
    let mut but6 = Button::new(column3, row3, but_width, but_height, "6");
    let mut but1 = Button::new(column1, row4, but_width, but_height, "1");
    let mut but2 = Button::new(column2, row4, but_width, but_height, "2");
    let mut but3 = Button::new(column3, row4, but_width, but_height, "3");
    let mut but_dot = Button::new(column1, row5, but_width, but_height, ".");
    let mut but0 = Button::new(column2, row5, but_width * 2, but_height, "0");

    but_dot.set_color(Color::Light2);

    wind.make_resizable(false);
    wind.end();
    wind.show();

    let but_vec = vec![
        &mut but1, &mut but2, &mut but3, &mut but4, &mut but5, &mut but6, &mut but7, &mut but8,
        &mut but9, &mut but0,
    ];

    let but_op_vec = vec![
        but_add, but_sub, but_mul, but_div, but_c, but_ce, but_back, but_eq,
    ];

    let (s, r) = app::channel::<Message>();

    for but in but_vec {
        but.set_color(Color::Light2);
        let label = but.label();
        but.emit(s, Message::Number(label.parse().unwrap()));
    }

    for mut but in but_op_vec {
        let label = but.label().clone();
        if label.as_str() == "CE" {
            but.set_color(Color::Red);
        } else {
            but.set_color(Color::Yellow);
        }
        let op = match label.as_str() {
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
        but.emit( s, Message::Op(op));
    }

    but_dot.emit(s, Message::Dot);

    while app.wait().unwrap() {
        match r.recv() {
            Some(val) => match val {
                Message::Number(num) => {
                    if out.value() == "0" {
                        txt.clear();
                    }
                    txt.push_str(&num.to_string());
                    out.set_value(txt.as_str());
                }
                Message::Dot => {
                    if operation == Ops::Eq {
                        txt.clear();
                        operation = Ops::None;
                        out.set_value("0.");
                        txt.push_str("0.");
                    }
                    if !txt.contains(".") {
                        txt.push_str(".");
                        out.set_value(txt.as_str());
                    }
                }
                Message::Op(op) => match op {
                    Ops::Add | Ops::Sub | Ops::Div | Ops::Mul => {
                        old_val.clear();
                        old_val.push_str(&out.value());
                        operation = op;
                        out.set_value("0");
                    }
                    Ops::Back => {
                        let val = out.value();
                        if val.len() > 1 {
                            txt.pop();
                            out.set_value(txt.as_str());
                        }
                    }
                    Ops::CE => {
                        txt.clear();
                        old_val.clear();
                        txt.push('0');
                        out.set_value(txt.as_str());
                    }
                    Ops::C => {
                        txt.clear();
                        txt.push('0');
                        out.set_value(txt.as_str());
                    }
                    Ops::Eq => {
                        new_val = out.value();
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
                        out.set_value(&val.to_string());
                    }
                    _ => (),
                },
            },
            None => (),
        }
    }
}
