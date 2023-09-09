use fltk::app::prefs::{Preferences, Root};

fn main() {
    let mut prefs = Preferences::new(Root::USER_L, "myfltk.org", "tempapp").unwrap();
    let mut opts = Preferences::new_group(&mut prefs, "Options").unwrap();
    opts.set_int("winsize", 400).unwrap();
    let size = opts.get_int("winsize").unwrap_or(300);
    opts.set_str("wintitle", "Hello").unwrap();
    let title = opts.get_str("wintitle").unwrap_or("My Window".to_string());
    println!("{}", size);
    println!("{}", title);
    println!("{:?}", opts.name());
    println!("{:?}", prefs.filename());
}
