use fltk::{
    app::{self, App},
    dialog,
    enums::{Color, Shortcut},
    group::Flex,
    menu::{CMenuItem, MenuBar, MenuFlag, MenuItem},
    prelude::*,
    text::{TextBuffer, TextEditor},
    window::Window,
};

fn menu_cb(m: &mut impl MenuExt) {
    let ed: TextEditor = app::widget_from_id("ed").unwrap();
    if let Ok(mpath) = m.item_pathname(None) {
        match mpath.as_str() {
            "&File/&New" => println!("new"),
            "&File/&Open" => println!("open"),
            "&File/&Quit" => app::quit(),
            "&Edit/Cut" => ed.cut(),
            "&Edit/Copy" => ed.copy(),
            "&Edit/Paste" => ed.paste(),
            "&Help/About" => dialog::message_default("Dialog box"),
            _ => (),
        }
    }
}

fn main() {
    let a = App::default();
    let cmenu = &[
        CMenuItem {
            text: Some("&File"),
            flags: MenuFlag::Submenu,
            ..Default::default()
        },
        CMenuItem {
            text: Some("&New"),
            shortcut: Shortcut::Ctrl | 'n',
            ..Default::default()
        },
        CMenuItem {
            text: Some("&Open"),
            shortcut: Shortcut::Ctrl | 'o',
            flags: MenuFlag::MenuDivider,
            ..Default::default()
        },
        CMenuItem {
            text: Some("&Quit"),
            shortcut: Shortcut::Ctrl | 'q',
            labelcolor: Color::Red,
            ..Default::default()
        },
        CMenuItem::empty(),
        CMenuItem {
            text: Some("&Edit"),
            flags: MenuFlag::Submenu,
            ..Default::default()
        },
        CMenuItem {
            text: Some("Cut"),
            shortcut: Shortcut::Ctrl | 'x',
            ..Default::default()
        },
        CMenuItem {
            text: Some("Copy"),
            shortcut: Shortcut::Ctrl | 'c',
            ..Default::default()
        },
        CMenuItem {
            text: Some("Paste"),
            shortcut: Shortcut::Ctrl | 'v',
            ..Default::default()
        },
        CMenuItem::empty(),
        CMenuItem {
            text: Some("&Help"),
            flags: MenuFlag::Submenu,
            ..Default::default()
        },
        CMenuItem {
            text: Some("About"),
            ..Default::default()
        },
        CMenuItem::empty(),
        CMenuItem::empty(),
    ];
    let cmenu = MenuItem::new_from_cmenu(cmenu);
    let buf = TextBuffer::default();
    let mut w = Window::default().with_size(400, 300);
    let mut col = Flex::default_fill().column();
    let mut c = MenuBar::default();
    c.set_callback(menu_cb);
    unsafe {
        c.set_menu(cmenu);
    }
    col.fixed(&c, 30);
    let mut ed = TextEditor::default().with_id("ed");
    ed.set_buffer(buf);
    col.end();
    w.end();
    w.show();
    a.run().unwrap();
}
