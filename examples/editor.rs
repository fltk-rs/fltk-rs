use fltk::{
    app::*,
    dialog::*,
    menu::*,
    text::{TextBuffer, TextEditor},
    window::Window,
};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::{fs, path};

pub struct Editor {
    pub editor: TextEditor,
    filename: String,
}

impl Editor {
    pub fn new(mut buf: &mut TextBuffer) -> Editor {
        Editor {
            editor: TextEditor::new(5, 40, 790, 555, &mut buf),
            filename: String::from(""),
        }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn set_filename(&mut self, name: &str) {
        self.filename = String::from(name);
    }

    pub fn style(&mut self) {
        self.editor.set_text_font(Font::Courrier);
        self.editor.set_linenumber_width(18);
        self.editor
            .set_linenumber_fgcolor(Color::from_u32(0x8b8386));
        self.editor.set_trigger(CallbackTrigger::Changed);
    }

    pub fn save_file(&mut self, saved: &mut bool) {
        let mut filename = self.filename.clone();
        if filename.is_empty() {
            let mut dlg = FileDialog::new(FileDialogType::BrowseSaveFile);
            dlg.set_option(FileDialogOptions::SaveAsConfirm);
            dlg.show();
            filename = dlg.filename().to_string_lossy().to_string();
            if filename.is_empty() {
                return;
            }
            match path::Path::new(&filename).exists() {
                true => {
                    fs::write(&filename, self.editor.buffer().text()).unwrap();
                    *saved = true;
                }
                false => alert("Please specify a file!"),
            }
        } else {
            match path::Path::new(&filename).exists() {
                true => {
                    fs::write(&filename, self.editor.buffer().text()).unwrap();
                    *saved = true;
                }
                false => alert("Please specify a file!"),
            }
        }
    }
}

impl Deref for Editor {
    type Target = TextEditor;

    fn deref(&self) -> &Self::Target {
        &self.editor
    }
}

impl DerefMut for Editor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.editor
    }
}

fn main() {
    let app = App::default().set_scheme(AppScheme::Gtk);
    let mut wind = Window::default()
        .with_size(800, 600)
        .center_screen()
        .with_label("RustyEd");
    let mut menu = MenuBar::new(0, 0, 800, 40, "");
    menu.set_color(Color::Light2);
    let buf = TextBuffer::default();
    let buf = Rc::from(RefCell::from(buf));
    buf.borrow_mut().set_tab_distance(4);
    let mut editor = Editor::new(buf.borrow_mut().deref_mut());
    editor.style();
    wind.make_resizable(true);
    wind.end();
    wind.show();
    let mut saved = false;
    // let inner_editor = editor.editor;
    let editor_rc = Rc::from(RefCell::from(editor));
    let editor_cloned = editor_rc.clone();
    editor_cloned
        .borrow_mut()
        .editor
        .set_callback(Box::new(move || saved = false));

    let editor_cloned = editor_rc.clone();
    menu.add(
        "File/New...",
        Shortcut::Ctrl + 'n',
        MenuFlag::Normal,
        Box::new(move || {
            if editor_cloned.borrow().buffer().text() != "" {
                let x = choice("File unsaved, Do you wish to continue?", "Yes", "No!", "");
                if x == 0 {
                    editor_cloned.borrow().buffer().set_text("");
                }
            }
        }),
    );

    let editor_cloned = editor_rc.clone();
    menu.add(
        "File/Open...",
        Shortcut::Ctrl + 'o',
        MenuFlag::Normal,
        Box::new(move || {
            let mut dlg = FileDialog::new(FileDialogType::BrowseFile);
            dlg.set_option(FileDialogOptions::NoOptions);
            dlg.set_filter("*.txt");
            dlg.show();
            editor_cloned
                .borrow_mut()
                .set_filename(&dlg.filename().to_string_lossy().to_string());
            if editor_cloned.borrow().filename.is_empty() {
                return;
            }
            match path::Path::new(&editor_cloned.borrow().filename()).exists() {
                true => editor_cloned.borrow().buffer().set_text(
                    fs::read_to_string(&editor_cloned.borrow().filename())
                        .unwrap()
                        .as_str(),
                ),
                false => alert("File does not exist!"),
            }
        }),
    );

    let editor_cloned = editor_rc.clone();
    menu.add(
        "File/Save",
        Shortcut::Ctrl + 's',
        MenuFlag::Normal,
        Box::new(move || editor_cloned.borrow_mut().save_file(&mut saved)),
    );

    let editor_cloned = editor_rc.clone();
    menu.add(
        "File/Save as...",
        Shortcut::None,
        MenuFlag::MenuDivider,
        Box::new(move || editor_cloned.borrow_mut().save_file(&mut saved)),
    );

    let editor_cloned = editor_rc.clone();
    menu.add(
        "File/Quit",
        Shortcut::None,
        MenuFlag::Normal,
        Box::new(move || {
            if saved == false {
                let x = choice("Would you like to save your work?", "Yes", "No", "");
                if x == 0 {
                    editor_cloned.borrow_mut().save_file(&mut saved);
                    std::process::exit(0);
                } else {
                    std::process::exit(0);
                }
            } else {
                std::process::exit(0);
            }
        }),
    );

    let editor_cloned = editor_rc.clone();
    menu.add(
        "Edit/Cut",
        Shortcut::Ctrl + 'x',
        MenuFlag::Normal,
        Box::new(move || editor_cloned.borrow().cut()),
    );

    let editor_cloned = editor_rc.clone();
    menu.add(
        "Edit/Copy",
        Shortcut::Ctrl + 'c',
        MenuFlag::Normal,
        Box::new(move || {
            editor_cloned.borrow().copy();
        }),
    );

    let editor_cloned = editor_rc.clone();
    menu.add(
        "Edit/Paste",
        Shortcut::Ctrl + 'v',
        MenuFlag::Normal,
        Box::new(move || editor_cloned.borrow().paste()),
    );

    menu.add(
        "Help/About",
        Shortcut::None,
        MenuFlag::Normal,
        Box::new(move || {
            message(
                "This is an example application written in Rust and using the FLTK Gui library.",
            );
        }),
    );

    let mut x = menu.item("Help/About").unwrap();
    x.set_label_color(Color::Red);

    let editor_cloned = editor_rc.clone();
    wind.set_callback(Box::new(move || {
        if saved == false {
            let x = choice("Would you like to save your work?", "Yes", "No", "");
            if x == 0 {
                editor_cloned.borrow_mut().save_file(&mut saved);
                std::process::exit(0);
            } else {
                std::process::exit(0);
            }
        } else {
            std::process::exit(0);
        }
    }));
    app.run().expect("Couldn't run editor");
}
