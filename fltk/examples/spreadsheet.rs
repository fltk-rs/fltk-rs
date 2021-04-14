// Basically a table where the cell contents can be modified

use fltk::{enums::*, *};
use std::cell::RefCell;
use std::rc::Rc;

// Needed to store cell information during the draw_cell call
#[derive(Default)]
struct CellData {
    _row: i32, // row
    _col: i32, // column
    _x: i32,
    _y: i32,
    _w: i32,
    _h: i32,
}

impl CellData {
    pub fn select(&mut self, row: i32, col: i32, x: i32, y: i32, w: i32, h: i32) {
        self._row = row;
        self._col = col;
        self._x = x;
        self._y = y;
        self._w = w;
        self._h = h;
    }
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(800, 600);
    // We need an input widget
    let mut inp = input::Input::default();
    inp.hide();
    let mut table = table::Table::default()
        .with_size(800 - 10, 600 - 10)
        .center_of(&wind);
    let data = Rc::from(RefCell::from(vec![vec![String::from(""); 26]; 28]));
    let cell = Rc::from(RefCell::from(CellData::default()));

    table.set_rows(28);
    table.set_row_header(true);
    table.set_row_resize(true);
    table.set_cols(26);
    table.set_col_header(true);
    table.set_col_width_all(80);
    table.set_col_resize(true);
    table.end();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    let cell_c = cell.clone();
    let data_c = data.clone();

    // Called when the table is drawn then when it's redrawn due to events
    table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
        table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
        table::TableContext::ColHeader => {
            draw_header(&format!("{}", (col + 65) as u8 as char), x, y, w, h)
        } // Column titles
        table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h), // Row titles
        table::TableContext::Cell => {
            if t.is_selected(row, col) {
                cell_c.borrow_mut().select(row, col, x, y, w, h); // Captures the cell information
            }
            draw_data(
                &data_c.borrow()[row as usize][col as usize].to_string(),
                x,
                y,
                w,
                h,
                t.is_selected(row, col),
            );
        }
        _ => (),
    });

    let cell_c = cell.clone();
    let mut inp_c = inp.clone();
    let data_c = data.clone();

    table.handle(move |_, ev| match ev {
        Event::Push => {
            let c = cell_c.borrow();
            inp_c.resize(c._x, c._y, c._w, c._h);
            inp_c.set_value(&data_c.borrow_mut()[c._row as usize][c._col as usize]);
            inp_c.show();
            inp_c.take_focus().ok();
            inp_c.redraw();
            true
        }
        _ => false,
    });

    wind.handle(move |_, ev| match ev {
        Event::KeyDown => {
            if app::event_key() == Key::Enter {
                // Press enter to store the data into the cell
                let c = cell.borrow();
                data.borrow_mut()[c._row as usize][c._col as usize] = inp.value();
                inp.set_value("");
                inp.hide();
                table.redraw();
                return true;
            }
            false
        }
        _ => false,
    });

    wind.set_callback(|_| {
        if app::event() == Event::Close {
            // Close only when the close button is clicked
            app::quit();
        }
    });

    app.run().unwrap();
}

fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
    draw::set_draw_color(Color::Black);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::pop_clip();
}

// The selected flag sets the color of the cell to a grayish color, otherwise white
fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(Color::from_u32(0xD3D3D3));
    } else {
        draw::set_draw_color(Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(Color::Gray0);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}
