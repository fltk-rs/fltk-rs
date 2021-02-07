use fltk::*;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::new(100, 100, 400, 300, "Help View");
    let mut h = misc::HelpView::new(10, 10, 380, 280, "");
    h.set_value("Hello <b><font color=red>again</font></b>");
    println!("{:?}", h.find("again", 0));
    win.end();
    win.show();
    app.run().unwrap();
}
