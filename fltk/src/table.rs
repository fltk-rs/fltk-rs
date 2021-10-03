use crate::enums::{Align, CallbackTrigger, Color, Damage, Event, Font, FrameType, LabelType};
use crate::image::Image;
use crate::prelude::*;
use crate::utils::FlString;
use crate::widget::Widget;
use fltk_sys::table::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a table
#[derive(Debug)]
pub struct Table {
    inner: *mut Fl_Table,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Table, Fl_Table);
crate::macros::widget::impl_widget_base!(Table, Fl_Table);
crate::macros::group::impl_group_ext!(Table, Fl_Table);
crate::macros::table::impl_table_ext!(Table, Fl_Table);

/// Defines the `TableContext`
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
#[derive(Debug)]
pub struct TableRow {
    inner: *mut Fl_Table_Row,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(TableRow, Fl_Table_Row);
crate::macros::widget::impl_widget_base!(TableRow, Fl_Table_Row);
crate::macros::group::impl_group_ext!(TableRow, Fl_Table_Row);
crate::macros::table::impl_table_ext!(TableRow, Fl_Table_Row);

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

crate::macros::widget::impl_widget_type!(TableRowSelectMode);

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
    /// Returns whether a row was selected
    pub fn row_selected(&mut self, row: i32) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Table_Row_row_selected(self.inner, row) != 0
        }
    }

    /// Selects a row
    /// # Errors
    /// Errors on failure to select row
    pub fn select_row(
        &mut self,
        row: i32,
        selection_flag: TableRowSelectFlag,
    ) -> Result<(), FltkError> {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Table_Row_select_row(self.inner, row, selection_flag as i32) {
                1 => Ok(()),
                0 | -1 => Err(FltkError::Internal(FltkErrorKind::TableError)),
                _ => unreachable!(),
            }
        }
    }

    /// Selects all rows
    pub fn select_all_rows(&mut self, selection_flag: TableRowSelectFlag) {
        assert!(!self.was_deleted());
        unsafe { Fl_Table_Row_select_all_rows(self.inner, selection_flag as i32) }
    }
}
