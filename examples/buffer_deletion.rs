use fltk::*;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::new(100, 100, 800, 600, "Test");
    let mut buf = text::TextBuffer::default();
    let mut _editor = text::TextEditor::new(0, 0, 800, 600, buf);
    win.show();

    win.set_callback(Box::new(move || {
        if app::event() == enums::Event::Close {
            println!("Closing, but deleting the buffer first!");
            unsafe {
                buf.delete();
            }
            app.quit();
        }
    }));

    app.run().unwrap();
}