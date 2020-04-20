use fltk::{app, text::*, window::*};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

struct Term<'a> {
    term: TextDisplay,
    current_dir: String,
    cmd: String,
    buf: &'a mut TextBuffer,
}

impl<'a> Term<'a> {
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
            buf: buf,
        }
    }
    pub fn style(&mut self) {
        let dir = &self.current_dir.clone();
        self.append(dir);
        self.term.set_color(Color::Black);
        self.term.set_text_color(Color::Green);
        self.term.set_text_font(Font::Courrier);
        self.term.set_cursor_color(Color::Green);
        self.term.set_cursor_style(CursorStyle::BlockCursor);
        self.term.show_cursor(true);
    }
    fn append(&mut self, txt: &str) {
        self.buf.append(txt);
        self.term.set_insert_position(self.buf.length());
        self.term
            .scroll(self.term.count_lines(0, self.buf.length(), true), 0);
    }
    fn run_command(&mut self) {
        let args = self.cmd.clone();
        let args: Vec<&str> = args.split_whitespace().collect();
        if args.len() > 0 {
            let mut cmd = Command::new(args[0]);
            if args.len() > 1 {
                if args[0] == "cd" {
                    let path = args[1];
                    self.change_dir(&PathBuf::from(path));
                    return;
                }
                cmd.args(&args[1..]);
            }
            let out = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).output();
            if out.is_err() {
                let msg = format!("{}: command not found!\n", self.cmd);
                self.append(&msg);
                return;
            }
            let stdout = out.unwrap().stdout;
            let stdout = String::from_utf8_lossy(&stdout).to_string();
            self.append(&stdout);
        }
    }
    pub fn change_dir(&mut self, path: &Path) {
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
        } else {
            self.append("Path does not exist!\n");
        }
    }
    // To have continuous streaming of output for long standing operations,
    // consider using Tokio Command or the likes
    pub fn handle_events(&mut self) {
        self.term
            .clone()
            .handle(Box::new(|ev: app::Event| {
                // println!("{:?}", app::event());
                // println!("{:?}", app::event_key());
                // println!("{:?}", app::event_text());
                match ev {
                    app::Event::KeyDown => match app::event_key() {
                        app::Key::Enter => {
                            self.append("\n");
                            self.run_command();
                            let current_dir = self.current_dir.clone();
                            self.append(&current_dir);
                            self.cmd.clear();
                            true
                        }
                        app::Key::BackSpace => {
                            if self.cmd.len() != 0 {
                                let text_len = self.term.buffer().text().len() as u32;
                                self.buf.remove(text_len - 1, text_len as u32);
                                self.cmd.pop().unwrap();
                                return true;
                            } else {
                                return false;
                            }
                        }
                        _ => {
                            let temp = app::event_text();
                            self.cmd.push_str(&temp);
                            self.append(&temp);
                            true
                        }
                    },
                    _ => false,
                }
            }));
    }
}

fn main() {
    let app = app::App::default().set_scheme(app::AppScheme::Plastic);
    let mut wind = Window::new(100, 100, 640, 480, "Rusty Terminal");
    let mut buf = TextBuffer::default();
    let mut term = Term::new(&mut buf);
    term.style();
    wind.make_resizable(true);
    wind.end();
    wind.show();
    term.handle_events();
    app.run().unwrap();
}
