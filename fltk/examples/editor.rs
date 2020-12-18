use fltk::*;
use std::{
    error,
    ops::{Deref, DerefMut},
    panic, path,
};

#[inline(always)]
pub fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}

pub struct Editor {
    pub editor: text::TextEditor,
    pub saved: bool,
    filename: String,
}

impl Editor {
    pub fn new(buf: text::TextBuffer) -> Editor {
        let mut e = Editor {
            editor: text::TextEditor::new(5, 35, 790, 560, ""),
            saved: true,
            filename: String::from(""),
        };

        #[cfg(target_os = "macos")]
        e.editor.resize(5, 5, 790, 590);

        e.editor.set_scrollbar_size(15);
        e.editor.set_buffer(Some(buf));
        e.editor.set_text_font(Font::Courier);
        e.editor.set_linenumber_width(32);
        e.editor.set_linenumber_fgcolor(Color::from_u32(0x8b8386));
        e.editor.set_trigger(CallbackTrigger::Changed);
        e
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn set_filename(&mut self, name: &str) {
        self.filename = String::from(name);
    }

    pub fn save_file(&mut self) -> Result<(), Box<dyn error::Error>> {
        let mut filename = self.filename.clone();
        if self.saved {
            if filename.is_empty() {
                let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseSaveFile);
                dlg.set_option(dialog::FileDialogOptions::SaveAsConfirm);
                dlg.show();
                filename = dlg.filename().to_string_lossy().to_string();
                if !filename.is_empty() {
                    match path::Path::new(&filename).exists() {
                        true => {
                            self.editor.buffer().unwrap().save_file(&filename)?;
                            self.saved = true;
                        }
                        false => dialog::alert(
                            center().0 - 200,
                            center().1 - 100,
                            "Please specify a file!",
                        ),
                    }
                }
            } else {
                match path::Path::new(&filename).exists() {
                    true => {
                        self.editor.buffer().unwrap().save_file(&filename)?;
                        self.saved = true;
                    }
                    false => {
                        dialog::alert(center().0 - 200, center().1 - 100, "Please specify a file!")
                    }
                }
            }
        } else {
            let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseSaveFile);
            dlg.set_option(dialog::FileDialogOptions::SaveAsConfirm);
            dlg.show();
            filename = dlg.filename().to_string_lossy().to_string();
            if !filename.is_empty() {
                match path::Path::new(&filename).exists() {
                    true => {
                        self.editor.buffer().unwrap().save_file(&filename)?;
                        self.saved = true;
                    }
                    false => {
                        dialog::alert(center().0 - 200, center().1 - 100, "Please specify a file!")
                    }
                }
            }
        }
        Ok(())
    }
}

impl Deref for Editor {
    type Target = text::TextEditor;

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
    Print,
    Quit,
    Cut,
    Copy,
    Paste,
    About,
}

fn main() {
    panic::set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            dialog::message(center().0 - 200, center().1 - 100, s);
        } else {
            dialog::message(center().0 - 200, center().1 - 100, &info.to_string());
        }
    }));

    let args: Vec<String> = std::env::args().collect();

    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let (s, r) = app::channel::<Message>();

    let mut printable = text::TextDisplay::default();
    printable.set_frame(FrameType::NoBox);
    printable.set_scrollbar_width(0);

    let mut wind = window::Window::default()
        .with_size(800, 600)
        .center_screen()
        .with_label("RustyEd");

    let mut menu = menu::SysMenuBar::new(0, 0, 800, 35, "");
    menu.set_color(Color::Light2);

    let mut buf = text::TextBuffer::default();
    printable.set_buffer(Some(buf.clone()));
    buf.set_tab_distance(4);

    let mut editor = Editor::new(buf);

    editor.emit(s, Message::Changed);

    let mut buf = editor.buffer().unwrap();

    if args.len() > 1 {
        let file = path::Path::new(&args[1]);
        assert!(
            file.exists() && file.is_file(),
            "An error occured while opening the file!"
        );
        buf.load_file(&args[1]).unwrap();
        editor.set_filename(&args[1]);
    }

    menu.add_emit(
        "&File/New...\t",
        Shortcut::Ctrl | 'n',
        menu::MenuFlag::Normal,
        s,
        Message::New,
    );

    menu.add_emit(
        "&File/Open...\t",
        Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        s,
        Message::Open,
    );

    menu.add_emit(
        "&File/Save\t",
        Shortcut::Ctrl | 's',
        menu::MenuFlag::Normal,
        s,
        Message::Save,
    );

    menu.add_emit(
        "&File/Save as...\t",
        Shortcut::Ctrl | 'w',
        menu::MenuFlag::Normal,
        s,
        Message::SaveAs,
    );

    menu.add_emit(
        "&File/Print...\t",
        Shortcut::Ctrl | 'p',
        menu::MenuFlag::MenuDivider,
        s,
        Message::Print,
    );

    menu.add_emit(
        "&File/Quit\t",
        Shortcut::Ctrl | 'q',
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );

    menu.add_emit(
        "&Edit/Cut\t",
        Shortcut::Ctrl | 'x',
        menu::MenuFlag::Normal,
        s,
        Message::Cut,
    );

    menu.add_emit(
        "&Edit/Copy\t",
        Shortcut::Ctrl | 'c',
        menu::MenuFlag::Normal,
        s,
        Message::Copy,
    );

    menu.add_emit(
        "&Edit/Paste\t",
        Shortcut::Ctrl | 'v',
        menu::MenuFlag::Normal,
        s,
        Message::Paste,
    );

    menu.add_emit(
        "&Help/About\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::About,
    );

    if let Some(mut item) = menu.find_item("&File/Quit\t") {
        item.set_label_color(Color::Red);
    }

    wind.make_resizable(true);
    wind.end();
    wind.show();

    wind.set_callback(move || {
        if app::event() == Event::Close {
            s.send(Message::Quit);
        }
    });

    // Handle drag and drop
    let mut dnd = false;
    let mut released = false;

    editor.handle(move |ev| match ev {
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
                let path = app::event_text();
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
        }
        _ => false,
    });

    while app.wait() {
        use Message::*;
        if let Some(msg) = r.recv() {
            match msg {
                Changed => editor.saved = false,
                New => {
                    if editor.buffer().unwrap().text() != "" {
                        let x = dialog::choice(center().0 - 200, center().1 - 100, "File unsaved, Do you wish to continue?", "Yes", "No!", "");
                        if x == 0 {
                            editor.buffer().unwrap().set_text("");
                        }
                    }
                },
                Open => {
                    let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
                    dlg.set_option(dialog::FileDialogOptions::NoOptions);
                    dlg.set_filter("*.{txt,rs,toml}");
                    dlg.show();
                    editor
                        .set_filename(&dlg.filename().to_string_lossy().to_string());
                    let filename = editor.filename();
                    if !filename.is_empty() {
                        match path::Path::new(&filename).exists() {
                            true => editor.buffer().unwrap().load_file(&filename).unwrap(),
                            false => dialog::alert(center().0 - 200, center().1 - 100, "File does not exist!"),
                        }
                    }
                },
                Save => editor.save_file().unwrap(),
                SaveAs => editor.save_file().unwrap(),
                Print => {
                    let mut printer = printer::Printer::default();
                    if printer.begin_job(0).is_ok() {
                        let (w, h) = printer.printable_rect();
                        printable.set_size(w - 40, h - 40);
                        // Needs cleanup
                        let line_count = printable.count_lines(0, printable.buffer().unwrap().length(), true) / 45;
                        for i in 0..=line_count {
                            printable.scroll(45 * i, 0);
                            printer.begin_page();
                            printer.print_widget(&printable, 20, 20);
                            printer.end_page();
                        }
                        printer.end_job();
                    }
                },
                Quit => {
                    if !editor.saved {
                        let x = dialog::choice(center().0 - 200, center().1 - 100, "Would you like to save your work?", "Yes", "No", "");
                        if x == 0 {
                            editor.save_file().unwrap();
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
                About => dialog::message(center().0 - 300, center().1 - 100, "This is an example application written in Rust and using the FLTK Gui library."),
            }
        }
    }
}
