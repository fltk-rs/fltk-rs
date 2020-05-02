use fltk::{app, button::*, frame::*, window::*};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

fn main() {
    let app = app::App::default().set_scheme(app::AppScheme::Gtk);
    let mut wind = Window::default()
        .with_size(160, 200)
        .center_screen()
        .with_label("Counter");
    let mut frame = Frame::default()
        .with_size(100, 40)
        .center_of(&wind)
        .with_label("0");
    frame.set_label_size(20);
    let mut but_inc = Button::default()
        .size_of(&frame)
        .above_of(&frame, 0)
        .with_label("+");
    let mut but_dec = Button::default()
        .size_of(&frame)
        .below_of(&frame, 0)
        .with_label("-");
    wind.make_resizable(true);
    wind.end();
    wind.show();
    let (s, r) = app::channel::<Message>();
    but_inc.set_callback(Box::new(move || s.send(Message::Increment)));
    but_dec.set_callback(Box::new(move || s.send(Message::Decrement)));
    
    while app.wait() {
        let msg = r.recv();
        let label: i32 = frame.label().parse().unwrap();
        match msg {
            Some(Message::Increment) => frame.set_label(&(label + 1).to_string()),
            Some(Message::Decrement) => frame.set_label(&(label - 1).to_string()),
            None => (),
        }
    }
}
