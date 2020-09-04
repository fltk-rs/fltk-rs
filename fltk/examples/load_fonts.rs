use fltk::*;

fn main() {
    let app = app::App::default();
    app::load_font("/opt/X11/share/fonts/TTF/luximb.ttf", "LuxiMono-Bold").unwrap();
    let mut wind = window::Window::new(300, 200, 400, 300, "Fonts");
    let mut frame = frame::Frame::new(0, 0, 400, 100, "Hello");
    frame.set_label_size(30);
    frame.set_label_font(Font::by_name("LuxiMono-Bold"));
    wind.set_color(Color::White);
    wind.end();
    wind.show();
    while app.wait().unwrap() {}
}