use crate::image::Image;
pub use crate::prelude::*;
use crate::widget::Widget;
use fltk_sys::table::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a table
#[derive(WidgetExt, GroupExt, TableExt, Debug)]
pub struct Table {
    _inner: *mut Fl_Table,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines the TableContext
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TableContext {
    None = 0,
    StartPage = 0x01,
    EndPage = 0x02,
    RowHeader = 0x04,
    ColHeader = 0x08,
    Cell = 0x10,
    Table = 0x20,
    RcResize = 0x40,
}

/// Declaration of the type of the draw_cell callback
/// callback args: TableContext, Row: i32, Column: i32, X: i32, Y: i32, Width: i32 and Height: i32
pub type DrawCellData = Box<dyn FnMut(TableContext, i32, i32, i32, i32, i32, i32)>;

/// Creates a table row
#[derive(WidgetExt, GroupExt, TableExt, Debug)]
pub struct TableRow {
    _inner: *mut Fl_Table_Row,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines the table row select mode
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TableRowSelectMode {
    SelectNone,
    SelectSingle,
    SelectMulti,
}

impl TableRow {
    /// Sets the type of the table row
    pub fn set_type(&mut self, val: TableRowSelectMode) {
        assert!(!self.was_deleted());
        unsafe { Fl_Table_Row_set_type(self._inner, val as i32) }
    }

    /// Gets the type of the table row
    pub fn get_type(&self) -> TableRowSelectMode {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Table_Row_get_type(self._inner)) }
    }

    /// Returns whether a row was selected
    pub fn row_selected(&mut self, row: i32) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Table_Row_row_selected(self._inner, row) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Selects a row
    pub fn select_row(&mut self, row: i32) -> Result<(), FltkError> {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Table_Row_select_row(self._inner, row) {
                1 => Ok(()),
                0 => Err(FltkError::Internal(FltkErrorKind::TableError)),
                -1 => Err(FltkError::Internal(FltkErrorKind::TableError)),
                _ => unreachable!(),
            }
        }
    }

    /// Selects all rows
    pub fn select_all_rows(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Table_Row_select_all_rows(self._inner) }
    }
}
