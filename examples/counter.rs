use fltk::{app, button::*, frame::*, window::*};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let app = app::App::default().set_scheme(app::AppScheme::Gtk);
    let mut wind = Window::default()
        .with_size(160, 200)
        .center_screen()
        .with_label("Counter");
    wind.make_resizable(true);
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
    let frame = Rc::from(RefCell::from(frame));
    let frame1 = frame.clone();
    but_inc.set_callback(Box::new(move || {
        let label = (frame1.borrow().label().parse::<i32>().unwrap() + 1).to_string();
        frame1.borrow_mut().set_label(&label);
    }));
    let frame1 = frame.clone();
    but_dec.set_callback(Box::new(move || {
        let label = (frame1.borrow().label().parse::<i32>().unwrap() - 1).to_string();
        frame1.borrow_mut().set_label(&label);
    }));
    // Theming
    wind.set_color(Color::from_u32(0xffebee));
    but_inc.set_color(Color::from_u32(0x304FFE));
    but_inc.set_selection_color(Color::Green);
    but_inc.set_label_size(18);
    but_inc.set_frame(FrameType::RoundUpBox);
    but_dec.set_color(Color::from_u32(0x2962FF));
    but_dec.set_selection_color(Color::Red);
    but_dec.set_frame(FrameType::RoundUpBox);
    but_dec.set_label_size(18);
    // End theming
    app.run().unwrap();
}
