use fltk::{app, draw::*, frame::*, window::*};

fn main() {
    let app = app::App::default().set_scheme(app::AppScheme::Gtk);
    let mut wind = Window::new(100, 100, 800, 600, "RustyPainter");
    let mut frame = Frame::new(5, 5, 790, 590, "");
    frame.set_color(Color::White);
    frame.set_frame(FrameType::DownBox);
    let mut x = 0;
    let mut y = 0;
    wind.end();
    wind.show();
    frame.handle(Box::new(move |ev| {
        // println!("{:?}", ev);
        set_draw_color(Color::Red);
        set_line_style(LineStyle::Solid, 3);
        match ev {
            app::Event::Push => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                draw_point(x, y);
                true
            }
            app::Event::Drag => {
                let coords = app::event_coords();
                // println!("{:?}", coords);
                if coords.0 < 5 || coords.0 > 795 || coords.1 < 5 || coords.1 > 595 {
                    return false;
                }
                draw_line(x, y, coords.0, coords.1);
                x = coords.0;
                y = coords.1;
                true
            }
            _ => false,
        }
    }));
    app.run().unwrap();
}
