use fltk::{prelude::*, button::Button, window::Window};

fn main() {
    let mut wind = Window::new().set(100, 100, 400, 300, "Hello from rust");
    wind.begin();

    let mut but1 = Button::new().set(80, 100, 80, 60, "Click me!");
    let mut but2 = Button::new().set(240, 100, 80, 60, "Click me!");
    let mut but3 = Button::new().set(160, 180, 80, 60, "Click me!");

    fl::register_callback(&but1.clone(), &mut || match fl::event() {
        fl::Event::Released => {
            println!("{:?}", fl::event());
            wind.set_label("Clicked button 1!");
            but1.set_label("Works");
            but2.set_label("No!");
            but3.activate();
        }
        _ => println!("{:?}", fl::event()),
    });

    fl::register_callback(&but2.clone(), &mut || match fl::event() {
        fl::Event::Released => {
            println!("{:?}", fl::event());
            wind.set_label("Clicked button 2!");
            but1.set_label("No!");
            but2.set_label("Works");
            but3.deactivate();
        }
        _ => println!("{:?}", fl::event()),
    });

    fl::register_callback(&but3, &mut || println!("You've clicked button 3"));

    wind.end();
    wind.show();
    fl::run();
}
