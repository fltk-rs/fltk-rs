use fltk::{button::*, window::*};

fn main() {
    draw_elements();
}

fn draw_elements() {
    let mut wind = Window::new().set(0, 0, 400, 300, "Hello from rust");

    let mut but1 = Button::new().set(80, 80, 80, 60, "Click me!");
    let mut but2 = Button::new().set(240, 80, 80, 60, "Click me!");
    let mut but3 = Button::new().set(160, 160, 80, 60, "Click me!");

    fl::register_callback(&but1.clone(), &mut || match fl::event() {
        fl::Event::Released => {
            println!("{:?}", fl::event());
            // wind.set_label("Clicked button 1!");
            but1.set_label("Works");
            but2.set_label("No!");
        }
        _ => println!("{:?}", fl::event()),
    });

    fl::register_callback(&but2.clone(), &mut || match fl::event() {
        fl::Event::Released => {
            println!("{:?}", fl::event());
            // wind.set_label("Clicked button 2!");
            but1.set_label("No!");
            but2.set_label("Works");
        }
        _ => println!("{:?}", fl::event()),
    });

    but3.set_callback(&mut || println!("{:?}", fl::event()));

    wind.end();
    wind.show();
    fl::run();
}
