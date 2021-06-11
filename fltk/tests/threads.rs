use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400, 300)
        .center_screen()
        .with_label("threads");
    let frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");

    wind.show();

    but.set_callback(move |_| {
        let mut frame = frame.clone();
        std::thread::spawn(move || {
            for i in 0..1000 {
                app::sleep(0.010);
                frame.set_label(format!("Hello {}", i).as_str());
            }
        });
    });

    app.run().unwrap();
}
