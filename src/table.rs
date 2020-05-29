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

/// Creates a table row
#[derive(WidgetExt, GroupExt, TableExt, Debug)]
pub struct TableRow {
    _inner: *mut Fl_Table_Row,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TableRowSelectMode {
    SelectNone,
    SelectSingle,
    SelectMulti,
}

impl TableRow {
    pub fn set_type(&mut self, val: TableRowSelectMode) {
        assert!(!self.was_deleted());
        unsafe { Fl_Table_Row_set_type(self._inner, val as i32) }
    }

    pub fn get_type(&self) -> TableRowSelectMode {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Table_Row_get_type(self._inner)) }
    }

    pub fn row_selected(&mut self, row: i32) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Table_Row_row_selected(self._inner, row) {
                0 => false,
                _ => true,
            }
        }
    }

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

    pub fn select_all_rows(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Table_Row_select_all_rows(self._inner) }
    }
}
