use fltk::{app, text::*, window::*};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::process::{Command, Stdio};
use std::rc::Rc;

fn append(disp: &mut TextDisplay, txt: &str) {
    disp.buffer().unwrap().append(txt);
    disp.set_insert_position(disp.buffer().unwrap().length());
    disp.scroll(
        disp.count_lines(0, disp.buffer().unwrap().length(), true),
        0,
    );
}

#[derive(Debug, Clone)]
struct Term {
    pub term: TextDisplay,
    current_dir: Rc<RefCell<String>>,
    cmd: Rc<RefCell<String>>,
}

impl Term {
    pub fn default() -> Term {
        let mut current_dir = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();
        current_dir.push_str("$ ");

        let mut term = TextDisplay::new(5, 5, 630, 470, "");
        let buf = TextBuffer::default();
        term.set_buffer(Some(buf));
        term.set_color(Color::Black);
        term.set_text_color(Color::Green);
        term.set_text_font(Font::Courier);
        term.set_cursor_color(Color::Green);
        term.set_cursor_style(TextCursor::Block);
        term.show_cursor(true);
        append(&mut term, &current_dir);

        let current_dir = Rc::from(RefCell::from(current_dir));
        let cmd = Rc::from(RefCell::from(String::from("")));
        let cmd_c = cmd.clone();
        let current_dir_c = current_dir.clone();

        let run_command = move || -> String {
            let args = cmd_c.borrow().clone();
            let args: Vec<&str> = args.split_whitespace().collect();

            if !args.is_empty() {
                let mut proc = Command::new(args[0]);
                if args.len() > 1 {
                    if args[0] == "cd" {
                        let path = Path::new(args[1]);
                        if path.exists() && path.is_dir() {
                            std::env::set_current_dir(path).unwrap();
                            let mut dir = std::env::current_dir()
                                .unwrap()
                                .to_string_lossy()
                                .to_string();
                            dir.push_str("$ ");
                            *current_dir_c.borrow_mut() = dir;
                            return String::from("");
                        } else {
                            return String::from("Path does not exist!\n");
                        }
                    } else {
                        proc.args(&args[1..]);
                    }
                }
                let out = proc.stdout(Stdio::piped()).stderr(Stdio::piped()).output();
                if let Ok(out) = out {
                    let stdout = out.stdout;
                    String::from_utf8_lossy(&stdout).to_string()
                } else {
                    let msg = format!("{}: command not found!\n", &*cmd_c.borrow());
                    msg
                }
            } else {
                String::from("")
            }
        };

        let cmd_c = cmd.clone();
        let current_dir_c = current_dir.clone();

        term.handle2(move |t, ev| {
            // println!("{:?}", app::event());
            // println!("{:?}", app::event_key());
            // println!("{:?}", app::event_text());
            match ev {
                Event::KeyDown => match app::event_key() {
                    Key::Enter => {
                        append(t, "\n");
                        let out = run_command();
                        append(t, &out);
                        append(t, &current_dir_c.borrow());
                        cmd_c.borrow_mut().clear();
                        true
                    }
                    Key::BackSpace => {
                        if !cmd_c.borrow().is_empty() {
                            let text_len = t.buffer().unwrap().text().len() as u32;
                            t.buffer().unwrap().remove(text_len - 1, text_len as u32);
                            cmd_c.borrow_mut().pop().unwrap();
                            true
                        } else {
                            false
                        }
                    }
                    _ => {
                        let temp = app::event_text();
                        cmd_c.borrow_mut().push_str(&temp);
                        append(t, &temp);
                        true
                    }
                },
                _ => false,
            }
        });

        Term {
            term,
            current_dir,
            cmd,
        }
    }
}

impl Deref for Term {
    type Target = TextDisplay;

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
    let mut wind = Window::new(100, 100, 640, 480, "Rusty Terminal");
    let _t = Term::default();
    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
