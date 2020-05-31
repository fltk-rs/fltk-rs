pub use crate::enums::*;
use crate::image::Image;
use crate::text::{StyleTableEntry, TextBuffer};
use crate::widget::Widget;
use crate::window::Window;
use std::convert::From;
use std::{fmt, io, os::raw};

/// Error types returned by fltk-rs + wrappers of std::io errors
#[derive(Debug)]
pub enum FltkError {
    IoError(io::Error),
    NullError(std::ffi::NulError),
    Internal(FltkErrorKind),
    Unknown(String),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FltkErrorKind {
    FailedToRun,
    FailedToLock,
    FailedToSetScheme,
    FailedOperation,
    ResourceNotFound,
    ImageFormatError,
    TableError,
}

impl std::error::Error for FltkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FltkError::IoError(err) => Some(err),
            FltkError::NullError(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for FltkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FltkError::IoError(ref err) => err.fmt(f),
            FltkError::NullError(ref err) => err.fmt(f),
            FltkError::Internal(ref err) => write!(f, "An internal error occured {:?}", err),
            FltkError::Unknown(ref err) => write!(f, "An unknown error occurred {:?}", err),
        }
    }
}

impl From<io::Error> for FltkError {
    fn from(err: io::Error) -> FltkError {
        FltkError::IoError(err)
    }
}

impl From<std::ffi::NulError> for FltkError {
    fn from(err: std::ffi::NulError) -> FltkError {
        FltkError::NullError(err)
    }
}

/// Defines the methods implemented by all widgets
pub unsafe trait WidgetExt {
    /// Creates a new widget, takes an x, y coordinates, as well as a width and height, plus a title
    /// # Arguments
    /// * `x` - The x coordinate in the screen
    /// * `y` - The y coordinate in the screen
    /// * `width` - The width of the widget
    /// * `heigth` - The height of the widget
    /// * `title` - The title or label of the widget
    fn new(x: i32, y: i32, width: i32, height: i32, title: &str) -> Self;
    /// Creates a default and zero initialized widget
    fn default() -> Self;
    /// Initialize to position x, y
    fn with_pos(self, x: i32, y: i32) -> Self;
    /// Initialilze to dimensions width and height
    fn with_size(self, width: i32, height: i32) -> Self;
    /// Initialize with label/title
    fn with_label(self, title: &str) -> Self;
    /// Sets the initial alignment of the widget
    fn with_align(self, align: Align) -> Self;
    /// Positions the widget below w
    fn below_of<W: WidgetExt>(self, w: &W, padding: i32) -> Self;
    /// Positions the widget above w
    fn above_of<W: WidgetExt>(self, w: &W, padding: i32) -> Self;
    /// Positions the widget to the right of w
    fn right_of<W: WidgetExt>(self, w: &W, padding: i32) -> Self;
    /// Positions the widget to the left of w
    fn left_of<W: WidgetExt>(self, w: &W, padding: i32) -> Self;
    /// Positions the widget to the center of w
    fn center_of<W: WidgetExt>(self, w: &W) -> Self;
    /// Takes the size of w
    fn size_of<W: WidgetExt>(self, w: &W) -> Self;
    /// Sets the widget's label
    fn set_label(&mut self, title: &str);
    /// Redraws a widget, necessary for resizing and changing positions
    fn redraw(&mut self);
    /// Shows the widget
    fn show(&mut self);
    /// Hides the widget
    fn hide(&mut self);
    /// Returns the x coordinate of the widget
    fn x(&self) -> i32;
    /// Returns the y coordinate of the widget
    fn y(&self) -> i32;
    /// Returns the width of the widget
    fn width(&self) -> i32;
    /// Returns the height of the widget
    fn height(&self) -> i32;
    /// Returns the label of the widget
    fn label(&self) -> String;
    /// transforms a widget to a base Fl_Widget, for internal use
    unsafe fn as_widget_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget;
    /// transforms a widget pointer to a Widget, for internal use
    unsafe fn from_widget_ptr(ptr: *mut fltk_sys::widget::Fl_Widget) -> Self;
    /// Activates the widget
    fn activate(&mut self);
    /// Deactivates the widget
    fn deactivate(&mut self);
    /// Redraws the label of the widget
    fn redraw_label(&mut self);
    /// Resizes and/or moves the widget, takes x, y, width and height
    fn resize(&mut self, x: i32, y: i32, width: i32, height: i32);
    /// Returns the tooltip text
    fn tooltip(&self) -> Option<String>;
    /// Sets the tooltip text
    fn set_tooltip(&mut self, txt: &str);
    /// Returns the widget type when applicable
    fn get_type<T: WidgetType>(&self) -> T;
    /// Sets the widget type
    fn set_type<T: WidgetType>(&mut self, typ: T);
    /// Returns the widget color
    fn color(&self) -> Color;
    /// Sets the widget's color
    fn set_color(&mut self, color: Color);
    /// Returns the widget label's color
    fn label_color(&self) -> Color;
    /// Sets the widget label's color
    fn set_label_color(&mut self, color: Color);
    /// Returns the widget label's font
    fn label_font(&self) -> Font;
    /// Sets the widget label's font
    fn set_label_font(&mut self, font: Font);
    /// Returns the widget label's size
    fn label_size(&self) -> i32;
    /// Sets the widget label's size
    fn set_label_size(&mut self, sz: i32);
    /// Returns the widget label's type
    fn label_type<T: WidgetType>(&self) -> T;
    /// Sets the widget label's type
    fn set_label_type<T: WidgetType>(&mut self, typ: T);
    /// Returns the widget's frame type
    fn frame<T: WidgetType>(&self) -> T;
    /// Sets the widget's frame type
    fn set_frame<T: WidgetType>(&mut self, typ: T);
    /// Returns whether the widget was changed
    fn changed(&self) -> bool;
    /// Mark the widget as changed
    fn set_changed(&mut self);
    /// Clears the changed status of the widget
    fn clear_changed(&mut self);
    /// Returns the alignment of the widget
    fn align(&self) -> Align;
    /// Sets the alignment of the widget
    fn set_align(&mut self, align: Align);
    /// Sets the image of the widget
    fn set_image<Image: ImageExt>(&mut self, image: &Image);
    /// Sets the resized image of the widget
    fn set_image_with_size<Image: ImageExt>(&mut self, image: &Image, w: i32, h: i32);
    /// Gets the image associated with the widget
    fn image(&self) -> Option<Image>;
    /// Sets the callback when the widget is triggered (clicks for example)
    fn set_callback(&mut self, cb: Box<dyn FnMut()>);
    /// Set a custom handler, where events are managed manually, akin to Fl_Widget::handle(int)
    /// Handled or ignored events shoult return true, unhandled events should return false
    fn handle(&mut self, cb: Box<dyn FnMut(Event) -> bool>);
    /// Sets the default callback trigger for a widget
    fn set_trigger(&mut self, trigger: CallbackTrigger);
    /// Set a custom draw method
    fn draw(&mut self, cb: Box<dyn FnMut()>);
    /// Returns the parent of the widget
    fn parent(&self) -> Option<Widget>;
    /// Gets the selection color of the widget
    fn selection_color(&mut self) -> Color;
    /// Sets the selection color of the widget
    fn set_selection_color(&mut self, color: Color);
    /// Runs the already registered callback
    fn do_callback(&mut self);
    /// Checks whether the self widget is inside another widget
    fn inside(&self, wid: Widget) -> bool;
    /// Returns the direct window holding the widget
    fn window(&self) -> Option<Window>;
    /// Returns the topmost window holding the widget
    fn top_window(&self) -> Option<Window>;
    /// Checks whether a widget is capable of taking events
    fn takes_events(&self) -> bool;
    /// Emits a message on callback using a sender
    fn emit<T: 'static + Copy + Send + Sync>(&mut self, sender: crate::app::Sender<T>, msg: T);
    /// Make the widget take focus
    fn take_focus(&mut self) -> Result<(), FltkError>;
    /// Set the widget to have visible focus
    fn set_visible_focus(&mut self);
    /// Clear visible focus
    fn clear_visible_focus(&mut self);
    /// Set the visible focus using a flag
    fn visible_focus(&mut self, v: bool);
    /// Return whether the widget has visible focus
    fn has_visible_focus(&mut self) -> bool;
    /// Manually delete a widget
    fn delete(&mut self);
    /// Check if a widget was deleted
    fn was_deleted(&self) -> bool;
    /// INTERNAL: Retakes ownership of the user callback data
    unsafe fn user_data(&self) -> Option<Box<dyn FnMut()>>;
    /// INTERNAL: Manually set the user data
    unsafe fn set_user_data(&mut self, data: *mut raw::c_void);
    /// INTERNAL: Retakes ownership of the user callback data
    unsafe fn raw_user_data(&self) -> *mut raw::c_void;
    /// INTERNAL: Cleanup after widget deletion
    unsafe fn cleanup(&mut self);
    /// INTERNAL: Unset the defined callback
    unsafe fn unset_callback(&mut self);
    /// INTERNAL: Retrieve the draw data
    unsafe fn draw_data(&mut self) -> Option<Box<dyn FnMut()>>;
    /// INTERNAL: Manually set the draw data
    unsafe fn set_draw_data(&mut self, data: *mut raw::c_void);
    /// INTERNAL: Unset the draw callback
    unsafe fn unset_draw_callback(&mut self);
}

/// Defines the methods implemented by all button widgets
pub unsafe trait ButtonExt: WidgetExt {
    /// Gets the shortcut associated with a button
    fn shortcut(&self) -> Shortcut;
    /// Sets the shortcut associated with a button
    fn set_shortcut(&mut self, shortcut: Shortcut);
    /// Clears the value of the button.
    /// Useful for round, radio, light, toggle and check buttons
    fn clear(&mut self);
    /// Returns whether a button is set or not
    /// Useful for round, radio, light, toggle and check buttons
    fn is_set(&self) -> bool;
    /// Sets whether a button is set or not
    /// Useful for round, radio, light, toggle and check buttons
    fn set(&mut self, flag: bool);
}

/// Defines the methods implemented by all group widgets
pub unsafe trait GroupExt: WidgetExt {
    /// Begins a group, used for widgets implementing the group trait
    fn begin(&self);
    /// Ends a group, used for widgets implementing the group trait
    fn end(&self);
    /// Find a widget within a group and return its index
    fn find<Widget: WidgetExt>(&self, widget: &Widget) -> u32;
    /// Add a widget to a group
    fn add<Widget: WidgetExt>(&mut self, widget: &Widget);
    /// Insert a widget to a group at a certain index
    fn insert<Widget: WidgetExt>(&mut self, widget: &Widget, index: u32);
    /// Remove a widget from a group, but does not delete it
    fn remove<Widget: WidgetExt>(&mut self, widget: &Widget);
    /// Clear a group from all widgets
    fn clear(&mut self);
    /// Return the number of children in a group
    fn children(&self) -> u32;
    /// Return child widget by index
    fn child(&self, idx: u32) -> Option<Widget>;
    /// Make the passed widget resizable
    fn resizable<Widget: WidgetExt>(&self, widget: &mut Widget);
}

/// Defines the methods implemented by all window widgets
pub unsafe trait WindowExt: GroupExt {
    /// Positions the window to the center of the screen
    fn center_screen(self) -> Self;
    /// Makes a window modal
    fn make_modal(&mut self, val: bool);
    /// Makes a window fullscreen
    fn fullscreen(&mut self, val: bool);
    /// Makes the window current
    fn make_current(&mut self);
    /// Sets the windows icon
    fn set_icon<Image: ImageExt>(&mut self, image: &Image);
    /// Returns the icon of the window
    fn icon(&self) -> Option<Image>;
    /// Make the window resizable
    fn make_resizable(&mut self, val: bool);
    /// Sets the cursor style within the window
    fn set_cursor(&mut self, cursor: CursorStyle);
    /// Returns whether a window is shown
    fn shown(&self) -> bool;
    /// Get the raw system handle of the window
    /// void pointer to: (Windows: HWND, X11: Xid, MacOS: NSWindow)
    unsafe fn raw_handle(&self) -> *const raw::c_void;
}

/// Defines the methods implemented by all input and output widgets
pub unsafe trait InputExt: WidgetExt {
    /// Returns the value inside the input/output widget
    fn value(&self) -> String;
    /// Sets the value inside an input/output widget
    fn set_value(&self, val: &str);
    /// Returns the maximum size (in bytes) accepted by an input/output widget
    fn maximum_size(&self) -> u32;
    /// Sets the maximum size (in bytes) accepted by an input/output widget
    fn set_maximum_size(&mut self, val: u32);
    /// Returns the postion inside an input/output widget
    fn position(&self) -> i32;
    /// Sets the postion inside an input/output widget
    fn set_position(&mut self, val: i32) -> Result<(), FltkError>;
    /// Returns the mark inside an input/output widget
    fn mark(&self) -> i32;
    /// Sets the mark inside an input/output widget
    fn set_mark(&mut self, val: i32) -> Result<(), FltkError>;
    /// Replace content with a &str
    fn replace(&mut self, beg: u32, end: u32, val: &str) -> Result<(), FltkError>;
    /// Insert a &str
    fn insert(&mut self, txt: &str) -> Result<(), FltkError>;
    /// Append a &str
    fn append(&mut self, txt: &str) -> Result<(), FltkError>;
    /// Copy the value within the widget
    fn copy(&mut self) -> Result<(), FltkError>;
    /// Undo changes
    fn undo(&mut self) -> Result<(), FltkError>;
    /// Cut the value within the widget
    fn cut(&mut self) -> Result<(), FltkError>;
    /// Return the text font
    fn text_font(&self) -> Font;
    /// Sets the text font
    fn set_text_font(&mut self, font: Font);
    /// Return the text color
    fn text_color(&self) -> Color;
    /// Sets the text color
    fn set_text_color(&mut self, color: Color);
    /// Return the text size
    fn text_size(&self) -> u32;
    /// Sets the text size
    fn set_text_size(&mut self, sz: u32);
    /// Returns whether the input/output widget is readonly
    fn readonly(&self) -> bool;
    /// Set readonly status of the input/output widget
    fn set_readonly(&mut self, val: bool);
    /// Return whether text is wrapped inside an input/output widget
    fn wrap(&self) -> bool;
    /// Set whether text is wrapped inside an input/output widget
    fn set_wrap(&mut self, val: bool);
}

/// Defines the methods implemented by all menu widgets
pub unsafe trait MenuExt: WidgetExt {
    /// Get a menu item by name
    fn find_item(&self, name: &str) -> Option<crate::menu::MenuItem>;
    /// Set selected item
    fn set_item(&mut self, item: &crate::menu::MenuItem) -> bool;
    /// Find an item's index by its label
    fn find_index(&self, label: &str) -> u32;
    /// Return the text font
    fn text_font(&self) -> Font;
    /// Sets the text font
    fn set_text_font(&mut self, c: Font);
    /// Return the text size
    fn text_size(&self) -> u32;
    /// Sets the text size
    fn set_text_size(&mut self, c: u32);
    /// Return the text color
    fn text_color(&self) -> Color;
    /// Sets the text color
    fn set_text_color(&mut self, c: Color);
    /// Add a menu item along with its callback
    fn add(
        &mut self,
        name: &str,
        shortcut: Shortcut,
        flag: crate::menu::MenuFlag,
        cb: Box<dyn FnMut()>,
    );
    /// Inserts a menu item at an index along with its callback
    fn insert(
        &mut self,
        idx: u32,
        name: &str,
        shortcut: Shortcut,
        flag: crate::menu::MenuFlag,
        cb: Box<dyn FnMut()>,
    );
    /// Remove a menu item by index
    fn remove(&mut self, idx: u32);
    /// Adds a simple text option to the Choice and MenuButton widgets
    fn add_choice(&mut self, text: &str);
    /// Gets the user choice from the Choice and MenuButton widgets
    fn choice(&self) -> Option<String>;
    /// Get index into menu of the last item chosen
    fn value(&self) -> i32;
    /// Set index into menu of the last item chosen,return true if the new value is different than the old one
    fn set_value(&mut self, v: i32) -> bool;
    /// Clears the items in a menu, effectively deleting them.
    fn clear(&mut self);
    /// Clears a submenu by index, failure return FltkErrorKind::FailedOperation
    fn clear_submenu(&mut self, idx: u32) -> Result<(), FltkError>;
    /// Get the size of the menu widget
    fn size(&self) -> u32;
    /// Get the text label of the menu item at index idx
    fn text(&self, idx: u32) -> Option<String>;
    /// Get the menu item at an index
    fn at(&self, idx: u32) -> Option<crate::menu::MenuItem>;
    /// Set the mode of a menu item by index and flag
    fn mode(&self, idx: u32) -> crate::menu::MenuFlag;
    /// Get the mode of a menu item
    fn set_mode(&mut self, idx: u32, flag: crate::menu::MenuFlag);
}

/// Defines the methods implemented by all valuator widgets
pub unsafe trait ValuatorExt: WidgetExt {
    /// Set bounds of a valuator
    fn set_bounds(&mut self, a: f64, b: f64);
    /// Get the minimum bound of a valuator
    fn minimum(&self) -> f64;
    /// Set the minimum bound of a valuator
    fn set_minimum(&mut self, a: f64);
    /// Get the maximum bound of a valuator
    fn maximum(&self) -> f64;
    /// Set the maximum bound of a valuator
    fn set_maximum(&mut self, a: f64);
    /// Set the range of a valuator
    fn set_range(&mut self, a: f64, b: f64);
    /// Set change step of a valuator
    fn set_step(&mut self, a: f64, b: i32);
    /// Get change step of a valuator
    fn step(&self) -> f64;
    /// Set the precision of a valuator
    fn set_precision(&mut self, digits: i32);
    /// Get the value of a valuator
    fn value(&self) -> f64;
    /// Set the value of a valuator
    fn set_value(&mut self, arg2: f64);
    /// Set the format of a valuator
    fn format(&mut self, arg2: &str) -> Result<(), FltkError>;
    /// Round the valuator
    fn round(&self, arg2: f64) -> f64;
    /// Clamp the valuator
    fn clamp(&self, arg2: f64) -> f64;
    /// Increment the valuator
    fn increment(&mut self, arg2: f64, arg3: i32) -> f64;
}

/// Defines the methods implemented by TextDisplay and TextEditor
pub unsafe trait DisplayExt: WidgetExt {
    /// Get the associated TextBuffer
    fn buffer<'a>(&'a self) -> &'a mut TextBuffer;
    /// Sets the associated TextBuffer
    fn set_buffer<'a>(&'a mut self, buffer: &'a mut TextBuffer);
    /// Return the text font
    fn text_font(&self) -> Font;
    /// Sets the text font
    fn set_text_font(&mut self, font: Font);
    /// Return the text color
    fn text_color(&self) -> Color;
    /// Sets the text color
    fn set_text_color(&mut self, color: Color);
    /// Return the text size
    fn text_size(&self) -> u32;
    /// Sets the text size
    fn set_text_size(&mut self, sz: u32);
    /// Scroll down the Display widget
    fn scroll(&mut self, top_line_num: u32, horiz_offset: u32);
    /// Insert into Display widget      
    fn insert(&self, text: &str);
    /// Set the insert position
    fn set_insert_position(&mut self, new_pos: u32);
    /// Return the insert position                
    fn insert_position(&self) -> u32;
    /// Gets the x and y positions of the cursor
    fn position_to_xy(&self, pos: u32) -> (u32, u32);
    /// Counts the lines from start to end                         
    fn count_lines(&self, start: u32, end: u32, is_line_start: bool) -> u32;
    /// Moves the cursor right
    fn move_right(&mut self) -> Result<(), FltkError>;
    /// Moves the cursor left
    fn move_left(&mut self) -> Result<(), FltkError>;
    /// Moves the cursor up
    fn move_up(&mut self) -> Result<(), FltkError>;
    /// Moves the cursor down
    fn move_down(&mut self) -> Result<(), FltkError>;
    /// Shows/hides the cursor
    fn show_cursor(&mut self, val: bool);
    /// Sets the style of the text widget
    fn set_styly_table_entry(
        &mut self,
        style_buffer: &mut TextBuffer,
        entries: &Vec<StyleTableEntry>,
    );
    /// Sets the cursor style
    fn set_cursor_style(&mut self, style: CursorStyle);
    /// Sets the cursor color
    fn set_cursor_color(&mut self, color: Color);
    /// Sets the scrollbar width
    fn set_scrollbar_width(&mut self, width: i32);
    /// Sets the scrollbar size in pixels
    fn set_scrollbar_size(&mut self, size: u32);
    /// Sets the scrollbar alignment
    fn set_scrollbar_align(&mut self, align: Align);
    /// Returns the cursor style
    fn cursor_style(&self) -> CursorStyle;
    /// Returns the cursor color
    fn cursor_color(&self) -> Color;
    /// Returns the scrollback width
    fn scrollbar_width(&self) -> u32;
    /// Returns the scrollbar size in pixels
    fn scrollbar_size(&self) -> u32;
    /// Returns the scrollbar alignment
    fn scrollbar_align(&self) -> Align;
    /// Returns the beginning of the line from the current position
    fn line_start(&self, pos: u32) -> u32;
    /// Returns the ending of the line from the current position
    fn line_end(&self, start_pos: u32, is_line_start: bool) -> u32;
    /// Skips lines from start_pos
    fn skip_lines(&mut self, start_pos: u32, lines: u32, is_line_start: bool) -> u32;
    /// Rewinds the lines
    fn rewind_lines(&mut self, start_pos: u32, lines: u32) -> u32;
    /// Goes to the next word
    fn next_word(&mut self);
    /// Goes to the previous word
    fn previous_word(&mut self);
    /// Returns the position of the start of the word, relative to the current position
    fn word_start(&self, pos: u32) -> u32;
    /// Returns the position of the end of the word, relative to the current position
    fn word_end(&self, pos: u32) -> u32;
    /// Convert an x pixel position into a column number.
    fn x_to_col(&self, x: f64) -> f64;
    /// Convert a column number into an x pixel position
    fn col_to_x(&self, col: f64) -> f64;
    /// Sets the linenumber width
    fn set_linenumber_width(&mut self, w: i32);
    /// Gets the linenumber width
    fn linenumber_width(&self) -> i32;
    /// Sets the linenumber font
    fn set_linenumber_font(&mut self, font: Font);
    /// Gets the linenumber font
    fn linenumber_font(&self) -> Font;
    /// Sets the linenumber size
    fn set_linenumber_size(&mut self, size: u32);
    /// Gets the linenumber size
    fn linenumber_size(&self) -> u32;
    /// Sets the linenumber foreground color
    fn set_linenumber_fgcolor(&mut self, color: Color);
    /// Gets the linenumber foreground color
    fn linenumber_fgcolor(&self) -> Color;
    /// Sets the linenumber background color
    fn set_linenumber_bgcolor(&mut self, color: Color);
    /// Gets the linenumber background color
    fn linenumber_bgcolor(&self) -> Color;
    /// Sets the linenumber alignment
    fn set_linenumber_align(&mut self, align: Align);
    /// Gets the linenumber alignment
    fn linenumber_align(&self) -> Align;
    /// Checks whether a pixel is within a text selection
    fn in_selection(&self, x: i32, y: i32) -> bool;
}

/// Defines the methods implemented by all browser types
pub unsafe trait BrowserExt: WidgetExt {
    /// Removes the specified line
    fn remove(&mut self, line: u32);
    /// Adds an item
    fn add(&mut self, item: &str);
    /// Inserts an item at an index
    fn insert(&mut self, line: u32, item: &str);
    /// Moves an item
    fn move_item(&mut self, to: u32, from: u32);
    /// Swaps 2 items
    fn swap(&mut self, a: u32, b: u32);
    /// Clears the browser widget
    fn clear(&mut self);
    /// Returns the number of items
    fn size(&self) -> u32;
    /// Set the number of items
    fn set_size(&mut self, w: i32, h: i32);
    /// Select an item at the specified line
    fn select(&mut self, line: u32);
    /// Returns whether the item is selected
    fn selected(&self, line: u32) -> bool;
    /// Returns the text of the selected item
    fn text(&self, line: u32) -> Option<String>;
    /// Sets the text of the selected item
    fn set_text(&mut self, line: u32, txt: &str);
    /// Load a file
    fn load(&mut self, path: &std::path::Path) -> Result<(), FltkError>;
    /// Return the text size
    fn text_size(&self) -> u32;
    /// Sets the text size
    fn set_text_size(&mut self, sz: u32);
    /// Sets the icon for browser elements
    fn set_icon<Img: ImageExt>(&mut self, line: u32, image: &Img);
    /// Returns the icon of a browser element
    fn icon(&self, line: u32) -> Option<Image>;
    /// Removes the icon of a browser element
    fn remove_icon(&mut self, line: u32);
}

/// Defines the methods implemented by table types
pub unsafe trait TableExt: GroupExt {
    /// Clears the table
    fn clear(&mut self);
    /// Sets the table frame, table_box
    fn set_table_frame(&mut self, frame: FrameType);
    /// Gets the table frame, table box
    fn table_frame(&self) -> FrameType;
    /// Sets the number of rows
    fn set_rows(&mut self, val: u32);
    /// Gets the number of rows
    fn rows(&self) -> u32;
    /// Sets the number of columns
    fn set_cols(&mut self, val: u32);
    /// Gets the number of columns
    fn cols(&self) -> u32;
    /// Returns the range of row and column numbers for all visible and partially visible cells in the table.
    fn visible_cells(&mut self, r1: &mut i32, r2: &mut i32, c1: &mut i32, c2: &mut i32);
    /// Returns whether the resize is interactive
    fn is_interactive_resize(&self) -> bool;
    /// Returns whether a row is resizable
    fn row_resize(&self) -> bool;
    /// Sets a row to be resizable
    fn set_row_resize(&mut self, flag: bool);
    /// Returns whether a column is resizable
    fn col_resize(&self) -> bool;
    /// Sets a column to be resizable
    fn set_col_resize(&mut self, flag: bool);
    /// Returns the current column minimum resize value.
    fn col_resize_min(&self) -> u32;
    /// Sets the current column minimum resize value.
    fn set_col_resize_min(&mut self, val: u32);
    /// Returns the current row minimum resize value.
    fn row_resize_min(&self) -> u32;
    /// Sets the current row minimum resize value.
    fn set_row_resize_min(&mut self, val: u32);
    /// Returns if row headers are enabled or not
    fn row_header(&self) -> bool;
    /// Sets whether a row headers are enabled or not
    fn set_row_header(&mut self, flag: bool);
    /// Returns if column headers are enabled or not
    fn col_header(&self) -> bool;
    /// Sets whether a column headers are enabled or not
    fn set_col_header(&mut self, flag: bool);
    /// Sets the column header height
    fn set_col_header_height(&mut self, height: i32);
    /// Gets the column header height
    fn col_header_height(&self) -> i32;
    /// Sets the row header width
    fn set_row_header_width(&mut self, width: i32);
    /// Gets the row header width
    fn row_header_width(&self) -> i32;
    /// Sets the row header color
    fn set_row_header_color(&mut self, val: Color);
    /// Gets the row header color
    fn row_header_color(&self) -> Color;
    /// Sets the column header color
    fn set_col_header_color(&mut self, val: Color);
    /// Gets the row header color
    fn col_header_color(&self) -> Color;
    /// Sets the row's height
    fn set_row_height(&mut self, row: i32, height: i32);
    /// Gets the row's height
    fn row_height(&self, row: i32) -> i32;
    /// Sets the columns's width
    fn set_col_width(&mut self, col: i32, width: i32);
    /// Gets the columns's width
    fn col_width(&self, col: i32) -> i32;
    /// Sets all rows height
    fn set_row_height_all(&mut self, height: i32);
    /// Sets all columns's width
    fn set_col_width_all(&mut self, width: i32);
    /// Sets the row's position
    fn set_row_position(&mut self, row: i32);
    /// Sets the columns's position
    fn set_col_position(&mut self, col: i32);
    /// Gets the row's position
    fn row_position(&self) -> i32;
    /// Gets the columns's position
    fn col_position(&self) -> i32;
    /// Sets the top row
    fn set_top_row(&mut self, row: i32);
    /// Gets the top row
    fn top_row(&self) -> i32;
    /// Returns whether a cell is selected
    fn is_selected(&self, r: i32, c: i32) -> bool;
    /// Gets the selection
    fn get_selection(
        &self,
        row_top: &mut i32,
        col_left: &mut i32,
        row_bot: &mut i32,
        col_right: &mut i32,
    );
    /// Sets the selection
    fn set_selection(&mut self, row_top: i32, col_left: i32, row_bot: i32, col_right: i32);
    /// Moves the cursor with shift select
    fn move_cursor_with_shift_select(
        &mut self,
        r: i32,
        c: i32,
        shiftselect: bool,
    ) -> Result<(), FltkError>;
    /// Moves the cursor
    fn move_cursor(&mut self, r: i32, c: i32) -> Result<(), FltkError>;
    /// Resets the internal array of widget sizes and positions.
    fn init_sizes(&mut self);
    /// Returns the scrollbar size
    fn scrollbar_size(&self) -> u32;
    /// Sets the scrollbar size
    fn set_scrollbar_size(&mut self, new_size: u32);
    /// Sets the tab key cell navigation
    fn set_tab_cell_nav(&mut self, val: u32);
    /// Returns the tab key cell navigation
    fn tab_cell_nav(&self) -> u32;
}

/// Defines the methods implemented by all image types
pub unsafe trait ImageExt {
    /// Creates a copy of the image
    fn copy(&self) -> Self;
    /// Draws the image at the presupplied coordinates and size
    fn draw(&mut self, x: i32, y: i32, width: i32, height: i32);
    /// Return the width of the image
    fn width(&self) -> i32;
    /// Return the height of the image
    fn height(&self) -> i32;
    /// Returns a void pointer of the image, for internal use
    unsafe fn as_ptr(&self) -> *mut raw::c_void;
    /// Retunrs a pointer of the image
    unsafe fn as_image_ptr(&self) -> *mut fltk_sys::image::Fl_Image;
    /// Transforms a raw image pointer to an image
    unsafe fn from_image_ptr(ptr: *mut fltk_sys::image::Fl_Image) -> Self;
    /// Returns the raw underlying image data
    fn to_rgb(&self) -> Vec<u8>;
    /// Scales the image
    fn scale(&mut self, width: i32, height: i32, proportional: bool, can_expand: bool);
}
