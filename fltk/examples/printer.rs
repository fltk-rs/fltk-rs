use fltk::*;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::new(100, 100, 400, 300, "");
    let mut but = button::Button::new(160, 200, 80, 40, "Click");
    win.end();
    win.show();

    but.set_callback2(|widget| {
        let mut printer = printer::Printer::default();
        if printer.begin_job(1).is_ok() {
            printer.begin_page();
            let (width, height) = printer.printable_rect();
            draw::set_draw_color(Color::Black);
            draw::set_line_style(draw::LineStyle::Solid, 2);
            draw::draw_rect(0, 0, width, height);
            draw::set_font(Font::Courier, 12);
            printer.set_origin(width / 2, height / 2);
            printer.print_widget(widget, -widget.width() / 2, -widget.height() / 2);
            printer.end_page();
            printer.end_job();
        }
    });

    app.run().unwrap();
}
