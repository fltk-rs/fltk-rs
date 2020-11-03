use fltk::*;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::new(100, 100, 400, 300, "");
    let mut frame = frame::Frame::new(160, 200, 80, 40, "");
    win.end();
    win.show();

    let but = button::Button::new(0, 0, 80, 40, "Click");

    let sur = surface::ImageSurface::new(but.width(), but.height(), false);
    surface::ImageSurface::push_current(&sur);
    draw::set_draw_color(Color::White);
    draw::draw_rectf(0, 0, but.width(), but.height());
    sur.draw(&but, 0, 0);
    app::delete_widget(but);
    let img = sur.image().unwrap();
    surface::ImageSurface::pop_current();

    frame.set_image(Some(img));

    app.run().unwrap();
}
