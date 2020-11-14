use fltk::*;

fn main() {

    let app = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let mut pack = group::Pack::default().size_of(&win);
    button::Button::default();
    button::Button::default();
    button::Button::default();
    button::Button::default();
    pack.end();
    pack.auto_layout();
    pack.make_resizable(true);
    win.make_resizable(true);
    win.end();
    win.show();

    app.run().unwrap();
}