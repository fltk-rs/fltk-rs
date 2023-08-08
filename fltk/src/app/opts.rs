use fltk_sys::fl;

/// global FLTK options
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Option {
    /// When switched on, moving the text cursor beyond the start or end of
    /// a text in a text widget will change focus to the next text widget.
    /// (This is considered 'old' behavior)
    ///
    /// When switched off (default), the cursor will stop at the end of the text.
    /// Pressing Tab or Ctrl-Tab will advance the keyboard focus.
    FrrowFocus = 0,
    /// If visible focus is switched on (default), FLTK will draw a dotted rectangle
    /// inside the widget that will receive the next keystroke. If switched
    /// off, no such indicator will be drawn and keyboard navigation
    /// is disabled.
    VisibleFocus,
    /// If text drag-and-drop is enabled (default), the user can select and drag text
    /// from any text widget. If disabled, no dragging is possible, however
    /// dropping text from other applications still works.
    DndText,
    /// If tooltips are enabled (default), hovering the mouse over a widget with a
    /// tooltip text will open a little tooltip window until the mouse leaves
    /// the widget. If disabled, no tooltip is shown.
    ShowTooltips,
    /// When switched on (default), Fl_Native_File_Chooser runs GTK file dialogs
    /// if the GTK library is available on the platform (linux/unix only).
    /// When switched off, GTK file dialogs aren't used even if the GTK library is available.
    FnfcUsesGtk,
    /// When switched on (default), Fl_Printer runs the GTK printer dialog
    /// if the GTK library is available on the platform (linux/unix only).
    /// When switched off, the GTK printer dialog isn't used even if the GTK library is available.
    PrinterUsesGtk,
    /// When switched on (default), the library shows in a transient yellow window the zoom factor
    /// value.
    /// When switched off, no such window gets displayed.
    ShowScaling,
    /// Meaningful for the Wayland/X11 platform only. When switched on (default), the library uses a Zenity-based file dialog.
    /// When switched off, the GTK file dialog is used instead.
    FnfcUsesZenity,
}

/// Get the option's value
pub fn option(opt: Option) -> bool {
    unsafe { fl::Fl_option(opt as i32) != 0 }
}

/// Set the option's value
pub fn set_option(opt: Option, val: bool) {
    unsafe { fl::Fl_set_option(opt as i32, val as i32) }
}
