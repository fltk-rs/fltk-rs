// FLTK offers 16 fonts by default. However, it's possible to load all system fonts to be able to choose from them:
// The following are the default FLTK fonts:
// - Helvetica,
// - HelveticaBold,
// - HelveticaItalic,
// - HelveticaBoldItalic,
// - Courier,
// - CourierBold,
// - CourierItalic,
// - CourierBoldItalic,
// - Times,
// - TimesBold,
// - TimesItalic,
// - TimesBoldItalic,
// - Symbol,
// - Screen,
// - ScreenBold,
// - Zapfdingbats,
//
// The system fonts depend on the system, and are not loaded by default. 
// These can be loaded using the app::set_fonts("*") function. Or any function calling it
// such as get_font_count or get_font_names. The fonts can then be used using the Font::by_index() or
// Font::by_name() methods.

use fltk::*;

fn main() {
    let font_count = app::get_font_count(); // Will load all system fonts
    let app = app::App::default();
    let mut wind = window::Window::new(300, 200, 400, 300, "Fonts");
    let mut frame = frame::Frame::new(0, 0, 400, 300, "");
    frame.set_label_size(30);
    wind.set_color(Color::White);
    wind.end();
    wind.show();
    println!("The system has {} fonts!\nStarting slideshow!", font_count);
    let mut i = 0;
    while app.wait().unwrap() {
        frame.set_label(&app::get_font_name(i).unwrap());
        frame.set_label_font(Font::by_index(i));
        std::thread::sleep(std::time::Duration::from_millis(500));
        i += 1;
        if i == font_count {
            i = 0;
        }
    }
}