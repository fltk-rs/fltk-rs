// Example showing a browser widget with right click popup menu

use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(211, 211, 211);

    let mut win = window::Window::default().with_size(900, 300);
    let mut b = browser::HoldBrowser::default()
        .with_size(900 - 10, 300 - 10)
        .center_of(&win);
    let widths = &[50, 50, 50, 70, 70, 40, 40, 70, 70, 50];

    b.set_column_widths(widths);
    b.set_column_char('\t');
    b.add("USER\tPID\t%CPU\t%MEM\tVSZ\tRSS\tTTY\tSTAT\tSTART\tTIME\tCOMMAND");
    b.add("root\t2888\t0.0\t0.0\t1352\t0\ttty3\tSW\tAug15\t0:00\t@b@f/sbin/mingetty tty3");
    b.add("erco\t2889\t0.0\t13.0\t221352\t0\ttty3\tR\tAug15\t1:34\t@b@f/usr/local/bin/render a35 0004");
    b.add("uucp\t2892\t0.0\t0.0\t1352\t0\tttyS0\tSW\tAug15\t0:00\t@b@f/sbin/agetty -h 19200 ttyS0 vt100");
    b.add("root\t13115\t0.0\t0.0\t1352\t0\ttty2\tSW\tAug30\t0:00\t@b@f/sbin/mingetty tty2");
    b.add(
        "root\t13464\t0.0\t0.0\t1352\t0\ttty1\tSW\tAug30\t0:00\t@b@f/sbin/mingetty tty1 --noclear",
    );

    let menu = menu::MenuItem::new(&["1st menu item\t", "2nd menu item\t", "3rd menu item\t"]);

    b.set_callback(move |_| {
        if app::event_mouse_button() == app::MouseButton::Right {
            // or app::event_button() == 3
            let coords = app::event_coords();
            match menu.popup(coords.0, coords.1) {
                None => println!("No value was chosen!"),
                Some(val) => println!("{}", val.label().unwrap()),
            }
        }
    });

    win.make_resizable(true);
    win.end();
    win.show();
    app.run().unwrap();
}
