use fltk::{
    app,
    enums::*,
    prelude::*,
    text::{SimpleTerminal, StyleTableEntry, TextBuffer},
    window::Window,
};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::process::{Command, Stdio};

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

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

        let mut term = SimpleTerminal::new(5, 5, WIDTH - 10, HEIGHT - 10, "");

        let sbuf = TextBuffer::default();

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

        term.set_highlight_data(sbuf.clone(), styles);

        Term {
            term,
            current_dir,
            cmd: String::from(""),
            sbuf,
        }
    }

    fn append_txt(&mut self, txt: &str) {
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

        if !args.is_empty() {
            let mut cmd = Command::new(args[0]);
            if args.len() > 1 {
                if args[0] == "cd" {
                    let path = args[1];
                    return self.change_dir(&Path::new(path));
                } else {
                    cmd.args(&args[1..]);
                }
            }
            let out = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).output();
            if let Ok(out) = out {
                let stdout = out.stdout;
                String::from_utf8_lossy(&stdout).to_string()
            } else {
                let msg = format!("{}: command not found!\n", self.cmd);
                msg
            }
        } else {
            String::from("")
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
            self.current_dir = current_dir;
            String::from("")
        } else {
            String::from("Path does not exist!\n")
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
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let mut wind = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("ColorTerminal");

    let mut term = Term::new();

    let dir = term.current_dir.clone();
    term.append_txt(&dir);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    let mut term_c = term.clone();
    term_c.handle(move |_, ev| {
        // println!("{:?}", app::event());
        // println!("{:?}", app::event_key());
        // println!("{:?}", app::event_text());
        match ev {
            Event::KeyDown => match app::event_key() {
                Key::Enter => {
                    term.append_txt("\n");
                    let out = term.run_command();
                    if out.contains("not found") {
                        term.append_error(&out);
                    } else {
                        term.append_txt(&out);
                    }
                    let current_dir = term.current_dir.clone();
                    term.append_txt(&current_dir);
                    term.cmd.clear();
                    true
                }
                Key::BackSpace => {
                    if !term.cmd.is_empty() {
                        let text_len = term.text().len() as u32;
                        term.buffer().unwrap().remove(text_len - 1, text_len);
                        term.sbuf.remove(text_len - 1, text_len);
                        term.cmd.pop().unwrap();
                        true
                    } else {
                        false
                    }
                }
                _ => {
                    let temp = app::event_text();
                    term.cmd.push_str(&temp);
                    term.append_txt(&temp);
                    true
                }
            },
            _ => false,
        }
    });

    app.run().unwrap();
}
