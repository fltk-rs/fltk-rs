use fltk::{app::*, button::*, menu::*, window::*};

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");

    wind.end();
    wind.show();

    but.set_callback(Box::new(move || {
        let v = vec!["1st val", "2nd val", "3rd val"];
        let mut x = MenuItem::new(v);
        match x.popup(100, 100) {
            None => println!("No value was chosen!"),
            Some(val) => println!("{}", val.label().unwrap()),
        }
    }));

    app.run().unwrap();
}