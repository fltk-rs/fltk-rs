use fltk::*;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);

    let mut pack = group::Pack::default().size_of(&win);
    pack.set_spacing(10);

    button::Button::default();
    button::Button::default();
    button::Button::default();

    let mut hpack = group::Pack::default().with_size(400, 0);
    hpack.set_spacing(10);
    hpack.set_type(group::PackType::Horizontal);

    button::Button::default();
    button::Button::default();
    button::Button::default();

    hpack.end();
    hpack.auto_layout();

    pack.end();
    pack.auto_layout();

    win.end();
    win.show();

    app.run().unwrap();
}
