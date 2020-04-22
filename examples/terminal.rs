use fltk::{app, text::*, window::*};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug)]
struct Term {
    pub term: TextDisplay,
    current_dir: String,
    cmd: String,
}

impl Term {
    pub fn new(mut buf: &mut TextBuffer) -> Term {
        let mut current_dir = std::env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        current_dir.push_str("/ $ ");
        Term {
            term: TextDisplay::new(5, 5, 630, 470, &mut buf),
            current_dir: current_dir,
            cmd: String::from(""),
        }
    }
    pub fn style(&mut self) {
        self.term.set_color(Color::Black);
        self.term.set_text_color(Color::Green);
        self.term.set_text_font(Font::Courrier);
        self.term.set_cursor_color(Color::Green);
        self.term.set_cursor_style(CursorStyle::BlockCursor);
        self.term.show_cursor(true);
    }
    fn append(&mut self, buf: &mut TextBuffer, txt: &str) {
        buf.append(txt);
        self.term.set_insert_position(buf.length());
        self.term
            .scroll(self.term.count_lines(0, buf.length(), true), 0);
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
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
            current_dir.push_str("/ $ ");
            self.current_dir = current_dir.clone();
            return String::from("");
        } else {
            return String::from("Path does not exist!\n");
        }
    }
}

fn main() {
    let app = app::App::default().set_scheme(app::AppScheme::Plastic);
    let mut wind = Window::new(100, 100, 640, 480, "Rusty Terminal");
    let mut buf = TextBuffer::default();
    let mut term = Term::new(&mut buf);
    term.style();
    let dir = term.current_dir.clone();
    term.append(&mut buf, &dir);
    wind.make_resizable(true);
    wind.end();
    wind.show();
    let inner = term.term;
    inner.clone().handle(Box::new(move |ev| {
            // println!("{:?}", app::event());
            // println!("{:?}", app::event_key());
            // println!("{:?}", app::event_text());
            match ev {
                app::Event::KeyDown => match app::event_key() {
                    app::Key::Enter => {
                        term.append(&mut buf, "\n");
                        let out = term.run_command();
                        term.append(&mut buf, &out);
                        let current_dir = term.current_dir.clone();
                        term.append(&mut buf, &current_dir);
                        term.cmd.clear();
                        true
                    }
                    app::Key::BackSpace => {
                        if term.cmd.len() != 0 {
                            let text_len = inner.buffer().text().len() as u32;
                            buf.remove(text_len - 1, text_len as u32);
                            term.cmd.pop().unwrap();
                            return true;
                        } else {
                            return false;
                        }
                    }
                    _ => {
                        let temp = app::event_text();
                        term.cmd.push_str(&temp);
                        term.append(&mut buf, &temp);
                        true
                    }
                },
                _ => false,
            }
    }));
    app.run().unwrap();
}
