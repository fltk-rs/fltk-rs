use fltk::{menu::*, input::*, dialog::FileDialog, window::*};

fn main() {
    let mut wind = Window::new().set(100, 100, 800, 600, "RustyEd");
    let _editor = MultilineInput::new().set(5, 40, 790, 550, "");
    let mut menu = MenuBar::new().set(0, 0, 800, 40, "");
    menu.add("File/New...", 0, 0, &mut || {
        println!("{:?}", fl::event());
        let mut dlg = FileDialog::new(0);
        dlg.show();
    });
    menu.add("File/Open...", 0, 0, &mut || println!("{:?}", fl::event()));
    menu.add("File/Save", 0, 0, &mut || println!("{:?}", fl::event()));
    menu.add("File/Save as...", 0, 128, &mut || println!("{:?}", fl::event()));
    menu.add("File/Quit", 0, 0, &mut || println!("{:?}", fl::event()));
    menu.add("Edit/Cut", 0, 0, &mut || println!("{:?}", fl::event()));
    menu.add("Edit/Copy", 0, 0, &mut || println!("{:?}", fl::event()));
    menu.add("Edit/Paste", 0, 0, &mut || println!("{:?}", fl::event()));
    menu.add("Help/About", 0, 0, &mut || unimplemented!());
    wind.end();
    wind.show();
    fl::run();
}