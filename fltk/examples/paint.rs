use fltk::{
    app,
    draw::{
        draw_line, draw_point, draw_rect_fill, set_draw_color, set_line_style, LineStyle, Offscreen,
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
        draw_rect_fill(0, 0, WIDTH - 10, HEIGHT - 10, Color::White);
        offs.end();
    }

    let offs = Rc::from(RefCell::from(offs));

    frame.draw({
        let offs = offs.clone();
        let mut scf = app::screen_scale(wind.screen_num());
        move |_| {
            let mut offs = offs.borrow_mut();
            let s =  app::screen_scale(wind.screen_num());
            if s != scf {
                offs.rescale();
                scf = s;
            }
            offs.copy(5, 5, WIDTH - 10, HEIGHT - 10, 0, 0);
        }
    });

    frame.handle({
        let mut x = 0;
        let mut y = 0;
        move |f, ev| {
            // println!("{}", ev);
            // println!("coords {:?}", app::event_coords());
            // println!("get mouse {:?}", app::get_mouse());
            let offs = offs.borrow_mut();
            match ev {
                Event::Push => {
                    offs.begin();
                    set_draw_color(Color::Red);
                    set_line_style(LineStyle::Solid, 3);
                    let coords = app::event_coords();
                    x = coords.0;
                    y = coords.1;
                    draw_point(x, y);
                    offs.end();
                    f.redraw();
                    set_line_style(LineStyle::Solid, 0);
                    true
                }
                Event::Drag => {
                    offs.begin();
                    set_draw_color(Color::Red);
                    set_line_style(LineStyle::Solid, 3);
                    let coords = app::event_coords();
                    draw_line(x, y, coords.0, coords.1);
                    x = coords.0;
                    y = coords.1;
                    offs.end();
                    f.redraw();
                    set_line_style(LineStyle::Solid, 0);
                    true
                }
                _ => false,
            }
        }
    });

    app.run().unwrap();
}
