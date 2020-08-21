use fltk::{app::*, window::*};

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");

    wind.end();
    wind.show();

    unsafe {
        let h = wind.raw_handle();
        let mut w = Window::find_by_handle(h).unwrap();
        w.set_color(Color::White);
        // wind.set_raw_handle(h);
    }

    app.run().unwrap();
}
