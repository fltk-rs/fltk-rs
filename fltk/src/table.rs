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
#[derive(WidgetBase, WidgetExt, GroupExt, TableExt, Debug)]
pub struct Table {
    _inner: *mut Fl_Table,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines the TableContext
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TableContext {
    /// No context
    None = 0,
    /// start of page context
    StartPage = 0x01,
    /// End of page context
    EndPage = 0x02,
    /// Row header context
    RowHeader = 0x04,
    /// Column header context
    ColHeader = 0x08,
    /// Cell context
    Cell = 0x10,
    /// Table context
    Table = 0x20,
    /// Row-Column resize context
    RcResize = 0x40,
}

/// Creates a table row
#[derive(WidgetBase, WidgetExt, GroupExt, TableExt, Debug)]
pub struct TableRow {
    _inner: *mut Fl_Table_Row,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines the table row select mode
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TableRowSelectMode {
    /// Disable select
    None,
    /// Select single elements
    Single,
    /// Select several elements
    Multi,
}

/// Defines the table row select flag
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TableRowSelectFlag {
    /// Deselect on click
    Deselect,
    /// Select on click
    Select,
    /// Toggle selection on click
    Toggle,
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
            Fl_Table_Row_row_selected(self._inner, row) != 0
        }
    }

    /// Selects a row
    pub fn select_row(
        &mut self,
        row: i32,
        selection_flag: TableRowSelectFlag,
    ) -> Result<(), FltkError> {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Table_Row_select_row(self._inner, row, selection_flag as i32) {
                1 => Ok(()),
                0 => Err(FltkError::Internal(FltkErrorKind::TableError)),
                -1 => Err(FltkError::Internal(FltkErrorKind::TableError)),
                _ => unreachable!(),
            }
        }
    }

    /// Selects all rows
    pub fn select_all_rows(&mut self, selection_flag: TableRowSelectFlag) {
        assert!(!self.was_deleted());
        unsafe { Fl_Table_Row_select_all_rows(self._inner, selection_flag as i32) }
    }
}
