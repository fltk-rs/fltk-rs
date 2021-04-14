use fltk::{
    app,
    button::Button,
    prelude::*,
    tree::{Tree, TreeSelect},
    window::Window,
};
use std::env;

fn main() {
    let path = env::current_dir().unwrap();
    let path: String = path
        .to_str()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(_, c)| match c {
            '\\' => '/', // change window paths to posix paths
            _ => c,
        })
        .collect();

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default().with_size(400, 300);
    let mut but = Button::new(160, 255, 80, 40, "Get Items");
    let mut tree = Tree::new(5, 10, 190, 240, "");
    tree.add(&path);

    let mut items = tree.get_items().unwrap();
    items.as_mut_slice()[0].set_label("/");

    let mut tree2 = Tree::new(205, 10, 190, 240, "");
    tree2.set_select_mode(TreeSelect::Multi);
    tree2.add("First");
    tree2.add("First/1st");
    tree2.add("First/2nd/3rd");
    tree2.add("Second");
    tree2.add("Third");

    wind.make_resizable(true);
    wind.show();

    but.set_callback(move |_| match tree2.get_selected_items() {
        None => println!("No items selected"),
        Some(vals) => println!("{} items selected", vals.as_slice()[0].label().unwrap()),
    });

    app.run().unwrap();
}
