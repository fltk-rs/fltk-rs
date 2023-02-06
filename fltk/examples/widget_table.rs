// This shows usage of the TableExt::find_cell() method to populate a table with widgets, like a GroupExt widget

use fltk::{prelude::*, *};

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(500, 400);
    let mut table = table::Table::default()
        .with_size(400, 130)
        .center_of_parent();
    table.set_frame(enums::FrameType::NoBox);
    table.set_scrollbar_size(-1);
    table.set_rows(5);
    table.set_cols(5);
    for i in 0..5 {
        for j in 0..5 {
            if let Some((x, y, w, h)) = table.find_cell(table::TableContext::Cell, i, j) {
                button::Button::new(x, y, w, h, None).with_label(&(i + j).to_string());
            }
        }
    }
    table.end();
    win.end();
    win.show();
    a.run().unwrap();
}
