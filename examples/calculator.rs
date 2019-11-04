use fltk::{button::Button, output::Output, prelude::*, window::Window};

fn main() {
    let (screen_width, screen_height) = fl::screen_size();
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

    let mut wind = Window::new().set(
        (screen_width / 2.0 - 200.0) as i32,
        (screen_height / 2.0 - 250.0) as i32,
        400,
        500,
        "FLTK-RS Calculator",
    );
    wind.set_color(Color::Light2);

    let mut out = Output::new().set(border, border, 360, 140, "");
    out.set_align(Align::AlignRight);
    out.set_text_size(30);
    out.set_value("0");

    let mut but_ce = Button::new().set(column1, row1, but_width, but_height, "CE");
    let mut but_c = Button::new().set(column2, row1, but_width, but_height, "C");
    let mut but_back = Button::new().set(column3, row1, but_width, but_height, "@<-");
    let mut but_div = Button::new().set(column4, row1, but_width, but_height, "/");
    let mut but_mul = Button::new().set(column4, row2, but_width, but_height, "x");
    let mut but_sub = Button::new().set(column4, row3, but_width, but_height, "-");
    let mut but_add = Button::new().set(column4, row4, but_width, but_height, "+");
    let mut but_eq = Button::new().set(column4, row5, but_width, but_height, "=");

    but_ce.set_color(Color::Red);
    but_c.set_color(Color::Yellow);
    but_back.set_color(Color::Yellow);
    but_div.set_color(Color::Yellow);
    but_mul.set_color(Color::Yellow);
    but_sub.set_color(Color::Yellow);
    but_add.set_color(Color::Yellow);
    but_eq.set_color(Color::Yellow);

    let mut but7 = Button::new().set(column1, row2, but_width, but_height, "7");
    let mut but8 = Button::new().set(column2, row2, but_width, but_height, "8");
    let mut but9 = Button::new().set(column3, row2, but_width, but_height, "9");
    let mut but4 = Button::new().set(column1, row3, but_width, but_height, "4");
    let mut but5 = Button::new().set(column2, row3, but_width, but_height, "5");
    let mut but6 = Button::new().set(column3, row3, but_width, but_height, "6");
    let mut but1 = Button::new().set(column1, row4, but_width, but_height, "1");
    let mut but2 = Button::new().set(column2, row4, but_width, but_height, "2");
    let mut but3 = Button::new().set(column3, row4, but_width, but_height, "3");
    let mut but_dot = Button::new().set(column1, row5, but_width, but_height, ".");
    let mut but0 = Button::new().set(column2, row5, but_width * 2, but_height, "0");

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

    for but in but_vec.into_iter() {
        but.set_color(Color::Light2);
    }

    fl::register_callback(&but_ce, &mut || {
        txt.clear();
        old_val.clear();
        txt.push('0');
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but_c, &mut || {
        txt.clear();
        txt.push('0');
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but_back, &mut || {
        txt.pop();
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but_div, &mut || {
        old_val = out.value();
        op = '/';
        txt.clear();
        txt.push('0');
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but_mul, &mut || {
        old_val = out.value();
        op = 'x';
        txt.clear();
        txt.push('0');
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but_sub, &mut || {
        old_val = out.value();
        op = '-';
        txt.clear();
        txt.push('0');
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but_add, &mut || {
        old_val = out.value();
        op = '+';
        txt.clear();
        txt.push('0');
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but_eq, &mut || {
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
    });

    fl::register_callback(&but0, &mut || {
        if out.value() == "0" {
            //
        } else {
            if op == '=' {
                txt.clear();
                out.set_value("");
                op = '_';
            }
            txt.push_str(but0.label().as_str());
            out.set_value(txt.as_str());
        }
    });

    fl::register_callback(&but1, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        if op == '=' {
            txt.clear();
            op = '_';
        }
        txt.push_str(but1.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but2, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        if op == '=' {
            txt.clear();
            op = '_';
        }
        txt.push_str(but2.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but3, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        if op == '=' {
            txt.clear();
            op = '_';
        }
        txt.push_str(but3.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but4, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        if op == '=' {
            txt.clear();
            op = '_';
        }
        txt.push_str(but4.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but5, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        if op == '=' {
            txt.clear();
            op = '_';
        }
        txt.push_str(but5.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but6, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        if op == '=' {
            txt.clear();
            op = '_';
        }
        txt.push_str(but6.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but7, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        if op == '=' {
            txt.clear();
            op = '_';
        }
        txt.push_str(but7.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but8, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        if op == '=' {
            txt.clear();
            op = '_';
        }
        txt.push_str(but8.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but9, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        if op == '=' {
            txt.clear();
            op = '_';
        }
        txt.push_str(but9.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but_dot, &mut || {
        if op == '=' {
            txt.clear();
            op = '_';
            out.set_value("0.");
            txt.push_str("0.");
        }
        if !out.value().contains(".") && !txt.contains(".") {
            txt.push_str(but_dot.label().as_str());
            out.set_value(txt.as_str());
        }
    });

    wind.end();
    wind.show();
    fl::run();
}
