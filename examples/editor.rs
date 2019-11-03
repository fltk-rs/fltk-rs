use fltk::{dialog::*, input::*, menu::*, window::*};
use std::fs;

fn main() {
    let mut filename = String::from("");
    let mut wind = Window::new().set(100, 100, 800, 600, "RustyEd");
    let mut editor = MultilineInput::new().set(5, 40, 790, 555, "");
    let mut menu = MenuBar::new().set(0, 0, 800, 40, "");
    menu.add(
        "File/New...",
        Shortcut::Ctrl + 'n',
        MenuFlag::Normal,
        &mut || {
            println!("{:?}", fl::event());
            let mut dlg = FileDialog::new(FileDialogType::BrowseFile);
            dlg.option();
            dlg.show();
            filename = dlg.filename();
        },
    );
    menu.add(
        "File/Open...",
        Shortcut::Ctrl + 'o',
        MenuFlag::Normal,
        &mut || {
            let mut dlg = FileDialog::new(FileDialogType::BrowseFile);
            dlg.show();
            filename = dlg.filename();
            editor.set_value(fs::read_to_string(&filename).unwrap().as_str());
        },
    );
    menu.add(
        "File/Save",
        Shortcut::Ctrl+ 's',
        MenuFlag::Normal,
        &mut || println!("{:?}", fl::event()),
    );
    menu.add("File/Save as...", 0, MenuFlag::MenuDivider, &mut || {
        println!("{:?}", fl::event())
    });
    menu.add("File/Quit", 0, MenuFlag::Normal, &mut || {
        println!("{:?}", fl::event())
    });
    menu.add(
        "Edit/Cut",
        Shortcut::Ctrl + 'x',
        MenuFlag::Normal,
        &mut || println!("{:?}", fl::event()),
    );
    menu.add(
        "Edit/Copy",
        Shortcut::Ctrl + 'c',
        MenuFlag::Normal,
        &mut || println!("{:?}", fl::event()),
    );
    menu.add(
        "Edit/Paste",
        Shortcut::Ctrl + 'v',
        MenuFlag::Normal,
        &mut || println!("{:?}", fl::event()),
    );
    menu.add("Help/About", 0, MenuFlag::Normal, &mut || unimplemented!());
    let _x = menu.get_item("File/Open");
    wind.end();
    wind.show();
    fl::run();
}
