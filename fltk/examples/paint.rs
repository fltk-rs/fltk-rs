use fltk::{
    app,
    draw::{
        draw_line, draw_point, draw_rectf, set_draw_color, set_line_style, LineStyle, Offscreen,
    },
    enums::{Color, Event, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};
use std::cell::RefCell;
use std::rc::Rc;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let mut wind = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("RustyPainter");
    let mut frame = Frame::default()
        .with_size(WIDTH - 10, HEIGHT - 10)
        .center_of(&wind);
    frame.set_color(Color::White);
    frame.set_frame(FrameType::DownBox);

    wind.end();
    wind.show();

    let offs = Offscreen::new(frame.width(), frame.height()).unwrap();
    #[cfg(not(target_os = "macos"))]
    {
        offs.begin();
        set_draw_color(Color::White);
        draw_rectf(0, 0, WIDTH - 10, HEIGHT - 10);
        offs.end();
    }

    let offs = Rc::from(RefCell::from(offs));
    let offs_rc = offs.clone();

    frame.draw(move |_| {
        if offs_rc.borrow().is_valid() {
            offs_rc.borrow().copy(5, 5, WIDTH - 10, HEIGHT - 10, 0, 0);
        } else {
            offs_rc.borrow_mut().begin();
            set_draw_color(Color::White);
            draw_rectf(0, 0, WIDTH - 10, HEIGHT - 10);
            offs_rc.borrow_mut().end();
        }
    });

    let mut x = 0;
    let mut y = 0;

    frame.handle(move |f, ev| match ev {
        // println!("{:?}", ev);
        // println!("coords {:?}", app::event_coords());
        // println!("get mouse {:?}", app::get_mouse());
        Event::Push => {
            offs.borrow().begin();
            set_draw_color(Color::Red);
            set_line_style(LineStyle::Solid, 3);
            let coords = app::event_coords();
            x = coords.0;
            y = coords.1;
            draw_point(x, y);
            offs.borrow().end();
            f.redraw();
            true
        }
        Event::Drag => {
            offs.borrow().begin();
            set_draw_color(Color::Red);
            set_line_style(LineStyle::Solid, 3);
            let coords = app::event_coords();
            draw_line(x, y, coords.0, coords.1);
            x = coords.0;
            y = coords.1;
            offs.borrow().end();
            f.redraw();
            true
        }
        _ => false,
    });

    app.run().unwrap();
}
