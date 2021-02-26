pub use crate::prelude::*;
use fltk_sys::printer::*;
use std::ffi::CString;

/// Defines a printer object.
/// Example usage:
/// ```no_run
/// use fltk::*;
/// let mut but = button::Button::default();
/// but.set_callback2(|widget| {
///     let mut printer = printer::Printer::default();
///     if printer.begin_job(1).is_ok() {
///         printer.begin_page();
///         let (width, height) = printer.printable_rect();
///         draw::set_draw_color(Color::Black);
///         draw::set_line_style(draw::LineStyle::Solid, 2);
///         draw::draw_rect(0, 0, width, height);
///         draw::set_font(Font::Courier, 12);
///         printer.set_origin(width / 2, height / 2);
///         printer.print_widget(widget, -widget.width() / 2, -widget.height() / 2);
///         printer.end_page();
///         printer.end_job();
///     }
/// });
/// ```
pub struct Printer {
    _inner: *mut Fl_Printer,
}

impl Printer {
    /// Creates a printer object
    pub fn default() -> Self {
        unsafe {
            let ptr = Fl_Printer_new();
            assert!(!ptr.is_null());
            Printer { _inner: ptr }
        }
    }

    /// Begins a print job
    /// `pagecount` The total number of pages to be created. Use 0 if this number is unknown
    /// Returns a tuple (frompage, topage) indicating the chosen pages by the user
    pub fn begin_job(&mut self, pagecount: u32) -> Result<(Option<u32>, Option<u32>), FltkError> {
        debug_assert!(
            pagecount <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        let frompage_: *mut i32 = std::ptr::null_mut();
        let topage_: *mut i32 = std::ptr::null_mut();
        unsafe {
            if Fl_Printer_begin_job(
                self._inner,
                pagecount as i32,
                frompage_,
                topage_,
                std::ptr::null_mut(),
            ) != 0
            {
                Err(FltkError::Internal(FltkErrorKind::FailedToRun))
            } else {
                let from = if frompage_.is_null() {
                    None
                } else {
                    Some(*frompage_ as u32)
                };
                let to = if topage_.is_null() {
                    None
                } else {
                    Some(*topage_ as u32)
                };
                Ok((from, to))
            }
        }
    }

    /// End the print page
    pub fn end_page(&mut self) {
        unsafe {
            Fl_Printer_end_page(self._inner);
        }
    }

    /// Ends the print job
    pub fn end_job(&mut self) {
        unsafe { Fl_Printer_end_job(self._inner) }
    }

    /// Begins a print page
    pub fn begin_page(&mut self) {
        unsafe {
            Fl_Printer_begin_page(self._inner);
        }
    }

    /// Returns the width and height of the printable rect
    pub fn printable_rect(&self) -> (i32, i32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            Fl_Printer_printable_rect(self._inner, &mut x, &mut y);
            (x, y)
        }
    }

    /// Returns the coordinates of the printable margins
    /// (left, top, right, bottom)
    pub fn margins(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut left = 0;
            let mut top = 0;
            let mut right = 0;
            let mut bottom = 0;
            Fl_Printer_margins(self._inner, &mut left, &mut top, &mut right, &mut bottom);
            (left, top, right, bottom)
        }
    }

    /// Get the origin coordinates of the printable rect
    pub fn origin(&self) -> (i32, i32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            Fl_Printer_origin(self._inner, &mut x, &mut y);
            (x, y)
        }
    }

    /// Set the origin coordinates of the printable rect
    pub fn set_origin(&mut self, x: i32, y: i32) {
        unsafe { Fl_Printer_set_origin(self._inner, x, y) }
    }

    /// Scale the printable rect
    pub fn scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe { Fl_Printer_scale(self._inner, scale_x, scale_y) }
    }

    /// Rotate the printable rect
    pub fn rotate(&mut self, angle: f32) {
        unsafe { Fl_Printer_rotate(self._inner, angle) }
    }

    /// Translate the printable rect
    pub fn translate(&mut self, x: i32, y: i32) {
        unsafe { Fl_Printer_translate(self._inner, x, y) }
    }

    /// Untranslate the printable rect
    pub fn untranslate(&mut self) {
        unsafe { Fl_Printer_untranslate(self._inner) }
    }

    /// Check whether the printer is the current printer
    pub fn is_current(&self) -> bool {
        unsafe { Fl_Printer_is_current(self._inner as *mut _) != 0 }
    }

    /// Set the printer to be the current printer
    pub fn set_current(&mut self) {
        unsafe { Fl_Printer_set_current(self._inner) }
    }

    /// Print a widget
    pub fn print_widget<W: WidgetExt>(&self, widget: &W, delta_x: i32, delta_y: i32) {
        unsafe {
            Fl_Printer_print_widget(
                self._inner,
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
                self._inner,
                win.as_widget_ptr() as *mut _,
                x_offset,
                y_offset,
            )
        }
    }

    /// Set the dialog "Title"
    pub fn set_dialog_title(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_title(msg.into_raw()) }
    }

    /// Set the dialog "Printer"
    pub fn set_dialog_printer(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_printer(msg.into_raw()) }
    }

    /// Set dialog "Range"
    pub fn set_dialog_range(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_range(msg.into_raw()) }
    }

    /// Set dialog "Copies"
    pub fn set_dialog_copies(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_copies(msg.into_raw()) }
    }

    /// Set dialog "All"
    pub fn set_dialog_all(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_all(msg.into_raw()) }
    }

    /// Set dialog "Pages"
    pub fn set_dialog_pages(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_pages(msg.into_raw()) }
    }

    /// Set dialog "From"
    pub fn set_dialog_from(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_from(msg.into_raw()) }
    }

    /// Set dialog "To"
    pub fn set_dialog_to(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_to(msg.into_raw()) }
    }

    /// Set dialog "Properties"
    pub fn set_dialog_properties(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_properties(msg.into_raw()) }
    }

    /// Set dialog "Number of copies"
    pub fn set_dialog_copy_number(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_copyNo(msg.into_raw()) }
    }

    /// Set dialog "Print" button
    pub fn set_dialog_print_button(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_print_button(msg.into_raw()) }
    }

    /// Set dialog "Cancel" button
    pub fn set_dialog_cancel_button(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_cancel_button(msg.into_raw()) }
    }

    /// Set dialog "Print to file" button
    pub fn set_dialog_print_to_file(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_dialog_print_to_file(msg.into_raw()) }
    }

    /// Set property "Title"
    pub fn set_property_title(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_property_title(msg.into_raw()) }
    }

    /// Set property "Page Size"
    pub fn set_property_pagesize(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_property_pagesize(msg.into_raw()) }
    }

    /// Set property "Mode"
    pub fn set_property_mode(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_property_mode(msg.into_raw()) }
    }

    /// Set property "Use"
    pub fn set_property_use(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_property_use(msg.into_raw()) }
    }

    /// Set property "Save"
    pub fn set_property_save(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_property_save(msg.into_raw()) }
    }

    /// Set property "Cancel"
    pub fn set_property_cancel(msg: &str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_Printer_set_property_cancel(msg.into_raw()) }
    }
}

impl Drop for Printer {
    fn drop(&mut self) {
        unsafe { Fl_Printer_delete(self._inner) }
    }
}
