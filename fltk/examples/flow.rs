use fltk::{prelude::*, *};

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(640, 480);
    let mut flow = group::Flow::default_fill();
    let mut btn = button::Button::default().with_size(80, 30).with_label("Click");
    flow.rule(&btn, "^<");
    win.end();
    win.resizable(&flow);
    win.show();
    a.run().unwrap();
}