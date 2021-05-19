use fltk::{
    app, dialog,
    enums::{CallbackTrigger, Color, Event, Font, FrameType, Shortcut},
    menu,
    prelude::*,
    printer, text, window,
};
use std::{
    error,
    ops::{Deref, DerefMut},
    panic, path,
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
    _menu: menu::SysMenuBar,
}

impl MyMenu {
    pub fn new(s: &app::Sender<Message>) -> Self {
        let mut menu = menu::SysMenuBar::default().with_size(800, 35);
        menu.add_emit(
            "&File/New...\t",
            Shortcut::Ctrl | 'n',
            menu::MenuFlag::Normal,
            *s,
            Message::New,
        );

        menu.add_emit(
            "&File/Open...\t",
            Shortcut::Ctrl | 'o',
            menu::MenuFlag::Normal,
            *s,
            Message::Open,
        );

        menu.add_emit(
            "&File/Save\t",
            Shortcut::Ctrl | 's',
            menu::MenuFlag::Normal,
            *s,
            Message::Save,
        );

        menu.add_emit(
            "&File/Save as...\t",
            Shortcut::Ctrl | 'w',
            menu::MenuFlag::Normal,
            *s,
            Message::SaveAs,
        );

        menu.add_emit(
            "&File/Print...\t",
            Shortcut::Ctrl | 'p',
            menu::MenuFlag::MenuDivider,
            *s,
            Message::Print,
        );

        menu.add_emit(
            "&File/Quit\t",
            Shortcut::Ctrl | 'q',
            menu::MenuFlag::Normal,
            *s,
            Message::Quit,
        );

        menu.add_emit(
            "&Edit/Cut\t",
            Shortcut::Ctrl | 'x',
            menu::MenuFlag::Normal,
            *s,
            Message::Cut,
        );

        menu.add_emit(
            "&Edit/Copy\t",
            Shortcut::Ctrl | 'c',
            menu::MenuFlag::Normal,
            *s,
            Message::Copy,
        );

        menu.add_emit(
            "&Edit/Paste\t",
            Shortcut::Ctrl | 'v',
            menu::MenuFlag::Normal,
            *s,
            Message::Paste,
        );

        menu.add_emit(
            "&Help/About\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            *s,
            Message::About,
        );

        if let Some(mut item) = menu.find_item("&File/Quit\t") {
            item.set_label_color(Color::Red);
        }

        Self { _menu: menu }
    }
}

pub struct MyApp {
    app: app::App,
    saved: bool,
    filename: String,
    r: app::Receiver<Message>,
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
        let _menu = MyMenu::new(&s);
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
                "An error occured while opening the file!"
            );
            buf.load_file(&args[1]).unwrap();
            args[1].clone()
        } else {
            String::new()
        };

        // Handle drag and drop
        editor.handle({
            let mut dnd = false;
            let mut released = false;
            let mut buf = buf.clone();
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
            }
        });

        // What shows when we attempt to print
        let mut printable = text::TextDisplay::default();
        printable.set_frame(FrameType::NoBox);
        printable.set_scrollbar_size(0);
        printable.set_buffer(Some(buf.clone()));

        Self {
            app,
            saved: true,
            filename,
            r,
            buf,
            editor,
            printable,
        }
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
                    self.buf.save_file(&filename).unwrap_or_else(|_| {
                        dialog::alert(center().0 - 200, center().1 - 100, "Please specify a file!")
                    });
                    self.saved = true;
                }
            } else if path::Path::new(&filename).exists() {
                self.buf.save_file(&filename)?;
                self.saved = true;
            } else {
                dialog::alert(center().0 - 200, center().1 - 100, "Please specify a file!")
            }
        } else {
            let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseSaveFile);
            dlg.set_option(dialog::FileDialogOptions::SaveAsConfirm);
            dlg.show();
            filename = dlg.filename().to_string_lossy().to_string();
            if !filename.is_empty() {
                self.buf.save_file(&filename).unwrap_or_else(|_| {
                    dialog::alert(center().0 - 200, center().1 - 100, "Please specify a file!")
                });
                self.saved = true;
            }
        }
        Ok(())
    }
    pub fn launch(&mut self) {
        while self.app.wait() {
            use Message::*;
            if let Some(msg) = self.r.recv() {
                match msg {
                    Changed => self.saved = false,
                    New => {
                        if self.buf.text() != "" {
                            let x = dialog::choice(center().0 - 200, center().1 - 100, "File unsaved, Do you wish to continue?", "Yes", "No!", "");
                            if x == 0 {
                                self.buf.set_text("");
                            }
                        }
                    },
                    Open => {
                        let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
                        dlg.set_option(dialog::FileDialogOptions::NoOptions);
                        dlg.set_filter("*.{txt,rs,toml}");
                        dlg.show();
                        let filename = dlg.filename().to_string_lossy().to_string();
                        if !filename.is_empty() {
                            if path::Path::new(&filename).exists() { self.buf.load_file(&filename).unwrap(); self.filename = filename; } else { dialog::alert(center().0 - 200, center().1 - 100, "File does not exist!") }
                        }
                    },
                    Save | SaveAs => self.save_file().unwrap(),
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
                        if self.saved {
                            self.app.quit();
                        } else {
                            let x = dialog::choice(center().0 - 200, center().1 - 100, "Would you like to save your work?", "Yes", "No", "");
                            if x == 0 {
                                self.save_file().unwrap();
                                self.app.quit();
                            } else {
                                self.app.quit();
                            }
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
    panic::set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            dialog::message(center().0 - 200, center().1 - 100, s);
        } else {
            dialog::message(center().0 - 200, center().1 - 100, &info.to_string());
        }
    }));
    let args: Vec<_> = std::env::args().collect();
    let mut app = MyApp::new(args);
    app.launch();
}
