use fltk::{
    app, dialog,
    enums::{CallbackTrigger, Color, Event, Font, FrameType, Shortcut},
    menu,
    prelude::*,
    printer, text, window,
};
use std::path::PathBuf;
use std::{
    error,
    ops::{Deref, DerefMut},
    path,
};

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

pub fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}

fn nfc_get_file(mode: dialog::NativeFileChooserType) -> Option<PathBuf> {
    let mut nfc = dialog::NativeFileChooser::new(mode);
    if mode == dialog::NativeFileChooserType::BrowseSaveFile {
        nfc.set_option(dialog::NativeFileChooserOptions::SaveAsConfirm);
    } else if mode == dialog::NativeFileChooserType::BrowseFile {
        nfc.set_option(dialog::NativeFileChooserOptions::NoOptions);
        nfc.set_filter("*.{txt,rs,toml}");
    }
    match nfc.try_show() {
        Err(e) => {
            eprintln!("{}", e);
            None
        }
        Ok(a) => match a {
            dialog::NativeFileChooserAction::Success => {
                let name = nfc.filename();
                if name.as_os_str().is_empty() {
                    dialog::alert(center().0 - 200, center().1 - 100, "Please specify a file!");
                    None
                } else {
                    Some(name)
                }
            }
            dialog::NativeFileChooserAction::Cancelled => None,
        },
    }
}

pub struct MyEditor {
    editor: text::TextEditor,
}

impl MyEditor {
    pub fn new(buf: text::TextBuffer) -> Self {
        let mut editor = text::TextEditor::new(5, 35, 790, 560, "");
        editor.set_buffer(Some(buf));

        #[cfg(target_os = "macos")]
        editor.resize(5, 5, 790, 590);

        editor.set_scrollbar_size(15);
        editor.set_text_font(Font::Courier);
        editor.set_linenumber_width(32);
        editor.set_linenumber_fgcolor(Color::from_u32(0x008b_8386));
        editor.set_trigger(CallbackTrigger::Changed);

        Self { editor }
    }
}

impl Deref for MyEditor {
    type Target = text::TextEditor;

    fn deref(&self) -> &Self::Target {
        &self.editor
    }
}

impl DerefMut for MyEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.editor
    }
}

pub struct MyMenu {
    menu: menu::SysMenuBar,
}

impl MyMenu {
    pub fn new(s: &app::Sender<Message>) -> Self {
        let mut menu = menu::SysMenuBar::default().with_size(800, 35);
        menu.set_frame(FrameType::FlatBox);
        menu.add_emit(
            "&File/&New...\t",
            Shortcut::Ctrl | 'n',
            menu::MenuFlag::Normal,
            *s,
            Message::New,
        );

        menu.add_emit(
            "&File/&Open...\t",
            Shortcut::Ctrl | 'o',
            menu::MenuFlag::Normal,
            *s,
            Message::Open,
        );

        menu.add_emit(
            "&File/&Save\t",
            Shortcut::Ctrl | 's',
            menu::MenuFlag::Normal,
            *s,
            Message::Save,
        );

        menu.add_emit(
            "&File/Save &as...\t",
            Shortcut::Ctrl | 'w',
            menu::MenuFlag::Normal,
            *s,
            Message::SaveAs,
        );

        menu.add_emit(
            "&File/&Print...\t",
            Shortcut::Ctrl | 'p',
            menu::MenuFlag::MenuDivider,
            *s,
            Message::Print,
        );

        menu.add_emit(
            "&File/&Quit\t",
            Shortcut::Ctrl | 'q',
            menu::MenuFlag::Normal,
            *s,
            Message::Quit,
        );

        menu.add_emit(
            "&Edit/Cu&t\t",
            Shortcut::Ctrl | 'x',
            menu::MenuFlag::Normal,
            *s,
            Message::Cut,
        );

        menu.add_emit(
            "&Edit/&Copy\t",
            Shortcut::Ctrl | 'c',
            menu::MenuFlag::Normal,
            *s,
            Message::Copy,
        );

        menu.add_emit(
            "&Edit/&Paste\t",
            Shortcut::Ctrl | 'v',
            menu::MenuFlag::Normal,
            *s,
            Message::Paste,
        );

        menu.add_emit(
            "&Help/&About\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            *s,
            Message::About,
        );

        Self { menu }
    }
}

pub struct MyApp {
    app: app::App,
    modified: bool,
    filename: Option<PathBuf>,
    r: app::Receiver<Message>,
    main_win: window::Window,
    menu: MyMenu,
    buf: text::TextBuffer,
    editor: MyEditor,
    printable: text::TextDisplay,
}

impl MyApp {
    pub fn new(args: Vec<String>) -> Self {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);
        app::background(211, 211, 211);
        let (s, r) = app::channel::<Message>();
        let mut buf = text::TextBuffer::default();
        buf.set_tab_distance(4);
        let mut main_win = window::Window::default()
            .with_size(800, 600)
            .center_screen()
            .with_label("RustyEd");
        let menu = MyMenu::new(&s);
        let modified = false;
        menu.menu.find_item("&File/&Save\t").unwrap().deactivate();
        let mut editor = MyEditor::new(buf.clone());
        editor.emit(s, Message::Changed);
        main_win.make_resizable(true);
        // only resize editor, not the menu bar
        main_win.resizable(&*editor);
        main_win.end();
        main_win.show();
        main_win.set_callback(move |_| {
            if app::event() == Event::Close {
                s.send(Message::Quit);
            }
        });
        let filename = if args.len() > 1 {
            let file = path::Path::new(&args[1]);
            assert!(
                file.exists() && file.is_file(),
                "An error occurred while opening the file!"
            );
            match buf.load_file(&args[1]) {
                Ok(_) => Some(PathBuf::from(args[1].clone())),
                Err(e) => {
                    dialog::alert(
                        center().0 - 200,
                        center().1 - 100,
                        &format!("An issue occured while loading the file: {e}"),
                    );
                    None
                }
            }
        } else {
            None
        };

        // Handle drag and drop
        editor.handle({
            let mut dnd = false;
            let mut released = false;
            let buf = buf.clone();
            move |_, ev| match ev {
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
                        let path = path.trim();
                        let path = path.replace("file://", "");
                        let path = std::path::PathBuf::from(&path);
                        if path.exists() {
                            // we use a timeout to avoid pasting the path into the buffer
                            app::add_timeout3(0.0, {
                                let mut buf = buf.clone();
                                move |_| match buf.load_file(&path) {
                                    Ok(_) => (),
                                    Err(e) => dialog::alert(
                                        center().0 - 200,
                                        center().1 - 100,
                                        &format!("An issue occured while loading the file: {e}"),
                                    ),
                                }
                            });
                        }
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
            }
        });

        // What shows when we attempt to print
        let mut printable = text::TextDisplay::default();
        printable.set_frame(FrameType::NoBox);
        printable.set_scrollbar_size(0);
        printable.set_buffer(Some(buf.clone()));

        Self {
            app,
            modified,
            filename,
            r,
            main_win,
            menu,
            buf,
            editor,
            printable,
        }
    }

    /** Called by "Save", test if file can be written, otherwise call save_file_as()
     * afterwards. Will return true if the file is succesfully saved. */
    pub fn save_file(&mut self) -> Result<bool, Box<dyn error::Error>> {
        match &self.filename {
            Some(f) => {
                self.buf.save_file(f)?;
                self.modified = false;
                self.menu
                    .menu
                    .find_item("&File/&Save\t")
                    .unwrap()
                    .deactivate();
                self.menu
                    .menu
                    .find_item("&File/&Quit\t")
                    .unwrap()
                    .set_label_color(Color::Black);
                let name = match &self.filename {
                    Some(f) => f.to_string_lossy().to_string(),
                    None => "(Untitled)".to_string(),
                };
                self.main_win.set_label(&format!("{name} - RustyEd"));
                Ok(true)
            }
            None => self.save_file_as(),
        }
    }

    /** Called by "Save As..." or by "Save" in case no file was set yet.
     * Returns true if the file was succesfully saved. */
    pub fn save_file_as(&mut self) -> Result<bool, Box<dyn error::Error>> {
        if let Some(c) = nfc_get_file(dialog::NativeFileChooserType::BrowseSaveFile) {
            self.buf.save_file(&c)?;
            self.modified = false;
            self.menu
                .menu
                .find_item("&File/&Save\t")
                .unwrap()
                .deactivate();
            self.menu
                .menu
                .find_item("&File/&Quit\t")
                .unwrap()
                .set_label_color(Color::Black);
            self.filename = Some(c);
            self.main_win
                .set_label(&format!("{:?} - RustyEd", self.filename.as_ref().unwrap()));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn launch(&mut self) {
        while self.app.wait() {
            use Message::*;
            if let Some(msg) = self.r.recv() {
                match msg {
                    Changed => {
                        if !self.modified {
                            self.modified = true;
                            self.menu.menu.find_item("&File/&Save\t").unwrap().activate();
                            self.menu.menu.find_item("&File/&Quit\t").unwrap().set_label_color(Color::Red);
                            let name = match &self.filename {
                                Some(f) => f.to_string_lossy().to_string(),
                                None => "(Untitled)".to_string(),
                            };
                            self.main_win.set_label(&format!("* {name} - RustyEd"));
                        }
                    }
                    New => {
                        if self.buf.text() != "" {
                            let clear = if let Some(x) = dialog::choice2(center().0 - 200, center().1 - 100, "File unsaved, Do you wish to continue?", "&Yes", "&No!", "") {
                                x == 0
                            } else {
                                false
                            };
                            if clear {
                                self.buf.set_text("");
                            }
                        }
                    },
                    Open => {
                        if let Some(c) = nfc_get_file(dialog::NativeFileChooserType::BrowseFile) {
                            if c.exists() {
                                match self.buf.load_file(&c) {
                                    Ok(_) => self.filename = Some(c),
                                    Err(e) => dialog::alert(center().0 - 200, center().1 - 100, &format!("An issue occured while loading the file: {e}")),
                                }
                            } else {
                                dialog::alert(center().0 - 200, center().1 - 100, "File does not exist!")
                            }
                        }
                    },
                    Save => { self.save_file().unwrap(); },
                    SaveAs => { self.save_file_as().unwrap(); },
                    Print => {
                        let mut printer = printer::Printer::default();
                        if printer.begin_job(0).is_ok() {
                            let (w, h) = printer.printable_rect();
                            self.printable.set_size(w - 40, h - 40);
                            // Needs cleanup
                            let line_count = self.printable.count_lines(0, self.printable.buffer().unwrap().length(), true) / 45;
                            for i in 0..=line_count {
                                self.printable.scroll(45 * i, 0);
                                printer.begin_page().ok();
                                printer.print_widget(&self.printable, 20, 20);
                                printer.end_page().ok();
                            }
                            printer.end_job();
                        }
                    },
                    Quit => {
                        if self.modified {
                            match dialog::choice2(center().0 - 200, center().1 - 100,
                                "Would you like to save your work?", "&Yes", "&No", "") {
                                Some(0) => {
                                    if self.save_file().unwrap() {
                                        self.app.quit();
                                    }
                                },
                                Some(1) => { self.app.quit() },
                                Some(_) | None  => (),
                            }
                        } else {
                            self.app.quit();
                        }
                    },
                    Cut => self.editor.cut(),
                    Copy => self.editor.copy(),
                    Paste => self.editor.paste(),
                    About => dialog::message(center().0 - 300, center().1 - 100, "This is an example application written in Rust and using the FLTK Gui library."),
                }
            }
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut app = MyApp::new(args);
    app.launch();
}
