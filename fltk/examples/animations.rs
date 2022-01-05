use fltk::{app, enums, frame::Frame, image::Pixmap, prelude::*, window::Window};

const PXM: &[&str] = &[
    "50 34 4 1",
    "  c #000000",
    "o c #ff9900",
    "@ c #ffffff",
    "# c None",
    "##################################################",
    "###      ##############################       ####",
    "### ooooo  ###########################  ooooo ####",
    "### oo  oo  #########################  oo  oo ####",
    "### oo   oo  #######################  oo   oo ####",
    "### oo    oo  #####################  oo    oo ####",
    "### oo     oo  ###################  oo     oo ####",
    "### oo      oo                     oo      oo ####",
    "### oo       oo  ooooooooooooooo  oo       oo ####",
    "### oo        ooooooooooooooooooooo        oo ####",
    "### oo     ooooooooooooooooooooooooooo    ooo ####",
    "#### oo   ooooooo ooooooooooooo ooooooo   oo #####",
    "####  oo oooooooo ooooooooooooo oooooooo oo  #####",
    "##### oo oooooooo ooooooooooooo oooooooo oo ######",
    "#####  o ooooooooooooooooooooooooooooooo o  ######",
    "###### ooooooooooooooooooooooooooooooooooo #######",
    "##### ooooooooo     ooooooooo     ooooooooo ######",
    "##### oooooooo  @@@  ooooooo  @@@  oooooooo ######",
    "##### oooooooo @@@@@ ooooooo @@@@@ oooooooo ######",
    "##### oooooooo @@@@@ ooooooo @@@@@ oooooooo ######",
    "##### oooooooo  @@@  ooooooo  @@@  oooooooo ######",
    "##### ooooooooo     ooooooooo     ooooooooo ######",
    "###### oooooooooooooo       oooooooooooooo #######",
    "###### oooooooo@@@@@@@     @@@@@@@oooooooo #######",
    "###### ooooooo@@@@@@@@@   @@@@@@@@@ooooooo #######",
    "####### ooooo@@@@@@@@@@@ @@@@@@@@@@@ooooo ########",
    "######### oo@@@@@@@@@@@@ @@@@@@@@@@@@oo ##########",
    "########## o@@@@@@ @@@@@ @@@@@ @@@@@@o ###########",
    "########### @@@@@@@     @     @@@@@@@ ############",
    "############  @@@@@@@@@@@@@@@@@@@@@  #############",
    "##############  @@@@@@@@@@@@@@@@@  ###############",
    "################    @@@@@@@@@    #################",
    "####################         #####################",
    "##################################################",
];

fn move_image(mut frm: Frame, handle: app::TimeoutHandle) {
    let (x, y) = (frm.x(), frm.y());
    frm.set_pos(x + 5, y);
    app::redraw();
    if frm.x() > 260 {
        app::remove_timeout3(handle)
    } else {
        app::repeat_timeout3(0.016, handle);
    }
}

fn main() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_label("timeout")
        .with_size(720, 486)
        .center_screen();
    let mut frame = Frame::new(-200, 150, 200, 200, "");
    let mut pxm = Pixmap::new(PXM).unwrap();
    pxm.scale(200, 200, true, true);
    frame.set_image_scaled(Some(pxm));
    wind.set_color(enums::Color::White);
    wind.end();
    wind.show_with_env_args();

    app::add_timeout3(0.016, move |handle| {
        let frame = frame.clone();
        move_image(frame, handle);
    });
    app.run().unwrap();
}
