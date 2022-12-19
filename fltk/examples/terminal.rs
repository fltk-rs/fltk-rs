mod pipe {
    use std::fs::File;
    use std::process::{Stdio};

    #[cfg(unix)]
    use std::os::unix::io::FromRawFd;
    #[cfg(windows)]
    use std::os::windows::io::FromRawHandle;

    pub struct Pipe(i32, i32);

    impl Pipe {
        /// Safety:
        /// Doesn't lock file descriptors. This is just for this demo!
        pub unsafe fn new() -> Self {
            use std::os::raw::*;
            if cfg!(unix) {
                let mut fds: [c_int; 2] = [0; 2];
                extern "C" {
                    fn pipe(arg: *mut i32) -> i32;
                }
                let res = pipe(fds.as_mut_ptr());
                if res != 0 {
                    panic!("Failed to create pipe!");
                }
                Self(fds[0], fds[1])
            } else if cfg!(windows) {
                extern "system" {
                    fn CreatePipe(
                        rp: *mut isize,
                        wp: *mut isize,
                        attrs: *mut (),
                        sz: c_ulong,
                    ) -> c_int;
                }
                let mut rp = -1isize;
                let mut wp = -1isize;
                let res = CreatePipe(&mut rp as _, &mut wp as _, std::ptr::null_mut(), 0);
                if res == 0 {
                    panic!("Failed to create pipe!");
                }
                Self(rp as i32, wp as i32)
            } else {
                panic!("Unknown platform!");
            }
        }

        pub fn reader_stream(&self) -> Stdio {
            #[cfg(unix)]
            unsafe {
                Stdio::from_raw_fd(self.1)
            }
            #[cfg(windows)]
            unsafe {
                Stdio::from_raw_handle(std::mem::transmute(self.1 as isize))
            }
        }

        pub fn reader(&self) -> File {
            #[cfg(unix)]
            unsafe {
                File::from_raw_fd(self.0)
            }
            #[cfg(windows)]
            unsafe {
                File::from_raw_handle(std::mem::transmute(self.0 as isize))
            }
        }
    }
}

mod ansi_term {
    use fltk::{enums::*, prelude::*, *};
    use std::io::{Read, Write};
    use std::process::{Command, Stdio};

    pub struct AnsiTerm {
        st: text::SimpleTerminal,
    }

    impl Default for AnsiTerm {
        fn default() -> Self {
            AnsiTerm::new(0, 0, 0, 0, None)
        }
    }

    impl AnsiTerm {
        pub fn new<L: Into<Option<&'static str>>>(
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            label: L,
        ) -> Self {
            let mut st = text::SimpleTerminal::new(x, y, w, h, label);
            st.set_ansi(true);

            let mut cmd = if cfg!(target_os = "windows") {
                Command::new("cmd.exe")
            } else {
                let mut cmd = Command::new("/bin/bash");
                cmd.args(&["-i"]);
                cmd
            };

            let pipe = unsafe { crate::pipe::Pipe::new() };
            let stdio = pipe.reader_stream();
            let stderr = pipe.reader_stream();
            let mut child = cmd
                .stdout(stdio)
                .stderr(stderr)
                .stdin(Stdio::piped())
                .spawn()
                .unwrap();
            let mut writer = child.stdin.take().unwrap();
            let mut reader = pipe.reader();
            std::thread::spawn({
                let mut st = st.clone();
                move || {
                    while child.try_wait().is_ok() {
                        let mut msg = [0u8; 1028];
                        if let Ok(sz) = reader.read(&mut msg) {
                            let msg = &msg[0..sz];
                            format(msg, &mut st);
                            app::awake();
                        }
                        std::thread::sleep(std::time::Duration::from_millis(30));
                    }
                }
            });

            let mut cmd = String::new();
            st.handle(move |t, ev| {
                let mut buf = t.buffer().unwrap();
                let mut sbuf = t.style_buffer().unwrap();
                match ev {
                    Event::KeyDown => match app::event_key() {
                        Key::Enter => {
                            let len = cmd.len() as i32;
                            let text_len = t.text().len() as i32;
                            buf.remove(text_len - len, text_len);
                            sbuf.remove(text_len - len, text_len);
                            writer.write_all(cmd.as_bytes()).unwrap();
                            writer.write_all(b"\n").unwrap();
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
                                buf.remove(text_len - len, text_len);
                                sbuf.remove(text_len - len, text_len);
                                true
                            } else {
                                false
                            }
                        }
                        Key::Escape => {
                            // handle escape
                            true
                        },
                        _ => {
                            if let Some(ch) = app::event_text().chars().next() {
                                if app::compose().is_some() {
                                    let temp = ch.to_string();
                                    cmd.push_str(&temp);
                                    t.append(&temp);
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
                            let key = app::event_key();
                            if key != Key::ControlL && key != Key::ControlR {
                                if let Some(ch) = char::from_u32(key.bits() as u32 - 96) {
                                    writer.write_all(&[ch as u8]).unwrap();
                                }
                            }
                        }
                        true
                    }
                    _ => false,
                }
            });
            Self { st }
        }
    }

    fltk::widget_extends!(AnsiTerm, text::SimpleTerminal, st);

    fn format(msg: &[u8], st: &mut text::SimpleTerminal) {
        // handle sticky title with bell
        if let Some(pos0) = msg.windows(4).position(|m| m == b"\x1b]0;") {
            let mut pos1 = pos0;
            while pos1 < msg.len() && msg[pos1] != b'[' {
                pos1 += 1;
            }
            st.append2(&msg[0..pos0]);
            st.append2(&msg[pos1 - 1..]);
        } else {
            st.append2(&msg);
        }
    }
}

use fltk::{prelude::*, *};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("AnsiTerminal");

    let _term = ansi_term::AnsiTerm::default()
        .with_size(WIDTH - 4, HEIGHT - 4)
        .center_of_parent();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
