use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

trait App {
    fn run(self);
}

#[derive(Copy, Clone)]
struct Counter {
    count: i32,
}

impl App for Counter {
    fn run(self) {
        app::GlobalState::new(self);
        let a = app::App::default().with_scheme(app::Scheme::Gtk);
        let (s, r) = app::channel();
        let mut wind = Window::default().with_size(160, 200).with_label("Counter");
        view(s);
        wind.end();
        wind.show();
        while a.wait() {
            if let Some(msg) = r.recv() {
                if msg {
                    wind.clear();
                    wind.begin();
                    view(s);
                    wind.end();
                }
            }
        }
    }
}

fn view(s: app::Sender<bool>) {
    let state = app::GlobalState::<Counter>::get();
    let mut frame = Frame::default()
        .with_size(100, 40)
        .center_of_parent()
        .with_label(&state.with(|c| c.count).to_string());
    frame.set_label_size(20);
    let mut but_inc = Button::default()
        .size_of(&frame)
        .above_of(&frame, 0)
        .with_label("+");
    let mut but_dec = Button::default()
        .size_of(&frame)
        .below_of(&frame, 0)
        .with_label("-");

    but_inc.set_callback(move |_| {
        state.with(|c| c.count += 1);
        s.send(true);
    });

    but_dec.set_callback(move |_| {
        state.with(|c| c.count -= 1);
        s.send(true);
    });
}

fn main() {
    Counter { count: 0 }.run();
}
