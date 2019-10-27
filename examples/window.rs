use fltk::button::Button;
use fltk::fl;
use fltk::window::Window;
use fltk::widget::WidgetTrait;

fn main() {
    let mut wind = Window::new().set(100, 100, 400, 300, "Hello from rust");
    wind.begin();
    let mut but = Button::new().set(150, 100, 80, 60, "Click me!");
    but.add_callback(|| println!("Works"));
    wind.end();
    wind.show();
    fl::run();
}
