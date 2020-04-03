pub use crate::enums::*;
use crate::text::StyleTableEntry;
use crate::widget::Widget;
use crate::image::Image;
use std::convert::From;
use std::error::Error;
use std::{fmt, io, os::raw};

/// Error types returned by fltk-rs + wrappers of std::io errors
#[derive(Debug)]
pub enum FltkError {
    Io(io::Error),
    Internal(FltkErrorKind),
    Unknown(String),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FltkErrorKind {
    FailedToRun,
    FailedToLock,
    FailedToSetScheme,
    ResourceNotFound,
}

impl FltkErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            FltkErrorKind::FailedToRun => "Failed to run FLTK!",
            FltkErrorKind::FailedToLock => "Failed to initialize app for multithreading!",
            FltkErrorKind::FailedToSetScheme => "Failed to set scheme",
            FltkErrorKind::ResourceNotFound => "Resource Not Found!",
        }
    }
}

impl Error for FltkError {
    fn description(&self) -> &str {
        match *self {
            FltkError::Io(ref err) => err.description(),
            FltkError::Internal(ref err) => err.as_str(),
            FltkError::Unknown(ref err) => err,
        }
    }
}

impl fmt::Display for FltkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FltkError::Io(ref err) => err.fmt(f),
            FltkError::Internal(ref err) => write!(f, "An internal error occured {:?}", err),
            FltkError::Unknown(ref err) => write!(f, "An unknown error occurred {:?}", err),
        }
    }
}

impl From<io::Error> for FltkError {
    fn from(err: io::Error) -> FltkError {
        FltkError::Io(err)
    }
}

/// Defines the methods implemented by all widgets
pub trait WidgetTrait {
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
    /// Positions the widget below w
    fn below_of<W: WidgetTrait>(self, w: &W, padding: i32) -> Self;
    /// Positions the widget above w
    fn above_of<W: WidgetTrait>(self, w: &W, padding: i32) -> Self;
    /// Positions the widget to the right of w
    fn right_of<W: WidgetTrait>(self, w: &W, padding: i32) -> Self;
    /// Positions the widget to the left of w
    fn left_of<W: WidgetTrait>(self, w: &W, padding: i32) -> Self;
    /// Positions the widget to the center of w
    fn center_of<W: WidgetTrait>(self, w: &W) -> Self;
    /// Takes the size of w
    fn size_of<W: WidgetTrait>(self, w: &W) -> Self;
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
    fn as_widget_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget;
    /// transforms a widget pointer to a Widget, for internal use
    fn from_widget_ptr(ptr: *mut fltk_sys::widget::Fl_Widget) -> Self;
    /// Activates the widget
    fn activate(&mut self);
    /// Deactivates the widget
    fn deactivate(&mut self);
    /// Redraws the label of the widget
    fn redraw_label(&mut self);
    /// Resizes and/or moves the widget, takes x, y, width and height
    fn resize(&mut self, x: i32, y: i32, width: i32, height: i32);
    /// Returns the tooltip text
    fn tooltip(&self) -> String;
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
    fn set_image<Image: ImageTrait>(&mut self, image: Image);
    /// Sets the callback when the widget is triggered (clicks for example)
    fn set_callback<'a>(&'a mut self, cb: Box<dyn FnMut() + 'a>);
    /// Set a custom handler, where events are managed manually, akin to Fl_Widget::handle(int)
    fn set_custom_handler<'a>(&'a mut self, cb: Box<dyn FnMut(Event) -> bool + 'a>);
    /// Sets the default callback trigger for a widget
    fn set_trigger(&mut self, trigger: CallbackTrigger);
}

/// Defines the methods implemented by all group widgets
pub trait GroupTrait: WidgetTrait {
    /// Begins a group, used for widgets implementing the group trait
    fn begin(&self);
    /// Ends a group, used for widgets implementing the group trait
    fn end(&self);
    /// Find a widget within a group and return its index
    fn find<Widget: WidgetTrait>(&self, widget: &Widget) -> usize;
    /// Add a widget to a group
    fn add<Widget: WidgetTrait>(&mut self, widget: &Widget);
    /// Insert a widget to a group at a certain index
    fn insert<Widget: WidgetTrait>(&mut self, widget: &Widget, index: usize);
    /// Remove a widget from a group
    fn remove(&mut self, index: usize);
    /// Clear a group from all widgets
    fn clear(&mut self);
    /// Return the number of children in a group
    fn children(&self) -> u32;
    /// Return child widget by index
    fn child(&self, idx: usize) -> Widget;
}

/// Defines the methods implemented by all window widgets
pub trait WindowTrait: GroupTrait {
    /// Positions the window to the center of the screen
    fn center_screen(self) -> Self;
    /// Makes a window modal
    fn make_modal(&mut self, val: bool);
    /// Makes a window fullscreen
    fn fullscreen(&mut self, val: bool);
    /// Makes the window current
    fn make_current(&mut self);
    /// Sets the windows icon
    fn set_icon<Image: ImageTrait>(&mut self, image: Image);
    /// Make the window resizable
    fn make_resizable(&self, val: bool);
}

/// Defines the methods implemented by all input and output widgets
pub trait InputTrait: WidgetTrait {
    /// Returns the value inside the input/output widget
    fn value(&self) -> String;
    /// Sets the value inside an input/output widget
    fn set_value(&self, val: &str);
    /// Returns the maximum size (in bytes) accepted by an input/output widget
    fn maximum_size(&self) -> usize;
    /// Sets the maximum size (in bytes) accepted by an input/output widget
    fn set_maximum_size(&mut self, val: usize);
    /// Returns the postion inside an input/output widget
    fn position(&self) -> i32;
    /// Sets the postion inside an input/output widget
    fn set_position(&mut self, val: i32);
    /// Returns the mark inside an input/output widget
    fn mark(&self) -> i32;
    /// Sets the mark inside an input/output widget
    fn set_mark(&mut self, val: i32);
    /// Replace content with a &str
    fn replace(&mut self, beg: usize, end: usize, val: &str);
    /// Insert a &str
    fn insert(&mut self, txt: &str);
    /// Append a &str
    fn append(&mut self, txt: &str);
    /// Copy the value within the widget
    fn copy(&mut self);
    /// Undo changes
    fn undo(&mut self);
    /// Cut the value within the widget
    fn cut(&mut self);
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
pub trait MenuTrait: WidgetTrait {
    /// Get a menu item by name
    fn get_item(&self, name: &str) -> crate::menu::MenuItem;
    /// Return the text font
    fn text_font(&self) -> Font;
    /// Sets the text font
    fn set_text_font(&mut self, c: Font);
    /// Return the text size
    fn text_size(&self) -> u32;
    /// Sets the text size
    fn set_text_size(&mut self, c: usize);
    /// Return the text color
    fn text_color(&self) -> Color;
    /// Sets the text color
    fn set_text_color(&mut self, c: Color);
    /// Add a menu item along with its callback
    fn add<'a>(
        &'a mut self,
        name: &str,
        shortcut: Shortcut,
        flag: crate::menu::MenuFlag,
        cb: Box<dyn FnMut() + 'a>,
    );
    /// Inserts a menu item at an index along with its callback
    fn insert<'a>(
        &'a mut self,
        idx: usize,
        name: &str,
        shortcut: Shortcut,
        flag: crate::menu::MenuFlag,
        cb: Box<dyn FnMut() + 'a>,
    );
    /// Adds a simple text option to the Choice and MenuButton widgets
    fn add_choice(&mut self, text: &str);
    /// Gets the user choice from the Choice and MenuButton widgets
    fn get_choice(&self) -> String;
}

/// Defines the methods implemented by all valuator widgets
pub trait ValuatorTrait: WidgetTrait {
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
    fn format(&mut self, arg2: &str);
    /// Round the valuator
    fn round(&self, arg2: f64) -> f64;
    /// Clamp the valuator
    fn clamp(&self, arg2: f64) -> f64;
    /// Increment the valuator
    fn increment(&mut self, arg2: f64, arg3: i32) -> f64;
}

/// Defines the methods implemented by TextDisplay and TextEditor
pub trait DisplayTrait: WidgetTrait {
    /// Set the text inside the widget
    fn set_text(&mut self, txt: &str);
    /// Returns the text inside the widget
    fn text(&self) -> String;
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
    /// Append text to Display widget
    fn append(&mut self, text: &str);
    /// Return buffer length of Display widget                  
    fn buffer_length(&self) -> usize;
    /// Scroll down the Display widget
    fn scroll(&mut self, top_line_num: usize, horiz_offset: usize);
    /// Insert into Display widget      
    fn insert(&self, text: &str);
    /// Set the insert position
    fn set_insert_position(&mut self, new_pos: usize);
    /// Return the insert position                
    fn insert_position(&self) -> usize;
    /// Counts the lines from start to end                         
    fn count_lines(&self, start: usize, end: usize, is_line_start: bool) -> usize;
    /// Moves the cursor right
    fn move_right(&mut self);
    /// Moves the cursor left
    fn move_left(&mut self);
    /// Moves the cursor up
    fn move_up(&mut self);
    /// Moves the cursor down
    fn move_down(&mut self);
    /// Remove text from start position to end position
    fn remove(&mut self, start: usize, end: usize);
    /// Shows/hides the cursor
    fn show_cursor(&mut self, val: bool);
    /// Sets the style of the text widget
    fn set_styly_table_entry(&mut self, entries: &Vec<StyleTableEntry>);
    /// Sets the cursor style
    fn set_cursor_style(&mut self, style: CursorStyle);
    /// Sets the cursor color
    fn set_cursor_color(&mut self, color: Color);
    /// Sets the scrollbar width
    fn set_scrollbar_width(&mut self, width: u32);
    /// Sets the scrollbar size in pixels
    fn set_scrollbar_size(&mut self, size: usize);
    /// Sets the scrollbar alignment
    fn set_scrollbar_align(&mut self, align: Align);
    /// Returns the cursor style
    fn cursor_style(&self) -> CursorStyle;
    /// Returns the cursor color
    fn cursor_color(&self) -> Color;
    /// Returns the scrollback width
    fn scrollbar_width(&self) -> u32;
    /// Returns the scrollbar size in pixels
    fn scrollbar_size(&self) -> usize;
    /// Returns the scrollbar alignment
    fn scrollbar_align(&self) -> Align;
}

/// Defines the methods implemented by all browser types
pub trait BrowserTrait {
    /// Removes the specified line
    fn remove(&mut self, line: usize);
    /// Adds an item
    fn add(&mut self, item: &str);
    /// Inserts an item at an index
    fn insert(&mut self, line: usize, item: &str);
    /// Moves an item
    fn move_item(&mut self, to: usize, from: usize);
    /// Swaps 2 items
    fn swap(&mut self, a: usize, b: usize);
    /// Clears the browser widget
    fn clear(&mut self);
    /// Returns the number of items
    fn size(&self) -> usize;
    /// Set the number of items
    fn set_size(&mut self, w: i32, h: i32);
    /// Select an item at the specified line
    fn select(&mut self, line: usize);
    /// Returns whether the item is selected
    fn selected(&self, line: usize) -> bool;
    /// Returns the text of the selected item
    fn text(&self, line: usize) -> String;
    /// Sets the text of the selected item
    fn set_text(&mut self, line: usize, txt: &str);
    /// Load a file
    fn load_file(&mut self, path: &std::path::Path);
    /// Return the text size
    fn text_size(&self) -> u32;
    /// Sets the text size
    fn set_text_size(&mut self, sz: u32);
    /// Sets the icon for browser elements
    fn set_icon<Img: ImageTrait>(&mut self, line: usize, image: Img);
    /// Returns the icon of a browser element
    fn icon(&self, line: usize) -> Image;
    /// Removes the icon of a browser element
    fn remove_icon(&mut self, line: usize);
}

/// Defines the methods implemented by all image types
pub trait ImageTrait {
    /// Creates an image object from a path
    fn new(path: std::path::PathBuf) -> Self;
    /// Draws the image at the presupplied coordinates and size
    fn draw(&mut self, x: i32, y: i32, width: i32, height: i32);
    /// Return the width of the image
    fn width(&self) -> i32;
    /// Return the height of the image
    fn height(&self) -> i32;
    /// Returns a void pointer of the image, for internal use
    fn as_ptr(&self) -> *mut raw::c_void;
    /// Retunrs a pointer of the image
    fn as_image_ptr(&self) -> *mut fltk_sys::image::Fl_Image;
    /// Transforms a raw image pointer to an image
    fn from_image_ptr(ptr: *mut fltk_sys::image::Fl_Image) -> Self;
}
