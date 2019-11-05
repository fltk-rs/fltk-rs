use fltk::{dialog::*, input::MultilineInput, menu::*, window::Window};
use std::{fs, path};

fn main() {
    let (screen_width, screen_height) = fl::screen_size();
    let mut filename = String::from("");
    let mut wind = Window::new().set(
        (screen_width / 2.0 - 400.0) as i32,
        (screen_height / 2.0 - 300.0) as i32,
        800,
        600,
        "RustyEd",
    );
    wind.set_color(Color::Light2);
    let mut editor = MultilineInput::new().set(5, 40, 790, 555, "");
    let mut menu = MenuBar::new().set(0, 0, 800, 40, "");
    menu.set_color(Color::Light2);
    
    menu.add(
        "File/New...",
        Shortcut::Ctrl + 'n',
        MenuFlag::Normal,
        &mut || {
            if editor.value() != "" {
                let x = choice("File unsaved, Do you wish to continue?", "Yes", "No!", "");
                if x == 0 {
                    editor.set_value("");
                }
            }
        },
    );

    menu.add(
        "File/Open...",
        Shortcut::Ctrl + 'o',
        MenuFlag::Normal,
        &mut || {
            let mut dlg = FileDialog::new(FileDialogType::BrowseFile);
            dlg.set_option(FileDialogOptions::NoOptions);
            dlg.show();
            filename = dlg.filename();
            match path::Path::new(&filename).exists() {
                true => editor.set_value(fs::read_to_string(&filename).unwrap().as_str()),
                false => alert("File does not exist!"),
            }
        },
    );

    menu.add(
        "File/Save",
        Shortcut::Ctrl + 's',
        MenuFlag::Normal,
        &mut || match path::Path::new(&filename).exists() {
            true => fs::write(&filename, editor.value()).unwrap(),
            false => alert("Please specify a file!"),
        },
    );

    menu.add("File/Save as...", 0, MenuFlag::MenuDivider, &mut || {
        let mut dlg = FileDialog::new(FileDialogType::BrowseSaveFile);
        dlg.set_option(FileDialogOptions::SaveAsConfirm);
        dlg.show();
        filename = dlg.filename();
        match path::Path::new(&filename).exists() {
            true => fs::write(&filename, editor.value()).unwrap(),
            false => alert("Please specify a file!"),
        }
    });

    menu.add("File/Quit", 0, MenuFlag::Normal, &mut || {
        std::process::exit(0);
    });

    menu.add(
        "Edit/Cut",
        Shortcut::Ctrl + 'x',
        MenuFlag::Normal,
        &mut || editor.cut(),
    );

    menu.add(
        "Edit/Copy",
        Shortcut::Ctrl + 'c',
        MenuFlag::Normal,
        &mut || {
            editor.copy();
        },
    );

    menu.add(
        "Edit/Paste",
        Shortcut::Ctrl + 'v',
        MenuFlag::Normal,
        &mut || fl::paste(editor.clone()),
    );

    menu.add("Help/About", 0, MenuFlag::Normal, &mut || {
        message("This is an example application written in Rust and using the FLTK Gui library.")
    });

    // let mut x = menu.get_item("Help/About");
    // x.set_label_color(Color::Red);

    wind.end();
    wind.show();
    fl::run();
}
