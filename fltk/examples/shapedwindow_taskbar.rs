use fltk::{prelude::*, *};

/*
   Event handling for win and dock_win can be changed.

   In fn prep_shape(), if you want a different shape as your background e.g a pie (circle),
   change draw::draw_rounded_rectf(0, 0, w, h, 16) to your desired background e.g draw::pie
                                   x, y, w, h, r
   * DO NOT CHANGE ANYTHING ELSE IN THE prep_shape() FUNCTION *

   .center_screen(); -- Not a requirement
   .with_size(1, 1); -- If you use .center_screen(), (x, y) MUST be >0 otherwise a panic!() will be thrown
   win.set_color(Color::from_rgb(25, 26, 55)); -- Can be refactored to any color, no requirement e.g enums::Color::White
   button::Button::default(); -- Not a requirement, remove if need be and import your own Widgets
*/

fn main() {
    let app = app::App::default();

    // Act as the application in the taskbar (scroll to event handling)
    let mut dock_win = window::Window::default()
        .with_size(1, 1) // So we can place it at the center of the screen (needs a size >0 to be centered)
        .with_label("TestApplication")
        .center_screen();
    dock_win.size_range(0, 0, 0, 0);
    dock_win.make_resizable(false);

    dock_win.show();
    dock_win.end();

    let mut win = window::Window::default()
        .with_size(900, 500)
        .with_label("TestApplication")
        .center_screen();
    win.set_color(enums::Color::from_rgb(26, 25, 55));

    let mut but = button::Button::default()
        .with_label("Button")
        .with_size(80, 80)
        .center_of_parent();
    but.set_frame(enums::FrameType::OFlatFrame);
    but.set_color(enums::Color::Cyan);
    but.clear_visible_focus();
    but.set_callback(|_| println!("Clicked"));

    win.show();
    win.end();

    let win_shape = prep_shape(win.w(), win.h());

    // Called after showing window
    win.set_shape(Some(win_shape));

    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut dock_win = dock_win.clone();
        move |wself, event| match event {
            enums::Event::Push => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;

                true
            }
            enums::Event::Drag => {
                wself.set_pos(app::event_x_root() - x, app::event_y_root() - y);

                // Changing dock window position so it's close enough to the center of the application (not "visible" to user)
                dock_win.set_pos(wself.x() + (wself.w() / 2), wself.y() + (wself.w() / 2));

                true
            }
            enums::Event::Close => {
                app.quit();

                true
            }
            enums::Event::Hide => {
                app.quit();

                true
            }
            _ => false,
        }
    });

    // Make main window appear when "opened" via Alt+Tab or Taskbar
    dock_win.handle({
        let mut win = win.clone();
        move |_wself, event| match event {
            enums::Event::Focus => {
                let win_shape = prep_shape(win.w(), win.h());

                win.show();
                win.set_shape(Some(win_shape));

                true
            }
            enums::Event::Hide => {
                win.hide();

                true
            }
            enums::Event::Close => {
                app.quit();

                true
            }
            _ => false,
        }
    });

    app.run().unwrap();
}

fn prep_shape(w: i32, h: i32) -> image::RgbImage {
    let surf = surface::ImageSurface::new(w, h, false);

    surface::ImageSurface::push_current(&surf);

    draw::set_draw_color(enums::Color::Black);
    draw::draw_rectf(-1, -1, w + 2, h + 2);

    draw::set_draw_color(enums::Color::White);
    draw::draw_rounded_rectf(0, 0, w, h, 16);

    let img = surf.image().unwrap();

    surface::ImageSurface::pop_current();

    img
}
