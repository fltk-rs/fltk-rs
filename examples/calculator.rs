#![allow(unused_variables)]
#![allow(unused_mut)]

use fltk::{app, button::*, output::*, window::*};
use std::rc::Rc;
use std::cell::RefCell;

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

    let mut op = Rc::from(RefCell::from(String::from("_")));
    let mut txt = Rc::from(RefCell::from(String::from("0")));
    let mut old_val = Rc::from(RefCell::from(String::from("0")));
    let mut new_val = String::from("0");

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
    let mut but0 = Button::new(column2, row5, but_width *2, but_height, "0");

    but_ce.set_color(Color::from_u32(0xff0000));
    but_c.set_color(Color::Yellow);
    but_back.set_color(Color::Yellow);
    but_div.set_color(Color::Yellow);
    but_mul.set_color(Color::Yellow);
    but_sub.set_color(Color::Yellow);
    but_add.set_color(Color::Yellow);
    but_eq.set_color(Color::Yellow);
    but_dot.set_color(Color::Light2);

    wind.make_resizable(false);
    wind.end();
    wind.show();

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
    ];

    let but_op_vec = vec![
        but_add,
        but_sub,
        but_mul,
        but_div,
    ];

    for but in but_vec {
        but.set_color(Color::Light2);
        let mut out = out.clone();
        let mut txt = txt.clone();
        let op = op.clone();
        let label = but.label();
        app::set_callback(
            but,
            Box::new(move || {
                if out.value() == "0" {
                    txt.borrow_mut().clear();
                }
                if op.borrow().as_str() == "=" {
                    txt.borrow_mut().clear();
                    op.borrow_mut().clear();
                    op.borrow_mut().push_str("_");
                }
                txt.borrow_mut().push_str(&label);
                out.set_value(txt.borrow().as_str());
            }),
        );
    }

    for mut but in but_op_vec {
        let mut txt = txt.clone();
        let out = out.clone();
        let old_val = old_val.clone();
        let op = op.clone();
        let label = but.label().clone();
        app::set_callback(
            &mut but,
            Box::new(move || {
                old_val.borrow_mut().clear();
                old_val.borrow_mut().push_str(&out.value());
                op.borrow_mut().clear();
                op.borrow_mut().push_str(&label);
                txt.borrow_mut().clear();
                txt.borrow_mut().push('0');
                out.set_value(txt.borrow().as_str());
            }),
        );
    }

    let mut txt_cloned = txt.clone();
    let old_val_cloned = old_val.clone();
    let mut out_c = out.clone();
    app::set_callback(
        & mut but_ce,
        Box::new(move || {
            txt_cloned.borrow_mut().clear();
            old_val_cloned.borrow_mut().clear();
            txt_cloned.borrow_mut().push('0');
            out_c.set_value(txt_cloned.borrow().as_str());
        }),
    );

    let mut txt_cloned = txt.clone();
    let mut out_c = out.clone();
    app::set_callback(
        & mut but_c,
        Box::new(move || {
            txt_cloned.borrow_mut().clear();
            txt_cloned.borrow_mut().push('0');
            out_c.set_value(txt_cloned.borrow().as_str());
        }),
    );

    let mut txt_cloned = txt.clone();
    let mut out_c = out.clone();
    app::set_callback(
        & mut but_back,
        Box::new(move || {
            let val = out_c.value();
            if val.len() > 1 {
                txt_cloned.borrow_mut().pop();
                out_c.set_value(txt_cloned.borrow().as_str());
            }
        }),
    );

    let mut txt_cloned = txt.clone();
    let op_cloned = op.clone();
    let mut out_c = out.clone();
    app::set_callback(
        & mut but_eq,
        Box::new(move || {
            new_val = out_c.value();
            let old: f64 = old_val.borrow().parse().unwrap();
            let new: f64 = new_val.parse().unwrap();
            let val = match op_cloned.borrow().as_str() {
                "/" => old / new,
                "x" => old * new,
                "+" => old + new,
                "-" => old - new,
                _ => new,
            };
            op_cloned.borrow_mut().clear();
            op_cloned.borrow_mut().push_str("_");
            txt_cloned.borrow_mut().clear();
            txt_cloned.borrow_mut().push_str(val.to_string().as_str());
            out_c.set_value(txt_cloned.borrow().as_str());
        }),
    );

    let mut out_c = out.clone();
    app::set_callback(
        & mut but_dot,
        Box::new(move || {
            if op.borrow().as_str() == "=" {
                txt.borrow_mut().clear();
                op.borrow_mut().clear();
                op.borrow_mut().push_str("_");
                out_c.set_value("0.");
                txt.borrow_mut().push_str("0.");
            }
            if !txt.borrow().contains(".") {
                txt.borrow_mut().push_str(".");
                out.set_value(txt.borrow().as_str());
            }
        }),
    );

    app.run().expect("Couldn't run calculator!");
}
