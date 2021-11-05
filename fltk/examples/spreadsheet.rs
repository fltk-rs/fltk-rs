// For a simpler boilerplate-less table, check the fltk-table crate

use fltk::{
    app, draw, enums, input,
    prelude::{GroupExt, InputExt, TableExt, WidgetBase, WidgetExt},
    table, window,
};
use std::cell::RefCell;
use std::rc::Rc;

pub type MyData = Vec<Vec<String>>;

// Needed to store cell information during the draw_cell call
#[derive(Default)]
struct CellData {
    row: i32, // row
    col: i32, // column
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl CellData {
    pub fn select(&mut self, row: i32, col: i32, x: i32, y: i32, w: i32, h: i32) {
        self.row = row;
        self.col = col;
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
}

pub struct MyTable {
    table: table::Table,
    data: Rc<RefCell<MyData>>,
    cell: Rc<RefCell<CellData>>,
}

impl MyTable {
    pub fn new(mut inp: input::Input) -> Self {
        let mut table = table::Table::default()
            .with_size(800 - 10, 600 - 10)
            .center_of_parent();
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

        let cell_c = cell.clone();
        let data_c = data.clone();

        // Called when the table is drawn then when it's redrawn due to events
        table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
            table::TableContext::StartPage => draw::set_font(enums::Font::Helvetica, 14),
            table::TableContext::ColHeader => {
                Self::draw_header(&format!("{}", (col + 65) as u8 as char), x, y, w, h)
            } // Column titles
            table::TableContext::RowHeader => {
                Self::draw_header(&format!("{}", row + 1), x, y, w, h)
            } // Row titles
            table::TableContext::Cell => {
                if t.is_selected(row, col) {
                    cell_c.borrow_mut().select(row, col, x, y, w, h); // Captures the cell information
                }
                Self::draw_data(
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
        let data_c = data.clone();

        table.handle(move |_, ev| match ev {
            // Event::Push will happen before the focus is moved,
            // thus giving the previous coordinates.
            // Event::Released gives an accurate position
            enums::Event::Released => {
                let c = cell_c.borrow();
                inp.resize(c.x, c.y, c.w, c.h);
                inp.set_value(&data_c.borrow_mut()[c.row as usize][c.col as usize]);
                inp.show();
                inp.take_focus().ok();
                inp.redraw();
                true
            }
            _ => false,
        });

        Self { table, data, cell }
    }

    pub fn redraw(&mut self) {
        self.table.redraw()
    }

    fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
        draw::push_clip(x, y, w, h);
        draw::draw_box(
            enums::FrameType::ThinUpBox,
            x,
            y,
            w,
            h,
            enums::Color::FrameDefault,
        );
        draw::set_draw_color(enums::Color::Black);
        draw::set_font(enums::Font::Helvetica, 14);
        draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
        draw::pop_clip();
    }

    // The selected flag sets the color of the cell to a grayish color, otherwise white
    fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool) {
        draw::push_clip(x, y, w, h);
        if selected {
            draw::set_draw_color(enums::Color::from_u32(0x00D3_D3D3));
        } else {
            draw::set_draw_color(enums::Color::White);
        }
        draw::draw_rectf(x, y, w, h);
        draw::set_draw_color(enums::Color::Gray0);
        draw::set_font(enums::Font::Helvetica, 14);
        draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
        draw::draw_rect(x, y, w, h);
        draw::pop_clip();
    }
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(800, 600);
    // We need an input widget
    let mut inp = input::Input::default();
    inp.hide();

    let mut table = MyTable::new(inp.clone());

    wind.make_resizable(true);
    wind.end();
    wind.show();

    wind.handle(move |_, ev| match ev {
        enums::Event::KeyDown => {
            if app::event_key() == enums::Key::Enter {
                // Press enter to store the data into the cell
                let c = table.cell.borrow();
                table.data.borrow_mut()[c.row as usize][c.col as usize] = inp.value();
                inp.set_value("");
                inp.hide();
                return true;
            }
            table.redraw();
            false
        }
        _ => false,
    });

    wind.set_callback(|_| {
        if app::event() == enums::Event::Close {
            // Close only when the close button is clicked
            app::quit();
        }
    });

    app.run().unwrap();
}
