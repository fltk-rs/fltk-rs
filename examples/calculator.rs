use fltk::{button::Button, frame::Frame, output::*, window::*};

fn main() {
    let mut txt = String::from("0");
    let mut old_val = String::from("0");
    let mut new_val = String::from("0");
    let mut op = '_';

    let mut wind = Window::new().set(200, 200, 400, 500, "FLTK-RS Calculator");
    let _lbl = Frame::new().set(20, 20, 360, 20, "");
    let mut out = Output::new().set(20, 60, 360, 100, "");
    out.set_align(Align::AlignRight);
    out.set_text_size(30);
    out.set_value(txt.as_str());

    let but_ce = Button::new().set(20, 180, 90, 60, "CE");
    let but_c = Button::new().set(110, 180, 90, 60, "C");
    let but_back = Button::new().set(200, 180, 90, 60, "@<-");
    let but_div = Button::new().set(290, 180, 90, 60, "/");
    let but_mul = Button::new().set(290, 240, 90, 60, "x");
    let but_sub = Button::new().set(290, 300, 90, 60, "-");
    let but_add = Button::new().set(290, 360, 90, 60, "+");
    let but_eq = Button::new().set(290, 420, 90, 60, "=");

    let but7 = Button::new().set(20, 240, 90, 60, "7");
    let but8 = Button::new().set(110, 240, 90, 60, "8");
    let but9 = Button::new().set(200, 240, 90, 60, "9");
    let but4 = Button::new().set(20, 300, 90, 60, "4");
    let but5 = Button::new().set(110, 300, 90, 60, "5");
    let but6 = Button::new().set(200, 300, 90, 60, "6");
    let but1 = Button::new().set(20, 360, 90, 60, "1");
    let but2 = Button::new().set(110, 360, 90, 60, "2");
    let but3 = Button::new().set(200, 360, 90, 60, "3");
    let but_dot = Button::new().set(20, 420, 90, 60, ".");
    let but0 = Button::new().set(110, 420, 180, 60, "0");

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
        op = '_';
        txt.clear();
        txt.push_str(val.to_string().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but0, &mut || {
        if out.value() == "0" {
            //
        } else {
            txt.push_str(but0.label().as_str());
            out.set_value(txt.as_str());
        }
    });

    fl::register_callback(&but1, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        txt.push_str(but1.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but2, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        txt.push_str(but2.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but3, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        txt.push_str(but3.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but4, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        txt.push_str(but4.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but5, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        txt.push_str(but5.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but6, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        txt.push_str(but6.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but7, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        txt.push_str(but7.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but8, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        txt.push_str(but8.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but9, &mut || {
        if out.value() == "0" {
            txt.clear();
        }
        txt.push_str(but9.label().as_str());
        out.set_value(txt.as_str());
    });

    fl::register_callback(&but_dot, &mut || {
        if !out.value().contains(".") {
            txt.push_str(but_dot.label().as_str());
            out.set_value(txt.as_str());
        }
    });

    wind.end();
    wind.show();
    fl::run();
}
