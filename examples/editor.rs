use fltk::{
    app::*,
    dialog::*,
    menu::*,
    text::{TextBuffer, TextEditor},
    window::Window,
};
use std::ops::{Deref, DerefMut};
use std::{fs, path};

#[derive(Debug, Clone)]
pub struct Editor {
    pub editor: TextEditor,
    filename: String,
}

impl Editor {
    pub fn new(buf: TextBuffer) -> Editor {
        Editor {
            editor: TextEditor::new(5, 40, 790, 555, buf),
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
        self.editor.set_text_font(Font::Courier);
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
                false => alert(200, 200, "Please specify a file!"),
            }
        } else {
            match path::Path::new(&filename).exists() {
                true => {
                    fs::write(&filename, self.editor.buffer().text()).unwrap();
                    *saved = true;
                }
                false => alert(200, 200, "Please specify a file!"),
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

#[derive(Copy, Clone)]
pub enum Message {
    Changed,
    New,
    Open,
    Save,
    SaveAs,
    Quit,
    Cut,
    Copy,
    Paste,
    About,
}

fn main() {
    let app = App::default().set_scheme(AppScheme::Gtk);

    let (s, r) = channel::<Message>();
    let mut saved = false;

    let mut wind = Window::default()
        .with_size(800, 600)
        .center_screen()
        .with_label("RustyEd");

    let mut menu = MenuBar::new(0, 0, 800, 40, "");
    menu.set_color(Color::Light2);

    let mut buf = TextBuffer::default();
    buf.set_tab_distance(4);

    let mut editor = Editor::new(buf);
    editor.style();

    editor.emit(s, Message::Changed);

    menu.add_emit(
        "File/New...",
        Shortcut::Ctrl + 'n',
        MenuFlag::Normal,
        s,
        Message::New,
    );

    menu.add_emit(
        "File/Open...",
        Shortcut::Ctrl + 'o',
        MenuFlag::Normal,
        s,
        Message::Open,
    );

    menu.add_emit(
        "File/Save",
        Shortcut::Ctrl + 's',
        MenuFlag::Normal,
        s,
        Message::Save,
    );

    menu.add_emit(
        "File/Save as...",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::SaveAs,
    );

    menu.add_emit(
        "File/Quit",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::Quit,
    );

    menu.add_emit(
        "Edit/Cut",
        Shortcut::Ctrl + 'x',
        MenuFlag::Normal,
        s,
        Message::Cut,
    );

    menu.add_emit(
        "Edit/Copy",
        Shortcut::Ctrl + 'c',
        MenuFlag::Normal,
        s,
        Message::Copy,
    );

    menu.add_emit(
        "Edit/Paste",
        Shortcut::Ctrl + 'v',
        MenuFlag::Normal,
        s,
        Message::Paste,
    );

    menu.add_emit(
        "Help/About",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::About,
    );

    let mut x = menu.find_item("File/Quit").unwrap();
    x.set_label_color(Color::Red);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    wind.set_callback(Box::new(move || {
        if event() == Event::Close {
            s.send(Message::Quit);
        }
    }));

    while app.wait().expect("Couldn't run editor!") {
        use Message::*;
        match r.recv() {
            Some(msg) => match msg {
                Changed => saved = false,
                New => {
                    if editor.buffer().text() != "" {
                        let x = choice(200, 200, "File unsaved, Do you wish to continue?", "Yes", "No!", "");
                        if x == 0 {
                            editor.buffer().set_text("");
                        }
                    }
                },
                Open => {
                    let mut dlg = FileDialog::new(FileDialogType::BrowseFile);
                    dlg.set_option(FileDialogOptions::NoOptions);
                    dlg.set_filter("*.txt");
                    dlg.show();
                    editor
                        .set_filename(&dlg.filename().to_string_lossy().to_string());
                    let filename = editor.filename.clone();
                    if filename.is_empty() {
                        return;
                    }
                    match path::Path::new(&editor.filename()).exists() {
                        true => editor.buffer().set_text(
                            fs::read_to_string(&editor.filename())
                                .unwrap()
                                .as_str(),
                        ),
                        false => alert(200, 200, "File does not exist!"),
                    }
                },
                Save => editor.save_file(&mut saved),
                SaveAs => editor.save_file(&mut saved),
                Quit => {
                    if saved == false {
                        let x = choice(200, 200, "Would you like to save your work?", "Yes", "No", "");
                        if x == 0 {
                            editor.save_file(&mut saved);
                            app.quit();
                        } else {
                            app.quit();
                        }
                    } else {
                        app.quit();
                    }
                },
                Cut => editor.cut(),
                Copy => editor.copy(),
                Paste => editor.paste(),
                About => message(200, 200, "This is an example application written in Rust and using the FLTK Gui library.",),
            },
            _ => ()
        }
    }
}
