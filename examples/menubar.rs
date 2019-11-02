use fltk::{menu::*, window::*};

fn main() {
    let mut wind = Window::new().set(100, 100, 400, 300, "Hello from rust");
    let mut menu = MenuBar::new().set(0, 0, 400, 40, "");
    menu.add("file", 0, &mut || println!("{:?}", fl::event()), 0);
    menu.add("open", 0, &mut || println!("{:?}", fl::event()), 0);
    wind.end();
    wind.show();
    fl::run();
}