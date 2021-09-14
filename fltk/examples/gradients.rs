#![allow(clippy::many_single_char_names)]

use fltk::{draw::*, enums::*, prelude::*, *};

fn create_vertical_gradient_frame(
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    col1: Color,
    col2: Color,
) -> frame::Frame {
    let mut frame = frame::Frame::new(x, y, w, h, "Vertical");
    frame.draw(move |f| {
        let imax = f.h();
        let d = if imax > 0 { imax } else { 1 };
        for i in 0..=imax {
            let w = 1.0 - i as f32 / d as f32;
            set_draw_color(Color::inactive(&Color::color_average(col1, col2, w)));
            draw_xyline(f.x(), f.y() + i, f.x() + f.w());
        }
        set_draw_color(Color::Black);
        draw_text2(&f.label(), f.x(), f.y(), f.w(), f.h(), f.align());
    });
    frame
}

fn create_horizontal_gradient_frame(
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    col1: Color,
    col2: Color,
) -> frame::Frame {
    let mut frame = frame::Frame::new(x, y, w, h, "Horizontal");
    frame.draw(move |f| {
        let imax = f.w();
        let d = if imax > 0 { imax } else { 1 };
        for i in 0..=imax {
            let w = 1.0 - i as f32 / d as f32;
            set_draw_color(Color::inactive(&Color::color_average(col1, col2, w)));
            draw_yxline(f.x() + i, f.y(), f.y() + f.h());
        }
        set_draw_color(Color::Black);
        draw_text2(&f.label(), f.x(), f.y(), f.w(), f.h(), f.align());
    });
    frame
}

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    create_vertical_gradient_frame(0, 0, 200, 100, Color::Red, Color::Cyan);
    create_horizontal_gradient_frame(200, 0, 200, 100, Color::Red, Color::Cyan);
    win.end();
    win.draw(|w| {
        // vertical gradient
        let imax = w.w();
        let d = if imax > 0 { imax } else { 1 };
        for i in 0..=imax {
            let v = 1.0 - i as f32 / d as f32;
            set_draw_color(Color::color_average(Color::Red, Color::Blue, v));
            draw_yxline(i, 0, w.h());
        }
        w.draw_children();
    });
    win.make_resizable(true);
    win.show();
    a.run().unwrap();
}
