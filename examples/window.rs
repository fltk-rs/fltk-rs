use fltk::fl;
use fltk::window::Window;
use fltk::button::Button;

fn main() {
    let mut wind = Window::new(100, 100, 400, 300, "hello from rust");
    wind.begin();
    let mut but = Button::new(150, 100, 80, 60, "click me!");
    wind.end();
    but.add_callback(&mut |mut but| {
        but.set_label("works");
        but.redraw();
        });
    wind.show();
    fl::run();
}