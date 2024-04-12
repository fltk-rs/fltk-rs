use crate::enums::{Align, Color, FrameType};
use crate::prelude::*;
use crate::utils::FlString;
use crate::widget::Widget;
use fltk_sys::group::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw::c_char,
    sync::atomic::{AtomicBool, Ordering},
};

static DEBUG: AtomicBool = AtomicBool::new(false);

/// Creates a group widget
#[derive(Debug)]
pub struct Group {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Group, Fl_Group);
crate::macros::widget::impl_widget_base!(Group, Fl_Group);
crate::macros::widget::impl_widget_default!(Group);
crate::macros::group::impl_group_ext!(Group, Fl_Group);

impl Group {
    #[deprecated(since = "1.2.18", note = "please use `try_current` instead")]
    /// Get the current group
    pub fn current() -> Group {
        unsafe {
            let ptr = Fl_Group_current();
            assert!(!ptr.is_null());
            Group::from_widget_ptr(ptr as _)
        }
    }

    /// Tries to get the current group
    pub fn try_current() -> Option<Group> {
        unsafe {
            let ptr = Fl_Group_current();
            if ptr.is_null() {
                None
            } else {
                Some(Group::from_widget_ptr(ptr as _))
            }
        }
    }

    /// Sets the current GroupExt widget which will take children
    pub fn set_current(grp: Option<&impl GroupExt>) {
        unsafe {
            if let Some(grp) = grp {
                Fl_Group_set_current(grp.as_widget_ptr() as _)
            } else {
                Fl_Group_set_current(std::ptr::null_mut())
            }
        }
    }
}

/// Creates a widget pack
#[derive(Debug)]
pub struct Pack {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Pack, Fl_Pack);
crate::macros::widget::impl_widget_base!(Pack, Fl_Pack);
crate::macros::widget::impl_widget_default!(Pack);
crate::macros::group::impl_group_ext!(Pack, Fl_Pack);

/// Defines pack types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PackType {
    /// Vertical layout pack
    Vertical = 0,
    /// Horizontal layout pack
    Horizontal = 1,
}

crate::macros::widget::impl_widget_type!(PackType);

impl Pack {
    /// Get the spacing of the pack
    pub fn spacing(&self) -> i32 {
        unsafe { Fl_Pack_spacing(self.inner.widget() as _) }
    }

    /// Set the spacing of the pack
    pub fn set_spacing(&mut self, spacing: i32) {
        unsafe {
            Fl_Pack_set_spacing(self.inner.widget() as _, spacing);
        }
    }

    /// Layout the children of the pack automatically.
    /// Must be called on existing children
    pub fn auto_layout(&mut self) {
        let children = self.children();
        if children == 0 {
            return;
        }
        let spacing = self.spacing() * (children - 1);
        let t = self.get_type::<PackType>();
        let w = (self.width() - spacing) / children;
        let h = (self.height() - spacing) / children;

        for i in 0..children {
            let mut c = self.child(i).unwrap();
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

/// Creates a scroll group
#[derive(Debug)]
pub struct Scroll {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Scroll, Fl_Scroll);
crate::macros::widget::impl_widget_base!(Scroll, Fl_Scroll);
crate::macros::widget::impl_widget_default!(Scroll);
crate::macros::group::impl_group_ext!(Scroll, Fl_Scroll);

/// Defines Scroll types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

crate::macros::widget::impl_widget_type!(ScrollType);

impl Scroll {
    /// Returns the vertical scrollbar
    pub fn scrollbar(&self) -> crate::valuator::Scrollbar {
        unsafe {
            let ptr = Fl_Scroll_scrollbar(self.inner.widget() as _);
            assert!(!ptr.is_null());
            crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Returns the horizontal scrollbar
    pub fn hscrollbar(&self) -> crate::valuator::Scrollbar {
        unsafe {
            let ptr = Fl_Scroll_hscrollbar(self.inner.widget() as _);
            assert!(!ptr.is_null());
            crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Returns the x position
    pub fn xposition(&self) -> i32 {
        unsafe { Fl_Scroll_xposition(self.inner.widget() as _) }
    }

    /// Returns the y position
    pub fn yposition(&self) -> i32 {
        unsafe { Fl_Scroll_yposition(self.inner.widget() as _) }
    }

    /// Scrolls to `x` and `y`
    pub fn scroll_to(&mut self, x: i32, y: i32) {
        unsafe { Fl_Scroll_scroll_to(self.inner.widget() as _, x, y) }
    }

    /// Gets the scrollbar size
    pub fn scrollbar_size(&self) -> i32 {
        unsafe { Fl_Scroll_scrollbar_size(self.inner.widget() as _) }
    }

    /// Sets the scrollbar size
    pub fn set_scrollbar_size(&mut self, new_size: i32) {
        unsafe { Fl_Scroll_set_scrollbar_size(self.inner.widget() as _, new_size) }
    }
}

/// Defines how Tabs handle overflow
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TabsOverflow {
    /// Compress tabs
    Compress = 0,
    /// Clip tabs
    Clip,
    /// Create a pulldown
    Pulldown,
    /// Drag tabs
    Drag,
}

/// Creates a tab which can contain widgets
#[derive(Debug)]
pub struct Tabs {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Tabs, Fl_Tabs);
crate::macros::widget::impl_widget_base!(Tabs, Fl_Tabs);
crate::macros::widget::impl_widget_default!(Tabs);
crate::macros::group::impl_group_ext!(Tabs, Fl_Tabs);

impl Tabs {
    /// Gets the currently visible group
    pub fn value(&self) -> Option<impl GroupExt> {
        unsafe {
            let ptr = Fl_Tabs_value(self.inner.widget() as _);
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
        unsafe {
            match Fl_Tabs_set_value(
                self.inner.widget() as _,
                w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget,
            ) {
                0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                _ => Ok(()),
            }
        }
    }

    /// Returns the tab group for the tab the user has currently down-clicked
    pub fn push(&self) -> Option<impl GroupExt> {
        unsafe {
            let ptr = Fl_Tabs_push(self.inner.widget() as _);
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
        unsafe {
            match Fl_Tabs_set_push(
                self.inner.widget() as _,
                w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget,
            ) {
                0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                _ => Ok(()),
            }
        }
    }

    /// Returns the position and size available to be used by its children
    pub fn client_area(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut i1 = 0;
            let mut i2 = 0;
            let mut i3 = 0;
            let mut i4 = 0;
            Fl_Tabs_client_area(self.inner.widget() as _, &mut i1, &mut i2, &mut i3, &mut i4);
            (i1, i2, i3, i4)
        }
    }

    /// Sets the tab label alignment
    pub fn set_tab_align(&mut self, a: Align) {
        unsafe { Fl_Tabs_set_tab_align(self.inner.widget() as _, a.bits()) }
    }

    /// Gets the tab label alignment.
    pub fn tab_align(&self) -> Align {
        unsafe { mem::transmute(Fl_Tabs_tab_align(self.inner.widget() as _)) }
    }

    /// Auto layout a tabs widget
    pub fn auto_layout(&mut self) {
        for c in self.clone().into_iter() {
            if let Some(mut c) = c.as_group() {
                c.resize(self.x(), self.y() + 30, self.w(), self.h() - 30);
            }
        }
        self.resize_callback(|t, x, y, w, h| {
            for c in t.clone().into_iter() {
                if let Some(mut c) = c.as_group() {
                    c.resize(x, y + 30, w, h - 30);
                }
            }
        });
    }

    /// Sets how the Tabs handles overflow
    pub fn handle_overflow(&mut self, ov: TabsOverflow) {
        unsafe { Fl_Tabs_handle_overflow(self.inner.widget() as _, ov as i32) }
    }
}

/// Creates a tile which can contain widgets. For the tiling to work correctly, the children of a Tile must cover the entire area of the widget, but not overlap. This means that all children must touch each other at their edges, and no gaps can be left inside the Tile.
/// More info can be found [here](https://www.fltk.org/doc-1.4/classFl__Tile.html#details)
#[derive(Debug)]
pub struct Tile {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Tile, Fl_Tile);
crate::macros::widget::impl_widget_base!(Tile, Fl_Tile);
crate::macros::widget::impl_widget_default!(Tile);
crate::macros::group::impl_group_ext!(Tile, Fl_Tile);

impl Tile {
    /**
    Drags the intersection at (\p oldx,\p oldy) to (\p newx,\p newy).

    This redraws all the necessary children.

    If no size ranges are set, the new intersection position is limited to the
    size of the tile group. The resizable() option is not taken into account here.

    If size ranges are set, the actual new position of the intersection will
    depend on the size range of every individual child. No child will be smaller
    than their minw and minh. After the new position is found, move_intersection()
    will call init_sizes(). The resizable() range is ignored.

    \param[in] oldx, oldy move the intersection at this coordinate, pass zero to
        disable drag in that direction.
    \param[in] newx, newy move the intersection as close to this new coordinate
        as possible
    */
    pub fn move_intersection(&mut self, oldx: i32, oldy: i32, newx: i32, newy: i32) {
        unsafe {
            Fl_Tile_move_intersection(self.inner.widget() as _, oldx, oldy, newx, newy);
        }
    }

    /// Set the allowed size range for the child at the given index
    pub fn size_range_by_index(&mut self, idx: i32, minw: i32, minh: i32, maxw: i32, maxh: i32) {
        unsafe {
            Fl_Tile_size_range_by_index(self.inner.widget() as _, idx, minw, minh, maxw, maxh)
        }
    }

    /// Set the allowed size range for the given child widget
    pub fn size_range_by_child<W: WidgetExt>(
        &mut self,
        w: &W,
        minw: i32,
        minh: i32,
        maxw: i32,
        maxh: i32,
    ) {
        unsafe {
            Fl_Tile_size_range_by_child(
                self.inner.widget() as _,
                w.as_widget_ptr() as _,
                minw,
                minh,
                maxw,
                maxh,
            )
        }
    }
}

/// Creates a wizard widget
#[derive(Debug)]
pub struct Wizard {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Wizard, Fl_Wizard);
crate::macros::widget::impl_widget_base!(Wizard, Fl_Wizard);
crate::macros::widget::impl_widget_default!(Wizard);
crate::macros::group::impl_group_ext!(Wizard, Fl_Wizard);

impl Wizard {
    /// Gets the next view of the wizard
    pub fn next(&mut self) {
        unsafe { Fl_Wizard_next(self.inner.widget() as _) }
    }

    /// Gets the previous view of the wizard
    pub fn prev(&mut self) {
        unsafe { Fl_Wizard_prev(self.inner.widget() as _) }
    }

    #[deprecated(since = "1.2.18", note = "please use `try_current_widget` instead")]
    /// Gets the underlying widget of the current view
    pub fn current_widget(&self) -> Widget {
        unsafe {
            let ptr = Fl_Wizard_value(self.inner.widget() as _) as *mut fltk_sys::widget::Fl_Widget;
            assert!(!ptr.is_null());
            Widget::from_widget_ptr(ptr)
        }
    }

    /// Gets the underlying widget of the current view
    pub fn try_current_widget(&self) -> Option<impl WidgetExt> {
        unsafe {
            let ptr = Fl_Wizard_value(self.inner.widget() as _) as *mut fltk_sys::widget::Fl_Widget;
            if ptr.is_null() {
                None
            } else {
                Some(Widget::from_widget_ptr(ptr))
            }
        }
    }

    /// Sets the underlying widget of the current view
    pub fn set_current_widget<W: WidgetExt>(&mut self, w: &W) {
        unsafe {
            Fl_Wizard_set_value(
                self.inner.widget() as _,
                w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget,
            )
        }
    }
}

/// Creates a color chooser widget
#[derive(Debug)]
pub struct ColorChooser {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(ColorChooser, Fl_Color_Chooser);
crate::macros::widget::impl_widget_base!(ColorChooser, Fl_Color_Chooser);
crate::macros::widget::impl_widget_default!(ColorChooser);
crate::macros::group::impl_group_ext!(ColorChooser, Fl_Color_Chooser);

impl ColorChooser {
    /// Return the rgb color
    pub fn rgb_color(&self) -> (u8, u8, u8) {
        unsafe {
            let r = (Fl_Color_Chooser_r(self.inner.widget() as _) * 255.0) as u8;
            let g = (Fl_Color_Chooser_g(self.inner.widget() as _) * 255.0) as u8;
            let b = (Fl_Color_Chooser_b(self.inner.widget() as _) * 255.0) as u8;
            (r, g, b)
        }
    }

    /// Return the hex color
    pub fn hex_color(&self) -> u32 {
        let (r, g, b) = self.rgb_color();
        crate::utils::rgb2hex(r, g, b)
    }

    /// Set the base color of the ColorChooser. Returns an error on failure to change the color (wrong input)
    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) -> Result<(), FltkError> {
        unsafe {
            let ret = Fl_Color_Chooser_set_rgb(
                self.inner.widget() as _,
                r as f64 / 255.0,
                g as f64 / 255.0,
                b as f64 / 255.0,
            );
            if ret == 1 {
                Ok(())
            } else {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            }
        }
    }

    /// Set the base color of the ColorChooser. Returns an error on failure to change the color (wrong input)
    pub fn set_tuple_rgb(&mut self, (r, g, b): (u8, u8, u8)) -> Result<(), FltkError> {
        unsafe {
            let ret = Fl_Color_Chooser_set_rgb(
                self.inner.widget() as _,
                r as f64 / 255.0,
                g as f64 / 255.0,
                b as f64 / 255.0,
            );
            if ret == 1 {
                Ok(())
            } else {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            }
        }
    }
}

crate::macros::widget::impl_widget_type!(FlexType);

/// Defines Flex types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlexType {
    /// row direction
    Column = 0,
    /// column direction
    Row,
}

/**
    a Flexbox widget
    # Example
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn main() {
        let a = app::App::default();
        let mut win = window::Window::default().with_size(400, 300);
        let mut col = group::Flex::default().size_of_parent();
        col.set_type(group::FlexType::Column);
        let expanding = button::Button::default().with_label("Expanding");
        let mut normal = button::Button::default().with_label("Normal");
        col.set_size(&mut normal, 30);
        col.end();
        win.end();
        win.show();
        a.run().unwrap();
    }
    ```
*/
#[derive(Debug)]
pub struct Flex {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Flex, Fl_Flex);
crate::macros::widget::impl_widget_base!(Flex, Fl_Flex);
crate::macros::widget::impl_widget_default!(Flex);
crate::macros::group::impl_group_ext!(Flex, Fl_Flex);

impl Flex {
    /// Create a new Flex widget.
    /// This code is here for backward compatibility with initial Fl_Flex code, which defaulted to Row instead of Column.
    /// The behavior will be changed in fltk-rs version 2.
    fn new<T: Into<Option<&'static str>>>(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        title: T,
    ) -> Self {
        let mut f = <Flex as WidgetBase>::new(x, y, width, height, title).row();
        f.set_pad(5);
        f.debug_();
        f
    }
    /// Add a widget to the Flex box
    pub fn add<W: WidgetExt>(&mut self, widget: &W) {
        <Self as GroupExt>::add(self, widget);
        self.recalc();
    }

    /// Add a widget to the Flex box
    pub fn insert<W: WidgetExt>(&mut self, widget: &W, idx: i32) {
        <Self as GroupExt>::insert(self, widget, idx);
        self.recalc();
    }

    /// Set the size of the widget, same as `fixed` (before it was changed in FLTK 1.4)
    #[deprecated(since = "1.4.8", note = "please use `fixed` instead")]
    pub fn set_size<W: WidgetExt>(&mut self, w: &W, size: i32) {
        unsafe { Fl_Flex_set_size(self.inner.widget() as _, w.as_widget_ptr() as _, size) }
    }

    /// Set the size of the widget, same as `set_size`, but more inline with the new FLTK Fl_Flex api
    pub fn fixed<W: WidgetExt>(&mut self, w: &W, size: i32) {
        unsafe { Fl_Flex_set_size(self.inner.widget() as _, w.as_widget_ptr() as _, size) }
    }

    /// Debug the flex layout
    pub fn debug(flag: bool) {
        DEBUG.store(flag, Ordering::Release);
    }

    fn debug_(&mut self) {
        if DEBUG.load(Ordering::Relaxed) {
            self.set_frame(FrameType::BorderBox);
            if self.get_type::<FlexType>() == FlexType::Row {
                self.set_color(Color::from_rgb(200, 0, 0));
            } else {
                self.set_color(Color::from_rgb(0, 0, 200));
            }
        }
    }

    /// Set the type to be a column
    pub fn column(mut self) -> Self {
        self.set_type(FlexType::Column);
        self.debug_();
        self
    }

    /// Set the type to a row
    pub fn row(mut self) -> Self {
        self.set_type(FlexType::Row);
        self.debug_();
        self
    }

    /// Recalculate children's coords and sizes
    pub fn recalc(&self) {
        let mut s = self.clone();
        s.resize(self.x(), self.y(), self.w(), self.h());
        s.redraw();
    }

    /// Recalculate children's coords and sizes
    pub fn layout(&self) {
        self.recalc();
    }

    /// Set the margin
    pub fn set_margin(&mut self, m: i32) {
        unsafe { Fl_Flex_set_margin(self.inner.widget() as _, m) }
    }

    /// Get the margin
    pub fn margin(&self) -> i32 {
        unsafe { Fl_Flex_margin(self.inner.widget() as _) }
    }

    /// Set the padding
    pub fn set_pad(&mut self, p: i32) {
        unsafe { Fl_Flex_set_pad(self.inner.widget() as _, p) }
    }

    /// Get the padding
    pub fn pad(&self) -> i32 {
        unsafe { Fl_Flex_pad(self.inner.widget() as _) }
    }

    /// Set the padding
    pub fn set_spacing(&mut self, p: i32) {
        unsafe { Fl_Flex_set_pad(self.inner.widget() as _, p) }
    }

    /// Get the padding
    pub fn spacing(&self) -> i32 {
        unsafe { Fl_Flex_pad(self.inner.widget() as _) }
    }

    /// Set the margins
    pub fn set_margins(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe { Fl_Flex_set_margins(self.inner.widget() as _, left, top, right, bottom) }
    }

    /// Get the margins -> returns (left, top, right, bottom)
    pub fn margins(&self) -> (i32, i32, i32, i32) {
        let mut left = 0;
        let mut top = 0;
        let mut right = 0;
        let mut bottom = 0;
        unsafe {
            Fl_Flex_margins(
                self.inner.widget() as _,
                &mut left,
                &mut top,
                &mut right,
                &mut bottom,
            );
        }
        (left, top, right, bottom)
    }
}

/**
    Defines a Vertical Grid (custom widget).
    Requires setting the params manually using the `set_params` method, which takes the rows, columns and spacing.
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn main() {
        let app = app::App::default();
        let mut win = window::Window::default().with_size(400, 300);
        let mut grid = group::VGrid::new(0, 0, 400, 300, "");
        grid.set_params(3, 3, 5);
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        grid.end();
        win.end();
        win.show();
        app.run().unwrap();
    }
    ```
*/
#[derive(Debug, Clone)]
pub struct VGrid {
    vpack: Pack,
    rows: i32,
    cols: i32,
    current: i32,
}

impl Default for VGrid {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, None)
    }
}

impl VGrid {
    /// Constructs a widget with the size of its parent
    pub fn default_fill() -> Self {
        Self::default().size_of_parent()
    }

    /// Creates a new vertical grid
    pub fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: T) -> VGrid {
        let vpack = Pack::new(x, y, w, h, label);
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
        debug_assert!(self.current < self.rows * self.cols);
        let rem = (self.current - 1) / self.cols;
        if rem < self.rows {
            let hpack = self.vpack.child(rem).unwrap();
            let mut hpack = unsafe { Pack::from_widget_ptr(hpack.as_widget_ptr()) };
            hpack.end();
            hpack.add(w);
            hpack.auto_layout();
            self.vpack.auto_layout();
            self.current += 1;
        }
    }

    /// End the grid
    pub fn end(&mut self) {
        use std::collections::VecDeque;
        let children = self.vpack.children();
        self.current = children - self.rows;
        debug_assert!(self.current <= self.rows * self.cols);
        let mut v = VecDeque::new();
        for i in self.rows..children {
            let c = self.vpack.child(i).unwrap();
            v.push_back(c);
        }
        for i in 0..self.rows {
            let hpack = self.vpack.child(i).unwrap();
            let mut hpack = unsafe { Pack::from_widget_ptr(hpack.as_widget_ptr()) };
            hpack.end();
            for _j in 0..self.cols {
                if let Some(w) = v.pop_front() {
                    self.vpack.remove(&w);
                    hpack.add(&w);
                }
                hpack.auto_layout();
            }
        }
        self.vpack.auto_layout();
    }
}

crate::widget_extends!(VGrid, Pack, vpack);

/**
    Defines a Horizontal Grid (custom widget).
    Requires setting the params manually using the `set_params` method, which takes the rows, columns and spacing.
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn main() {
        let app = app::App::default();
        let mut win = window::Window::default().with_size(400, 300);
        let mut grid = group::HGrid::new(0, 0, 400, 300, "");
        grid.set_params(3, 3, 5);
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        grid.end();
        win.end();
        win.show();
        app.run().unwrap();
    }
    ```
*/
#[derive(Debug, Clone)]
pub struct HGrid {
    hpack: Pack,
    rows: i32,
    cols: i32,
    current: i32,
}

impl Default for HGrid {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, None)
    }
}

impl HGrid {
    /// Constructs a widget with the size of its parent
    pub fn default_fill() -> Self {
        Self::default().size_of_parent()
    }

    /// Creates a new horizontal grid
    pub fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: T) -> HGrid {
        let mut hpack = Pack::new(x, y, w, h, label);
        hpack.set_type(PackType::Horizontal);
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
        debug_assert!(self.current < self.rows * self.cols);
        let rem = (self.current - 1) / self.rows;
        if rem < self.cols {
            let vpack = self.hpack.child(rem).unwrap();
            let mut vpack = unsafe { Pack::from_widget_ptr(vpack.as_widget_ptr()) };
            vpack.end();
            vpack.add(w);
            vpack.auto_layout();
            self.hpack.auto_layout();
            self.current += 1;
        }
    }

    /// End the grid
    pub fn end(&mut self) {
        use std::collections::VecDeque;
        let children = self.hpack.children();
        self.current = children - self.cols;
        debug_assert!(self.current <= self.rows * self.cols);
        let mut v = VecDeque::new();
        for i in self.cols..children {
            let c = self.hpack.child(i).unwrap();
            v.push_back(c);
        }
        for i in 0..self.cols {
            let vpack = self.hpack.child(i).unwrap();
            let mut vpack = unsafe { Pack::from_widget_ptr(vpack.as_widget_ptr()) };
            vpack.end();
            for _j in 0..self.rows {
                if let Some(w) = v.pop_front() {
                    self.hpack.remove(&w);
                    vpack.add(&w);
                }
                vpack.auto_layout();
            }
        }
        self.hpack.auto_layout();
    }
}

crate::widget_extends!(HGrid, Pack, hpack);

/// A wrapper around a Flex column
#[derive(Debug, Clone)]
pub struct Column {
    p: Flex,
}

impl Default for Column {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, None)
    }
}

impl Column {
    /// Constructs a widget with the size of its parent
    pub fn default_fill() -> Self {
        Self::default().size_of_parent().center_of_parent()
    }

    /// Create a new column
    pub fn new<T: Into<Option<&'static str>>>(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        label: T,
    ) -> Column {
        let mut p = Flex::new(x, y, width, height, label);
        p.set_type(FlexType::Column);
        Column { p }
    }

    /// Add a widget to the column with automatic layouting
    pub fn add<W: WidgetExt>(&mut self, w: &W) {
        self.p.add(w);
    }
}

crate::widget_extends!(Column, Flex, p);

/// A wrapper around a Flex row
#[derive(Debug, Clone)]
pub struct Row {
    p: Flex,
}

impl Default for Row {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, None)
    }
}

impl Row {
    /// Constructs a widget with the size of its parent
    pub fn default_fill() -> Self {
        Self::default().size_of_parent().center_of_parent()
    }

    /// Create a new row
    pub fn new<T: Into<Option<&'static str>>>(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        label: T,
    ) -> Row {
        let mut p = Flex::new(x, y, width, height, label);
        p.set_type(FlexType::Row);
        Row { p }
    }

    /// Add a widget to the row with automatic layouting
    pub fn add<W: WidgetExt>(&mut self, w: &W) {
        self.p.add(w);
    }
}

crate::widget_extends!(Row, Flex, p);

/// Experimental Group widgets, the api might change.
/// # Warning
/// The api might change if changes happen upstream
pub mod experimental {
    use super::*;
    use crate::enums::Font;
    use std::ops::Range;

    /// Grid range
    pub struct GridRange {
        start: usize,
        end: usize,
    }

    impl GridRange {
        /// Check the length of the GridRange
        pub fn len(&self) -> usize {
            self.end - self.start
        }

        /// Check whether the GridRange is empty
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl From<Range<usize>> for GridRange {
        fn from(val: Range<usize>) -> Self {
            Self {
                start: val.start,
                end: val.end,
            }
        }
    }

    impl From<usize> for GridRange {
        fn from(val: usize) -> Self {
            (val..val + 1).into()
        }
    }

    bitflags::bitflags! {
        /// Defines alignment rules used by FLTK's Grid
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct GridAlign: u16 {
            /** Align the widget at the center of the cell. */
            const  CENTER          = 0x0000;
            /** Align the widget at the top of the cell. */
            const  TOP             = 0x0001;
            /** Align the widget at the bottom of the cell. */
            const  BOTTOM          = 0x0002;
            /** Align the widget at the left side of the cell. */
            const  LEFT            = 0x0004;
            /** Align the widget at the right side of the cell. */
            const  RIGHT           = 0x0008;
            /** Stretch the widget horizontally to fill the cell. */
            const  HORIZONTAL      = 0x0010;
            /** Stretch the widget vertically to fill the cell. */
            const  VERTICAL        = 0x0020;
            /** Stretch the widget in both directions to fill the cell. */
            const  FILL            = 0x0030;
            /** Stretch the widget proportionally. */
            const  PROPORTIONAL    = 0x0040;
            /** Align the widget at the top left of the cell. */
            const  TOP_LEFT        =  GridAlign::TOP.bits() |  GridAlign::LEFT.bits();
            /** Align the widget at the top right of the cell. */
            const  TOP_RIGHT       =  GridAlign::TOP.bits() |  GridAlign::RIGHT.bits();
            /** Align the widget at the bottom left of the cell. */
            const  BOTTOM_LEFT     =  GridAlign::BOTTOM.bits() |  GridAlign::LEFT.bits();
            /** Align the widget at the bottom right of the cell. */
            const  BOTTOM_RIGHT    =  GridAlign::BOTTOM.bits() |  GridAlign::RIGHT.bits();
        }
    }
    /// Fltk's grid widget
    #[derive(Debug)]
    pub struct Grid {
        inner: crate::widget::WidgetTracker,
        is_derived: bool,
    }

    crate::macros::widget::impl_widget_ext!(Grid, Fl_Grid);
    crate::macros::widget::impl_widget_base!(Grid, Fl_Grid);
    crate::macros::widget::impl_widget_default!(Grid);
    crate::macros::group::impl_group_ext!(Grid, Fl_Grid);

    impl Grid {
        /// Set the layout of the grid, along with the margin and gap
        pub fn set_layout_ext(&mut self, rows: i32, cols: i32, margin: i32, gap: i32) {
            unsafe { Fl_Grid_set_layout(self.inner.widget() as _, rows, cols, margin, gap) }
        }
        /// Set the layout of the grid
        pub fn set_layout(&mut self, rows: i32, cols: i32) {
            unsafe { Fl_Grid_set_layout(self.inner.widget() as _, rows, cols, -1, -1) }
        }
        /// Layout the grid
        pub fn layout(&mut self) {
            unsafe { Fl_Grid_layout(self.inner.widget() as _) }
        }
        /// Clear the layout
        pub fn clear_layout(&mut self) {
            unsafe { Fl_Grid_clear_layout(self.inner.widget() as _) }
        }
        /// Set whether the Grid needs layout
        pub fn set_need_layout(&mut self, set: bool) {
            unsafe { Fl_Grid_set_need_layout(self.inner.widget() as _, set as _) }
        }
        /// Get whether the Grid needs layout
        pub fn need_layout(&self) -> bool {
            unsafe { Fl_Grid_need_layout(self.inner.widget() as _) != 0 }
        }
        /// Set the grid's margin
        pub fn set_margin(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
            unsafe { Fl_Grid_set_margin(self.inner.widget() as _, left, top, right, bottom) }
        }
        /// Set the grid's gap
        pub fn set_gap(&mut self, row_gap: i32, col_gap: i32) {
            unsafe { Fl_Grid_set_gap(self.inner.widget() as _, row_gap, col_gap) }
        }
        #[allow(dead_code)]
        /// Set the widget at row/column and alignment
        pub fn set_widget_<W: WidgetExt>(
            &mut self,
            wi: &mut W,
            row: i32,
            col: i32,
            align: GridAlign,
        ) -> *mut () {
            unsafe {
                Fl_Grid_set_widget(
                    self.inner.widget() as _,
                    wi.as_widget_ptr() as _,
                    row,
                    col,
                    align.bits(),
                ) as _
            }
        }
        /// Set the widget at row/column using ranges
        pub fn set_widget<W: 'static + Clone + WidgetExt>(
            &mut self,
            widget: &mut W,
            row: impl Into<GridRange>,
            col: impl Into<GridRange>,
        ) -> Result<(), FltkError> {
            let row = row.into();
            let col = col.into();
            self.set_widget_ext(widget, row, col, GridAlign::FILL)
        }
        /// Set the widget at row/column along with row span and column span and alignment
        fn set_widget_ext_<W: WidgetExt>(
            &mut self,
            wi: &mut W,
            row: i32,
            col: i32,
            rowspan: i32,
            colspan: i32,
            align: GridAlign,
        ) -> *mut () {
            unsafe {
                Fl_Grid_set_widget_ext(
                    self.inner.widget() as _,
                    wi.as_widget_ptr() as _,
                    row,
                    col,
                    rowspan,
                    colspan,
                    align.bits(),
                ) as _
            }
        }
        /// Set the widget at row/column using ranges along with the alignment
        pub fn set_widget_ext<W: 'static + Clone + WidgetExt>(
            &mut self,
            widget: &mut W,
            row: impl Into<GridRange>,
            col: impl Into<GridRange>,
            align: GridAlign,
        ) -> Result<(), FltkError> {
            let row = row.into();
            let col = col.into();
            let e = self.set_widget_ext_(
                widget,
                row.start as _,
                col.start as _,
                row.len() as _,
                col.len() as _,
                align,
            );
            if e.is_null() {
                Err(FltkError::Internal(FltkErrorKind::FailedGridSetWidget))
            } else {
                Ok(())
            }
        }
        /// Set the column width
        pub fn set_col_width(&mut self, col: i32, value: i32) {
            unsafe { Fl_Grid_set_col_width(self.inner.widget() as _, col, value) }
        }
        /// Set the column weight
        pub fn set_col_weight(&mut self, col: i32, value: i32) {
            unsafe { Fl_Grid_set_col_weight(self.inner.widget() as _, col, value) }
        }
        /// Set the column gap
        pub fn set_col_gap(&mut self, col: i32, value: i32) {
            unsafe { Fl_Grid_set_col_gap(self.inner.widget() as _, col, value) }
        }
        /// Set the row height
        pub fn set_row_height(&mut self, row: i32, value: i32) {
            unsafe { Fl_Grid_set_row_height(self.inner.widget() as _, row, value) }
        }
        /// Set the row weight
        pub fn set_row_weight(&mut self, row: i32, value: i32) {
            unsafe { Fl_Grid_set_row_weight(self.inner.widget() as _, row, value) }
        }
        /// Set the row gap
        pub fn set_row_gap(&mut self, row: i32, value: i32) {
            unsafe { Fl_Grid_set_row_gap(self.inner.widget() as _, row, value) }
        }
        /// Show the grid
        pub fn show_grid(&mut self, set: bool) {
            unsafe { Fl_Grid_show_grid(self.inner.widget() as _, set as _) }
        }
        /// Show the grid with a certain color
        pub fn show_grid_with_color(&mut self, set: bool, col: Color) {
            unsafe { Fl_Grid_show_grid_with_color(self.inner.widget() as _, set as _, col.bits()) }
        }
        /// Debug the grid
        pub fn debug(&mut self, level: i32) {
            unsafe { Fl_Grid_debug(self.inner.widget() as _, level) }
        }
    }

    /// Creates a scrollable display widget to handle terminal-like behavior, such as
    /// logging events or debug information.
    /// Replaces SimpleTerminal widget
    ///
    #[derive(Debug)]
    pub struct Terminal {
        inner: crate::widget::WidgetTracker,
        is_derived: bool,
    }

    crate::macros::widget::impl_widget_ext!(Terminal, Fl_Terminal);
    crate::macros::widget::impl_widget_base!(Terminal, Fl_Terminal);
    crate::macros::widget::impl_widget_default!(Terminal);
    crate::macros::group::impl_group_ext!(Terminal, Fl_Terminal);

    ///   Determines when Fl_Terminal calls redraw() if new text is added.
    /// RATE_LIMITED is the recommended setting, using redraw_rate(float) to determine
    /// the maximum rate of redraws.
    /// see redraw_style(), redraw_rate()
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct RedrawStyle {
        bits: u32,
    }

    impl RedrawStyle {
        /// App must call redraw() as needed to update text to screen
        pub const NoRedraw: RedrawStyle = RedrawStyle { bits: 0x0000 };
        /// timer controlled redraws. (DEFAULT)
        pub const RateLimited: RedrawStyle = RedrawStyle { bits: 0x0001 };
        /// redraw triggered after *every* append() / printf() / etc. operation
        pub const PerWrite: RedrawStyle = RedrawStyle { bits: 0x0002 };

        /// Gets the inner representation
        pub const fn bits(&self) -> u32 {
            self.bits
        }
        /// Build a RedrawStyle enum with an arbitrary value.
        pub const fn new(val: u32) -> Self {
            RedrawStyle { bits: val }
        }
    }

    bitflags::bitflags! {
        /// Bits for the per-character attributes, which control text features
        /// such as italic, bold, underlined text, etc.
        /// Can be combined with | operator
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct Attrib: u8 {
            /// all attributes off
            const Normal =  0x00 ;
            /// bold text: uses bold font, color brighter than normal
            const Bold = 0x01 ;
            /// dim text; color slightly darker than normal
            const Dim =  0x02 ;
            /// italic font text
            const Italic =  0x04 ;
            /// underlined text
            const Underline =  0x08 ;
            /// <EM>(reserved for internal future use)</EM>
            const _Reserved1 =   0x10 ;
            /// inverse text; fg/bg color are swapped
            const Inverse =   0x20 ;
            /// <EM>(reserved for internal future use)</EM>
            const _Reserved2 = 0x40 ;
            /// strikeout text
            const Strikeout = 0x80 ;
        }
    }

    bitflags::bitflags! {
        /// Output translation flags for special control character translations.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct OutFlags: u8 {
            ///< no output translation
            const OFF        = 0x00;
            ///< carriage return generates a vertical line-feed (\\r -> \\n)
            const CR_TO_LF   = 0x01;
            ///< line-feed generates a carriage return (\\n -> \\r)
            const LF_TO_CR   = 0x02;
            ///< line-feed generates a carriage return line-feed (\\n -> \\r\\n)
            const LF_TO_CRLF = 0x04;
        }
    }

    ///    'xterm color' values, used in set_text_fg_color_xterm and set_text_bg_color_xterm
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    #[allow(missing_docs)] // These color names are self-documenting
    #[non_exhaustive]
    pub enum XtermColor {
        Black = 0,
        Red = 1,
        Green = 2,
        Yellow = 3,
        Blue = 4,
        Magenta = 5,
        Cyan = 6,
        White = 7,
    }

    bitflags::bitflags! {
        /// Per-character 8 bit flags (u8) used to manage special states for characters.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct CharFlags: u8 {
            /// No flags
            const NONE   = 0x00;
            /// this char's fg color is an XTERM color; can be affected by Dim+Bold
            const FG_XTERM   = 0x01;
            /// this char's bg color is an XTERM color; can be affected by Dim+Bold
            const BG_XTERM   = 0x02;
            /// used internally for line re-wrap during screen resizing
            const _EOL        = 0x04;
            /// Reserved
            const _RESV_A     = 0x08;
            /// Reserved
            const _RESV_B     = 0x10;
            /// Reserved
            const _RESV_C     = 0x20;
            /// Reserved
            const _RESV_D     = 0x40;
            /// Reserved
            const _RESV_E     = 0x80;
        }
    }

    ///    Class to manage the terminal's individual UTF-8 characters.
    ///    Includes fg/bg color, attributes (BOLD, UNDERLINE..)
    /// *This is a low-level "protected" class in the fltk library*
    pub struct Utf8Char {
        inner: *const Fl_Terminal_Utf8Char, // This points to a C++ Fl_Terminal::Utf8Char structure
    }

    impl std::fmt::Debug for Utf8Char {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let x = self.text_utf8();
            write!(
                f,
                "Utf8Char {:?} '{}'  fg:{} bg:{} {:?}",
                x,
                std::str::from_utf8(x).unwrap(),
                self.fgcolor(),
                self.bgcolor(),
                self.attrib()
            )
        }
    }

    ///    Class to read characters from the terminal's buffer rows.
    ///    Includes indexing access and iterators
    ///    *This is a low-level "protected" class*
    pub struct BuffRow<'a> {
        inner: *const Fl_Terminal_Utf8Char, // This points to an array of Fl_Terminal::Utf8Char
        /// Parent terminal widget that owns this buffer
        _parent: &'a Terminal,
        /// Number of characters in the row
        pub length: usize,
        /// sizeof(Fl_Terminal::Utf8Char)
        pub char_size: usize,
    }

    impl Terminal {
        /// Returns whether the terminal is in ANSI mode.
        pub fn ansi(&self) -> bool {
            unsafe { Fl_Terminal_ansi(self.inner.widget() as _) != 0 }
        }

        /// Enable/disable ANSI mode. If true, ANSI and VT100/xterm codes will be processed.
        /// If false, these codes won't be processed and will either be ignored or print the
        /// error character "¿", depending on the value of show_unknown().
        pub fn set_ansi(&mut self, arg1: bool) {
            unsafe { Fl_Terminal_set_ansi(self.inner.widget() as _, arg1 as i32) }
        }

        /// Appends text to the terminal at current cursor position using the current text color/attributes.
        /// Redraws are managed automatically by default; see redraw_style()
        pub fn append(&mut self, s: &str) {
            let raw_s = CString::safe_new(s).into_raw();
            unsafe {
                Fl_Terminal_append(self.inner.widget() as _, raw_s as _);
                // Take ownership of raw_s back so it will be dropped
                let _raw_s = CString::from_raw(raw_s);
            }
        }

        /// Appends data to the terminal at current cursor position using the current text color/attributes
        /// Redraws are managed automatically by default; see redraw_style()
        pub fn append_u8(&mut self, s: &[u8]) {
            unsafe {
                Fl_Terminal_append_u8(self.inner.widget() as _, s.as_ptr() as _, s.len() as _)
            }
        }

        /// Appends text to the terminal at current cursor position using the current text color/attributes.
        /// Slightly more efficient than append_utf8
        /// Redraws are managed automatically by default; see redraw_style()
        pub fn append_ascii(&mut self, s: &str) {
            let raw_s = CString::safe_new(s).into_raw();
            unsafe {
                Fl_Terminal_append_ascii(self.inner.widget() as _, raw_s as _);
                // Take ownership of raw_s back so it will be dropped
                let _raw_s = CString::from_raw(raw_s);
            }
        }

        /// Appends text to the terminal at current cursor position using the current text color/attributes.
        /// Handles UTF-8 chars split across calls
        /// Redraws are managed automatically by default; see redraw_style()
        pub fn append_utf8(&mut self, s: &str) {
            let raw_s = CString::safe_new(s).into_raw();
            unsafe {
                Fl_Terminal_append_utf8(self.inner.widget() as _, raw_s as _);
                // Take ownership of raw_s back so it will be dropped
                let _raw_s = CString::from_raw(raw_s);
            }
        }

        /// Appends data to the terminal at current cursor position using the current text color/attributes
        /// Handles UTF-8 chars split across calls
        /// Redraws are managed automatically by default; see redraw_style()
        pub fn append_utf8_u8(&mut self, s: &[u8]) {
            unsafe {
                Fl_Terminal_append_utf8_u8(self.inner.widget() as _, s.as_ptr() as _, s.len() as _)
            }
        }

        /// Clears the screen to the current `textbgcolor()`, and homes the cursor.
        pub fn clear(&mut self) {
            unsafe { Fl_Terminal_clear(self.inner.widget() as _) }
        }

        /// Clear any current mouse selection.
        pub fn clear_mouse_selection(&mut self) {
            unsafe { Fl_Terminal_clear_mouse_selection(self.inner.widget() as _) }
        }

        ///  Clears the screen to a specific color `val` and homes the cursor.
        /// Does not affect the value of text_bg_color or text_bg_color_default
        pub fn clear_to_color(&mut self, val: Color) {
            unsafe { Fl_Terminal_clear_to_color(self.inner.widget() as _, val.bits()) }
        }

        ///   Clear the terminal screen only; does not affect the cursor position.
        ///
        /// Also clears the current mouse selection.
        ///
        /// If `scroll_to_hist` is true, the screen is cleared by scrolling the
        /// contents into the scrollback history, where it can be retrieved with the
        /// scrollbar. If false, the screen is cleared
        /// and the scrollback history is unchanged.
        ///
        /// Similar to the escape sequence `\<ESC\>[2J`.
        pub fn clear_screen(&mut self, arg1: bool) {
            unsafe { Fl_Terminal_clear_screen(self.inner.widget() as _, arg1 as i32) }
        }

        ///   Clear the terminal screen and home the cursor
        ///
        /// Also clears the current mouse selection.
        ///
        /// If `scroll_to_hist` is true, the screen is cleared by scrolling the
        /// contents into the scrollback history, where it can be retrieved with the
        /// scrollbar. If false, the screen is cleared
        /// and the scrollback history is unchanged.
        ///
        /// Similar to the escape sequence `\<ESC\>[2J\<ESC\>[H`.
        pub fn clear_screen_home(&mut self, arg1: bool) {
            unsafe { Fl_Terminal_clear_screen_home(self.inner.widget() as _, arg1 as i32) }
        }

        /// Clears the scroll history buffer and adjusts scrollbar, forcing it to redraw()
        pub fn clear_history(&mut self) {
            unsafe { Fl_Terminal_clear_history(self.inner.widget() as _) }
        }

        /// Get the background color for the terminal's Fl_Group::box().
        pub fn color(&self) -> Color {
            Color::from_rgbi(unsafe { Fl_Terminal_color(self.inner.widget() as _) })
        }

        /// Sets the background color for the terminal's Fl_Group::box().
        ///
        /// If the textbgcolor() and textbgcolor_default() are set to the special
        /// "see through" color 0xffffffff when any text was added, changing color()
        /// affects the color that shows through behind that existing text.
        ///
        /// Otherwise, whatever specific background color was set for existing text will
        ///  persist after changing color().
        ///
        /// To see the effects of a change to color(), follow up with a call to redraw().
        ///
        /// The default value is 0x0.
        pub fn set_color(&mut self, color: Color) {
            unsafe { Fl_Terminal_set_color(self.inner.widget() as _, color.bits()) }
        }

        /// Return the cursor's current column position on the screen.
        pub fn cursor_col(&self) -> i32 {
            unsafe { Fl_Terminal_cursor_col(self.inner.widget() as _) }
        }

        /// Set the cursor's current column position on the screen.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn set_cursor_col(&mut self, val: i32) {
            unsafe { Fl_Terminal_set_cursor_col(self.inner.widget() as _, val) }
        }

        /// Return the cursor's current row position on the screen.
        pub fn cursor_row(&self) -> i32 {
            unsafe { Fl_Terminal_cursor_row(self.inner.widget() as _) }
        }

        /// Set the cursor's current row position on the screen.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn set_cursor_row(&mut self, val: i32) {
            unsafe { Fl_Terminal_set_cursor_row(self.inner.widget() as _, val) }
        }

        /// Moves cursor up `count` lines.
        ///  If cursor hits screen top, it either stops (does not wrap) if `do_scroll`
        ///  is false, or scrolls down if `do_scroll` is true.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn cursor_up(&mut self, count: i32, do_scroll: bool) {
            unsafe { Fl_Terminal_cursor_up(self.inner.widget() as _, count, do_scroll as i32) }
        }

        /// Moves cursor down `count` lines.
        ///  If cursor hits screen bottom, it either stops (does not wrap) if `do_scroll`
        ///  is false, or wraps and scrolls up if `do_scroll` is true.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn cursor_down(&mut self, count: i32, do_scroll: bool) {
            unsafe { Fl_Terminal_cursor_down(self.inner.widget() as _, count, do_scroll as i32) }
        }

        /// Moves cursor left `count` columns, and cursor stops (does not wrap) if it hits screen edge.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn cursor_left(&mut self, count: i32) {
            unsafe { Fl_Terminal_cursor_left(self.inner.widget() as _, count) }
        }

        /// Moves cursor right `count` columns. If cursor hits right edge of screen,
        ///  it either stops (does not wrap) if `do_scroll` is false, or wraps and
        ///  scrolls up one line if `do_scroll` is true.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn cursor_right(&mut self, count: i32, do_scroll: bool) {
            unsafe { Fl_Terminal_cursor_right(self.inner.widget() as _, count, do_scroll as i32) }
        }

        /// Scroll the selection up(+)/down(-) number of rows
        /// *This is a low-level "protected" function of the fltk library*
        pub fn scroll(&mut self, count: i32) {
            unsafe { Fl_Terminal_scroll(self.inner.widget() as _, count) }
        }

        /// Clear from cursor to End Of Display (EOD), like "`<ESC>[J<ESC>[0J`".
        pub fn clear_eod(&mut self) {
            unsafe { Fl_Terminal_clear_eod(self.inner.widget() as _) }
        }

        /// Clear from cursor to End Of Line (EOL), like "`<ESC>[K`".
        pub fn clear_eol(&mut self) {
            unsafe { Fl_Terminal_clear_eol(self.inner.widget() as _) }
        }

        /// Clear entire line cursor is currently on.
        pub fn clear_cur_line(&mut self) {
            unsafe { Fl_Terminal_clear_cur_line(self.inner.widget() as _) }
        }

        /// Clear entire line for specified row.
        pub fn clear_line(&mut self, drow: i32) {
            unsafe { Fl_Terminal_clear_line(self.inner.widget() as _, drow) }
        }

        /// Clear from cursor to Start Of Display (EOD), like "`<ESC>[1J`".
        pub fn clear_sod(&mut self) {
            unsafe { Fl_Terminal_clear_sod(self.inner.widget() as _) }
        }

        /// Clear from cursor to Start Of Line (SOL), like "`<ESC>[1K`".
        pub fn clear_sol(&mut self) {
            unsafe { Fl_Terminal_clear_sol(self.inner.widget() as _) }
        }

        ///   Insert char `c` at the current cursor position for `rep`` times.
        ///   Works only for single-byte characters, `c` can't be multi-byte UTF-8.
        ///   Does not wrap; characters at end of line are lost.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn insert_char(&mut self, c: char, rep: i32) {
            let c = if c.len_utf8() > 1 { b' ' } else { c as u8 };
            unsafe { Fl_Terminal_insert_char(self.inner.widget() as _, c as c_char, rep) }
        }

        /// Insert char `c` for `rep` times at display row `drow` and column `dcol`.
        ///   Works only for single-byte characters, `c` can't be multi-byte UTF-8.
        ///   Does not wrap; characters at end of line are lost.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn insert_char_eol(&mut self, c: char, drow: i32, dcol: i32, rep: i32) {
            let c = if c.len_utf8() > 1 { b' ' } else { c as u8 };
            unsafe {
                Fl_Terminal_insert_char_eol(self.inner.widget() as _, c as c_char, drow, dcol, rep)
            }
        }

        /// Insert `count` rows at current cursor position.
        ///  Causes rows below to scroll down, and empty lines created.
        ///  Lines deleted by scroll down are NOT moved into the scroll history.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn insert_rows(&mut self, count: i32) {
            unsafe { Fl_Terminal_insert_rows(self.inner.widget() as _, count) }
        }

        /// Delete char(s) at (`drow`,`dcol`) for `count` times.
        pub fn delete_chars(&mut self, drow: i32, dcol: i32, count: i32) {
            unsafe { Fl_Terminal_delete_chars(self.inner.widget() as _, drow, dcol, count) }
        }

        /// Delete char(s) at cursor position for `count` times.
        pub fn delete_cur_chars(&mut self, count: i32) {
            unsafe { Fl_Terminal_delete_cur_chars(self.inner.widget() as _, count) }
        }

        ///  Delete `count` rows at cursor position.
        ///   Causes rows to scroll up, and empty lines created at bottom of screen.
        ///    Lines deleted by scroll up are NOT moved into the scroll history.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn delete_rows(&mut self, count: i32) {
            unsafe { Fl_Terminal_delete_rows(self.inner.widget() as _, count) }
        }

        /// Get the cursor's background color used for the cursor itself.
        pub fn cursor_bg_color(&self) -> Color {
            Color::from_rgbi(unsafe { Fl_Terminal_cursor_bg_color(self.inner.widget() as _) })
        }

        /// Set the cursor's background color used for the cursor itself.
        pub fn set_cursor_bg_color(&mut self, color: Color) {
            unsafe { Fl_Terminal_set_cursor_bg_color(self.inner.widget() as _, color.bits()) }
        }

        /// Get the cursor's foreground color used for the cursor itself.
        pub fn cursor_fg_color(&self) -> Color {
            Color::from_rgbi(unsafe { Fl_Terminal_cursor_fg_color(self.inner.widget() as _) })
        }

        /// Set the cursor's foreground color used for the cursor itself.
        pub fn set_cursor_fg_color(&mut self, color: Color) {
            unsafe { Fl_Terminal_set_cursor_fg_color(self.inner.widget() as _, color.bits()) }
        }

        /// Get the current mouse selection. Returns `None` if no selection, or `Some([srow, scol, erow, ecol])` if there is a selection,
        ///   where row and col represent start/end positions in the ring buffer.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn get_selection(&self) -> Option<[i32; 4]> {
            let mut retval: [i32; 4] = [0; 4];
            let ret =
                unsafe { Fl_Terminal_get_selection(self.inner.widget() as _, retval.as_mut_ptr()) };
            if ret != 0 {
                Some(retval)
            } else {
                None
            }
        }

        /// Move cursor to the home position (top/left).
        pub fn cursor_home(&mut self) {
            unsafe { Fl_Terminal_cursor_home(self.inner.widget() as _) }
        }

        /// Return terminal's display width in columns of text characters.
        pub fn display_columns(&self) -> i32 {
            unsafe { Fl_Terminal_display_columns(self.inner.widget() as _) }
        }

        /// Set terminal's display width in columns of text characters.
        pub fn set_display_columns(&mut self, val: i32) {
            unsafe { Fl_Terminal_set_display_columns(self.inner.widget() as _, val) }
        }

        /// Return terminal's display height in lines of text.
        pub fn display_rows(&self) -> i32 {
            unsafe { Fl_Terminal_display_rows(self.inner.widget() as _) }
        }

        /// Set terminal's display height in lines of text.
        pub fn set_display_rows(&mut self, val: i32) {
            unsafe { Fl_Terminal_set_display_rows(self.inner.widget() as _, val) }
        }

        /// Sets the number of lines of screen history.
        pub fn set_history_lines(&mut self, arg1: i32) {
            unsafe { Fl_Terminal_set_history_lines(self.inner.widget() as _, arg1) }
        }

        /// Gets the number of lines of screen history.
        pub fn history_lines(&self) -> i32 {
            unsafe { Fl_Terminal_history_lines(self.inner.widget() as _) }
        }

        /// Sets the terminal's scrollback history buffer size in lines of text (rows).
        pub fn set_history_rows(&mut self, arg1: i32) {
            unsafe { Fl_Terminal_set_history_rows(self.inner.widget() as _, arg1) }
        }

        /// Gets the terminal's scrollback history buffer size in lines of text (rows).
        pub fn history_rows(&self) -> i32 {
            unsafe { Fl_Terminal_history_rows(self.inner.widget() as _) }
        }

        /// Returns how many lines are "in use" by the screen history buffer.
        pub fn history_use(&self) -> i32 {
            unsafe { Fl_Terminal_history_use(self.inner.widget() as _) }
        }

        /// Set the bottom margin
        pub fn set_margin_bottom(&mut self, arg1: i32) {
            unsafe { Fl_Terminal_set_margin_bottom(self.inner.widget() as _, arg1) }
        }

        /// Return the bottom margin
        pub fn margin_bottom(&self) -> i32 {
            unsafe { Fl_Terminal_margin_bottom(self.inner.widget() as _) }
        }

        /// Set the left margin
        pub fn set_margin_left(&mut self, arg1: i32) {
            unsafe { Fl_Terminal_set_margin_left(self.inner.widget() as _, arg1) }
        }

        /// Return the left margin
        pub fn margin_left(&self) -> i32 {
            unsafe { Fl_Terminal_margin_left(self.inner.widget() as _) }
        }

        /// Set the right margin
        pub fn set_margin_right(&mut self, arg1: i32) {
            unsafe { Fl_Terminal_set_margin_right(self.inner.widget() as _, arg1) }
        }

        /// Return the right margin
        pub fn margin_right(&self) -> i32 {
            unsafe { Fl_Terminal_margin_right(self.inner.widget() as _) }
        }

        /// Set the top margin
        pub fn set_margin_top(&mut self, arg1: i32) {
            unsafe { Fl_Terminal_set_margin_top(self.inner.widget() as _, arg1) }
        }

        /// Return the top margin
        pub fn margin_top(&self) -> i32 {
            unsafe { Fl_Terminal_margin_top(self.inner.widget() as _) }
        }

        /// Sets the combined output translation flags to `val`.
        ///
        /// `val` can be sensible combinations of the OutFlags bit flags.
        ///
        /// The default is LF_TO_CRLF, so that \\n will generate both carriage-return (CR)
        /// and line-feed (LF).
        ///
        /// For \\r and \\n to be handled literally, use output_translate(Terminal::OutFlags::OFF);
        /// To disable all output translations, use 0 or Terminal::OutFlags::OFF.
        pub fn set_output_translate(&mut self, val: OutFlags) {
            unsafe { Fl_Terminal_set_output_translate(self.inner.widget() as _, val.bits() as u32) }
        }

        /// Return the current combined output translation flags.
        pub fn output_translate(&self) -> OutFlags {
            let result = unsafe { Fl_Terminal_output_translate(self.inner.widget() as _) as i32 };
            OutFlags::from_bits(result as u8).unwrap_or_else(|| {
                panic!("Unknown OutFlags value {} from output_translate", result)
            })
        }

        /// Prints single ASCII char `c` at current cursor position, and advances the cursor.
        /// - `c` must be ASCII, not utf-8
        /// - Does not trigger redraws
        pub fn print_char(&mut self, c: char) {
            unsafe { Fl_Terminal_print_char(self.inner.widget() as _, c as std::os::raw::c_char) }
        }

        ///   Prints single UTF-8 char `c` at current cursor position, and advances the cursor if the character
        ///   is printable. Handles ASCII and control codes (CR, LF, etc).
        ///
        ///  The character is displayed at the current cursor position
        ///  using the current text color/attributes.
        ///
        /// Handles control codes and can be used to construct ANSI/XTERM escape sequences.
        /// - `c` must be a single char only (whether UTF-8 or ASCII)
        /// - `c` can be an ASCII character, though not as efficent as print_char()
        /// - Invalid UTF-8 chars show the error character (¿) depending on show_unknown(bool).
        /// - Does not trigger redraws
        pub fn print_char_utf8(&mut self, c: char) {
            let txt = c.to_string();
            unsafe {
                Fl_Terminal_print_char_utf8(
                    self.inner.widget() as _,
                    txt.as_ptr() as _,
                    txt.len() as _,
                )
            }
        }

        /// Print the ASCII character `c` at the terminal's display position `(drow,dcol)`.
        ///   The character MUST be printable (in range 0x20 - 0x7e), and is displayed
        ///   using the current text color/attributes. Characters outside that range are either
        ///   ignored or print the error character (¿), depending on show_unknown(bool).
        ///
        /// No range checking is done on drow,dcol:
        /// - drow must be in range `0..(display_rows()-1)`
        /// - dcol must be in range `0..(display_columns()-1)`
        /// - Does not trigger redraws
        /// - Does NOT handle control codes, ANSI or XTERM escape sequences.
        pub fn plot_char(&mut self, c: char, row: i32, col: i32) {
            unsafe {
                Fl_Terminal_plot_char(
                    self.inner.widget() as _,
                    c as std::os::raw::c_char,
                    row,
                    col,
                )
            }
        }

        /// Print a single UTF-8 character len at display position `(drow,dcol)`.
        /// The character is displayed using the current text color/attributes.
        ///
        /// This is a very low level method.
        /// No range checking is done on drow,dcol:
        /// -  drow must be in range `0..(display_rows()-1)`
        /// -  dcol must be in range `0..(display_columns()-1)`
        /// - Does not trigger redraws
        /// - Does not handle ANSI or XTERM escape sequences
        /// - Invalid UTF-8 chars show the error character (¿) depending on show_unknown(bool).
        pub fn plot_char_utf8(&mut self, c: char, drow: i32, dcol: i32) {
            let txt = c.to_string();
            unsafe {
                Fl_Terminal_plot_char_utf8(
                    self.inner.widget() as _,
                    txt.as_ptr() as _,
                    txt.len() as _,
                    drow,
                    dcol,
                )
            }
        }

        /// Set the maximum rate redraw speed in floating point seconds if redraw_style() is set to RATE_LIMITED.
        pub fn set_redraw_rate(&mut self, set: f32) {
            unsafe { Fl_Terminal_set_redraw_rate(self.inner.widget() as _, set) }
        }

        /// Get max rate redraw speed in floating point seconds.
        pub fn redraw_rate(&self) -> f32 {
            unsafe { Fl_Terminal_redraw_rate(self.inner.widget() as _) }
        }

        /// Set how Terminal manages screen redrawing.
        pub fn set_redraw_style(&mut self, set: RedrawStyle) {
            unsafe { Fl_Terminal_set_redraw_style(self.inner.widget() as _, set.bits() as i32) }
        }

        /// Get the redraw style.
        pub fn redraw_style(&self) -> RedrawStyle {
            let result = unsafe { Fl_Terminal_redraw_style(self.inner.widget() as _) as u32 };
            RedrawStyle::new(result) // Construct a style with the given value
        }

        /// Resets terminal to default colors, clears screen, history and mouse selection, homes cursor, resets tabstops.
        pub fn reset_terminal(&mut self) {
            unsafe { Fl_Terminal_reset_terminal(self.inner.widget() as _) }
        }

        /// Returns the scrollbar's actual size; actual width for vertical scrollbars, actual height for horizontal scrollbars.
        pub fn scrollbar_actual_size(&self) -> i32 {
            unsafe { Fl_Terminal_scrollbar_actual_size(self.inner.widget() as _) }
        }

        /// Get the current size of the scrollbar's trough, in pixels.
        /// If this value is zero (default), this widget is using fltk's
        /// master scrollbar_size() value
        pub fn scrollbar_size(&self) -> i32 {
            unsafe { Fl_Terminal_scrollbar_size(self.inner.widget() as _) }
        }

        /// Set the width of the scrollbar's trough to val, in pixels.
        /// If this value is zero (default), this widget will use fltk's
        /// master scrollbar_size() value
        pub fn set_scrollbar_size(&mut self, val: i32) {
            unsafe { Fl_Terminal_set_scrollbar_size(self.inner.widget() as _, val) }
        }

        /// Get mouse selection background color.
        pub fn selection_bg_color(&self) -> Color {
            Color::from_rgbi(unsafe { Fl_Terminal_selection_bg_color(self.inner.widget() as _) })
        }

        /// Set mouse selection background color.
        pub fn set_selection_bg_color(&mut self, color: Color) {
            unsafe { Fl_Terminal_set_selection_bg_color(self.inner.widget() as _, color.bits()) }
        }

        /// Get mouse selection foreground color.
        pub fn selection_fg_color(&self) -> Color {
            Color::from_rgbi(unsafe { Fl_Terminal_selection_fg_color(self.inner.widget() as _) })
        }

        /// Set mouse selection foreground color.
        pub fn set_selection_fg_color(&mut self, color: Color) {
            unsafe { Fl_Terminal_set_selection_fg_color(self.inner.widget() as _, color.bits()) }
        }

        /// Return the "show unknown" flag. if true, show unknown chars as '¿'
        pub fn show_unknown(&self) -> bool {
            unsafe { Fl_Terminal_show_unknown(self.inner.widget() as _) != 0 }
        }

        /// Set the "show unknown" flag. if true, show unknown chars as '¿' (default off)
        pub fn set_show_unknown(&mut self, arg1: bool) {
            unsafe { Fl_Terminal_set_show_unknown(self.inner.widget() as _, arg1 as i32) }
        }

        /// Return the text attribute bits (underline, inverse, etc) for subsequent appends.
        pub fn text_attrib(&self) -> Attrib {
            // Attrib::from_bits( unsafe { Fl_Terminal_text_attrib(self.inner.widget()) as _ } ).unwrap()
            let result = unsafe { Fl_Terminal_text_attrib(self.inner.widget() as _) };
            Attrib::from_bits(result).unwrap_or_else(|| panic!("Unknown Attrib value {}", result))
        }

        /// Set text attribute bits (underline, inverse, etc) for subsequent appends.
        pub fn set_text_attrib(&mut self, arg1: Attrib) {
            unsafe { Fl_Terminal_set_text_attrib(self.inner.widget() as _, arg1.bits()) }
        }

        /// Set text background color to fltk color val.
        /// Use this for temporary color changes, similar to \<ESC\>[48;2;{R};{G};{B}m
        ///
        /// This setting does not affect the 'default' text colors used by \<ESC\>[0m, \<ESC\>c, reset_terminal(), etc.
        /// To change both the current and default bg color, also use text_bg_color_default(Fl_Color).
        pub fn set_text_bg_color(&mut self, color: Color) {
            unsafe { Fl_Terminal_set_text_bg_color(self.inner.widget() as _, color.bits()) }
        }

        /// Get the text background color.
        pub fn text_bg_color(&self) -> Color {
            Color::from_rgbi(unsafe { Fl_Terminal_text_bg_color(self.inner.widget() as _) })
        }

        /// Set the default text background color used by \<ESC\>c, \<ESC\>[0m, and reset_terminal().
        /// Does not affect the 'current' text fg color; use set_text_bg_color(Fl_Color) to set that.
        pub fn set_text_bg_color_default(&mut self, color: Color) {
            unsafe { Fl_Terminal_set_text_bg_color_default(self.inner.widget() as _, color.bits()) }
        }

        /// Return the default text background color.
        pub fn text_bg_color_default(&self) -> Color {
            Color::from_rgbi(unsafe { Fl_Terminal_text_bg_color_default(self.inner.widget() as _) })
        }

        /// Sets the background text color as one of the 8 'xterm color' values.
        ///
        /// This will be the background color used for all newly printed text, similar to the \<ESC\>[#m escape sequence, where # is between 40 and 47.
        ///
        /// This color will be reset to the default bg color if reset_terminal() is called, or by \<ESC\>c, \<ESC\>[0m, etc.
        ///
        /// The xterm color intensity values can be influenced by the Dim/Bold/Normal modes (which can be set with e.g. \<ESC\>[1m, textattrib(), etc), so the actual RGB values of these colors allow room for Dim/Bold to influence their brightness. For instance, "Normal Red" is not full brightness to allow "Bold Red" to be brighter. This goes for all colors except 'Black', which is not influenced by Dim or Bold; Black is always Black.
        ///
        /// These background colors are slightly dimmer than the corresponding xterm foregroumd colors.
        ///
        /// The 8 color xterm values are:
        /// 0 = Black, 1 = Red, 2 = Green, 3 = Yellow, 4 = Blue,5 = Magenta, 6 = Cyan, 7 = White
        pub fn set_text_bg_color_xterm(&mut self, color: XtermColor) {
            unsafe { Fl_Terminal_set_text_bg_color_xterm(self.inner.widget() as _, color as u8) }
        }
        ///  Set the text color for the terminal.
        ///  This is a convenience method that sets *both* textfgcolor() and textfgcolor_default(),
        ///  ensuring both are set to the same value.
        pub fn set_text_color(&mut self, color: Color) {
            unsafe { Fl_Terminal_set_text_color(self.inner.widget() as _, color.bits()) }
        }
        /// Set text foreground drawing color to fltk color val.
        /// Use this for temporary color changes, similar to \<ESC\>[38;2;{R};{G};{B}m
        ///
        /// This setting does not affect the 'default' text colors used by \<ESC\>[0m, \<ESC\>c, reset_terminal(), etc.
        /// To change both the current and default fg color, also use textfgcolor_default(Fl_Color)
        pub fn set_text_fg_color(&mut self, color: Color) {
            unsafe { Fl_Terminal_set_text_fg_color(self.inner.widget() as _, color.bits()) }
        }

        /// Get the text foreground color.
        pub fn text_fg_color(&self) -> Color {
            Color::from_rgbi(unsafe { Fl_Terminal_text_fg_color(self.inner.widget() as _) })
        }

        /// Set the default text foreground color used by \<ESC\>c, \<ESC\>[0m, and reset_terminal().
        /// Does not affect the 'current' text fg color; use set_text_fg_color(Fl_Color) to set that.
        pub fn set_text_fg_color_default(&mut self, color: Color) {
            unsafe { Fl_Terminal_set_text_fg_color_default(self.inner.widget() as _, color.bits()) }
        }

        /// Return the default text foreground color.
        pub fn text_fg_color_default(&self) -> Color {
            Color::from_rgbi(unsafe { Fl_Terminal_text_fg_color_default(self.inner.widget() as _) })
        }

        /// Sets the foreground text color as one of the 8 'xterm color' values.
        ///
        /// This will be the foreground color used for all newly printed text, similar to the \<ESC\>[#m escape sequence, where # is between 30 and 37.
        ///
        /// This color will be reset to the default bg color if reset_terminal() is called, or by \<ESC\>c, \<ESC\>[0m, etc.
        ///
        /// The xterm color intensity values can be influenced by the Dim/Bold/Normal modes (which can be set with e.g. \<ESC\>[1m, textattrib(), etc), so the actual RGB values of these colors allow room for Dim/Bold to influence their brightness. For instance, "Normal Red" is not full brightness to allow "Bold Red" to be brighter. This goes for all colors except 'Black', which is not influenced by Dim or Bold; Black is always Black.
        ///
        /// The 8 color xterm values are:
        /// 0 = Black, 1 = Red, 2 = Green, 3 = Yellow, 4 = Blue,5 = Magenta, 6 = Cyan, 7 = White
        pub fn set_text_fg_color_xterm(&mut self, color: XtermColor) {
            unsafe { Fl_Terminal_set_text_fg_color_xterm(self.inner.widget() as _, color as u8) }
        }

        /// Get the text font
        pub fn text_font(&self) -> Font {
            Font::by_index(unsafe { Fl_Terminal_text_font(self.inner.widget() as _) } as usize)
        }

        /// Sets the font used for all text displayed in the terminal.
        /// This affects all existing text (in display and history) as well as any newly printed text.
        /// Only monospace fonts are recommended.
        pub fn set_text_font(&mut self, font: Font) {
            unsafe { Fl_Terminal_set_text_font(self.inner.widget() as _, font.bits()) }
        }

        /// Gets the text size
        pub fn text_size(&self) -> i32 {
            unsafe { Fl_Terminal_text_size(self.inner.widget() as _) }
        }

        /// Sets the font size used for all text displayed in the terminal.
        /// This affects all existing text (in display and history) as well as any newly printed text.
        /// Changing this will affect the display_rows() and display_columns().
        pub fn set_text_size(&mut self, val: i32) {
            unsafe { Fl_Terminal_set_text_size(self.inner.widget() as _, val) }
        }

        /// Gets the selection text
        /// *This is a low-level "protected" function of the fltk library*
        pub fn selection_text(&self) -> String {
            assert!(self.is_derived);
            unsafe {
                let ptr = Fl_Terminal_selection_text(self.inner.widget() as _);
                if ptr.is_null() {
                    String::new()
                } else {
                    CStr::from_ptr(ptr).to_string_lossy().to_string()
                }
            }
        }

        // Various methods to access the ring buffer

        ///  Return the ending row# in the display area.
        pub fn disp_erow(&self) -> i32 {
            unsafe { Fl_Terminal_disp_erow(self.inner.widget() as _) }
        }

        /// Return the number of rows in the display area.
        pub fn disp_rows(&self) -> i32 {
            unsafe { Fl_Terminal_disp_rows(self.inner.widget() as _) }
        }

        /// Return the number of columns in the display area (always the same as ring_cols())
        pub fn disp_cols(&self) -> i32 {
            unsafe { Fl_Terminal_disp_cols(self.inner.widget() as _) }
        }

        /// Return the starting row# in the display area.
        pub fn disp_srow(&self) -> i32 {
            unsafe { Fl_Terminal_disp_srow(self.inner.widget() as _) }
        }

        /// Return the number of columns in the scrollback history (always the same as ring_cols())
        pub fn hist_cols(&self) -> i32 {
            unsafe { Fl_Terminal_hist_cols(self.inner.widget() as _) }
        }

        /// Return the ending row# of the scrollback history.
        pub fn hist_erow(&self) -> i32 {
            unsafe { Fl_Terminal_hist_erow(self.inner.widget() as _) }
        }

        /// Return the number of rows in the scrollback history.
        pub fn hist_rows(&self) -> i32 {
            unsafe { Fl_Terminal_hist_rows(self.inner.widget() as _) }
        }

        /// Return the starting row# of the scrollback history.
        pub fn hist_srow(&self) -> i32 {
            unsafe { Fl_Terminal_hist_srow(self.inner.widget() as _) }
        }

        /// Return number of rows in use by the scrollback history.
        pub fn hist_use(&self) -> i32 {
            unsafe { Fl_Terminal_hist_use(self.inner.widget() as _) }
        }

        /// Return the starting row of the \"in use\" scrollback history.
        pub fn hist_use_srow(&self) -> i32 {
            unsafe { Fl_Terminal_hist_use_srow(self.inner.widget() as _) }
        }

        /// Is global row/column inside the current mouse selection?
        /// *This is a low-level "protected" function of the fltk library*
        pub fn is_inside_selection(&self, row: i32, col: i32) -> bool {
            unsafe { Fl_Terminal_is_inside_selection(self.inner.widget() as _, row, col) != 0 }
        }

        /// Returns true if there's a mouse selection.
        pub fn is_selection(&self) -> bool {
            unsafe { Fl_Terminal_is_selection(self.inner.widget() as _) != 0 }
        }

        /// Returns the current offset into the ring buffer.
        pub fn offset(&self) -> i32 {
            unsafe { Fl_Terminal_offset(self.inner.widget() as _) }
        }

        /// Return the number of columns in the ring buffer.
        pub fn ring_cols(&self) -> i32 {
            unsafe { Fl_Terminal_ring_cols(self.inner.widget() as _) }
        }

        /// Return the ending row# in the ring buffer (Always ring_rows()-1)
        pub fn ring_erow(&self) -> i32 {
            unsafe { Fl_Terminal_ring_erow(self.inner.widget() as _) }
        }

        /// Return the starting row# in the ring buffer (Always 0)
        pub fn ring_srow(&self) -> i32 {
            unsafe { Fl_Terminal_ring_srow(self.inner.widget() as _) }
        }

        /// Return the number of rows in the ring buffer.
        pub fn ring_rows(&self) -> i32 {
            unsafe { Fl_Terminal_ring_rows(self.inner.widget() as _) }
        }

        /// Return the Utf8Char for character under cursor.
        pub fn u8c_cursor(&self) -> Utf8Char {
            unsafe {
                let x = self.inner.widget();
                let utf8_p = Fl_Terminal_u8c_cursor(x as _);
                Utf8Char { inner: utf8_p }
            }
        }

        /// Return u8c for beginning of row drow of the display.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn u8c_disp_row(&self, drow: i32) -> BuffRow {
            // Fl_Terminal_u8c_disp_row returns pointer to the first C++ Utf8Char object,
            //  which becomes the `inner` element in the Rust BuffRow object
            let row_p = unsafe { Fl_Terminal_u8c_disp_row(self.inner.widget() as _, drow) };
            BuffRow::new(row_p, self)
        }

        /// Return u8c for beginning of row hrow inside the scrollback history.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn u8c_hist_row(&self, hrow: i32) -> BuffRow {
            // Fl_Terminal_u8c_hist_row returns pointer to the first C++ Utf8Char object,
            //  which becomes the `inner` element in the Rust BuffRow object
            let row_p = unsafe { Fl_Terminal_u8c_hist_row(self.inner.widget() as _, hrow) };
            BuffRow::new(row_p, self)
        }

        /// Return u8c for beginning of row hurow inside the 'in use' part of the\n scrollback history.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn u8c_hist_use_row(&self, hurow: i32) -> BuffRow {
            // Fl_Terminal_u8c_hist_use_row returns pointer to the first  C++ Utf8Char object,
            //  which becomes the `inner` element in the Rust BuffRow object
            let row_p = unsafe { Fl_Terminal_u8c_hist_use_row(self.inner.widget() as _, hurow) };
            BuffRow::new(row_p, self)
        }

        /// Return u8c for beginning of row grow in the ring buffer.
        /// *This is a low-level "protected" function of the fltk library*
        pub fn u8c_ring_row(&self, grow: i32) -> BuffRow {
            // Fl_Terminal_u8c_ring_use_row returns pointer to the first  C++ Utf8Char object,
            //  which becomes the `inner` element in the Rust BuffRow object
            let row_p = unsafe { Fl_Terminal_u8c_ring_row(self.inner.widget() as _, grow) };
            BuffRow::new(row_p, self)
        }
    }

    // So far only implementing "getter" methods. Todo: methods to modify Utf8Char
    impl Utf8Char {
        /// Construct a new Utf8Char, single-byte only. This is really only useful for testing.
        ///  'c' must be "printable" ASCII in the range (0x20 <= c <= 0x7e).
        ///     Anything outside of that is silently ignored.
        ///
        /// Allocated Utf8Char will never be deleted.
        pub fn new(c: u8) -> Self {
            unsafe {
                let u8c = Fl_Terminal_Utf8Char_new_obj(c);
                Utf8Char { inner: u8c }
            }
        }

        /// Return the actual displayed color of char `u8c` possibly influenced by BOLD or DIM if the char is from Xterm.
        ///    BG color will be derived from the widget color if a widget is specified and the color is `TransparentBg`,
        ///    and that won't be influenced by charflag attributes.
        pub fn attr_bgcolor(&self, term: Option<&Terminal>) -> Color {
            Color::from_rgbi(match term {
                None => unsafe { Fl_Terminal_Utf8Char_attr_bgcolor(self.inner, std::ptr::null()) },
                Some(t) => unsafe {
                    Fl_Terminal_Utf8Char_attr_bgcolor(self.inner, t.inner.widget() as _)
                },
            })
        }

        // /// Return the actual displayed color of char `u8c` possibly influenced by BOLD or DIM if the char is from Xterm.
        // ///    If a `grp` widget is specified (i.e. not `None`), don't let the color be
        // ///    influenced by the attribute bits *if* it matches the `grp` widget's own `color()`.
        // pub fn attr_color(&self, grp: Option<*const Fl_Widget>) -> Color {
        //     Color::from_rgbi(match grp {
        //         None => unsafe { Fl_Terminal_Utf8Char_attr_color(self.inner, std::ptr::null()) },
        //         Some(g) => unsafe { Fl_Terminal_Utf8Char_attr_color(self.inner, g) },
        //     })
        // }

        /// Return the actual displayed fg color of char `u8c` possibly influenced by BOLD or DIM if the char is from Xterm.
        ///    If a `term` widget is specified (i.e. not `None`), don't let the color be
        ///    influenced by the attribute bits *if* it matches the `term` widget's own `color()`.
        pub fn attr_fgcolor(&self, term: Option<&Terminal>) -> Color {
            Color::from_rgbi(match term {
                None => unsafe { Fl_Terminal_Utf8Char_attr_fgcolor(self.inner, std::ptr::null()) },
                Some(t) => unsafe {
                    Fl_Terminal_Utf8Char_attr_fgcolor(self.inner, t.inner.widget() as _)
                },
            })
        }

        /// Return the attributes for this character.
        pub fn attrib(&self) -> Attrib {
            let result = unsafe { Fl_Terminal_Utf8Char_attrib(self.inner) };
            Attrib::from_bits(result).unwrap_or_else(|| panic!("Unknown Attrib value {}", result))
        }

        /// Return the background color for this character.
        pub fn bgcolor(&self) -> Color {
            Color::from_rgbi(unsafe { Fl_Terminal_Utf8Char_bgcolor(self.inner) })
        }

        /// Return the foreground color for this character.
        pub fn fgcolor(&self) -> Color {
            let result = unsafe { Fl_Terminal_Utf8Char_fgcolor(self.inner) };
            Color::from_rgbi(result)
        }

        /// Return the xterm CharFlags bits
        pub fn charflags(&self) -> CharFlags {
            let result = unsafe { Fl_Terminal_Utf8Char_charflags(self.inner) as i32 };
            CharFlags::from_bits(result as u8)
                .unwrap_or_else(|| panic!("Unknown CharFlags value {}", result))
        }

        /// Returns true if the character text in this struct matches the given ASCII character
        pub fn is_char(&self, c: u8) -> bool {
            let result = unsafe { Fl_Terminal_Utf8Char_is_char(self.inner, c as c_char) as i32 };
            result != 0
        }

        /// Return the length of this character in bytes (UTF-8 can be multibyte)
        pub fn length(&self) -> usize {
            unsafe { Fl_Terminal_Utf8Char_length(self.inner) as usize }
        }

        /// Return the maximum length in bytes of a UTF-8 character
        pub fn max_utf8(&self) -> usize {
            unsafe { Fl_Terminal_Utf8Char_max_utf8(self.inner) as usize }
        }

        /// Return the width of this character in floating point pixels.
        ///
        ///    WARNING: Uses current font, so assumes font and font_size
        ///             have already been set to current font!
        pub fn pwidth(&self) -> f64 {
            unsafe { Fl_Terminal_Utf8Char_pwidth(self.inner) as f64 }
        }

        /// Return the width of this character in integer pixels.
        ///
        ///    WARNING: Uses current font, so assumes font and font_size
        ///             have already been set to current font!
        pub fn pwidth_int(&self) -> usize {
            unsafe { Fl_Terminal_Utf8Char_pwidth_int(self.inner) as usize }
        }

        /// Return the UTF-8 text string for this character.
        pub fn text_utf8(&self) -> &[u8] {
            unsafe {
                let ptr = Fl_Terminal_Utf8Char_text_utf8(self.inner);
                let len = Fl_Terminal_Utf8Char_length(self.inner);
                std::slice::from_raw_parts(ptr, len as usize)
            }
        }

        /// Return the size of a Utf8Char object in the underlying C++ code
        pub fn size() -> usize {
            unsafe { Fl_Terminal_Utf8Char_size() as usize }
        }
    }

    impl<'a> BuffRow<'a> {
        /// Generate a new BuffRow object based on a pointer from C++ Fl_Terminal
        pub fn new(ptr: *const Fl_Terminal_Utf8Char, parent: &'a Terminal) -> Self {
            unsafe {
                BuffRow {
                    // inner is the pointer to the first C++ Utf8Char in the row
                    inner: ptr,
                    _parent: parent,
                    // length: (i + 1) as usize,
                    length: parent.ring_cols() as usize,
                    char_size: Fl_Terminal_Utf8Char_size() as usize,
                }
            }
        }

        /// Trim trailing blanks off of BuffRow object.
        /// Does not affect the data in the RingBuff, just this object's access.
        pub fn trim(mut self) -> Self {
            unsafe {
                let mut last_char = self.inner.add((self.length - 1) * self.char_size);
                let c = Utf8Char { inner: last_char };
                // If the last character is a blank, trim the length back.
                if c.text_utf8() == b" " {
                    // Record the attributes etc of the last character
                    let attr = c.attrib();
                    let fg = c.fgcolor();
                    let bg = c.bgcolor();
                    self.length -= 1; // Already checked the last character
                    while self.length > 0 {
                        last_char = last_char.sub(self.char_size);
                        let c = Utf8Char { inner: last_char };
                        if c.text_utf8() != b" "
                            || c.attrib() != attr
                            || c.fgcolor() != fg
                            || c.bgcolor() != bg
                        {
                            break; // Found a non-blank character or one with attrib changes
                        }
                        self.length -= 1;
                    }
                }
            }
            self
        }

        /// Index into row array of Utf8Char
        pub fn col(&self, idx: usize) -> Utf8Char {
            if idx > self.length {
                panic!("Index {} out of range", idx);
            }
            unsafe {
                let base = self.inner;
                Utf8Char {
                    inner: base.add(idx * self.char_size),
                }
            }
        }

        /// Iterator object to step through a sequence of Utf8Char in a BuffRow
        pub fn iter(&self) -> BuffRowIter {
            BuffRowIter::new(self, self.length)
        }
    }

    /// Iterator object to step through a sequence of Utf8Char in a BuffRow
    pub struct BuffRowIter<'a> {
        parent: &'a BuffRow<'a>,
        ptr: *const Fl_Terminal_Utf8Char, // This points to an array of Fl_Terminal::Utf8Char
        end: *const Fl_Terminal_Utf8Char, // points just past the ptr array end
    }

    impl<'a> BuffRowIter<'a> {
        fn new(parent: &'a BuffRow, len: usize) -> BuffRowIter<'a> {
            unsafe {
                BuffRowIter {
                    parent,
                    ptr: parent.inner,
                    end: parent.inner.add(len * parent.char_size),
                }
            }
        }
    }

    impl<'a> Iterator for BuffRowIter<'a> {
        type Item = Utf8Char;
        fn next(&mut self) -> Option<Self::Item> {
            if self.ptr < self.end {
                let result = Utf8Char { inner: self.ptr };
                unsafe {
                    self.ptr = self.ptr.add(self.parent.char_size);
                }
                Some(result)
            } else {
                None
            }
        }
    }
}
