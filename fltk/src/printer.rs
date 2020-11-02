pub use crate::prelude::*;
use fltk_sys::printer::*;

/// Defines a printer object
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
    pub fn begin_job(&mut self, pagecount: u32) -> Result<(Option<i32>, Option<i32>), FltkError> {
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
                    Some(*frompage_)
                };
                let to = if topage_.is_null() {
                    None
                } else {
                    Some(*topage_)
                };
                Ok((from, to))
            }
        }
    }

    /// End the print page
    pub fn end_page(&mut self) -> Result<(), FltkError> {
        unsafe {
            if Fl_Printer_end_page(self._inner) != 0 {
                Err(FltkError::Internal(FltkErrorKind::FailedToRun))
            } else {
                Ok(())
            }
        }
    }

    /// Ends the print job
    pub fn end_job(&mut self) {
        unsafe { Fl_Printer_end_job(self._inner) }
    }

    /// Begins a print page
    pub fn begin_page(&mut self) -> Result<(), FltkError> {
        unsafe {
            if Fl_Printer_begin_page(self._inner) != 0 {
                Err(FltkError::Internal(FltkErrorKind::FailedToRun))
            } else {
                Ok(())
            }
        }
    }

    /// Returns the width and height of the printable rect
    pub fn printable_rect(&self) -> Result<(i32, i32), FltkError> {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            if Fl_Printer_printable_rect(self._inner, &mut x, &mut y) != 0 {
                Err(FltkError::Internal(FltkErrorKind::FailedToRun))
            } else {
                Ok((x, y))
            }
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
    pub fn print_widget<W: WidgetBase>(&self, widget : &W, delta_x : i32, delta_y : i32) {
        unsafe {
            Fl_Printer_print_widget(self._inner, widget.as_widget_ptr() as *mut _, delta_x, delta_y)
        }
    }
    
    /// Print a window
    pub fn print_window<W: WindowExt>(&self, win : &W, x_offset : i32, y_offset : i32) {
        unsafe {
            Fl_Printer_print_window(self._inner, win.as_widget_ptr() as *mut _, x_offset, y_offset)
        }
    }
}

impl Drop for Printer {
    fn drop(&mut self) {
        unsafe { Fl_Printer_delete(self._inner) }
    }
}
