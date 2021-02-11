use fltk::{app, button::*, dialog::*, window::*};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");

    let mut but = Button::new(160, 210, 80, 40, "Click me!");

    wind.end();
    wind.show();

    let opts = FileChooserType::Multi;
    println!("{:?}", opts);

    but.set_callback(|| {
        let file = file_chooser("Choose File", "*.rs", ".", true).unwrap();
        println!("{}", file);

        // OR

        // let mut chooser = FileChooser::new(
        //     ".",                    // directory
        //     "*",                    // filter or pattern
        //     FileChooserType::Multi, // chooser type
        //     "Title Of Chooser",     // title
        // );

        // chooser.show();

        // chooser.window().set_pos(300, 300);

        // // Block until user picks something.
        // //     (The other way to do this is to use a callback())
        // //
        // while chooser.shown() {
        //     app::wait();
        // }

        // // User hit cancel?
        // if chooser.value(1).is_none() {
        //     println!("(User hit 'Cancel')");
        //     return;
        // }

        // // Print what the user picked
        // println!("--------------------");
        // println!("DIRECTORY: '{}'", chooser.directory().unwrap());
        // println!("    VALUE: '{}'", chooser.value(1).unwrap()); // value starts at 1!
        // println!("    COUNT: {} files selected", chooser.count());

        // // Multiple files? Show all of them
        // if chooser.count() > 1 {
        //     for t in 1..=chooser.count() {
        //         println!(" VALUE[{}]: '{}'", t, chooser.value(t).unwrap());
        //     }
        // }
    });

    app.run().unwrap();
}
