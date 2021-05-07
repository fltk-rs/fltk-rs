use fltk::{
    app,
    enums::{Color, Event, Font, Key},
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt},
    text::{SimpleTerminal, StyleTableEntry, TextBuffer},
    window::Window,
};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::process::{Command, Stdio};

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

pub trait TerminalFuncs {
    fn append_txt(&mut self, txt: &str);
    fn append_dir(&mut self, dir: &str);
    fn append_error(&mut self, txt: &str);
    fn run_command(&mut self, cmd: &str, cwd: &mut String) -> String;
    fn change_dir(&mut self, path: &Path, current: &mut String) -> String;
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

    fn run_command(&mut self, cmd: &str, cwd: &mut String) -> String {
        let args: Vec<&str> = cmd.split_whitespace().collect();

        if !args.is_empty() {
            let mut cmd = Command::new(args[0]);
            if args.len() > 1 {
                if args[0] == "cd" {
                    let path = args[1];
                    return self.change_dir(&Path::new(path), cwd);
                } else {
                    cmd.args(&args[1..]);
                }
            }
            let out = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).output();
            if let Ok(out) = out {
                let stdout = out.stdout;
                String::from_utf8_lossy(&stdout).to_string()
            } else {
                let msg = format!("{}: command not found!\n", args[0]);
                msg
            }
        } else {
            String::from("")
        }
    }

    fn change_dir(&mut self, path: &Path, current: &mut String) -> String {
        if path.exists() && path.is_dir() {
            std::env::set_current_dir(path).unwrap();
            let mut path = std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .to_string();
            path.push_str("$ ");
            *current = path;
            String::new()
        } else {
            String::from("Path does not exist!\n")
        }
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

        term.handle(move |t, ev| {
            // println!("{:?}", app::event());
            // println!("{:?}", app::event_key());
            // println!("{:?}", app::event_text());
            match ev {
                Event::KeyDown => match app::event_key() {
                    Key::Enter => {
                        t.append_txt("\n");
                        let out = t.run_command(&cmd, &mut curr);
                        if out.contains("not found") {
                            t.append_error(&out);
                        } else {
                            t.append_txt(&out);
                        }
                        t.append_dir(&curr);
                        cmd.clear();
                        true
                    }
                    Key::BackSpace => {
                        if !cmd.is_empty() {
                            let text_len = t.text().len() as i32;
                            t.buffer().unwrap().remove(text_len - 1, text_len);
                            sbuf.remove(text_len - 1, text_len);
                            cmd.pop().unwrap();
                            true
                        } else {
                            false
                        }
                    }
                    _ => {
                        let temp = app::event_text();
                        cmd.push_str(&temp);
                        t.append_txt(&temp);
                        true
                    }
                },
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
