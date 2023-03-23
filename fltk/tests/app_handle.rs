use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400, 300)
        .center_screen()
        .with_label("handle");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");

    wind.show();

    but.set_callback(move |_| {
        std::thread::spawn(move || {
            for i in 31..1000 {
                app::sleep(0.010);
                app::awake();
                let ret = app::handle_main(i);
                if let Ok(ret) = ret {
                    println!("Handled? {ret}");
                }
            }
        });
    });

    frame.handle(|f, ev| {
        if ev > 30.into() {
            f.set_label(&format!("{ev}"));
            true
        } else {
            false
        }
    });

    app.run().unwrap();
}
