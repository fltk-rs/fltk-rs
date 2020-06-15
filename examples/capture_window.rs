use fltk::*;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::new(100, 100, 400, 300, "");
    let mut but = button::Button::new(160, 240, 80, 40, "Capture!");
    win.end();
    win.show();
    but.set_callback(Box::new(move || { 
        let image = draw::capture_window(&mut win).unwrap().into_jpeg().unwrap();
        image.write_to_file(&std::path::PathBuf::from("test.jpg")).unwrap();
    }));
    app.run().unwrap();
}