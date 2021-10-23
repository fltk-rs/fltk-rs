use fltk::{
    app,
    enums::{Color, Event, Shortcut, Font, Key},
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt},
    text::{SimpleTerminal, StyleTableEntry, TextBuffer},
    utils,
    window::Window,
};
use std::io::{self, BufRead, BufReader};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::process::{Command, Stdio};

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

pub trait TerminalFuncs {
    fn append_txt(&mut self, txt: &str);
    fn append_dir(&mut self, dir: &str);
    fn append_error(&mut self, txt: &str);
    fn run_command(&mut self, cmd: &str, cwd: &mut String, r: app::Receiver<bool>);
    fn change_dir(&mut self, path: &Path, current: &mut String) -> io::Result<()>;
}

impl TerminalFuncs for SimpleTerminal {
    fn append_txt(&mut self, txt: &str) {
        self.append(txt);
        self.style_buffer().unwrap().append(&"A".repeat(txt.len()));
    }

    fn append_dir(&mut self, dir: &str) {
        self.append(dir);
        self.style_buffer().unwrap().append(&"C".repeat(dir.len()));
    }

    fn append_error(&mut self, txt: &str) {
        self.append(txt);
        self.style_buffer().unwrap().append(&"B".repeat(txt.len()));
    }

    fn run_command(&mut self, cmd: &str, cwd: &mut String, r: app::Receiver<bool>) {
        let args: Vec<String> = cmd.split_whitespace().map(|s| s.to_owned()).collect();

        if !args.is_empty() {
            let mut cmd_c = Command::new(&args[0]);
            if args[0] == "cd" {
                let path = &args[1];
                match self.change_dir(Path::new(&path), cwd) {
                    Ok(_) => self.append_dir(cwd),
                    _ => {
                        self.append_error("Path does not exist!");
                        self.append_txt("\n");
                        self.append_dir(cwd);
                    }
                }
            } else {
                let proc = cmd_c
                    .args(&args[1..])
                    .stderr(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn();

                if proc.is_err() {
                    self.append_error("Command not found!");
                    self.append_txt("\n");
                    self.append_dir(cwd);
                    return;
                }

                let reader = BufReader::new(proc.unwrap().stdout.unwrap());
                let mut term = self.clone();
                let cwd = cwd.clone();
                std::thread::spawn(move || {
                    reader
                        .lines()
                        .filter_map(|line| line.ok())
                        .try_for_each(|line| {
                            if let Some(msg) = r.recv() {
                                match msg {
                                    true => {
                                        term.append_error("Received sigint signal!\n");
                                        app::awake();
                                        return None;
                                    },
                                    false => (),
                                }
                            }
                            term.append_txt(&line);
                            term.append_txt("\n");
                            app::awake();
                            Some(())
                        });
                    term.append_dir(&cwd);
                });
            }
        } else {
            self.append_dir(&cwd);
        }
    }

    fn change_dir(&mut self, path: &Path, current: &mut String) -> io::Result<()> {
        std::env::set_current_dir(path)?;
        let mut path = std::env::current_dir()?.to_string_lossy().to_string();
        path.push_str("$ ");
        *current = path;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Term {
    term: SimpleTerminal,
}

impl Term {
    pub fn new() -> Term {
        let mut cmd = String::new();

        // Enable different colored text in TestDisplay
        let styles: Vec<StyleTableEntry> = vec![
            StyleTableEntry {
                color: Color::Green,
                font: Font::Courier,
                size: 16,
            },
            StyleTableEntry {
                color: Color::Red,
                font: Font::Courier,
                size: 16,
            },
            StyleTableEntry {
                color: Color::from_u32(0x8000ff),
                font: Font::Courier,
                size: 16,
            },
        ];

        let mut sbuf = TextBuffer::default();
        let mut term = SimpleTerminal::new(5, 5, WIDTH - 10, HEIGHT - 10, "");

        term.set_highlight_data(sbuf.clone(), styles);

        let mut curr = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();
        curr.push_str("$ ");

        term.append_dir(&curr);

        let (s, r) = app::channel();

        term.handle(move |t, ev| {
            // println!("{:?}", app::event());
            // println!("{:?}", app::event_key());
            // println!("{:?}", app::event_text());
            match ev {
                Event::KeyDown => match app::event_key() {
                    Key::Enter => {
                        t.append_txt("\n");
                        t.run_command(&cmd, &mut curr, r);
                        cmd.clear();
                        true
                    }
                    Key::BackSpace => {
                        if !cmd.is_empty() {
                            let c = cmd.pop().unwrap();
                            let len = if c.is_ascii() {
                                1
                            } else {
                                utils::char_len(c) as i32
                            };
                            let text_len = t.text().len() as i32;
                            t.buffer().unwrap().remove(text_len - len, text_len);
                            sbuf.remove(text_len - len, text_len);
                            true
                        } else {
                            false
                        }
                    }
                    _ => {
                        if let Some(ch) = app::event_text().chars().next() {
                            if app::compose().is_some() {
                                let temp = ch.to_string();
                                cmd.push_str(&temp);
                                t.append_txt(&temp);
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                },
                Event::KeyUp => {
                    if app::event_state() == Shortcut::Ctrl {
                        if app::event_key() == Key::from_char('c') {
                            s.send(true);
                        }
                    }
                    false
                }
                _ => false,
            }
        });

        Self { term }
    }
}

impl Deref for Term {
    type Target = SimpleTerminal;

    fn deref(&self) -> &Self::Target {
        &self.term
    }
}

impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.term
    }
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let mut wind = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("ColorTerminal");

    let _term = Term::new();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
