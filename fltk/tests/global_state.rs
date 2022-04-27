use fltk::{
    app,
    button::Button,
    frame::Frame,
    prelude::*,
    window::Window,
};

#[derive(Copy, Clone)]
struct Counter {
    count: i32,
}

fn view() {
    let state = app::GlobalState::<Counter>::get();
    let sender = app::Sender::<bool>::get();
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
        sender.send(true);
    });

    but_dec.set_callback(move |_| {
        state.with(|c| c.count -= 1);
        sender.send(true);
    });
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let _state = app::GlobalState::new(Counter { count: 0 });
    let (_, r) = app::channel();
    let mut wind = Window::default().with_size(160, 200).with_label("Counter");
    view();
    wind.end();
    wind.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            if msg {
                wind.clear();
                wind.begin();
                view();
                wind.end();
            }
        }
    }
}
