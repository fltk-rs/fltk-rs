use fltk::{app, text::*, window::*};
use std::process::{Command, Output, Stdio};

fn main() {
    let app = app::App::default().set_scheme(app::AppScheme::Plastic);
    let mut wind = Window::new(100, 100, 640, 480, "Rusty Terminal");
    let mut current_dir = std::env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    current_dir.push_str("/ $ ");
    let mut buf = TextBuffer::default();
    let mut term = TextDisplay::new(5, 5, 630, 470, &mut buf);
    term.set_color(Color::Black);
    term.set_text_color(Color::White);
    term.set_text_font(Font::Courrier);
    term.set_cursor_color(Color::White);
    term.set_cursor_style(CursorStyle::BlockCursor);
    let mut cmd = String::from("");
    term.clone().set_custom_handler(Box::new(|ev: app::Event| {
        // println!("{:?}", app::event());
        // println!("{:?}", app::event_key());
        // println!("{:?}", app::event_text());
        match ev {
            // fltk bug with Event::KeyDown 
            app::Event::Shortcut => match app::event_key() {
                app::Key::Enter => {
                    term.append("\n");
                    run_command(&mut term, &cmd);
                    term.append(&current_dir);
                    cmd.clear();
                    true
                }
                app::Key::BackSpace => {
                    if cmd.len() != 0 {
                        term.remove(term.text().len() - 1, term.text().len());
                        cmd.pop().unwrap();
                        return true;
                    } else {
                        return false;
                    }
                }
                _ => {
                    let temp = app::event_text();
                    cmd.push_str(&temp);
                    term.append(&temp);
                    true
                }
            },
            _ => false,
        }
    }));
    term.append(&current_dir);
    term.show_cursor(true);
    wind.make_resizable(true);
    wind.show();
    app.run().unwrap();
}

// To have continuous streaming of output for long standing operations,
// consider using Tokio Command or the likes
fn run_command(term: &mut TextDisplay, cmd: &str) {
    let args: Vec<&str> = cmd.split_whitespace().collect();
    let stdout: Result<Output, std::io::Error>;
    if args.len() > 0 {
        if args.len() > 1 {
            stdout = Command::new(args[0])
                .args(&args[1..])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();
        } else {
            stdout = Command::new(args[0])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();
        }
        if stdout.is_err() {
            let msg = format!("{}: command not found!\n", cmd);
            term.append(&msg);
            return;
        }
        let stdout = stdout.unwrap().stdout;
        let stdout = String::from_utf8_lossy(&stdout).to_string();
        term.append(&stdout);
    }
}
