use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

#[test]
fn idle() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400, 300)
        .center_screen()
        .with_label("Idle");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");

    wind.show();

    let (s, r) = app::channel::<i32>();

    but.set_callback(move |_| {
        std::thread::spawn(move || {
            for i in 0..1000 {
                app::sleep(0.010);
                s.send(i);
            }
        });
    });

    app::add_idle(move || {
        if let Some(msg) = r.recv() {
            frame.set_label(format!("Hello {}", msg).as_str());
        }
    });

    app.run().unwrap();
}
