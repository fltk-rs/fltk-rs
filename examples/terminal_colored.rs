use fltk::{app, text::*, window::*};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};


#[derive(Debug, Clone)]
struct Term {
    pub term: SimpleTerminal,
    current_dir: String,
    cmd: String,
    sbuf: TextBuffer,
}

impl Term {
    pub fn new() -> Term {
        let mut current_dir = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();

        current_dir.push_str("$ ");

        let mut term = SimpleTerminal::new(5, 5, 630, 470);

        let mut sbuf = TextBuffer::default();

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

        term.set_highlight_data(&mut sbuf, styles);

        Term {
            term: term,
            current_dir: current_dir,
            cmd: String::from(""),
            sbuf: sbuf,
        }
    }

    fn append(&mut self, txt: &str) {
        self.term.append(txt);
        if txt == self.current_dir.as_str() {
            self.sbuf.append(&"C".repeat(txt.len()));
        } else {
            self.sbuf.append(&"A".repeat(txt.len()));
        }
    }

    fn append_error(&mut self, txt: &str) {
        self.term.append(txt);
        self.sbuf.append(&"B".repeat(txt.len()));
    }

    fn run_command(&mut self) -> String {
        let args = self.cmd.clone();
        let args: Vec<&str> = args.split_whitespace().collect();

        if args.len() > 0 {
            let mut cmd = Command::new(args[0]);
            if args.len() > 1 {
                if args[0] == "cd" {
                    let path = args[1];
                    return self.change_dir(&PathBuf::from(path));
                } else {
                    cmd.args(&args[1..]);
                }
            }
            let out = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).output();
            if out.is_err() {
                let msg = format!("{}: command not found!\n", self.cmd);
                return msg;
            } else {
                let stdout = out.unwrap().stdout;
                let stdout = String::from_utf8_lossy(&stdout).to_string();
                return stdout;
            }
        } else {
            return String::from("");
        }
    }

    pub fn change_dir(&mut self, path: &Path) -> String {
        if path.exists() && path.is_dir() {
            std::env::set_current_dir(path).unwrap();
            let mut current_dir = std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .to_string();
            current_dir.push_str("$ ");
            self.current_dir = current_dir.clone();
            return String::from("");
        } else {
            return String::from("Path does not exist!\n");
        }
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
    let app = app::App::default().set_scheme(app::AppScheme::Plastic);
    let mut wind = Window::new(100, 100, 640, 480, "Color Terminal");

    let mut term = Term::new();

    let dir = term.current_dir.clone();
    term.append(&dir);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    let mut term_c = term.clone();
    term_c.handle(Box::new(move |ev| {
        // println!("{:?}", app::event());
        // println!("{:?}", app::event_key());
        // println!("{:?}", app::event_text());
        match ev {
            Event::KeyDown => match app::event_key() {
                Key::Enter => {
                    term.append("\n");
                    let out = term.run_command();
                    if out.contains("not found") {
                        term.append_error(&out);
                    } else {
                        term.append(&out);
                    }
                    let current_dir = term.current_dir.clone();
                    term.append(&current_dir);
                    term.cmd.clear();
                    true
                }
                Key::BackSpace => {
                    if term.cmd.len() != 0 {
                        let text_len = term.text().len() as u32;
                        term.buffer().remove(text_len - 1, text_len);
                        term.sbuf.remove(text_len - 1, text_len);
                        term.cmd.pop().unwrap();
                        return true;
                    } else {
                        return false;
                    }
                }
                _ => {
                    let temp = app::event_text();
                    term.cmd.push_str(&temp);
                    term.append(&temp);
                    true
                }
            },
            _ => false,
        }
    }));

    app.run().unwrap();
}
