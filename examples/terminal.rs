use fltk::{text::*, window::*};
use std::process::{Command, Stdio};

fn main() {
    let app = fl::App::default().set_scheme(AppScheme::Gtk);
    let mut wind = Window::new(100, 100, 640, 480, "Rusty Terminal");
    let mut current_dir = std::env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    current_dir.push_str("/ $ ");
    let mut term = TextEditor::new(5, 5, 630, 470);
    let mut cmd = String::from("");
    term.clone()
        .set_custom_handler(Box::new(|ev: Event| match ev {
            fl::Event::KeyUp => {
                if fl::event_key() == fl::Key::Enter as i32 {
                    run_command(&mut term);
                    term.append(&current_dir);
                    cmd.clear();
                } else if fl::event_key() == fl::Key::BackSpace as i32 {
                    if cmd.len() == 0 {
                        term.append(" ");
                    }
                } else {
                    cmd.push(fl::event_char());
                }
                true
            }
            _ => false,
        }));
    term.append(&current_dir);
    wind.make_resizable(true);
    wind.show();
    app.run().unwrap();
}

// To have continuous streaming of output for long standing operations, 
// consider using Tokio Command or the likes
fn run_command(term: &mut TextEditor) {
    let txt = term.text();
    let mut lines: Vec<_> = txt.lines().collect();
    lines.reverse();
    let args: Vec<&str> = (&lines[0]).split_whitespace().collect();
    if args.len() > 2 {
        let stdout = Command::new(&args[2])
            .args(&args[3..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();
        if stdout.is_err() {
            let msg = format!("{}: command not found!\n", &args[2]);
            term.append(&msg);
            return;
        }
        let stdout = stdout.unwrap().stdout;
        let stdout = String::from_utf8_lossy(&stdout).to_string();
        term.append(&stdout);
    }
}
