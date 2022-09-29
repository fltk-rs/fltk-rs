use fltk::{
    app,
    draw::{draw_line, draw_point, draw_rect_fill, set_draw_color, set_line_style, LineStyle},
    enums::{Color, Event, FrameType},
    frame::Frame,
    prelude::*,
    surface::ImageSurface,
    window::Window,
};
use std::cell::RefCell;
use std::rc::Rc;

struct Canvas {
    frame: Frame,
    #[allow(dead_code)]
    surf: Rc<RefCell<ImageSurface>>,
}

impl Canvas {
    pub fn new(w: i32, h: i32) -> Self {
        let mut frame = Frame::default().with_size(w, h).center_of_parent();
        frame.set_color(Color::White);
        frame.set_frame(FrameType::DownBox);

        let surf = ImageSurface::new(frame.width(), frame.height(), false);
        ImageSurface::push_current(&surf);
        draw_rect_fill(0, 0, w, h, Color::White);
        ImageSurface::pop_current();

        let surf = Rc::from(RefCell::from(surf));

        frame.draw({
            let surf = surf.clone();
            move |f| {
                let surf = surf.borrow();
                let mut img = surf.image().unwrap();
                img.draw(f.x(), f.y(), f.w(), f.h());
            }
        });

        frame.handle({
            let mut x = 0;
            let mut y = 0;
            let surf = surf.clone();
            move |f, ev| {
                // println!("{}", ev);
                // println!("coords {:?}", app::event_coords());
                // println!("get mouse {:?}", app::get_mouse());
                let surf = surf.borrow_mut();
                match ev {
                    Event::Push => {
                        ImageSurface::push_current(&surf);
                        set_draw_color(Color::Red);
                        set_line_style(LineStyle::Solid, 3);
                        let coords = app::event_coords();
                        x = coords.0;
                        y = coords.1;
                        draw_point(x, y);
                        ImageSurface::pop_current();
                        f.redraw();
                        true
                    }
                    Event::Drag => {
                        ImageSurface::push_current(&surf);
                        set_draw_color(Color::Red);
                        set_line_style(LineStyle::Solid, 3);
                        let coords = app::event_coords();
                        draw_line(x, y, coords.0, coords.1);
                        x = coords.0;
                        y = coords.1;
                        ImageSurface::pop_current();
                        f.redraw();
                        true
                    }
                    _ => false,
                }
            }
        });
        Self { frame, surf }
    }
}

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fltk::widget_extends!(Canvas, Frame, frame);

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let mut wind = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("RustyPainter");

    Canvas::new(WIDTH - 10, HEIGHT - 10);

    wind.end();
    wind.show();

    app.run().unwrap();
}
