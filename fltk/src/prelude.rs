use crate::enums::{
    Align, CallbackTrigger, Color, ColorDepth, Cursor, Damage, Event, Font, FrameType, LabelType,
    Shortcut,
};
use std::convert::From;
use std::string::FromUtf8Error;
use std::{fmt, io};

/// Error types returned by fltk-rs + wrappers of std errors
#[derive(Debug)]
#[non_exhaustive]
pub enum FltkError {
    /// i/o error
    IoError(io::Error),
    /// Utf-8 conversion error
    Utf8Error(FromUtf8Error),
    /// Null string conversion error
    NullError(std::ffi::NulError),
    /// Internal fltk error
    Internal(FltkErrorKind),
    /// Error using an erroneous env variable
    EnvVarError(std::env::VarError),
    /// Parsing error
    ParseIntError(std::num::ParseIntError),
    /// Unknown error
    Unknown(String),
}

unsafe impl Send for FltkError {}
unsafe impl Sync for FltkError {}

/// Error kinds enum for `FltkError`
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum FltkErrorKind {
    /// Failed to run the application
    FailedToRun,
    /// Failed to initialize the multithreading
    FailedToLock,
    /// Failed to set the general scheme of the application
    FailedToSetScheme,
    /// Failed operation, mostly unknown reason!
    FailedOperation,
    /// System resource (file, image) not found
    ResourceNotFound,
    /// Image format error when opening an image of an unsupported format
    ImageFormatError,
    /// Error filling table
    TableError,
    /// Error due to printing
    PrintError,
    /// Invalid color
    InvalidColor,
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
            FltkError::Internal(ref err) => write!(f, "An internal error occurred {:?}", err),
            FltkError::EnvVarError(ref err) => write!(f, "An env var error occurred {:?}", err),
            FltkError::Utf8Error(ref err) => {
                write!(f, "A UTF8 conversion error occurred {:?}", err)
            }
            FltkError::ParseIntError(ref err) => {
                write!(f, "An int parsing error occurred {:?}", err)
            }
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

impl From<std::env::VarError> for FltkError {
    fn from(err: std::env::VarError) -> FltkError {
        FltkError::EnvVarError(err)
    }
}

impl From<std::string::FromUtf8Error> for FltkError {
    fn from(err: std::string::FromUtf8Error) -> FltkError {
        FltkError::Utf8Error(err)
    }
}

impl From<std::num::ParseIntError> for FltkError {
    fn from(err: std::num::ParseIntError) -> FltkError {
        FltkError::ParseIntError(err)
    }
}

/// A trait defined for all enums passable to the [`WidgetExt::set_type()`](`crate::prelude::WidgetExt::set_type`) method
pub trait WidgetType {
    /// Get the integral representation of the widget type
    fn to_i32(self) -> i32;
    /// Get the widget type from its integral representation
    fn from_i32(val: i32) -> Self;
}

/// Defines the methods implemented by all widgets
///
/// For multithreaded usage, see the [`widget` module documentation's note](crate::widget)
/// # Safety
/// fltk-rs traits depend on some FLTK internal code
/// # Warning
/// fltk-rs traits are non-exhaustive,
/// to avoid future breakage if you try to implement them manually,
/// use the Deref and DerefMut pattern or the `widget_extends!` macro
pub unsafe trait WidgetExt {
    /// Initialize to a position x, y
    fn with_pos(self, x: i32, y: i32) -> Self
    where
        Self: Sized;
    /// Initialize to size width, height
    fn with_size(self, width: i32, height: i32) -> Self
    where
        Self: Sized;
    /// Initialize with a label
    fn with_label(self, title: &str) -> Self
    where
        Self: Sized;
    /// Initialize with alignment
    fn with_align(self, align: crate::enums::Align) -> Self
    where
        Self: Sized;
    /// Initialize with type
    fn with_type<T: WidgetType>(self, typ: T) -> Self
    where
        Self: Sized;
    /// Initialize at bottom of another widget
    fn below_of<W: WidgetExt>(self, wid: &W, padding: i32) -> Self
    where
        Self: Sized;
    /// Initialize above of another widget
    fn above_of<W: WidgetExt>(self, wid: &W, padding: i32) -> Self
    where
        Self: Sized;
    /// Initialize right of another widget
    fn right_of<W: WidgetExt>(self, wid: &W, padding: i32) -> Self
    where
        Self: Sized;
    /// Initialize left of another widget
    fn left_of<W: WidgetExt>(self, wid: &W, padding: i32) -> Self
    where
        Self: Sized;
    /// Initialize center of another widget
    fn center_of<W: WidgetExt>(self, w: &W) -> Self
    where
        Self: Sized;
    /// Initialize center of another widget on the x axis
    fn center_x<W: WidgetExt>(self, w: &W) -> Self
    where
        Self: Sized;
    /// Initialize center of another widget on the y axis
    fn center_y<W: WidgetExt>(self, w: &W) -> Self
    where
        Self: Sized;
    /// Initialize center of parent
    fn center_of_parent(self) -> Self
    where
        Self: Sized;
    /// Initialize to the size of another widget
    fn size_of<W: WidgetExt>(self, w: &W) -> Self
    where
        Self: Sized;
    /// Initialize to the size of the parent
    fn size_of_parent(self) -> Self
    where
        Self: Sized;
    /// Set to position x, y
    fn set_pos(&mut self, x: i32, y: i32);
    /// Set to dimensions width and height
    fn set_size(&mut self, width: i32, height: i32);
    /// Sets the widget's label.
    /// labels support special symbols preceded by an `@` [sign](https://www.fltk.org/doc-1.3/symbols.png).
    /// and for the [associated formatting](https://www.fltk.org/doc-1.3/common.html).
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
    /// Returns the width of the widget
    fn w(&self) -> i32;
    /// Returns the height of the widget
    fn h(&self) -> i32;
    /// Returns the label of the widget
    fn label(&self) -> String;
    /// Measures the label's width and height
    fn measure_label(&self) -> (i32, i32);
    /// transforms a widget to a base `Fl_Widget`, for internal use
    /// # Safety
    /// Can return multiple mutable pointers to the same widget
    unsafe fn as_widget_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget;
    /// Checks whether the self widget is inside another widget
    fn inside<W: WidgetExt>(&self, wid: &W) -> bool
    where
        Self: Sized;
    /// Returns the widget type when applicable
    fn get_type<T: WidgetType>(&self) -> T
    where
        Self: Sized;
    /// Sets the widget type
    fn set_type<T: WidgetType>(&mut self, typ: T)
    where
        Self: Sized;
    /// Sets the image of the widget
    fn set_image<I: ImageExt>(&mut self, image: Option<I>)
    where
        Self: Sized;
    /// Sets the image of the widget scaled to the widget's size
    fn set_image_scaled<I: ImageExt>(&mut self, image: Option<I>)
    where
        Self: Sized;
    /// Gets the image associated with the widget
    fn image(&self) -> Option<Box<dyn ImageExt>>
    where
        Self: Sized;
    /// Sets the deactivated image of the widget
    fn set_deimage<I: ImageExt>(&mut self, image: Option<I>)
    where
        Self: Sized;
    /// Sets the deactivated image of the widget scaled to the widget's size
    fn set_deimage_scaled<I: ImageExt>(&mut self, image: Option<I>)
    where
        Self: Sized;
    /// Gets the deactivated image associated with the widget
    fn deimage(&self) -> Option<Box<dyn ImageExt>>
    where
        Self: Sized;
    /// Sets the callback when the widget is triggered (clicks for example)
    /// takes the widget as a closure argument
    fn set_callback<F: FnMut(&mut Self) + 'static>(&mut self, cb: F)
    where
        Self: Sized;
    /// Emits a message on callback using a sender
    fn emit<T: 'static + Clone + Send + Sync>(&mut self, sender: crate::app::Sender<T>, msg: T)
    where
        Self: Sized;
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
    fn label_type(&self) -> LabelType;
    /// Sets the widget label's type
    fn set_label_type(&mut self, typ: LabelType);
    /// Returns the widget's frame type
    fn frame(&self) -> FrameType;
    /// Sets the widget's frame type
    fn set_frame(&mut self, typ: FrameType);
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
    /// Returns the parent of the widget
    fn parent(&self) -> Option<crate::group::Group>;
    /// Gets the selection color of the widget
    fn selection_color(&self) -> Color;
    /// Sets the selection color of the widget
    fn set_selection_color(&mut self, color: Color);
    /// Runs the already registered callback
    fn do_callback(&mut self);
    /// Returns the direct window holding the widget
    fn window(&self) -> Option<Box<dyn WindowExt>>;
    /// Returns the topmost window holding the widget
    fn top_window(&self) -> Option<Box<dyn WindowExt>>;
    /// Checks whether a widget is capable of taking events
    fn takes_events(&self) -> bool;
    /// Make the widget take focus
    /// # Errors
    /// Errors on failure to take focus
    fn take_focus(&mut self) -> Result<(), FltkError>;
    /// Set the widget to have visible focus
    fn set_visible_focus(&mut self);
    /// Clear visible focus
    fn clear_visible_focus(&mut self);
    /// Set the visible focus using a flag
    fn visible_focus(&mut self, v: bool);
    /// Return whether the widget has visible focus
    fn has_visible_focus(&self) -> bool;
    /// Return whether the widget has focus
    fn has_focus(&self) -> bool;
    /// Check if a widget was deleted
    fn was_deleted(&self) -> bool;
    /// Return whether the widget was damaged
    fn damage(&self) -> bool;
    /// Signal the widget as damaged and it should be redrawn in the next event loop cycle
    fn set_damage(&mut self, flag: bool);
    /// Return the damage mask
    fn damage_type(&self) -> Damage;
    /// Signal the type of damage a widget received
    fn set_damage_type(&mut self, mask: Damage);
    /// Clear the damaged flag
    fn clear_damage(&mut self);
    /// Sets the default callback trigger for a widget, equivalent to `when()`
    fn set_trigger(&mut self, trigger: CallbackTrigger);
    /// Return the callback trigger, equivalent to `when()`
    fn trigger(&self) -> CallbackTrigger;
    /// Return the widget as a window if it's a window
    fn as_window(&self) -> Option<Box<dyn WindowExt>>;
    /// Return the widget as a group widget if it's a group widget
    fn as_group(&self) -> Option<crate::group::Group>;
    #[doc(hidden)]
    /// INTERNAL: Retakes ownership of the user callback data
    /// # Safety
    /// Can return multiple mutable references to the `user_data`
    unsafe fn user_data(&self) -> Option<Box<dyn FnMut()>>;
    #[doc(hidden)]
    /// INTERNAL: Get the raw user data of the widget
    /// # Safety
    /// Can return multiple mutable references to the `user_data`
    unsafe fn raw_user_data(&self) -> *mut std::os::raw::c_void;
    #[doc(hidden)]
    /// INTERNAL: Set the raw user data of the widget
    /// # Safety
    /// Can return multiple mutable references to the `user_data`
    unsafe fn set_raw_user_data(&mut self, data: *mut std::os::raw::c_void);
    /// Upcast a `WidgetExt` to a Widget
    /// # Safety
    /// Allows for potentially unsafe casts between incompatible widget types
    #[allow(clippy::wrong_self_convention)]
    unsafe fn into_widget<W: WidgetBase>(&self) -> W
    where
        Self: Sized;
    /// Returns whether a widget is visible
    fn visible(&self) -> bool;
    /// Returns whether a widget or any of its parents are visible (recursively)
    fn visible_r(&self) -> bool;
    /// Return whether two widgets object point to the same widget
    fn is_same<W: WidgetExt>(&self, other: &W) -> bool
    where
        Self: Sized;
    /// Returns whether a widget is active
    fn active(&self) -> bool;
    /// Returns whether a widget or any of its parents are active (recursively)
    fn active_r(&self) -> bool;
    #[doc(hidden)]
    /**
        Return the default callback function, this allows storing then running within the overridden callback.
        Works only for FLTK types (with no callback defined in the Rust side)
        ```rust,no_run
            use fltk::{prelude::*, *};
            fn main() {
                let a = app::App::default();
                let mut win = window::Window::default().with_size(400, 300);
                let scroll = group::Scroll::default().size_of_parent();
                let _btn = button::Button::new(160, 500, 80, 40, "click");
                let mut scrollbar = scroll.scrollbar();
                scrollbar.set_callback({
                    let mut cb = scrollbar.callback();
                    move |_| {
                        println!("print something, and also run the default callback");
                        if let Some(cb) = cb.as_mut() {
                            (*cb)();
                        }
                    }
                });
                win.end();
                win.show();
                a.run().unwrap();
            }
        ```
    */
    fn callback(&self) -> Option<Box<dyn FnMut()>>;
    /// Does a simple resize ignoring class-specific resize functionality
    fn widget_resize(&mut self, x: i32, y: i32, w: i32, h: i32);
    /// Handle a specific event
    fn handle_event(&mut self, event: Event);
    /// Check whether a widget is derived
    fn is_derived(&self) -> bool {
        unimplemented!();
    }
}

/// Defines the extended methods implemented by all widgets
/// # Safety
/// fltk-rs traits depend on some FLTK internal code
/// # Warning
/// fltk-rs traits are non-exhaustive,
/// to avoid future breakage if you try to implement them manually,
/// use the Deref and DerefMut pattern or the `widget_extends!` macro
pub unsafe trait WidgetBase: WidgetExt {
    /// Creates a new widget, takes an x, y coordinates, as well as a width and height, plus a title
    /// # Arguments
    /// * `x` - The x coordinate in the screen
    /// * `y` - The y coordinate in the screen
    /// * `width` - The width of the widget
    /// * `heigth` - The height of the widget
    /// * `title` - The title or label of the widget
    /// The title is expected to be a static str or None.
    /// To use dynamic strings use `with_label(self, &str)` or `set_label(&mut self, &str)`
    /// labels support special symbols preceded by an `@` [sign](https://www.fltk.org/doc-1.3/symbols.png).
    /// and for the [associated formatting](https://www.fltk.org/doc-1.3/common.html).
    fn new<T: Into<Option<&'static str>>>(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        title: T,
    ) -> Self;
    /// Constructs a widget with the size of its parent
    fn default_fill() -> Self;
    /// Deletes widgets and their children.
    fn delete(wid: Self)
    where
        Self: Sized;
    /// transforms a widget pointer to a Widget, for internal use
    /// # Safety
    /// The pointer must be valid
    unsafe fn from_widget_ptr(ptr: *mut fltk_sys::widget::Fl_Widget) -> Self;
    /// Get a widget from base widget
    /// # Safety
    /// The underlying object must be valid
    unsafe fn from_widget<W: WidgetExt>(w: W) -> Self;
    /// Set a custom handler, where events are managed manually, akin to `Fl_Widget::handle(int)`.
    /// Handled or ignored events should return true, unhandled events should return false.
    /// takes the widget as a closure argument
    fn handle<F: FnMut(&mut Self, Event) -> bool + 'static>(&mut self, cb: F);
    /// Set a custom draw method.
    /// takes the widget as a closure argument.
    /// macOS requires that `WidgetBase::draw` actually calls drawing functions
    fn draw<F: FnMut(&mut Self) + 'static>(&mut self, cb: F);
    #[doc(hidden)]
    /// INTERNAL: Retrieve the draw data
    /// # Safety
    /// Can return multiple mutable references to the `draw_data`
    unsafe fn draw_data(&self) -> Option<Box<dyn FnMut()>>;
    #[doc(hidden)]
    /// INTERNAL: Retrieve the handle data
    /// # Safety
    /// Can return multiple mutable references to the `handle_data`
    unsafe fn handle_data(&self) -> Option<Box<dyn FnMut(Event) -> bool>>;
    /// Perform a callback on resize.
    /// Avoid resizing the parent or the same widget to avoid infinite recursion
    fn resize_callback<F: FnMut(&mut Self, i32, i32, i32, i32) + 'static>(&mut self, cb: F);
    /// Makes the widget derived
    /// # Safety
    /// Calling this on a non-derived widget can cause undefined behavior
    unsafe fn assume_derived(&mut self) {
        unimplemented!();
    }
}

/// Defines the methods implemented by all button widgets.
/// More details can be found in the [wiki](https://github.com/fltk-rs/fltk-rs/wiki/buttons).
/// # Safety
/// fltk-rs traits depend on some FLTK internal code
/// # Warning
/// fltk-rs traits are non-exhaustive,
/// to avoid future breakage if you try to implement them manually,
/// use the Deref and DerefMut pattern or the `widget_extends!` macro
pub unsafe trait ButtonExt: WidgetExt {
    /// Gets the shortcut associated with a button
    fn shortcut(&self) -> Shortcut;
    /// Sets the shortcut associated with a button
    fn set_shortcut(&mut self, shortcut: Shortcut);
    /// Clears the value of the button.
    /// Useful for round, radio, light, toggle and check buttons
    fn clear(&mut self);
    /// Returns whether a button is set or not.
    /// Useful for round, radio, light, toggle and check buttons
    fn is_set(&self) -> bool;
    /// Sets whether a button is set or not.
    /// Useful for round, radio, light, toggle and check buttons
    fn set(&mut self, flag: bool);
    /// Returns whether a button is set or not.
    /// Useful for round, radio, light, toggle and check buttons
    fn value(&self) -> bool;
    /// Sets whether a button is set or not.
    /// Useful for round, radio, light, toggle and check buttons
    fn set_value(&mut self, flag: bool);
    /// Set the `down_box` of the widget
    fn set_down_frame(&mut self, f: FrameType);
    /// Get the down frame type of the widget
    fn down_frame(&self) -> FrameType;
}

/// Defines the methods implemented by all group widgets.
/// These widgets include Window types and others found in the group module: Group, Scroll, Pack, Tile, Flex ...etc.
/// Widgets implementing the GroupExt trait, are characterized by having to call `::end()` method to basically close them.
/// More details can be found in the [wiki](https://github.com/fltk-rs/fltk-rs/wiki/group_widgets).
/// ```rust
/// use fltk::{app, button::Button, window::Window, prelude::GroupExt};
/// let a = app::App::default();
/// let win = Window::default();
/// let btn = Button::default();
/// // Instantiate other widgets
/// win.end();
/// ```
/// In the above example, the button `btn` will be parented by the window.
/// After `end`ing such GroupExt widgets, any other widgets instantiated after the `end` call, will be instantiated outside.
/// These can still be added using the `::add(&other_widget)` method (or using `::insert`):
/// ```rust
/// use fltk::{app, button::Button, window::Window, prelude::GroupExt};
/// let a = app::App::default();
/// let mut win = Window::default();
/// win.end();
/// let btn = Button::default();
/// win.add(&btn);
/// ```
/// Another option is to reopen the widget:
/// ```rust
/// use fltk::{app, button::Button, window::Window, prelude::GroupExt};
/// let a = app::App::default();
/// let win = Window::default();
/// win.end();
/// win.begin();
/// let btn = Button::default();
/// // other widgets
/// win.end();
/// ```
/// # Safety
/// fltk-rs traits depend on some FLTK internal code
/// # Warning
/// fltk-rs traits are non-exhaustive,
/// to avoid future breakage if you try to implement them manually,
/// use the Deref and DerefMut pattern or the `widget_extends!` macro
pub unsafe trait GroupExt: WidgetExt {
    /// Begins a group, used for widgets implementing the group trait
    fn begin(&self);
    /// Ends a group, used for widgets implementing the group trait
    fn end(&self);
    /// Clear a group from all widgets
    fn clear(&mut self);
    #[doc(hidden)]
    /// Clear a group from all widgets using FLTK's clear call.
    /// # Safety
    /// Ignores widget tracking
    unsafe fn unsafe_clear(&mut self);
    /// Return the number of children in a group
    fn children(&self) -> i32;
    /// Return child widget by index
    fn child(&self, idx: i32) -> Option<crate::widget::Widget>;
    /// Find a widget within a group and return its index
    fn find<W: WidgetExt>(&self, widget: &W) -> i32
    where
        Self: Sized;
    /// Add a widget to a group
    fn add<W: WidgetExt>(&mut self, widget: &W)
    where
        Self: Sized;
    /// Insert a widget to a group at a certain index
    fn insert<W: WidgetExt>(&mut self, widget: &W, index: i32)
    where
        Self: Sized;
    /// Remove a widget from a group, but does not delete it
    fn remove<W: WidgetExt>(&mut self, widget: &W)
    where
        Self: Sized;
    /// Remove a child widget by its index
    fn remove_by_index(&mut self, idx: i32);
    /// The resizable widget defines both the resizing frame and the resizing behavior of the group and its children.
    fn resizable<W: WidgetExt>(&self, widget: &W)
    where
        Self: Sized;
    /// Make the group itself resizable, should be called before the widget is shown
    fn make_resizable(&mut self, val: bool);
    /// Adds a widget to the group and makes it the resizable widget
    fn add_resizable<W: WidgetExt>(&mut self, widget: &W)
    where
        Self: Sized;
    /// Clips children outside the group boundaries
    fn set_clip_children(&mut self, flag: bool);
    /// Get whether `clip_children` is set
    fn clip_children(&self) -> bool;
    /// Draw a child widget, the call should be in a [`WidgetBase::draw`](`crate::prelude::WidgetBase::draw`) method
    fn draw_child<W: WidgetExt>(&self, w: &mut W)
    where
        Self: Sized;
    /// Update a child widget, the call should be in a [`WidgetBase::draw`](`crate::prelude::WidgetBase::draw`) method
    fn update_child<W: WidgetExt>(&self, w: &mut W)
    where
        Self: Sized;
    /// Draw the outside label, the call should be in a [`WidgetBase::draw`](`crate::prelude::WidgetBase::draw`) method
    fn draw_outside_label<W: WidgetExt>(&self, w: &mut W)
    where
        Self: Sized;
    /// Draw children, the call should be in a [`WidgetBase::draw`](`crate::prelude::WidgetBase::draw`) method
    fn draw_children(&mut self);
    /// Resets the internal array of widget sizes and positions
    fn init_sizes(&mut self);
    /// Get the bounds of all children widgets (left, upper, right, bottom)
    fn bounds(&self) -> Vec<(i32, i32, i32, i32)>;
    /// Converts a widget implementing GroupExt into a Group widget
    /// # Safety
    /// If the widget wasn't created by fltk-rs,
    /// vtable differences mean certain methods can't be overridden (e.g. handle & draw)
    #[allow(clippy::wrong_self_convention)]
    unsafe fn into_group(&self) -> crate::group::Group;
}

/// Defines the methods implemented by all window widgets.
/// More details can be found in the [wiki](https://github.com/fltk-rs/fltk-rs/wiki/windows).
/// Windows (which can be found in the window module) implement GroupExt as well.
/// # Safety
/// fltk-rs traits depend on some FLTK internal code
/// # Warning
/// fltk-rs traits are non-exhaustive,
/// to avoid future breakage if you try to implement them manually,
/// use the Deref and DerefMut pattern or the `widget_extends!` macro
pub unsafe trait WindowExt: GroupExt {
    /// Positions the window to the center of the screen
    fn center_screen(self) -> Self
    where
        Self: Sized;
    /// Makes a window modal, should be called before `show`
    fn make_modal(&mut self, val: bool);
    /// Makes a window fullscreen
    fn fullscreen(&mut self, val: bool);
    /// Makes the window current
    fn make_current(&mut self);
    /// Returns the icon of the window
    fn icon(&self) -> Option<Box<dyn ImageExt>>;
    /// Sets the windows icon.
    /// Supported formats are bmp, jpeg, png and rgb.
    fn set_icon<T: ImageExt>(&mut self, image: Option<T>)
    where
        Self: Sized;
    /// Sets the cursor style within the window.
    /// Needs to be called after the window is shown
    fn set_cursor(&mut self, cursor: Cursor);
    /// Returns whether a window is shown
    fn shown(&self) -> bool;
    /// Sets whether the window has a border
    fn set_border(&mut self, flag: bool);
    /// Returns whether a window has a border
    fn border(&self) -> bool;
    /// Frees the position of the window
    fn free_position(&mut self);
    /// Get the raw system handle of the window
    fn raw_handle(&self) -> crate::window::RawHandle;
    #[doc(hidden)]
    /// Set the window associated with a raw handle.
    /// `RawHandle` is a void pointer to: (Windows: `HWND`, X11: `Xid` (`u64`), macOS: `NSWindow`)
    /// # Safety
    /// The data must be valid and is OS-dependent. The window must be shown.
    unsafe fn set_raw_handle(&mut self, handle: crate::window::RawHandle);
    /// Get the graphical draw region of the window
    fn region(&self) -> crate::draw::Region;
    /// Set the graphical draw region of the window
    /// # Safety
    /// The data must be valid.
    unsafe fn set_region(&mut self, region: crate::draw::Region);
    /// Iconifies the window.
    /// You can tell that the window is iconized by checking that it's shown and not visible
    fn iconize(&mut self);
    /// Returns whether the window is fullscreen or not
    fn fullscreen_active(&self) -> bool;
    /// Returns the decorated width
    fn decorated_w(&self) -> i32;
    /// Returns the decorated height
    fn decorated_h(&self) -> i32;
    /// Set the window's minimum width, minimum height, max width and max height.
    /// You can pass 0 as max_w and max_h to allow unlimited upward resize of the window.
    fn size_range(&mut self, min_w: i32, min_h: i32, max_w: i32, max_h: i32);
    /// Set the hotspot widget of the window
    fn hotspot<W: WidgetExt>(&mut self, w: &W)
    where
        Self: Sized;
    /// Set the shape of the window.
    /// Supported image formats are BMP, RGB and Pixmap.
    /// The window covers non-transparent/non-black shape of the image.
    /// The image must not be scaled(resized) beforehand.
    /// The size will be adapted to the window's size
    fn set_shape<I: ImageExt>(&mut self, image: Option<I>)
    where
        Self: Sized;
    /// Get the shape of the window
    fn shape(&self) -> Option<Box<dyn ImageExt>>;
    /// Get the window's x coord from the screen
    fn x_root(&self) -> i32;
    /// Get the window's y coord from the screen
    fn y_root(&self) -> i32;
    /// Set the cursor image
    fn set_cursor_image(&mut self, image: crate::image::RgbImage, hot_x: i32, hot_y: i32);
    /// Set the window's default cursor
    fn default_cursor(&mut self, cursor: Cursor);
    /// Get the screen number
    fn screen_num(&self) -> i32;
    /// Set the screen number
    fn set_screen_num(&mut self, n: i32);
    /// wait for the window to be displayed after calling `show()`.
    /// More info [here](https://www.fltk.org/doc-1.4/classFl__Window.html#aafbec14ca8ff8abdaff77a35ebb23dd8)
    fn wait_for_expose(&self);
    /// Get the window's opacity
    fn opacity(&self) -> f64;
    /// Set the window's opacity,
    /// Ranges from 0.0 to 1.0, where 1.0 is fully opaque and 0.0 is fully transparent.
    /// This should be called on a shown window.
    /// On X11, opacity support depends on the window manager and can be queried:
    /// ```ignore
    /// $ xprop -root _NET_SUPPORTED | grep -o _NET_WM_WINDOW_OPACITY
    /// ```
    fn set_opacity(&mut self, val: f64);
    /// Get the window's XA_WM_CLASS property
    fn xclass(&self) -> Option<String>;
    /// Set the window's XA_WM_CLASS property.
    /// This should be called before showing the window
    fn set_xclass(&mut self, s: &str);
    /// Clear the modal state of the window
    fn clear_modal_states(&mut self);
    /// removes the window border and sets the window on top, by settings the NOBORDER and OVERRIDE flags
    fn set_override(&mut self);
    /// Checks whether the OVERRIDE flag was set
    fn is_override(&self) -> bool;
    /// Forces the position of the window
    fn force_position(&mut self, flag: bool);
    /// Set the icon label
    fn set_icon_label(&mut self, label: &str);
    /// Get the icon label
    fn icon_label(&self) -> Option<String>;
}

/// Defines the methods implemented by all input and output widgets.
/// More details can be found in the [wiki](https://github.com/fltk-rs/fltk-rs/wiki/inout_widgets).
/// # Safety
/// fltk-rs traits depend on some FLTK internal code
/// # Warning
/// fltk-rs traits are non-exhaustive,
/// to avoid future breakage if you try to implement them manually,
/// use the Deref and DerefMut pattern or the `widget_extends!` macro
pub unsafe trait InputExt: WidgetExt {
    /// Returns the value inside the input/output widget
    fn value(&self) -> String;
    /// Sets the value inside an input/output widget
    fn set_value(&mut self, val: &str);
    /// Returns the maximum size (in bytes) accepted by an input/output widget
    fn maximum_size(&self) -> i32;
    /// Sets the maximum size (in bytes) accepted by an input/output widget
    fn set_maximum_size(&mut self, val: i32);
    /// Returns the index position inside an input/output widget
    fn position(&self) -> i32;
    /// Sets the index position inside an input/output widget
    /// # Errors
    /// Errors on failure to set the cursor position in the text
    fn set_position(&mut self, val: i32) -> Result<(), FltkError>;
    /// Returns the index mark inside an input/output widget
    fn mark(&self) -> i32;
    /// Sets the index mark inside an input/output widget
    /// # Errors
    /// Errors on failure to set the mark
    fn set_mark(&mut self, val: i32) -> Result<(), FltkError>;
    /// Replace content with a &str
    /// # Errors
    /// Errors on failure to replace text
    fn replace(&mut self, beg: i32, end: i32, val: &str) -> Result<(), FltkError>;
    /// Insert a &str
    /// # Errors
    /// Errors on failure to insert text
    fn insert(&mut self, txt: &str) -> Result<(), FltkError>;
    /// Append a &str
    /// # Errors
    /// Errors on failure to append text
    fn append(&mut self, txt: &str) -> Result<(), FltkError>;
    /// Copy the value within the widget
    /// # Errors
    /// Errors on failure to copy selection
    fn copy(&mut self) -> Result<(), FltkError>;
    /// Undo changes
    /// # Errors
    /// Errors on failure to undo
    fn undo(&mut self) -> Result<(), FltkError>;
    /// Cut the value within the widget
    /// # Errors
    /// Errors on failure to cut selection
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
    fn text_size(&self) -> i32;
    /// Sets the text size
    fn set_text_size(&mut self, sz: i32);
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
/// These are found in the menu module: MenuBar, SysMenuBar, Choice, MenuButton ...etc.
/// Menus function in 2 main ways which are discussed in the [wiki](https://github.com/fltk-rs/fltk-rs/wiki/menus)
/// # Safety
/// fltk-rs traits depend on some FLTK internal code
/// # Warning
/// fltk-rs traits are non-exhaustive,
/// to avoid future breakage if you try to implement them manually,
/// use the Deref and DerefMut pattern or the `widget_extends!` macro
pub unsafe trait MenuExt: WidgetExt {
    /// Get a menu item by name
    fn find_item(&self, name: &str) -> Option<crate::menu::MenuItem>;
    /// Set selected item
    fn set_item(&mut self, item: &crate::menu::MenuItem) -> bool;
    /// Find an item's index by its label
    fn find_index(&self, label: &str) -> i32;
    /// Return the text font
    fn text_font(&self) -> Font;
    /// Sets the text font
    fn set_text_font(&mut self, c: Font);
    /// Return the text size
    fn text_size(&self) -> i32;
    /// Sets the text size
    fn set_text_size(&mut self, c: i32);
    /// Return the text color
    fn text_color(&self) -> Color;
    /// Sets the text color
    fn set_text_color(&mut self, c: Color);
    /// Add a menu item along with its callback.
    /// The characters "&", "/", "\\", and "\_" (underscore) are treated as special characters in the label string. The "&" character specifies that the following character is an accelerator and will be underlined.
    /// The "\\" character is used to escape the next character in the string. Labels starting with the "\_" (underscore) character cause a divider to be placed after that menu item.
    /// Takes the menu item as a closure argument
    fn add<F: FnMut(&mut Self) + 'static>(
        &mut self,
        name: &str,
        shortcut: Shortcut,
        flag: crate::menu::MenuFlag,
        cb: F,
    ) -> i32
    where
        Self: Sized;
    /// Inserts a menu item at an index along with its callback.
    /// The characters "&", "/", "\\", and "\_" (underscore) are treated as special characters in the label string. The "&" character specifies that the following character is an accelerator and will be underlined.
    /// The "\\" character is used to escape the next character in the string. Labels starting with the "\_" (underscore) character cause a divider to be placed after that menu item.
    /// Takes the menu item as a closure argument
    fn insert<F: FnMut(&mut Self) + 'static>(
        &mut self,
        idx: i32,
        name: &str,
        shortcut: Shortcut,
        flag: crate::menu::MenuFlag,
        cb: F,
    ) -> i32
    where
        Self: Sized;
    /// Add a menu item along with an emit (sender and message).
    /// The characters "&", "/", "\\", and "\_" (underscore) are treated as special characters in the label string. The "&" character specifies that the following character is an accelerator and will be underlined.
    /// The "\\" character is used to escape the next character in the string. Labels starting with the "\_" (underscore) character cause a divider to be placed after that menu item.
    fn add_emit<T: 'static + Clone + Send + Sync>(
        &mut self,
        label: &str,
        shortcut: Shortcut,
        flag: crate::menu::MenuFlag,
        sender: crate::app::Sender<T>,
        msg: T,
    ) -> i32
    where
        Self: Sized;
    /// Inserts a menu item along with an emit (sender and message).
    /// The characters "&", "/", "\\", and "\_" (underscore) are treated as special characters in the label string. The "&" character specifies that the following character is an accelerator and will be underlined.
    /// The "\\" character is used to escape the next character in the string. Labels starting with the "\_" (underscore) character cause a divider to be placed after that menu item.
    fn insert_emit<T: 'static + Clone + Send + Sync>(
        &mut self,
        idx: i32,
        label: &str,
        shortcut: Shortcut,
        flag: crate::menu::MenuFlag,
        sender: crate::app::Sender<T>,
        msg: T,
    ) -> i32
    where
        Self: Sized;
    /// Remove a menu item by index
    fn remove(&mut self, idx: i32);
    /// Adds a simple text option to the Choice and `MenuButton` widgets.
    /// The characters "&", "/", "\\", "|", and "\_" (underscore) are treated as special characters in the label string. The "&" character specifies that the following character is an accelerator and will be underlined.
    /// The "\\" character is used to escape the next character in the string. Labels starting with the "\_" (underscore) character cause a divider to be placed after that menu item.
    fn add_choice(&mut self, text: &str);
    /// Gets the user choice from the Choice and `MenuButton` widgets
    fn choice(&self) -> Option<String>;
    /// Get index into menu of the last item chosen, returns -1 if no item was chosen
    fn value(&self) -> i32;
    /// Set index into menu of the last item chosen,return true if the new value is different than the old one
    fn set_value(&mut self, v: i32) -> bool;
    /// Clears the items in a menu, effectively deleting them.
    fn clear(&mut self);
    /// Clears a submenu by index
    /// # Errors
    /// Errors on failure to clear the submenu, failure returns an [`FltkErrorKind::FailedOperation`](`crate::prelude::FltkErrorKind::FailedOperation`)
    fn clear_submenu(&mut self, idx: i32) -> Result<(), FltkError>;
    /// Clears the items in a menu, effectively deleting them, and recursively force-cleans capturing callbacks
    /// # Safety
    /// Deletes `user_data` and any captured objects in the callback
    unsafe fn unsafe_clear(&mut self);
    #[doc(hidden)]
    /// Clears a submenu by index. Also recursively force-cleans capturing callbacks
    /// # Safety
    /// Deletes `user_data` and any captured objects in the callback, , failure returns an [`FltkErrorKind::FailedOperation`](`crate::prelude::FltkErrorKind::FailedOperation`)
    /// # Errors
    /// Errors on failure to clear the submenu
    unsafe fn unsafe_clear_submenu(&mut self, idx: i32) -> Result<(), FltkError>;
    /// Get the size of the menu widget
    fn size(&self) -> i32;
    /// Get the text label of the menu item at index idx
    fn text(&self, idx: i32) -> Option<String>;
    /// Get the menu item at an index
    fn at(&self, idx: i32) -> Option<crate::menu::MenuItem>;
    /// Get the mode of a menu item by index and flag
    fn mode(&self, idx: i32) -> crate::menu::MenuFlag;
    /// Set the mode of a menu item
    fn set_mode(&mut self, idx: i32, flag: crate::menu::MenuFlag);
    /// End the menu
    fn end(&mut self);
    /// Set the `down_box` of the widget
    fn set_down_frame(&mut self, f: FrameType);
    /// Get the down frame type of the widget
    fn down_frame(&self) -> FrameType;
    /// Make a menu globally accessible from any window
    fn global(&mut self);
    /// Get the menu element
    fn menu(&self) -> Option<crate::menu::MenuItem>;
    /// Set the menu element
    /// # Safety
    /// The MenuItem must be in a format recognized by FLTK (Null termination after submenus)
    unsafe fn set_menu(&mut self, item: crate::menu::MenuItem);
}

/// Defines the methods implemented by all valuator widgets
/// More details can be found in the [wiki](https://github.com/fltk-rs/fltk-rs/wiki/valuators).
/// # Safety
/// fltk-rs traits depend on some FLTK internal code
/// # Warning
/// fltk-rs traits are non-exhaustive,
/// to avoid future breakage if you try to implement them manually,
/// use the Deref and DerefMut pattern or the `widget_extends!` macro
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
    /// Set change step of a valuator.
    /// Rounds to multiples of a/b, or no rounding if a is zero
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
    /// # Errors
    /// Errors on failure to set the format of the widget
    fn format(&mut self, arg2: &str) -> Result<(), FltkError>;
    /// Round the valuator
    fn round(&self, arg2: f64) -> f64;
    /// Clamp the valuator
    fn clamp(&self, arg2: f64) -> f64;
    /// Increment the valuator
    fn increment(&mut self, arg2: f64, arg3: i32) -> f64;
}

/// Defines the methods implemented by `TextDisplay` and `TextEditor`
/// More details can be found in the [wiki](https://github.com/fltk-rs/fltk-rs/wiki/text).
/// # Safety
/// fltk-rs traits depend on some FLTK internal code
/// # Warning
/// fltk-rs traits are non-exhaustive,
/// to avoid future breakage if you try to implement them manually,
/// use the Deref and DerefMut pattern or the `widget_extends!` macro
pub unsafe trait DisplayExt: WidgetExt {
    /// Get the associated `TextBuffer`
    fn buffer(&self) -> Option<crate::text::TextBuffer>;
    /// Sets the associated `TextBuffer`
    fn set_buffer<B: Into<Option<crate::text::TextBuffer>>>(&mut self, buffer: B);
    /// Get the associated style `TextBuffer`
    fn style_buffer(&self) -> Option<crate::text::TextBuffer>;
    /// Return the text font
    fn text_font(&self) -> Font;
    /// Sets the text font
    fn set_text_font(&mut self, font: Font);
    /// Return the text color
    fn text_color(&self) -> Color;
    /// Sets the text color
    fn set_text_color(&mut self, color: Color);
    /// Return the text size
    fn text_size(&self) -> i32;
    /// Sets the text size
    fn set_text_size(&mut self, sz: i32);
    /// Scroll down the Display widget
    fn scroll(&mut self, top_line_num: i32, horiz_offset: i32);
    /// Insert into Display widget      
    fn insert(&self, text: &str);
    /// Set the insert position
    fn set_insert_position(&mut self, new_pos: i32);
    /// Return the insert position                
    fn insert_position(&self) -> i32;
    /// Gets the x and y positions of the cursor
    fn position_to_xy(&self, pos: i32) -> (i32, i32);
    /// Counts the lines from start to end                         
    fn count_lines(&self, start: i32, end: i32, is_line_start: bool) -> i32;
    /// Moves the cursor right
    /// # Errors
    /// Errors on failure to move the cursor
    fn move_right(&mut self) -> Result<(), FltkError>;
    /// Moves the cursor left
    /// # Errors
    /// Errors on failure to move the cursor
    fn move_left(&mut self) -> Result<(), FltkError>;
    /// Moves the cursor up
    /// # Errors
    /// Errors on failure to move the cursor
    fn move_up(&mut self) -> Result<(), FltkError>;
    /// Moves the cursor down
    /// # Errors
    /// Errors on failure to move the cursor
    fn move_down(&mut self) -> Result<(), FltkError>;
    /// Shows/hides the cursor
    fn show_cursor(&mut self, val: bool);
    /// Sets the style of the text widget
    fn set_highlight_data<B: Into<Option<crate::text::TextBuffer>>>(
        &mut self,
        style_buffer: B,
        entries: Vec<crate::text::StyleTableEntry>,
    );
    /// Sets the style of the text widget
    fn set_highlight_data_ext<B: Into<Option<crate::text::TextBuffer>>>(
        &mut self,
        style_buffer: B,
        entries: Vec<crate::text::StyleTableEntryExt>,
    );
    /// Unset the style of the text widget
    fn unset_highlight_data(&mut self, style_buffer: crate::text::TextBuffer);
    /// Sets the cursor style
    fn set_cursor_style(&mut self, style: crate::text::Cursor);
    /// Sets the cursor color
    fn set_cursor_color(&mut self, color: Color);
    /// Sets the scrollbar size in pixels
    fn set_scrollbar_size(&mut self, size: i32);
    /// Sets the scrollbar alignment
    fn set_scrollbar_align(&mut self, align: Align);
    /// Returns the cursor style
    fn cursor_style(&self) -> crate::text::Cursor;
    /// Returns the cursor color
    fn cursor_color(&self) -> Color;
    /// Returns the scrollbar size in pixels
    fn scrollbar_size(&self) -> i32;
    /// Returns the scrollbar alignment
    fn scrollbar_align(&self) -> Align;
    /// Returns the beginning of the line from the current position.
    /// Returns new position as index
    fn line_start(&self, pos: i32) -> i32;
    /// Returns the ending of the line from the current position.
    /// Returns new position as index
    fn line_end(&self, start_pos: i32, is_line_start: bool) -> i32;
    /// Skips lines from `start_pos`
    fn skip_lines(&mut self, start_pos: i32, lines: i32, is_line_start: bool) -> i32;
    /// Rewinds the lines
    fn rewind_lines(&mut self, start_pos: i32, lines: i32) -> i32;
    /// Goes to the next word
    fn next_word(&mut self);
    /// Goes to the previous word
    fn previous_word(&mut self);
    /// Returns the position of the start of the word, relative to the current position
    fn word_start(&self, pos: i32) -> i32;
    /// Returns the position of the end of the word, relative to the current position
    fn word_end(&self, pos: i32) -> i32;
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
    fn set_linenumber_size(&mut self, size: i32);
    /// Gets the linenumber size
    fn linenumber_size(&self) -> i32;
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
    /// Sets the wrap mode of the Display widget.
    /// If the wrap mode is `AtColumn`, wrap margin is the column.
    /// If the wrap mode is `AtPixel`, wrap margin is the pixel.
    /// For more [info](https://www.fltk.org/doc-1.4/classFl__Text__Display.html#ab9378d48b949f8fc7da04c6be4142c54)
    fn wrap_mode(&mut self, wrap: crate::text::WrapMode, wrap_margin: i32);
    /// Correct a column number based on an unconstrained position
    fn wrapped_column(&self, row: i32, column: i32) -> i32;
    /// Correct a row number from an unconstrained position
    fn wrapped_row(&self, row: i32) -> i32;
    /// Set the grammar underline color
    fn set_grammar_underline_color(&mut self, color: Color);
    /// Get the grammar underline color
    fn grammar_underline_color(&self) -> Color;
    /// Set the spelling underline color
    fn set_spelling_underline_color(&mut self, color: Color);
    /// Get the spelling underline color
    fn spelling_underline_color(&self) -> Color;
    /// Set the secondary selection color
    fn set_secondary_selection_color(&mut self, color: Color);
    /// Get the secondary selection color
    fn secondary_selection_color(&self) -> Color;
    /// Scrolls the text buffer to show the current insert position
    fn show_insert_position(&mut self);
}

/// Defines the methods implemented by all browser types
/// More info can be found in the [wiki](https://github.com/fltk-rs/fltk-rs/wiki/browsers)
/// # Safety
/// fltk-rs traits depend on some FLTK internal code
/// # Warning
/// fltk-rs traits are non-exhaustive,
/// to avoid future breakage if you try to implement them manually,
/// use the Deref and DerefMut pattern or the `widget_extends!` macro
pub unsafe trait BrowserExt: WidgetExt {
    /// Removes the specified line.
    /// Lines start at 1
    fn remove(&mut self, line: i32);
    /// Adds an item
    fn add(&mut self, item: &str);
    /// Adds an item with associated data
    fn add_with_data<T: Clone + 'static>(&mut self, item: &str, data: T);
    /// Inserts an item at an index.
    /// Lines start at 1
    fn insert(&mut self, line: i32, item: &str);
    /// Inserts an item at an index with associated data.
    /// Lines start at 1
    fn insert_with_data<T: Clone + 'static>(&mut self, line: i32, item: &str, data: T);
    /// Moves an item.
    /// Lines start at 1
    fn move_item(&mut self, to: i32, from: i32);
    /// Swaps 2 items.
    /// Lines start at 1
    fn swap(&mut self, a: i32, b: i32);
    /// Clears the browser widget
    fn clear(&mut self);
    /// Returns the number of items
    fn size(&self) -> i32;
    /// Select an item at the specified line.
    /// Lines start at 1
    fn select(&mut self, line: i32);
    /// Returns whether the item is selected
    /// Lines start at 1
    fn selected(&self, line: i32) -> bool;
    /// Returns the text of the item at `line`.
    /// Lines start at 1
    fn text(&self, line: i32) -> Option<String>;
    /// Returns the text of the selected item.
    /// Lines start at 1
    fn selected_text(&self) -> Option<String>;
    /// Sets the text of the selected item.
    /// Lines start at 1
    fn set_text(&mut self, line: i32, txt: &str);
    /// Load a file
    /// # Errors
    /// Errors on non-existent paths
    fn load<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), FltkError>;
    /// Return the text size
    fn text_size(&self) -> i32;
    /// Sets the text size.
    /// Lines start at 1
    fn set_text_size(&mut self, sz: i32);
    /// Sets the icon for browser elements.
    /// Lines start at 1
    fn set_icon<Img: ImageExt>(&mut self, line: i32, image: Option<Img>);
    /// Returns the icon of a browser element.
    /// Lines start at 1
    fn icon(&self, line: i32) -> Option<Box<dyn ImageExt>>;
    /// Removes the icon of a browser element.
    /// Lines start at 1
    fn remove_icon(&mut self, line: i32);
    /// Scrolls the browser so the top item in the browser is showing the specified line.
    /// Lines start at 1
    fn top_line(&mut self, line: i32);
    /// Scrolls the browser so the bottom item in the browser is showing the specified line.
    /// Lines start at 1
    fn bottom_line(&mut self, line: i32);
    /// Scrolls the browser so the middle item in the browser is showing the specified line.
    /// Lines start at 1
    fn middle_line(&mut self, line: i32);
    /// Gets the current format code prefix character, which by default is '\@'.
    /// More info [here](https://www.fltk.org/doc-1.3/classFl__Browser.html#a129dca59d64baf166503ba59341add69)
    fn format_char(&self) -> char;
    /// Sets the current format code prefix character to \p c. The default prefix is '\@'.
    /// c should be ascii
    fn set_format_char(&mut self, c: char);
    /// Gets the current column separator character. The default is '\t'
    fn column_char(&self) -> char;
    /// Sets the column separator to c. This will only have an effect if you also use `set_column_widths()`.
    /// c should be ascii
    fn set_column_char(&mut self, c: char);
    /// Gets the current column width array
    fn column_widths(&self) -> Vec<i32>;
    /// Sets the current column width array
    fn set_column_widths(&mut self, arr: &[i32]);
    /// Returns whether a certain line is displayed
    fn displayed(&self, line: i32) -> bool;
    /// Makes a specified line visible
    fn make_visible(&mut self, line: i32);
    /// Gets the vertical scroll position of the list as a pixel position
    fn position(&self) -> i32;
    /// Sets the vertical scroll position of the list as a pixel position
    fn set_position(&mut self, pos: i32);
    /// Gets the horizontal scroll position of the list as a pixel position
    fn hposition(&self) -> i32;
    /// Sets the horizontal scroll position of the list as a pixel position
    fn set_hposition(&mut self, pos: i32);
    /// Returns the type of scrollbar associated with the browser
    fn has_scrollbar(&self) -> crate::browser::BrowserScrollbar;
    /// Sets the type of scrollbar associated with the browser
    fn set_has_scrollbar(&mut self, mode: crate::browser::BrowserScrollbar);
    /// Gets the scrollbar size
    fn scrollbar_size(&self) -> i32;
    /// Sets the scrollbar size
    fn set_scrollbar_size(&mut self, new_size: i32);
    /// Sorts the items of the browser
    fn sort(&mut self);
    /// Returns the vertical scrollbar
    fn scrollbar(&self) -> crate::valuator::Scrollbar;
    /// Returns the horizontal scrollbar
    fn hscrollbar(&self) -> crate::valuator::Scrollbar;
    /// Returns the selected line, returns 0 if no line is selected
    fn value(&self) -> i32;
    /// Set the data associated with the line
    fn set_data<T: Clone + 'static>(&mut self, line: i32, data: T);
    /// Get the data associated with the line
    /// # Safety
    /// Type correctness is insured by the developer
    unsafe fn data<T: Clone + 'static>(&self, line: i32) -> Option<T>;
    /// Hides a the specified line
    fn hide_line(&mut self, line: i32);
}

/// Defines the methods implemented by table types.
/// More details can be found in the [wiki](https://github.com/fltk-rs/fltk-rs/wiki/tables).
/// # Safety
/// fltk-rs traits depend on some FLTK internal code
/// # Warning
/// fltk-rs traits are non-exhaustive,
/// to avoid future breakage if you try to implement them manually,
/// use the Deref and DerefMut pattern or the `widget_extends!` macro
pub unsafe trait TableExt: GroupExt {
    /// Clears the table
    fn clear(&mut self);
    /// Sets the table frame
    fn set_table_frame(&mut self, frame: FrameType);
    /// Gets the table frame
    fn table_frame(&self) -> FrameType;
    /// Sets the number of rows
    fn set_rows(&mut self, val: i32);
    /// Gets the number of rows
    fn rows(&self) -> i32;
    /// Sets the number of columns
    fn set_cols(&mut self, val: i32);
    /// Gets the number of columns
    fn cols(&self) -> i32;
    /// The range of row and column numbers for all visible and partially visible cells in the table.
    /// Returns (`row_top`, `col_left`, `row_bot`, `col_right`)
    fn visible_cells(&self) -> (i32, i32, i32, i32);
    /// The range of row and column numbers for all visible and partially visible cells in the table.
    /// Returns (`row_top`, `col_left`, `row_bot`, `col_right`)
    fn try_visible_cells(&self) -> Option<(i32, i32, i32, i32)>;
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
    fn col_resize_min(&self) -> i32;
    /// Sets the current column minimum resize value.
    fn set_col_resize_min(&mut self, val: i32);
    /// Returns the current row minimum resize value.
    fn row_resize_min(&self) -> i32;
    /// Sets the current row minimum resize value.
    fn set_row_resize_min(&mut self, val: i32);
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
    /// Sets the column's width
    fn set_col_width(&mut self, col: i32, width: i32);
    /// Gets the column's width
    fn col_width(&self, col: i32) -> i32;
    /// Sets all rows height
    fn set_row_height_all(&mut self, height: i32);
    /// Sets all column's width
    fn set_col_width_all(&mut self, width: i32);
    /// Sets the row's position
    fn set_row_position(&mut self, row: i32);
    /// Sets the column's position
    fn set_col_position(&mut self, col: i32);
    /// Gets the row's position
    fn row_position(&self) -> i32;
    /// Gets the column's position
    fn col_position(&self) -> i32;
    /// Sets the top row
    fn set_top_row(&mut self, row: i32);
    /// Gets the top row
    fn top_row(&self) -> i32;
    /// Returns whether a cell is selected
    fn is_selected(&self, r: i32, c: i32) -> bool;
    /// Gets the selection.
    /// Returns (`row_top`, `col_left`, `row_bot`, `col_right`).
    /// Returns -1 if no selection.
    fn get_selection(&self) -> (i32, i32, i32, i32);
    /// Tries to get the selection.
    /// Returns an Option((`row_top`, `col_left`, `row_bot`, `col_right`))
    fn try_get_selection(&self) -> Option<(i32, i32, i32, i32)>;
    /// Sets the selection
    fn set_selection(&mut self, row_top: i32, col_left: i32, row_bot: i32, col_right: i32);
    /// Unset selection
    fn unset_selection(&mut self);
    /// Moves the cursor with shift select
    /// # Errors
    /// Errors on failure to move the cursor
    fn move_cursor_with_shift_select(
        &mut self,
        r: i32,
        c: i32,
        shiftselect: bool,
    ) -> Result<(), FltkError>;
    /// Moves the cursor
    /// # Errors
    /// Errors on failure to move the cursor
    fn move_cursor(&mut self, r: i32, c: i32) -> Result<(), FltkError>;
    /// Returns the scrollbar size
    fn scrollbar_size(&self) -> i32;
    /// Sets the scrollbar size
    fn set_scrollbar_size(&mut self, new_size: i32);
    /// Sets whether tab key cell navigation is enabled
    fn set_tab_cell_nav(&mut self, val: bool);
    /// Returns whether tab key cell navigation is enabled
    fn tab_cell_nav(&self) -> bool;
    /// Override `draw_cell`.
    /// callback args: &mut self, `TableContext`, Row: i32, Column: i32, X: i32, Y: i32, Width: i32 and Height: i32.
    /// takes the widget as a closure argument
    fn draw_cell<
        F: FnMut(&mut Self, crate::table::TableContext, i32, i32, i32, i32, i32, i32) + 'static,
    >(
        &mut self,
        cb: F,
    );
    #[doc(hidden)]
    /// INTERNAL: Retrieve the draw cell data
    /// # Safety
    /// Can return multiple mutable references to the `draw_cell_data`
    unsafe fn draw_cell_data(&self) -> Option<Box<dyn FnMut()>>;
    /// Get the callback column, should be called from within a callback
    fn callback_col(&self) -> i32;
    /// Get the callback row, should be called from within a callback
    fn callback_row(&self) -> i32;
    /// Get the callback context, should be called from within a callback
    fn callback_context(&self) -> crate::table::TableContext;
    /// Returns the table's vertical scrollbar
    fn scrollbar(&self) -> crate::valuator::Scrollbar;
    /// Returns the table's horizontal scrollbar
    fn hscrollbar(&self) -> crate::valuator::Scrollbar;
    /// Find a cell's coords and size by row and column
    fn find_cell(
        &self,
        ctx: crate::table::TableContext,
        row: i32,
        col: i32,
    ) -> Option<(i32, i32, i32, i32)>;
    /// Get the cursor to row/col
    fn cursor2rowcol(
        &self,
    ) -> Option<(
        crate::table::TableContext,
        i32,
        i32,
        crate::table::TableResizeFlag,
    )>;
}

/// Defines the methods implemented by all image types
/// # Safety
/// fltk-rs traits depend on some FLTK internal code
/// # Warning
/// fltk-rs traits are non-exhaustive,
/// to avoid future breakage if you try to implement them manually,
/// use the Deref and DerefMut pattern
pub unsafe trait ImageExt {
    /// Performs a deep copy of the image
    fn copy(&self) -> Self
    where
        Self: Sized;
    /// Draws the image at the presupplied coordinates and size
    fn draw(&mut self, x: i32, y: i32, width: i32, height: i32);
    /// Draws the image at the presupplied coordinates and size and offset cx, cy
    fn draw_ext(&mut self, x: i32, y: i32, width: i32, height: i32, cx: i32, cy: i32);
    /// Return the width of the image
    fn width(&self) -> i32;
    /// Return the height of the image
    fn height(&self) -> i32;
    /// Return the width of the image
    fn w(&self) -> i32;
    /// Return the height of the image
    fn h(&self) -> i32;
    /// Returns a pointer of the image
    /// # Safety
    /// Can return multiple mutable pointers to the image
    unsafe fn as_image_ptr(&self) -> *mut fltk_sys::image::Fl_Image;
    /// Transforms a raw image pointer to an image
    /// # Safety
    /// The pointer must be valid
    unsafe fn from_image_ptr(ptr: *mut fltk_sys::image::Fl_Image) -> Self
    where
        Self: Sized;
    /// Returns the underlying raw rgb image data
    fn to_rgb_data(&self) -> Vec<u8>;
    /// Returns the underlying raw image data
    fn to_raw_data(&self) -> *const *const u8;
    /// Transforms the image into an `RgbImage`
    /// # Errors
    /// Errors on failure to transform to `RgbImage`
    fn to_rgb(&self) -> Result<crate::image::RgbImage, FltkError>;
    /// Transforms the image into an `RgbImage`
    /// # Errors
    /// Errors on failure to transform to `RgbImage`
    fn to_rgb_image(&self) -> Result<crate::image::RgbImage, FltkError>;
    /// Scales the image
    fn scale(&mut self, width: i32, height: i32, proportional: bool, can_expand: bool);
    /// Return the count of pointers in an image (Pixmaps have more than 1, bitmaps have 0, Rgb based images have 1)
    fn count(&self) -> i32;
    /// Gets the image's data width
    fn data_w(&self) -> i32;
    /// Gets the image's data height
    fn data_h(&self) -> i32;
    /// Gets the image's depth
    fn depth(&self) -> ColorDepth;
    /// Gets the image's line data size
    fn ld(&self) -> i32;
    /// Greys the image
    fn inactive(&mut self);
    /// Deletes the image
    /// # Safety
    /// An image shouldn't be deleted while it's being used by a widget
    unsafe fn delete(img: Self)
    where
        Self: Sized;
    /// Checks if the image was deleted
    fn was_deleted(&self) -> bool;
    #[doc(hidden)]
    /// INTERNAL: Manually increment the atomic refcount
    /// # Safety
    /// The underlying image pointer must be valid
    unsafe fn increment_arc(&mut self);
    #[doc(hidden)]
    /// INTERNAL: Manually decrement the atomic refcount
    /// # Safety
    /// The underlying image pointer must be valid
    unsafe fn decrement_arc(&mut self);
    /// Transforms an Image base into another Image
    /// # Safety
    /// Can be unsafe if used to downcast to an image of different format
    unsafe fn into_image<I: ImageExt>(self) -> I
    where
        Self: Sized;
}

/// Defines the methods implemented by all surface types, currently `ImageSurface`
pub trait SurfaceDevice {
    /// Checks whether this surface is the current surface
    fn is_current(&self) -> bool;
    /// Get the current surface
    fn surface() -> Self;
    /// Push a surface as a current surface
    fn push_current(new_current: &Self);
    /// Pop the current surface
    fn pop_current();
}

/// Defines a set of convenience functions for constructing and anchoring custom widgets.
/// Usage: fltk::widget_extends!(CustomWidget, BaseWidget, member);
/// It basically implements Deref and DerefMut on the custom widget, and adds the aforementioned methods.
#[macro_export]
macro_rules! widget_extends {
    ($widget:ty, $base:ty, $member:tt) => {
        impl $widget {
            /// Initialize to position x, y
            pub fn with_pos(mut self, x: i32, y: i32) -> Self {
                let w = self.w();
                let h = self.h();
                self.resize(x, y, w, h);
                self
            }

            /// Initialize to size width, height
            pub fn with_size(mut self, width: i32, height: i32) -> Self {
                let x = self.x();
                let y = self.y();
                let w = self.width();
                let h = self.height();
                if w == 0 || h == 0 {
                    self.widget_resize(x, y, width, height);
                } else {
                    self.resize(x, y, width, height);
                }
                self
            }

            /// Initialize with a label
            pub fn with_label(mut self, title: &str) -> Self {
                self.set_label(title);
                self
            }

            /// Initialize with alignment
            pub fn with_align(mut self, align: $crate::enums::Align) -> Self {
                self.set_align(align);
                self
            }

            /// Initialize with type
            pub fn with_type<T: $crate::prelude::WidgetType>(mut self, typ: T) -> Self {
                assert!(!self.was_deleted());
                self.set_type(typ);
                self
            }

            /// Initialize at bottom of another widget
            pub fn below_of<W: $crate::prelude::WidgetExt>(
                mut self,
                wid: &W,
                padding: i32,
            ) -> Self {
                assert!(!wid.was_deleted());
                assert!(!self.was_deleted());
                let w = self.w();
                let h = self.h();
                debug_assert!(
                    w != 0 && h != 0,
                    "below_of requires the size of the widget to be known!"
                );
                self.resize(wid.x(), wid.y() + wid.h() + padding, w, h);
                self
            }

            /// Initialize above of another widget
            pub fn above_of<W: $crate::prelude::WidgetExt>(
                mut self,
                wid: &W,
                padding: i32,
            ) -> Self {
                assert!(!wid.was_deleted());
                assert!(!self.was_deleted());
                let w = self.w();
                let h = self.h();
                debug_assert!(
                    w != 0 && h != 0,
                    "above_of requires the size of the widget to be known!"
                );
                self.resize(wid.x(), wid.y() - padding - h, w, h);
                self
            }

            /// Initialize right of another widget
            pub fn right_of<W: $crate::prelude::WidgetExt>(
                mut self,
                wid: &W,
                padding: i32,
            ) -> Self {
                assert!(!wid.was_deleted());
                assert!(!self.was_deleted());
                let w = self.w();
                let h = self.h();
                debug_assert!(
                    w != 0 && h != 0,
                    "right_of requires the size of the widget to be known!"
                );
                self.resize(wid.x() + wid.width() + padding, wid.y(), w, h);
                self
            }

            /// Initialize left of another widget
            pub fn left_of<W: $crate::prelude::WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
                assert!(!wid.was_deleted());
                assert!(!self.was_deleted());
                let w = self.w();
                let h = self.h();
                debug_assert!(
                    w != 0 && h != 0,
                    "left_of requires the size of the widget to be known!"
                );
                self.resize(wid.x() - w - padding, wid.y(), w, h);
                self
            }

            /// Initialize center of another widget
            pub fn center_of<W: $crate::prelude::WidgetExt>(mut self, w: &W) -> Self {
                assert!(!w.was_deleted());
                assert!(!self.was_deleted());
                debug_assert!(
                    w.width() != 0 && w.height() != 0,
                    "center_of requires the size of the widget to be known!"
                );
                let sw = self.width() as f64;
                let sh = self.height() as f64;
                let ww = w.width() as f64;
                let wh = w.height() as f64;
                let sx = (ww - sw) / 2.0;
                let sy = (wh - sh) / 2.0;
                let wx = if w.as_window().is_some() { 0 } else { w.x() };
                let wy = if w.as_window().is_some() { 0 } else { w.y() };
                self.resize(sx as i32 + wx, sy as i32 + wy, sw as i32, sh as i32);
                self.redraw();
                self
            }

            /// Initialize center of another widget on the x axis
            pub fn center_x<W: $crate::prelude::WidgetExt>(mut self, w: &W) -> Self {
                assert!(!w.was_deleted());
                assert!(!self.was_deleted());
                debug_assert!(
                    w.width() != 0 && w.height() != 0,
                    "center_of requires the size of the widget to be known!"
                );
                let sw = self.width() as f64;
                let sh = self.height() as f64;
                let ww = w.width() as f64;
                let sx = (ww - sw) / 2.0;
                let sy = self.y();
                let wx = if w.as_window().is_some() { 0 } else { w.x() };
                self.resize(sx as i32 + wx, sy, sw as i32, sh as i32);
                self.redraw();
                self
            }

            /// Initialize center of another widget on the y axis
            pub fn center_y<W: $crate::prelude::WidgetExt>(mut self, w: &W) -> Self {
                assert!(!w.was_deleted());
                assert!(!self.was_deleted());
                debug_assert!(
                    w.width() != 0 && w.height() != 0,
                    "center_of requires the size of the widget to be known!"
                );
                let sw = self.width() as f64;
                let sh = self.height() as f64;
                let wh = w.height() as f64;
                let sx = self.x();
                let sy = (wh - sh) / 2.0;
                let wy = if w.as_window().is_some() { 0 } else { w.y() };
                self.resize(sx, sy as i32 + wy, sw as i32, sh as i32);
                self.redraw();
                self
            }

            /// Initialize center of parent
            pub fn center_of_parent(mut self) -> Self {
                assert!(!self.was_deleted());
                if let Some(w) = self.parent() {
                    debug_assert!(
                        w.width() != 0 && w.height() != 0,
                        "center_of requires the size of the widget to be known!"
                    );
                    let sw = self.width() as f64;
                    let sh = self.height() as f64;
                    let ww = w.width() as f64;
                    let wh = w.height() as f64;
                    let sx = (ww - sw) / 2.0;
                    let sy = (wh - sh) / 2.0;
                    let wx = if w.as_window().is_some() { 0 } else { w.x() };
                    let wy = if w.as_window().is_some() { 0 } else { w.y() };
                    self.resize(sx as i32 + wx, sy as i32 + wy, sw as i32, sh as i32);
                    self.redraw();
                }
                self
            }

            /// Initialize to the size of another widget
            pub fn size_of<W: $crate::prelude::WidgetExt>(mut self, w: &W) -> Self {
                assert!(!w.was_deleted());
                assert!(!self.was_deleted());
                debug_assert!(
                    w.width() != 0 && w.height() != 0,
                    "size_of requires the size of the widget to be known!"
                );
                let x = self.x();
                let y = self.y();
                self.resize(x, y, w.width(), w.height());
                self
            }

            /// Initialize to the size of the parent
            pub fn size_of_parent(mut self) -> Self {
                assert!(!self.was_deleted());
                if let Some(parent) = self.parent() {
                    let w = parent.width();
                    let h = parent.height();
                    let x = self.x();
                    let y = self.y();
                    self.resize(x, y, w, h);
                }
                self
            }
        }

        impl std::ops::Deref for $widget {
            type Target = $base;

            fn deref(&self) -> &Self::Target {
                &self.$member
            }
        }

        impl std::ops::DerefMut for $widget {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$member
            }
        }
    };
}
