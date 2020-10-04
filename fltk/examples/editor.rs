// The code could certainly be refactored into several source files!

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
        let mut e = Editor {
            editor: TextEditor::new(5, 35, 790, 560, ""),
            filename: String::from(""),
        };

        #[cfg(target_os = "macos")]
        e.editor.resize(5, 5, 790, 590);

        e.editor.set_buffer(Some(buf));
        e
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn set_filename(&mut self, name: &str) {
        self.filename = String::from(name);
    }

    pub fn style(&mut self) {
        self.editor.set_text_font(Font::Courier);
        self.editor.set_linenumber_width(32);
        self.editor
            .set_linenumber_fgcolor(Color::from_u32(0x8b8386));
        self.editor.set_trigger(CallbackTrigger::Changed);
    }

    pub fn save_file(&mut self, saved: &mut bool) {
        let mut filename = self.filename.clone();
        if *saved {
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
                        fs::write(&filename, self.editor.buffer().unwrap().text()).unwrap();
                        *saved = true;
                    }
                    false => alert(200, 200, "Please specify a file!"),
                }
            } else {
                match path::Path::new(&filename).exists() {
                    true => {
                        fs::write(&filename, self.editor.buffer().unwrap().text()).unwrap();
                        *saved = true;
                    }
                    false => alert(200, 200, "Please specify a file!"),
                }
            }
        } else {
            let mut dlg = FileDialog::new(FileDialogType::BrowseSaveFile);
            dlg.set_option(FileDialogOptions::SaveAsConfirm);
            dlg.show();
            filename = dlg.filename().to_string_lossy().to_string();
            if filename.is_empty() {
                return;
            }
            match path::Path::new(&filename).exists() {
                true => {
                    fs::write(&filename, self.editor.buffer().unwrap().text()).unwrap();
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
    let args: Vec<String> = std::env::args().collect();
    let app = App::default().with_scheme(Scheme::Gtk);

    let (s, r) = channel::<Message>();
    let mut saved = true;

    let mut wind = Window::default()
        .with_size(800, 600)
        .center_screen()
        .with_label("RustyEd");

    let mut menu = SysMenuBar::new(0, 0, 800, 35, "");
    menu.set_color(Color::Light2);

    let mut buf = TextBuffer::default();
    buf.set_tab_distance(4);

    let mut editor = Editor::new(buf);
    editor.style();

    editor.emit(s, Message::Changed);

    let mut buf = editor.buffer().unwrap();

    if args.len() > 1 {
        buf.load_file(&args[1]).unwrap();
        editor.set_filename(&args[1]);
    }

    // Handle drag and drop
    let mut dnd = false;
    let mut released = false;
    editor.handle(Box::new(move |ev| {
        match ev {
            Event::DndEnter => {
                dnd = true;
                true
            }
            Event::DndDrag => true,
            Event::DndRelease => {
                released = true;
                true
            }
            Event::Paste => {
                if dnd && released {
                    let path = event_text();
                    let path = std::path::Path::new(&path);
                    assert!(path.exists());
                    buf.load_file(&path).unwrap();
                    dnd = false;
                    released = false;
                    true
                } else {
                    false
                }
            }
            Event::DndLeave => {
                dnd = false;
                released = false;
                true
            },
            _ => false,
        }
    }));

    menu.add_emit(
        "&File/New...\t",
        Shortcut::Ctrl | 'n',
        MenuFlag::Normal,
        s,
        Message::New,
    );

    menu.add_emit(
        "&File/Open...\t",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        s,
        Message::Open,
    );

    menu.add_emit(
        "&File/Save\t",
        Shortcut::Ctrl | 's',
        MenuFlag::Normal,
        s,
        Message::Save,
    );

    menu.add_emit(
        "&File/Save as...\t",
        Shortcut::Ctrl | 'w',
        MenuFlag::MenuDivider,
        s,
        Message::SaveAs,
    );

    menu.add_emit(
        "&File/Quit\t",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        s,
        Message::Quit,
    );

    menu.add_emit(
        "&Edit/Cut\t",
        Shortcut::Ctrl | 'x',
        MenuFlag::Normal,
        s,
        Message::Cut,
    );

    menu.add_emit(
        "&Edit/Copy\t",
        Shortcut::Ctrl | 'c',
        MenuFlag::Normal,
        s,
        Message::Copy,
    );

    menu.add_emit(
        "&Edit/Paste\t",
        Shortcut::Ctrl | 'v',
        MenuFlag::Normal,
        s,
        Message::Paste,
    );

    menu.add_emit(
        "&Help/About\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::About,
    );

    let mut x = menu.find_item("&File/Quit\t").unwrap();
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
        if let Some(msg) = r.recv() {
            match msg {
                Changed => saved = false,
                New => {
                    if editor.buffer().unwrap().text() != "" {
                        let x = choice(200, 200, "File unsaved, Do you wish to continue?", "Yes", "No!", "");
                        if x == 0 {
                            editor.buffer().unwrap().set_text("");
                        }
                    }
                },
                Open => {
                    let mut dlg = FileDialog::new(FileDialogType::BrowseFile);
                    dlg.set_option(FileDialogOptions::NoOptions);
                    dlg.set_filter("*.{txt,rs,toml}");
                    dlg.show();
                    editor
                        .set_filename(&dlg.filename().to_string_lossy().to_string());
                    let filename = editor.filename.clone();
                    if !filename.is_empty() {
                        match path::Path::new(&editor.filename()).exists() {
                            true => editor.buffer().unwrap().set_text(
                                fs::read_to_string(&editor.filename())
                                    .unwrap()
                                    .as_str(),
                            ),
                            false => alert(200, 200, "File does not exist!"),
                        }
                    }
                },
                Save => editor.save_file(&mut saved),
                SaveAs => editor.save_file(&mut false),
                Quit => {
                    if !saved {
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
            }
        }
    }
}
