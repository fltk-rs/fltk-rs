use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::printer::*;
use std::ffi::CString;

/// Defines a printer object.
/// Example usage:
/// ```rust,no_run
/// use fltk::{prelude::*, *};
/// let mut but = button::Button::default();
/// but.set_callback(|widget| {
///     let mut printer = printer::Printer::default();
///     if printer.begin_job(1).is_ok() {
///         printer.begin_page().ok();
///         let (width, height) = printer.printable_rect();
///         draw::set_draw_color(enums::Color::Black);
///         draw::set_line_style(draw::LineStyle::Solid, 2);
///         draw::draw_rect(0, 0, width, height);
///         draw::set_font(enums::Font::Courier, 12);
///         printer.set_origin(width / 2, height / 2);
///         printer.print_widget(widget, -widget.width() / 2, -widget.height() / 2);
///         printer.end_page().ok();
///         printer.end_job();
///     }
/// });
/// ```
pub struct Printer {
    inner: *mut Fl_Printer,
}

impl Printer {
    /// Creates a printer object
    pub fn default() -> Self {
        unsafe {
            let ptr = Fl_Printer_new();
            assert!(!ptr.is_null());
            Printer { inner: ptr }
        }
    }

    /// Begins a print job.
    /// `pagecount` The total number of pages to be created. Use 0 if this number is unknown
    /// Returns a tuple (frompage, topage) indicating the chosen pages by the user
    /// # Errors
    /// Errors on failure to print
    pub fn begin_job(&mut self, pagecount: i32) -> Result<(Option<i32>, Option<i32>), FltkError> {
        let mut frompage_ = 0;
        let mut topage_ = 0;
        unsafe {
            if Fl_Printer_begin_job(
                self.inner,
                pagecount as i32,
                &mut frompage_,
                &mut topage_,
                std::ptr::null_mut(),
            ) == 0
            {
                let from = if frompage_ == 0 {
                    None
                } else {
                    Some(frompage_ as i32)
                };
                let to = if topage_ == 0 {
                    None
                } else {
                    Some(topage_ as i32)
                };
                Ok((from, to))
            } else {
                Err(FltkError::Internal(FltkErrorKind::FailedToRun))
            }
        }
    }

    /// End the print page
    /// # Errors
    /// Errors on failure to end the page
    pub fn end_page(&mut self) -> Result<(), FltkError> {
        unsafe {
            if Fl_Printer_end_page(self.inner) == 0 {
                Ok(())
            } else {
                Err(FltkError::Internal(FltkErrorKind::FailedToRun))
            }
        }
    }

    /// Ends the print job
    pub fn end_job(&mut self) {
        unsafe { Fl_Printer_end_job(self.inner) }
    }

    /// Begins a print page
    /// # Errors
    /// Errors on failure to begin the page
    pub fn begin_page(&mut self) -> Result<(), FltkError> {
        unsafe {
            if Fl_Printer_begin_page(self.inner) == 0 {
                Ok(())
            } else {
                Err(FltkError::Internal(FltkErrorKind::FailedToRun))
            }
        }
    }

    /// Returns the width and height of the printable rect
    pub fn printable_rect(&self) -> (i32, i32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            Fl_Printer_printable_rect(self.inner, &mut x, &mut y);
            (x, y)
        }
    }

    /// Returns the coordinates of the printable margins.
    /// (left, top, right, bottom)
    pub fn margins(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut left = 0;
            let mut top = 0;
            let mut right = 0;
            let mut bottom = 0;
            Fl_Printer_margins(self.inner, &mut left, &mut top, &mut right, &mut bottom);
            (left, top, right, bottom)
        }
    }

    /// Get the origin coordinates of the printable rect
    pub fn origin(&self) -> (i32, i32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            Fl_Printer_origin(self.inner, &mut x, &mut y);
            (x, y)
        }
    }

    /// Set the origin coordinates of the printable rect
    pub fn set_origin(&mut self, x: i32, y: i32) {
        unsafe { Fl_Printer_set_origin(self.inner, x, y) }
    }

    /// Scale the printable rect
    pub fn scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe { Fl_Printer_scale(self.inner, scale_x, scale_y) }
    }

    /// Rotate the printable rect
    pub fn rotate(&mut self, angle: f32) {
        unsafe { Fl_Printer_rotate(self.inner, angle) }
    }

    /// Translate the printable rect
    pub fn translate(&mut self, x: i32, y: i32) {
        unsafe { Fl_Printer_translate(self.inner, x, y) }
    }

    /// Untranslate the printable rect
    pub fn untranslate(&mut self) {
        unsafe { Fl_Printer_untranslate(self.inner) }
    }

    /// Check whether the printer is the current printer
    pub fn is_current(&self) -> bool {
        unsafe { Fl_Printer_is_current(self.inner as *mut _) != 0 }
    }

    /// Set the printer to be the current printer
    pub fn set_current(&mut self) {
        unsafe { Fl_Printer_set_current(self.inner) }
    }

    /// Print a widget
    pub fn print_widget<W: WidgetExt>(&self, widget: &W, delta_x: i32, delta_y: i32) {
        unsafe {
            Fl_Printer_print_widget(
                self.inner,
                widget.as_widget_ptr() as *mut _,
                delta_x,
                delta_y,
            )
        }
    }

    /// Print a window
    pub fn print_window<W: WindowExt>(&self, win: &W, x_offset: i32, y_offset: i32) {
        unsafe {
            Fl_Printer_print_window(
                self.inner,
                win.as_widget_ptr() as *mut _,
                x_offset,
                y_offset,
            )
        }
    }

    /// Set the dialog "Title"
    pub fn set_dialog_title(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_title(msg.into_raw()) }
    }

    /// Set the dialog "Printer"
    pub fn set_dialog_printer(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_printer(msg.into_raw()) }
    }

    /// Set dialog "Range"
    pub fn set_dialog_range(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_range(msg.into_raw()) }
    }

    /// Set dialog "Copies"
    pub fn set_dialog_copies(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_copies(msg.into_raw()) }
    }

    /// Set dialog "All"
    pub fn set_dialog_all(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_all(msg.into_raw()) }
    }

    /// Set dialog "Pages"
    pub fn set_dialog_pages(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_pages(msg.into_raw()) }
    }

    /// Set dialog "From"
    pub fn set_dialog_from(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_from(msg.into_raw()) }
    }

    /// Set dialog "To"
    pub fn set_dialog_to(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_to(msg.into_raw()) }
    }

    /// Set dialog "Properties"
    pub fn set_dialog_properties(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_properties(msg.into_raw()) }
    }

    /// Set dialog "Number of copies"
    pub fn set_dialog_copy_number(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_copyNo(msg.into_raw()) }
    }

    /// Set dialog "Print" button
    pub fn set_dialog_print_button(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_print_button(msg.into_raw()) }
    }

    /// Set dialog "Cancel" button
    pub fn set_dialog_cancel_button(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_cancel_button(msg.into_raw()) }
    }

    /// Set dialog "Print to file" button
    pub fn set_dialog_print_to_file(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_print_to_file(msg.into_raw()) }
    }

    /// Set property "Title"
    pub fn set_property_title(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_property_title(msg.into_raw()) }
    }

    /// Set property "Page Size"
    pub fn set_property_pagesize(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_property_pagesize(msg.into_raw()) }
    }

    /// Set property "Mode"
    pub fn set_property_mode(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_property_mode(msg.into_raw()) }
    }

    /// Set property "Use"
    pub fn set_property_use(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_property_use(msg.into_raw()) }
    }

    /// Set property "Save"
    pub fn set_property_save(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_property_save(msg.into_raw()) }
    }

    /// Set property "Cancel"
    pub fn set_property_cancel(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_property_cancel(msg.into_raw()) }
    }
}

impl Drop for Printer {
    fn drop(&mut self) {
        unsafe { Fl_Printer_delete(self.inner) }
    }
}
