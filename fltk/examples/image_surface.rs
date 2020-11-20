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
    let img = sur.image().unwrap();
    surface::ImageSurface::pop_current();

    frame.set_image(Some(img));

    // We need the destructor of SvgFileSurface to actually create the image
    {
        let sur = surface::SvgFileSurface::new(but.width(), but.height(), "temp.svg");
        surface::SvgFileSurface::push_current(&sur);
        draw::set_draw_color(Color::White);
        draw::draw_rectf(0, 0, but.width(), but.height());
        sur.draw(&but, 0, 0);
        surface::SvgFileSurface::pop_current();
    }

    app::delete_widget(but);

    app.run().unwrap();
}
