use crate::enums::{Align, CallbackTrigger, Color, Damage, Event, Font, FrameType, LabelType};
use crate::image::Image;
use crate::prelude::*;
use crate::utils::FlString;
use crate::widget::Widget;
use fltk_sys::group::*;
use std::{
    ffi::{CStr, CString},
    mem,
    ops::{Deref, DerefMut},
    os::raw,
};

/// Creates a widget group
#[derive(WidgetBase, WidgetExt, GroupExt, Debug)]
pub struct Group {
    inner: *mut Fl_Group,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl Group {
    /// Get the current group
    pub fn current() -> Box<dyn GroupExt> {
        unsafe {
            let ptr = Fl_Group_current();
            assert!(!ptr.is_null());
            Box::new(Group::from_widget_ptr(ptr as _))
        }
    }
}

/// Creates a widget pack
#[derive(WidgetBase, WidgetExt, GroupExt, Debug)]
pub struct Pack {
    inner: *mut Fl_Pack,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines pack types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum PackType {
    /// Vertical layout pack
    Vertical = 0,
    /// Horizontal layout pack
    Horizontal = 1,
}

/// Creates a scroll group
#[derive(WidgetBase, WidgetExt, GroupExt, Debug)]
pub struct Scroll {
    inner: *mut Fl_Scroll,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines Scroll types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum ScrollType {
    /// Never show bars
    None = 0,
    /// Show vertical bar
    Horizontal = 1,
    /// Show vertical bar
    Vertical = 2,
    /// Show both horizontal and vertical bars
    Both = 3,
    /// Always show bars
    AlwaysOn = 4,
    /// Show horizontal bar always
    HorizontalAlways = 5,
    /// Show vertical bar always
    VerticalAlways = 6,
    /// Always show both horizontal and vertical bars
    BothAlways = 7,
}

impl Scroll {
    /// Returns the vertical scrollbar
    pub fn scrollbar(&self) -> impl ValuatorExt {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Scroll_scrollbar(self.inner);
            assert!(!ptr.is_null());
            crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Returns the horizontal scrollbar
    pub fn hscrollbar(&self) -> impl ValuatorExt {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Scroll_hscrollbar(self.inner);
            assert!(!ptr.is_null());
            crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Returns the x position
    pub fn xposition(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_xposition(self.inner) as i32 }
    }

    /// Returns the y position
    pub fn yposition(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_yposition(self.inner) as i32 }
    }

    /// Scrolls from `from` to `to`
    pub fn scroll_to(&mut self, from: i32, to: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_scroll_to(self.inner, from as i32, to as i32) }
    }

    /// Gets the scrollbar size
    pub fn scrollbar_size(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_scrollbar_size(self.inner) as i32 }
    }

    /// Sets the scrollbar size
    pub fn set_scrollbar_size(&mut self, new_size: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_set_scrollbar_size(self.inner, new_size as i32) }
    }
}

/// Creates a tab which can contain widgets
#[derive(WidgetBase, WidgetExt, GroupExt, Debug)]
pub struct Tabs {
    inner: *mut Fl_Tabs,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl Tabs {
    /// Gets the currently visible group
    pub fn value(&mut self) -> Option<impl GroupExt> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Tabs_value(self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(Group::from_widget_ptr(
                    ptr as *mut fltk_sys::widget::Fl_Widget,
                ))
            }
        }
    }

    /// Sets the currently visible group
    /// # Errors
    /// Errors when the value can't be set for the group widget
    pub fn set_value<Grp: GroupExt>(&mut self, w: &Grp) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Tabs_set_value(
                self.inner,
                w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget,
            ) {
                0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                _ => Ok(()),
            }
        }
    }

    /// Returns the tab group for the tab the user has currently down-clicked
    pub fn push(&self) -> Option<impl GroupExt> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Tabs_push(self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(Group::from_widget_ptr(
                    ptr as *mut fltk_sys::widget::Fl_Widget,
                ))
            }
        }
    }

    /// This is called by the tab widget's handle() method to set the tab group widget the user last pushed
    /// # Errors
    /// Errors if `set_push` can't be set for the group widget
    pub fn set_push<Grp: GroupExt>(&mut self, w: &Grp) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Tabs_set_push(
                self.inner,
                w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget,
            ) {
                0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                _ => Ok(()),
            }
        }
    }

    /// Returns the position and size available to be used by its children
    pub fn client_area(&mut self) -> (i32, i32, i32, i32) {
        assert!(!self.was_deleted());
        unsafe {
            let mut i1 = 0;
            let mut i2 = 0;
            let mut i3 = 0;
            let mut i4 = 0;
            Fl_Tabs_client_area(self.inner, &mut i1, &mut i2, &mut i3, &mut i4);
            (i1, i2, i3, i4)
        }
    }

    /// Sets the tab label alignment
    pub fn set_tab_align(&mut self, a: Align) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tabs_set_tab_align(self.inner, a.bits() as i32) }
    }

    /// Gets the tab label alignment.
    pub fn tab_align(&self) -> Align {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tabs_tab_align(self.inner)) }
    }
}

/// Creates a tile which can contain widgets
#[derive(WidgetBase, WidgetExt, GroupExt, Debug)]
pub struct Tile {
    inner: *mut Fl_Tile,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a wizard widget
#[derive(WidgetBase, WidgetExt, GroupExt, Debug)]
pub struct Wizard {
    inner: *mut Fl_Wizard,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl Wizard {
    /// Gets the next view of the wizard
    pub fn next(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Wizard_next(self.inner) }
    }

    /// Gets the previous view of the wizard
    pub fn prev(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Wizard_prev(self.inner) }
    }

    /// Gets the underlying widget of the current view
    pub fn current_widget(&mut self) -> Box<dyn WidgetExt> {
        unsafe {
            assert!(!self.was_deleted());
            Box::new(Widget::from_widget_ptr(
                Fl_Wizard_value(self.inner) as *mut fltk_sys::widget::Fl_Widget
            ))
        }
    }

    /// Sets the underlying widget of the current view
    pub fn set_current_widget<W: WidgetExt>(&mut self, w: &W) {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Wizard_set_value(
                self.inner,
                w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget,
            )
        }
    }
}

/// Creates a color chooser widget
#[derive(WidgetBase, WidgetExt, GroupExt, Debug)]
pub struct ColorChooser {
    inner: *mut Fl_Color_Chooser,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl ColorChooser {
    /// Return the rgb color
    pub fn rgb_color(&self) -> (u8, u8, u8) {
        unsafe {
            assert!(!self.was_deleted());
            let r = (Fl_Color_Chooser_r(self.inner) * 255.0) as u8;
            let g = (Fl_Color_Chooser_g(self.inner) * 255.0) as u8;
            let b = (Fl_Color_Chooser_b(self.inner) * 255.0) as u8;
            (r, g, b)
        }
    }

    /// Return the hex color
    pub fn hex_color(&self) -> u32 {
        assert!(!self.was_deleted());
        let (r, g, b) = self.rgb_color();
        crate::utils::rgb2hex(r, g, b)
    }
}

impl Pack {
    /// Get the spacing of the pack
    pub fn spacing(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Pack_spacing(self.inner) }
    }

    /// Set the spacing of the pack
    pub fn set_spacing(&mut self, spacing: i32) {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Pack_set_spacing(self.inner, spacing);
        }
    }

    /// Layout the children of the pack automatically.
    /// Must be called on existing children
    pub fn auto_layout(&mut self) {
        let children = self.children() as i32;
        if children == 0 {
            return;
        }
        let spacing = self.spacing() * (children - 1);
        let t = self.get_type::<PackType>();
        let w = (self.width() - spacing) / children;
        let h = (self.height() - spacing) / children;

        for i in 0..children {
            let mut c = self.child(i as i32).unwrap();
            let c_w = c.width();
            let c_h = c.height();
            if t == PackType::Vertical {
                c.set_size(c_w, h);
            } else {
                c.set_size(w, c_h);
            }
        }
    }
}

/// Defines a Vertical Grid (custom widget).
/// Requires setting the params manually using the `set_params` method, which takes the rows, columns and spacing.
/// Requires explicit calls to add, which is overloaded especially for the layout.
/// ```rust,no_run
/// use fltk::{prelude::*, *};
/// let mut grid = group::VGrid::new(0, 0, 400, 300, "");
/// grid.set_params(3, 3, 5);
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// ```
#[derive(Debug, Clone)]
pub struct VGrid {
    vpack: Pack,
    rows: i32,
    cols: i32,
    current: i32,
}

impl VGrid {
    /// Creates a new vertical grid
    pub fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: T) -> VGrid {
        let vpack = Pack::new(x, y, w, h, label);
        vpack.end();
        VGrid {
            vpack,
            rows: 1,
            cols: 1,
            current: 0,
        }
    }

    /// Sets the params for the grid
    pub fn set_params(&mut self, rows: i32, cols: i32, spacing: i32) {
        self.vpack.set_spacing(spacing);
        let rows = if rows < 1 { 1 } else { rows };
        let cols = if cols < 1 { 1 } else { cols };
        self.rows = rows;
        self.cols = cols;
        for _ in 0..rows {
            let mut p = Pack::new(0, 0, self.vpack.width(), 0, "");
            p.set_type(PackType::Horizontal);
            p.set_spacing(spacing);
            p.end();
            self.vpack.add(&p);
        }
    }

    /// Adds widgets to the grid
    pub fn add<W: WidgetExt>(&mut self, w: &W) {
        let rem = self.current / self.cols;
        if rem < self.rows {
            let hpack = self.vpack.child(rem as i32).unwrap();
            let mut hpack = unsafe { Pack::from_widget_ptr(hpack.as_widget_ptr()) };
            hpack.end();
            hpack.add(w);
            hpack.auto_layout();
            self.vpack.auto_layout();
            self.current += 1;
        }
    }
}

impl Deref for VGrid {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.vpack
    }
}

impl DerefMut for VGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vpack
    }
}

/// Defines a Horizontal Grid (custom widget).
/// Requires setting the params manually using the `set_params` method, which takes the rows, columns and spacing.
/// Requires explicit calls to add, which is overloaded especially for the layout.
/// ```rust,no_run
/// use fltk::{prelude::*, *};
/// let mut grid = group::HGrid::new(0, 0, 400, 300, "");
/// grid.set_params(3, 3, 5);
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// grid.add(&button::Button::default());
/// ```
#[derive(Debug, Clone)]
pub struct HGrid {
    hpack: Pack,
    rows: i32,
    cols: i32,
    current: i32,
}

impl HGrid {
    /// Creates a new horizontal grid
    pub fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: T) -> HGrid {
        let mut hpack = Pack::new(x, y, w, h, label);
        hpack.set_type(PackType::Horizontal);
        hpack.end();
        HGrid {
            hpack,
            rows: 1,
            cols: 1,
            current: 0,
        }
    }

    /// Sets the params for the grid
    pub fn set_params(&mut self, rows: i32, cols: i32, spacing: i32) {
        self.hpack.set_spacing(spacing);
        let rows = if rows < 1 { 1 } else { rows };
        let cols = if cols < 1 { 1 } else { cols };
        self.rows = rows;
        self.cols = cols;
        for _ in 0..cols {
            let mut p = Pack::new(0, 0, 0, self.hpack.height(), "");
            p.set_spacing(spacing);
            p.end();
            self.hpack.add(&p);
        }
    }

    /// Adds widgets to the grid
    pub fn add<W: WidgetExt>(&mut self, w: &W) {
        let rem = self.current / self.rows;
        if rem < self.cols {
            let vpack = self.hpack.child(rem as i32).unwrap();
            let mut vpack = unsafe { Pack::from_widget_ptr(vpack.as_widget_ptr()) };
            vpack.end();
            vpack.add(w);
            vpack.auto_layout();
            self.hpack.auto_layout();
            self.current += 1;
        }
    }
}

impl Deref for HGrid {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.hpack
    }
}

impl DerefMut for HGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hpack
    }
}

/// A wrapper around a vertical pack, with `auto_layout`ing using the add method
pub struct Column {
    p: Pack,
}

impl Column {
    /// Default init a column filling the parent
    pub fn default() -> Self {
        let mut p = Pack::default().size_of_parent().center_of_parent();
        p.set_type(PackType::Vertical);
        p.end();
        Column { p }
    }
    /// Add a widget to the column with automatic layouting
    pub fn add<W: WidgetExt>(&mut self, w: &W) {
        self.p.add(w);
        self.p.auto_layout();
    }
}

impl Deref for Column {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.p
    }
}

impl DerefMut for Column {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.p
    }
}

/// A wrapper around a Horizontal pack, with `auto_layout`ing using the add method
pub struct Row {
    p: Pack,
}

impl Row {
    /// Default init a row filling the parent
    pub fn default() -> Self {
        let mut p = Pack::default().size_of_parent().center_of_parent();
        p.set_type(PackType::Horizontal);
        p.end();
        Row { p }
    }
    /// Add a widget to the row with automatic layouting
    pub fn add<W: WidgetExt>(&mut self, w: &W) {
        self.p.add(w);
        self.p.auto_layout();
    }
}

impl Deref for Row {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.p
    }
}

impl DerefMut for Row {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.p
    }
}
