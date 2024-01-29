use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window, enums::Color};

fn main() {
    let app = app::App::default();
    app::set_color(Color::White, 0x00, 0xff, 0xff);
    assert_eq!(Color::White.to_rgb(), (0x00, 0xff, 0xff));
    let mut wind = Window::default().with_size(400, 300);
    let mut frame = Frame::default().with_size(200, 100).center_of(&wind);
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();

    but.set_callback(move |_| frame.set_label("Hello world"));

    app.run().unwrap();
}
