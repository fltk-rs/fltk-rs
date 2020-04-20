use fltk::{
    app::*,
    dialog::*,
    menu::*,
    text::{TextBuffer, TextEditor},
    window::Window,
};
use std::ops::{Deref, DerefMut};
use std::{fs, path};

pub struct Editor<'a> {
    pub editor: TextEditor,
    pub buf: &'a mut TextBuffer,
    filename: String,
    saved: bool,
}

impl<'a> Editor<'a> {
    pub fn new(mut buf: &mut TextBuffer) -> Editor {
        Editor {
            editor: TextEditor::new(5, 40, 790, 555, &mut buf),
            buf: buf,
            filename: String::from(""),
            saved: true,
        }
    }

    pub fn saved(&self) -> bool {
        self.saved
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn set_filename(&mut self, name: &str) {
        self.filename = String::from(name);
    }

    pub fn style(&mut self) {
        self.editor.set_text_font(Font::Courrier);
        self.buf.set_tab_distance(4);
        self.editor.set_linenumber_width(18);
        self.editor
            .set_linenumber_fgcolor(Color::from_u32(0x8b8386));
        self.editor.set_trigger(CallbackTrigger::Changed);
    }

    pub fn save_file(&mut self) {
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
                    self.saved = true;
                }
                false => alert("Please specify a file!"),
            }
        } else {
            match path::Path::new(&filename).exists() {
                true => {
                    fs::write(&filename, self.editor.buffer().text()).unwrap();
                    self.saved = true;
                }
                false => alert("Please specify a file!"),
            }
        }
    }
}

impl<'a> Deref for Editor<'a> {
    type Target = TextEditor;

    fn deref(&self) -> &Self::Target {
        &self.editor
    }
}

impl<'a> DerefMut for Editor<'a> {
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
    let mut buf = TextBuffer::default();
    let mut editor = Editor::new(&mut buf);
    editor.style();
    wind.make_resizable(true);
    wind.end();
    wind.show();
    editor
        .editor
        .clone()
        .set_callback(Box::new(|| editor.saved = false));
    unsafe {
        menu.add(
            "File/New...",
            Shortcut::Ctrl + 'n',
            MenuFlag::Normal,
            Box::new(|| {
                if editor.buffer().text() != "" {
                    let x = choice("File unsaved, Do you wish to continue?", "Yes", "No!", "h");
                    if x == 0 {
                        editor.buf.set_text("");
                    }
                }
            }),
        );

        menu.add(
            "File/Open...",
            Shortcut::Ctrl + 'o',
            MenuFlag::Normal,
            Box::new(|| {
                let mut dlg = FileDialog::new(FileDialogType::BrowseFile);
                dlg.set_option(FileDialogOptions::NoOptions);
                dlg.set_filter("*.txt");
                dlg.show();
                editor.set_filename(&dlg.filename().to_string_lossy().to_string());
                if editor.filename.is_empty() {
                    return;
                }
                match path::Path::new(&editor.filename()).exists() {
                    true => editor
                        .buf
                        .set_text(fs::read_to_string(&editor.filename()).unwrap().as_str()),
                    false => alert("File does not exist!"),
                }
            }),
        );

        menu.add(
            "File/Save",
            Shortcut::Ctrl + 's',
            MenuFlag::Normal,
            Box::new(|| editor.save_file()),
        );

        menu.add(
            "File/Save as...",
            Shortcut::None,
            MenuFlag::MenuDivider,
            Box::new(|| editor.save_file()),
        );

        menu.add(
            "File/Quit",
            Shortcut::None,
            MenuFlag::Normal,
            Box::new(|| {
                if editor.saved() == false {
                    let x = choice("Would you like to save your work?", "Yes", "No", "");
                    if x == 0 {
                        editor.save_file();
                        std::process::exit(0);
                    } else {
                        std::process::exit(0);
                    }
                } else {
                    std::process::exit(0);
                }
            }),
        );

        menu.add(
            "Edit/Cut",
            Shortcut::Ctrl + 'x',
            MenuFlag::Normal,
            Box::new(|| editor.cut()),
        );

        menu.add(
            "Edit/Copy",
            Shortcut::Ctrl + 'c',
            MenuFlag::Normal,
            Box::new(|| {
                editor.copy();
            }),
        );

        menu.add(
            "Edit/Paste",
            Shortcut::Ctrl + 'v',
            MenuFlag::Normal,
            Box::new(|| editor.paste()),
        );

        menu.add(
            "Help/About",
            Shortcut::None,
            MenuFlag::Normal,
            Box::new(|| {
                message(
                "This is an example application written in Rust and using the FLTK Gui library.",
            );
            }),
        );
    }

    let mut x = menu.item("Help/About").unwrap();
    x.set_label_color(Color::Red);
    wind.set_callback(Box::new(|| {
        if editor.saved == false {
            let x = choice("Would you like to save your work?", "Yes", "No", "");
            if x == 0 {
                editor.save_file();
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
