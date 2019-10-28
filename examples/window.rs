use fltk::{button::Button, fl, widget, widget::WidgetTrait, window::Window};

fn main() {
    let mut wind = Window::new().set(100, 100, 400, 300, "Hello from rust");
    wind.begin();
    let mut but1 = Button::new().set(80, 100, 80, 60, "Click me!");
    let mut but2 = Button::new().set(240, 100, 80, 60, "Click me!");

    widget::register_callback(&but1.clone(), &mut || match fl::event() {
        fl::Event::Released => {
            println!("{:?}", fl::event());
            wind.set_label("Clicked button 1!");
            but1.set_label("Works");
            but2.set_label("No!");
        }
        _ => println!("{:?}", fl::event()),
    });

    widget::register_callback(&but2.clone(), &mut || match fl::event() {
        fl::Event::Released => {
            println!("{:?}", fl::event());
            wind.set_label("Clicked button 2!");
            but1.set_label("No!");
            but2.set_label("Works");
        }
        _ => println!("{:?}", fl::event()),
    });

    wind.end();
    wind.show();
    fl::run();
}
