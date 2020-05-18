use fltk::{app::*, button::*, tree::*, window::*};

fn main() {
    let app = App::default().set_scheme(AppScheme::Gleam);
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut but = Button::new(160, 260, 80, 40, "Get Items");
    let mut tree = Tree::new(0, 0, 400, 250, "");
    tree.set_select_mode(TreeSelect::Multi);
    tree.add("First");
    tree.add("First/1st");
    tree.add("First/2nd/3rd");
    tree.add("Second");
    tree.add("Third");
    let mut f = tree.first().unwrap();
    // let mut f2 = tree.first().unwrap();
    f.set_label("Hi");
    wind.make_resizable(true);
    wind.end();
    wind.show();
    println!("{}", f.label());
    but.set_callback(Box::new(move || match tree.get_selected_items() {
        None => println!("No items selected"),
        Some(vals) => println!("{} items selected", vals.len()),
    }));
    app.run().unwrap();
}
