#![allow(unused_variables)]
#![allow(unused_mut)]

use fltk::{app, button::*, output::*, window::*};

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

    let mut txt = String::from("0");
    let mut old_val = String::from("0");
    let mut new_val = String::from("0");
    let mut op = '_';

    let mut wind = Window::default()
        .with_label("FLTK Calc")
        .with_size(400, 500)
        .center_screen();
    wind.set_color(Color::Light2);

    let mut out = Output::new(border, border, 360, 140, "");
    out.set_text_size(30);
    out.set_value("0");

    let mut but_ce = Button::new(column1, row1, but_width, but_height, "CE");
    let mut but_c = Button::new(column2, row1, but_width, but_height, "C");
    let mut but_back = Button::new(column3, row1, but_width, but_height, "@<-");
    let mut but_div = Button::new(column4, row1, but_width, but_height, "/");
    let mut but_mul = Button::new(column4, row2, but_width, but_height, "x");
    let mut but_sub = Button::new(column4, row3, but_width, but_height, "-");
    let mut but_add = Button::new(column4, row4, but_width, but_height, "+");
    let mut but_eq = Button::new(column4, row5, but_width, but_height, "=");

    // but_ce.set_color(Color::from_rgb(255, 0, 0));
    but_ce.set_color(Color::from_u32(0xff0000));
    but_c.set_color(Color::Yellow);
    but_back.set_color(Color::Yellow);
    but_div.set_color(Color::Yellow);
    but_mul.set_color(Color::Yellow);
    but_sub.set_color(Color::Yellow);
    but_add.set_color(Color::Yellow);
    but_eq.set_color(Color::Yellow);

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
    let mut but0 = Button::default()
        .with_pos(column2, row5)
        .with_size(but_width * 2, but_height)
        .with_label("0");

    let but_vec = vec![
        &mut but1,
        &mut but2,
        &mut but3,
        &mut but4,
        &mut but5,
        &mut but6,
        &mut but7,
        &mut but8,
        &mut but9,
        &mut but0,
        &mut but_dot,
    ];

    for but in but_vec {
        but.set_color(Color::Light2);
    }

    wind.make_resizable(false);
    wind.end();
    wind.show();

    app::set_callback(
        & mut but_ce,
        Box::new(|| {
            txt.clear();
            old_val.clear();
            txt.push('0');
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but_c,
        Box::new(|| {
            txt.clear();
            txt.push('0');
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but_back,
        Box::new(|| {
            txt.pop();
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but_div,
        Box::new(|| {
            old_val = out.value();
            op = '/';
            txt.clear();
            txt.push('0');
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but_mul,
        Box::new(|| {
            old_val = out.value();
            op = 'x';
            txt.clear();
            txt.push('0');
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but_sub,
        Box::new(|| {
            old_val = out.value();
            op = '-';
            txt.clear();
            txt.push('0');
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but_add,
        Box::new(|| {
            old_val = out.value();
            op = '+';
            txt.clear();
            txt.push('0');
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but_eq,
        Box::new(|| {
            new_val = out.value();
            let old: f64 = old_val.parse().unwrap();
            let new: f64 = new_val.parse().unwrap();
            let val = match op {
                '/' => old / new,
                'x' => old * new,
                '+' => old + new,
                '-' => old - new,
                _ => new,
            };
            op = '=';
            txt.clear();
            txt.push_str(val.to_string().as_str());
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but0,
        Box::new(|| {
            if out.value() == "0" {
                //
            } else {
                if op == '=' {
                    txt.clear();
                    out.set_value("");
                    op = '_';
                }
                txt.push_str("0");
                out.set_value(txt.as_str());
            }
        }),
    );

    app::set_callback(
        & mut but1,
        Box::new(|| {
            if out.value() == "0" {
                txt.clear();
            }
            if op == '=' {
                txt.clear();
                op = '_';
            }
            txt.push_str("1");
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but2,
        Box::new(|| {
            if out.value() == "0" {
                txt.clear();
            }
            if op == '=' {
                txt.clear();
                op = '_';
            }
            txt.push_str("2");
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but3,
        Box::new(|| {
            if out.value() == "0" {
                txt.clear();
            }
            if op == '=' {
                txt.clear();
                op = '_';
            }
            txt.push_str("3");
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but4,
        Box::new(|| {
            if out.value() == "0" {
                txt.clear();
            }
            if op == '=' {
                txt.clear();
                op = '_';
            }
            txt.push_str("4");
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but5,
        Box::new(|| {
            if out.value() == "0" {
                txt.clear();
            }
            if op == '=' {
                txt.clear();
                op = '_';
            }
            txt.push_str("5");
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but6,
        Box::new(|| {
            if out.value() == "0" {
                txt.clear();
            }
            if op == '=' {
                txt.clear();
                op = '_';
            }
            txt.push_str("6");
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but7,
        Box::new(|| {
            if out.value() == "0" {
                txt.clear();
            }
            if op == '=' {
                txt.clear();
                op = '_';
            }
            txt.push_str("7");
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but8,
        Box::new(|| {
            if out.value() == "0" {
                txt.clear();
            }
            if op == '=' {
                txt.clear();
                op = '_';
            }
            txt.push_str("8");
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but9,
        Box::new(|| {
            if out.value() == "0" {
                txt.clear();
            }
            if op == '=' {
                txt.clear();
                op = '_';
            }
            txt.push_str("9");
            out.set_value(txt.as_str());
        }),
    );

    app::set_callback(
        & mut but_dot,
        Box::new(|| {
            if op == '=' {
                txt.clear();
                op = '_';
                out.set_value("0.");
                txt.push_str("0.");
            }
            if !txt.contains(".") {
                txt.push_str(".");
                out.set_value(txt.as_str());
            }
        }),
    );
    app.run().expect("Couldn't run calculator!");
}
