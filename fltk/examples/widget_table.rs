use fltk::{prelude::*, *};

const IMG: &str = include_str!("../../screenshots/RustLogo.svg");

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(402, 275);
    let mut table = table::Table::default().size_of(&wind);

    table.set_rows(3);
    table.set_cols(5);
    table.set_row_height_all(90);
    table.set_col_width_all(80);

    table.end();

    let mut img = image::SvgImage::from_data(IMG).unwrap();

    table.draw_cell(move |t, ctx, row, col, x, y, w, h| {
        if let table::TableContext::Cell = ctx {
            img.scale(w, h - 20, true, false);
            let mut button = button::Button::new(x, y, w, h, None);
            button.set_label(&format!("Image {}", row + col));
            button.set_align(enums::Align::Bottom | enums::Align::Inside);
            button.set_frame(enums::FrameType::FlatBox);
            button.set_image(Some(img.clone()));
            button.set_callback(|b| println!("Selected: {}", b.label()));
            t.add(&button);
        }
    });

    wind.make_resizable(true);
    wind.end();
    wind.show();

    while table.children() == 0 {
        app::wait();
    }

    app::redraw();

    app.run().unwrap();
}
