use fltk::*;
use std::{env, fs};

fn main() {
    let mut paths = vec![];
    let current_dir = env::current_dir().unwrap();
    let path: String = current_dir
        .to_str()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(_, c)| match c {
            '\\' => '/', // change window paths to posix paths
            _ => c,
        })
        .collect();
    let files = fs::read_dir(".").unwrap();

    for file in files {
        let entry = file.unwrap();
        let typ = entry.file_type().unwrap();
        if !typ.is_dir() && !typ.is_symlink() {
            paths.push(
                std::path::PathBuf::from(path.clone())
                    .join(entry.file_name().into_string().unwrap()),
            );
        }
    }

    let text_buffer = text::TextBuffer::default();

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(800, 600);
    let mut tree = tree::Tree::new(5, 10, 300, 580, "");
    let mut disp = text::TextDisplay::default().with_size(480, 580).right_of(&tree, 10);

    wind.make_resizable(true);
    wind.show();

    disp.set_buffer(text_buffer);

    for path in paths {
        tree.add(&path.into_os_string().into_string().unwrap());
    }

    let mut items = tree.get_items().unwrap();
    let root = &mut items.as_mut_slice()[0];
    if let Some(label) = root.label() {
        if label == "ROOT" {
            root.set_label("/");
        }
    }

    tree.set_callback2(move |t| {
        if let Some(selected) = t.get_selected_items() {
            disp.buffer().unwrap().load_file(selected[0].label().unwrap()).unwrap();
        }
    });

    app.run().unwrap();
}
