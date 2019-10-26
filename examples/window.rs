use fltk::button::Button;
use fltk::fl;
use fltk::widget::Widget;
use fltk::window::Window;

fn main() {
    let mut wind = Window::new(100, 100, 400, 300, "hello from rust");
    wind.begin();
    let _but = Button::new(150, 100, 80, 60, "click!");
    wind.end();
    wind.show();
    fl::run();
}
