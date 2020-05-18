use fltk::{app::*, tree::*, window::*};

fn main() {
    let app = App::default().set_scheme(AppScheme::Gleam);
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut tree = Tree::new(0, 0, 400, 250, "");
    tree.add("First");
    tree.add("First/1st");
    tree.add("First/2nd/3rd");
    tree.add("Second");
    tree.add("Third");
    tree.set_selectmode(TreeSelect::Multi);
    let mut but = fltk::button::Button::new(160, 260, 80, 40, "Get Items");
    wind.make_resizable(true);
    wind.end();
    wind.show();
    but.set_callback(Box::new(move ||  println!("{}", tree.get_selected_items().unwrap().len()) ));
    app.run().unwrap();
}
