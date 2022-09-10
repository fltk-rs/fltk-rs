use crate::enums::{Color, Font, FrameType, Key};
use crate::image::Image;
use crate::prelude::*;
use crate::utils::FlString;
use crate::widget::Widget;
use fltk_sys::tree::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Defines the Tree sort order
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TreeSort {
    /// Don't sort
    None = 0,
    /// Sort ascending
    Ascending = 1,
    /// Sort descending
    Descending = 2,
}

/// Defines the Tree's connector sort order
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TreeConnectorStyle {
    /// No line
    None = 0,
    /// Dotted line
    Dotted = 1,
    /// Solid line
    Solid = 2,
}

/// Defines the Tree select mode
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TreeSelect {
    /// Select none
    None = 0,
    /// Select single
    Single = 1,
    /// Select multi
    Multi = 2,
    /// Select single and make draggable
    SingleDraggable = 3,
}

/// Defines the `TreeItem`'s select mode
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TreeItemSelect {
    /// Deselect when clicked
    Deselect = 0,
    /// Selected when clicked
    Select = 1,
    /// Toggle when clicked
    Toggle = 2,
}

/// Defines the Tree's callback reason
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TreeReason {
    /// No callback trigger
    None = 0,
    /// Trigger callback when selected
    Selected,
    /// Trigger callback when deselected
    Deselected,
    /// Trigger callback when reselected
    Reselected,
    /// Trigger callback when opened
    Opened,
    /// Trigger callback when closed
    Closed,
    /// Trigger callback when dragged
    Dragged,
}

/// Defines the `TreeItem`'s reselect mode
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TreeItemReselectMode {
    /// Reselect once
    Once = 0,
    /// Always reselect
    Always,
}

/// Defines the `TreeItem`'s draw mode
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TreeItemDrawMode {
    /// Default draw mode
    Default = 0,
    /// Draws label and widget
    LabelAndWidget = 1,
    /// Draws the height from widget
    HeightFromWidget = 2,
}

/// Defines a tree widget
#[derive(Debug)]
pub struct Tree {
    inner: *mut Fl_Tree,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Tree, Fl_Tree);
crate::macros::widget::impl_widget_base!(Tree, Fl_Tree);

/// Defines a tree item
#[derive(Debug, Clone)]
pub struct TreeItem {
    inner: *mut Fl_Tree_Item,
    parent: *const Fl_Tree_Item,
    tree: Tree,
    is_root: bool,
    is_derived: bool,
}

/// Defines a tree item array
#[derive(Debug)]
struct TreeItemArray {
    inner: *mut Fl_Tree_Item_Array,
}

impl Tree {
    /// Creates a Tree from a raw `Fl_Tree pointer`
    /// # Safety
    /// The pointer must be valid
    pub unsafe fn from_raw(ptr: *mut Fl_Tree) -> Option<Tree> {
        if ptr.is_null() {
            None
        } else {
            let tracker = fltk_sys::fl::Fl_Widget_Tracker_new(ptr as *mut fltk_sys::fl::Fl_Widget);
            if tracker.is_null() {
                return None;
            }
            let x = Tree {
                inner: ptr,
                tracker,
                is_derived: false,
            };
            Some(x)
        }
    }

    /// Begins the Tree widget
    pub fn begin(&self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_begin(self.inner) }
    }

    /// Ends the Tree widget
    pub fn end(&self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_end(self.inner) }
    }

    /// Shows the Tree widget
    pub fn show_self(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_show_self(self.inner) }
    }

    /// Sets the root label
    pub fn set_root_label(&mut self, new_label: &str) {
        assert!(!self.was_deleted());
        let new_label = CString::safe_new(new_label);
        unsafe { Fl_Tree_root_label(self.inner, new_label.as_ptr()) }
    }

    /// Gets the Tree's root
    pub fn root(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_root(self.inner)) }
    }

    /// Sets the Tree's root
    pub fn set_root(&mut self, new_item: Option<TreeItem>) {
        assert!(!self.was_deleted());
        let ptr = match new_item {
            None => std::ptr::null_mut(),
            Some(item) => item.inner,
        };
        unsafe { Fl_Tree_set_root(self.inner, ptr) }
    }

    /// Adds a `TreeItem`
    pub fn add(&mut self, path: &str) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        let path = CString::safe_new(path);
        unsafe {
            let x = Fl_Tree_add(self.inner, path.as_ptr() as *mut raw::c_char);
            TreeItem::from_raw(x)
        }
    }

    /// Adds a `TreeItem`
    pub fn add_item(&mut self, path: &str, item: &TreeItem) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        let path = CString::safe_new(path);
        unsafe {
            let x = Fl_Tree_add_item(self.inner, path.as_ptr() as *mut raw::c_char, item.inner);
            TreeItem::from_raw(x)
        }
    }

    /// Inserts a `TreeItem` above another tree item
    pub fn insert_above(&mut self, above: &TreeItem, name: &str) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        if above.inner.is_null() {
            return None;
        }
        let name = CString::safe_new(name);
        unsafe {
            let x =
                Fl_Tree_insert_above(self.inner, above.inner, name.as_ptr() as *mut raw::c_char);
            TreeItem::from_raw(x)
        }
    }

    /// Inserts a `TreeItem` at a position `pos`
    pub fn insert(&mut self, item: &TreeItem, name: &str, pos: i32) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        if item.inner.is_null() {
            return None;
        }
        let name = CString::safe_new(name);
        unsafe {
            let x = Fl_Tree_insert(
                self.inner,
                item.inner,
                name.as_ptr() as *mut raw::c_char,
                pos as i32,
            );
            TreeItem::from_raw(x)
        }
    }

    /// Removes a `TreeItem`
    /// # Errors
    /// Errors on failure to remove item
    pub fn remove(&mut self, item: &TreeItem) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        if item.inner.is_null() {
            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
        }
        unsafe {
            match Fl_Tree_remove(self.inner, item.inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Clears a tree
    pub fn clear(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_clear(self.inner) }
    }

    /// Clears all children
    pub fn clear_children(&mut self, item: &TreeItem) {
        assert!(!self.was_deleted());
        assert!(!item.inner.is_null());
        unsafe { Fl_Tree_clear_children(self.inner as *mut Fl_Tree, item.inner) }
    }

    /// Finds a tree item
    pub fn find_item(&self, path: &str) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        let path = CString::safe_new(path);
        unsafe {
            let x = Fl_Tree_find_item(self.inner, path.as_ptr() as *mut raw::c_char);
            if x.is_null() {
                None
            } else {
                TreeItem::from_raw(x as *mut Fl_Tree_Item)
            }
        }
    }

    /// finds a clicked item
    pub fn find_clicked(&self, yonly: bool) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe {
            TreeItem::from_raw(Fl_Tree_find_clicked(self.inner, yonly as i32) as *mut Fl_Tree_Item)
        }
    }

    #[deprecated(since = "1.2.21", note = "use callback_item() instead")]
    /// Set the item that was last clicked.
    pub fn set_item_clicked(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_item_clicked(self.inner)) }
    }

    /// Gets the first tree item
    pub fn first(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_first(self.inner)) }
    }

    /// Gets the first visible tree item
    pub fn first_visible_item(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_first_visible_item(self.inner)) }
    }

    /// Gets the next tree item
    pub fn next(&self, item: &TreeItem) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        if item.inner.is_null() {
            return None;
        }
        unsafe { TreeItem::from_raw(Fl_Tree_next(self.inner, item.inner)) }
    }

    /// Gets the previous tree item
    pub fn prev(&self, item: &TreeItem) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        if item.inner.is_null() {
            return None;
        }
        unsafe { TreeItem::from_raw(Fl_Tree_prev(self.inner, item.inner)) }
    }

    /// Gets the last tree item
    pub fn last(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_last(self.inner)) }
    }

    /// Gets the last visible tree item
    pub fn last_visible_item(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_last_visible_item(self.inner)) }
    }

    /// Gets the next visible tree item
    pub fn next_visible_item(&self, start: &TreeItem, direction_key: Key) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        if start.inner.is_null() {
            return None;
        }
        unsafe {
            TreeItem::from_raw(Fl_Tree_next_visible_item(
                self.inner,
                start.inner,
                direction_key.bits() as i32,
            ))
        }
    }

    /// Gets the first selected tree item
    pub fn first_selected_item(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_first_selected_item(self.inner)) }
    }

    /// Gets the last selected tree item
    pub fn last_selected_item(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_last_selected_item(self.inner)) }
    }

    /// Gets the next tree item, `direction_key` is by default [`Key::Down`](`crate::enums::Key::Down`)
    pub fn next_item(
        &self,
        item: &TreeItem,
        direction_key: Key,
        visible: bool,
    ) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        if item.inner.is_null() {
            return None;
        }
        unsafe {
            TreeItem::from_raw(Fl_Tree_next_item(
                self.inner,
                item.inner,
                direction_key.bits() as i32,
                visible as i32,
            ))
        }
    }

    /// Gets the next selected tree item, `direction_key` is by default [`Key::Down`](`crate::enums::Key::Down`)
    pub fn next_selected_item(&mut self, item: &TreeItem, direction_key: Key) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        if item.inner.is_null() {
            return None;
        }
        unsafe {
            TreeItem::from_raw(Fl_Tree_next_selected_item(
                self.inner,
                item.inner,
                direction_key.bits() as i32,
            ))
        }
    }

    /// Gets the selected tree items
    pub fn get_selected_items(&self) -> Option<Vec<TreeItem>> {
        assert!(!self.was_deleted());
        unsafe {
            let mut items = TreeItemArray {
                inner: std::ptr::null_mut(),
            };
            let ret = Fl_Tree_get_selected_items(self.inner, &mut items.inner);
            if ret == 0 {
                None
            } else {
                items.into_vec()
            }
        }
    }

    /// Gets the all tree items
    pub fn get_items(&self) -> Option<Vec<TreeItem>> {
        assert!(!self.was_deleted());
        unsafe {
            let mut items = TreeItemArray {
                inner: std::ptr::null_mut(),
            };
            let ret = Fl_Tree_get_items(self.inner, &mut items.inner);
            if ret == 0 {
                None
            } else {
                items.into_vec()
            }
        }
    }

    /// Opens a tree item, causing the children to be shown
    /// # Errors
    /// Errors on failure to open an item
    pub fn open(&mut self, path: &str, do_callback: bool) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        let path = CString::safe_new(path);
        unsafe {
            match Fl_Tree_open(
                self.inner,
                path.as_ptr() as *mut raw::c_char,
                do_callback as i32,
            ) {
                0 | 1 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Toggle the open state
    pub fn open_toggle(&mut self, item: &TreeItem, do_callback: bool) {
        assert!(!self.was_deleted());
        assert!(!item.inner.is_null());
        unsafe { Fl_Tree_open_toggle(self.inner, item.inner, do_callback as i32) }
    }

    /// Close a tree item, causing the children to be hidden
    /// # Errors
    /// Errors on failure to close an item
    pub fn close(&mut self, path: &str, do_callback: bool) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        let path = CString::safe_new(path);
        unsafe {
            match Fl_Tree_close(
                self.inner,
                path.as_ptr() as *mut raw::c_char,
                do_callback as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Returns whether an item is open
    pub fn is_open(&self, path: &str) -> bool {
        assert!(!self.was_deleted());
        let path = CString::safe_new(path);
        unsafe { Fl_Tree_is_open(self.inner, path.as_ptr() as *mut raw::c_char) != 0 }
    }

    /// Returns whether an item is closed
    pub fn is_close(&self, path: &str) -> bool {
        assert!(!self.was_deleted());
        let path = CString::safe_new(path);
        unsafe { Fl_Tree_is_close(self.inner, path.as_ptr() as *mut raw::c_char) != 0 }
    }

    /// Select a tree item
    /// # Errors
    /// Errors on failure to select an item
    pub fn select(&mut self, path: &str, do_callback: bool) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        let path = CString::safe_new(path);
        unsafe {
            match Fl_Tree_select(
                self.inner,
                path.as_ptr() as *mut raw::c_char,
                do_callback as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Toggle the select state of the specified
    pub fn select_toggle(&mut self, item: &TreeItem, do_callback: bool) {
        assert!(!self.was_deleted());
        assert!(!item.inner.is_null());
        unsafe { Fl_Tree_select_toggle(self.inner, item.inner, do_callback as i32) }
    }

    /// Deselect an item at `path` and determine whether to do the callback
    /// # Errors
    /// Errors on failure to deselect item
    pub fn deselect(&mut self, path: &str, do_callback: bool) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        let path = CString::safe_new(path);
        unsafe {
            match Fl_Tree_deselect(
                self.inner,
                path.as_ptr() as *mut raw::c_char,
                do_callback as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Deselect all items
    /// # Errors
    /// Errors on failure to deselect all items
    pub fn deselect_all(&mut self, item: &TreeItem, do_callback: bool) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        if item.inner.is_null() {
            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
        }
        unsafe {
            match Fl_Tree_deselect_all(self.inner, item.inner, do_callback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Select only the specified item, deselecting all others that might be selected.
    /// # Errors
    /// Errors on failure to select an item
    pub fn select_only(
        &mut self,
        selected_item: &TreeItem,
        do_callback: bool,
    ) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        if selected_item.inner.is_null() {
            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
        }
        unsafe {
            match Fl_Tree_select_only(self.inner, selected_item.inner, do_callback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Select all items
    /// # Errors
    /// Errors on failure to select an item
    pub fn select_all(&mut self, item: &TreeItem, do_callback: bool) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        if item.inner.is_null() {
            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
        }
        unsafe {
            match Fl_Tree_select_all(self.inner, item.inner, do_callback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Extend the selection between and including `from` and `to` in a certain direction
    /// # Errors
    /// Errors on failure to extend selection in direction
    pub fn extend_selection_dir(
        &mut self,
        from: &TreeItem,
        to: &TreeItem,
        direction_key: Key,
        val: TreeItemSelect,
        visible: bool,
    ) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        if from.inner.is_null() || to.inner.is_null() {
            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
        }
        unsafe {
            match Fl_Tree_extend_selection_dir(
                self.inner,
                from.inner,
                to.inner,
                direction_key.bits() as i32,
                val as i32,
                visible as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Extend the selection between and including `from` and `to`
    /// # Errors
    /// Errors on failure to extend selection
    pub fn extend_selection(
        &mut self,
        from: &TreeItem,
        to: &TreeItem,
        val: TreeItemSelect,
        visible: bool,
    ) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        if from.inner.is_null() || to.inner.is_null() {
            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
        }
        unsafe {
            match Fl_Tree_extend_selection(
                self.inner,
                from.inner,
                to.inner,
                val as i32,
                visible as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Set the item that currently should have keyboard focus
    pub fn set_item_focus(&mut self, item: &TreeItem) {
        assert!(!self.was_deleted());
        assert!(!item.inner.is_null());
        unsafe { Fl_Tree_set_item_focus(self.inner, item.inner) }
    }

    /// Get the item that currently has keyboard focus
    pub fn get_item_focus(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_get_item_focus(self.inner)) }
    }

    /// Returns whether an item is selected
    pub fn is_selected(&self, path: &str) -> bool {
        assert!(!self.was_deleted());
        let path = CString::safe_new(path);
        unsafe { Fl_Tree_is_selected(self.inner, path.as_ptr() as *mut raw::c_char) != 0 }
    }

    /// Gets the items' label font
    pub fn item_label_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_item_labelfont(self.inner)) }
    }

    /// Sets the items' label font
    pub fn set_item_label_font(&mut self, val: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_item_labelfont(self.inner, val.bits() as i32) }
    }

    /// Gets the items' label size
    pub fn item_label_size(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_item_labelsize(self.inner) as i32 }
    }

    /// Sets the items' label size
    pub fn set_item_label_size(&mut self, val: i32) {
        assert!(!self.was_deleted());
        let val = if val < 1 { 1 } else { val };
        unsafe { Fl_Tree_set_item_labelsize(self.inner, val as i32) }
    }

    /// Gets the items' foreground color
    pub fn item_label_fgcolor(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_item_labelfgcolor(self.inner)) }
    }

    /// Sets the items' foreground color
    pub fn set_item_label_fgcolor(&mut self, val: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_item_labelfgcolor(self.inner, val.bits() as u32) }
    }

    /// Gets the items' background color
    pub fn item_label_bgcolor(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_item_labelbgcolor(self.inner)) }
    }

    /// Sets the items' foreground color
    pub fn set_item_label_bgcolor(&mut self, val: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_item_labelbgcolor(self.inner, val.bits() as u32) }
    }

    /// Gets the items' connector color
    pub fn connector_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_connectorcolor(self.inner)) }
    }

    /// Sets the items' foreground color
    pub fn set_connector_color(&mut self, val: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_connectorcolor(self.inner, val.bits() as u32) }
    }

    /// Gets the left margin
    pub fn margin_left(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_marginleft(self.inner) as i32 }
    }

    /// Sets the left margin
    pub fn set_margin_left(&mut self, val: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_marginleft(self.inner, val as i32) }
    }

    /// Gets the top margin
    pub fn margin_top(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_margintop(self.inner) as i32 }
    }

    /// Sets the top margin
    pub fn set_margin_top(&mut self, val: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_margintop(self.inner, val as i32) }
    }

    /// Gets the bottom margin
    pub fn margin_bottom(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_marginbottom(self.inner) as i32 }
    }

    /// Sets the bottom margin
    pub fn set_margin_bottom(&mut self, val: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_marginbottom(self.inner, val as i32) }
    }

    /// Gets the line spacing
    pub fn line_spacing(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_linespacing(self.inner) as i32 }
    }

    /// Sets the line spacing
    pub fn set_line_spacing(&mut self, val: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_linespacing(self.inner, val as i32) }
    }

    /// Gets the open child bottom margin
    pub fn open_child_margin_bottom(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_openchild_marginbottom(self.inner) as i32 }
    }

    /// Sets the open child bottom margin
    pub fn set_open_child_margin_bottom(&mut self, val: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_openchild_marginbottom(self.inner, val as i32) }
    }

    /// Gets the user icon left margin
    pub fn user_icon_margin_left(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_usericonmarginleft(self.inner) as i32 }
    }

    /// Sets the user icon left margin
    pub fn set_user_icon_margin_left(&mut self, val: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_usericonmarginleft(self.inner, val as i32) }
    }

    /// Gets the label's left margin
    pub fn label_margin_left(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_labelmarginleft(self.inner) as i32 }
    }

    /// Sets the label's left margin
    pub fn set_label_margin_left(&mut self, val: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_labelmarginleft(self.inner, val as i32) }
    }

    /// Gets the widget's left margin
    pub fn widget_margin_left(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_widgetmarginleft(self.inner) as i32 }
    }

    /// Sets the widget's left margin
    pub fn set_widget_margin_left(&mut self, val: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_widgetmarginleft(self.inner, val as i32) }
    }

    /// Gets the connector's width
    pub fn connector_width(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_connectorwidth(self.inner) as i32 }
    }

    /// Sets the connector's width
    pub fn set_connector_width(&mut self, val: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_connectorwidth(self.inner, val as i32) }
    }

    /// Gets the user icon
    pub fn user_icon(&self) -> Option<Box<dyn ImageExt>> {
        assert!(!self.was_deleted());
        unsafe {
            let image_ptr = Fl_Tree_usericon(self.inner);
            if image_ptr.is_null() {
                None
            } else {
                Some(Box::new(Image::from_image_ptr(
                    image_ptr as *mut fltk_sys::image::Fl_Image,
                )))
            }
        }
    }

    /// Sets the user icon
    pub fn set_user_icon<Img: ImageExt>(&mut self, image: Option<Img>) {
        assert!(!self.was_deleted());
        if let Some(image) = image {
            assert!(!image.was_deleted());
            unsafe { Fl_Tree_set_usericon(self.inner, image.as_image_ptr() as *mut _) }
        } else {
            unsafe { Fl_Tree_set_usericon(self.inner, std::ptr::null_mut::<raw::c_void>()) }
        }
    }

    /// Gets the open icon
    pub fn open_icon(&self) -> Option<Box<dyn ImageExt>> {
        assert!(!self.was_deleted());
        unsafe {
            let image_ptr = Fl_Tree_openicon(self.inner);
            if image_ptr.is_null() {
                None
            } else {
                Some(Box::new(Image::from_image_ptr(
                    image_ptr as *mut fltk_sys::image::Fl_Image,
                )))
            }
        }
    }

    /// Sets the open icon
    pub fn set_open_icon<Img: ImageExt>(&mut self, image: Option<Img>) {
        assert!(!self.was_deleted());
        if let Some(image) = image {
            assert!(!image.was_deleted());
            unsafe { Fl_Tree_set_openicon(self.inner, image.as_image_ptr() as *mut _) }
        } else {
            unsafe { Fl_Tree_set_openicon(self.inner, std::ptr::null_mut::<raw::c_void>()) }
        }
    }

    /// Gets the close icon
    pub fn close_icon(&self) -> Option<Box<dyn ImageExt>> {
        assert!(!self.was_deleted());
        unsafe {
            let image_ptr = Fl_Tree_closeicon(self.inner);
            if image_ptr.is_null() {
                None
            } else {
                Some(Box::new(Image::from_image_ptr(
                    image_ptr as *mut fltk_sys::image::Fl_Image,
                )))
            }
        }
    }

    /// Sets the close icon
    pub fn set_close_icon<Img: ImageExt>(&mut self, image: Option<Img>) {
        assert!(!self.was_deleted());
        if let Some(image) = image {
            assert!(!image.was_deleted());
            unsafe { Fl_Tree_set_closeicon(self.inner, image.as_image_ptr() as *mut _) }
        } else {
            unsafe { Fl_Tree_set_closeicon(self.inner, std::ptr::null_mut::<raw::c_void>()) }
        }
    }

    /// Returns true if the collapse icon is enabled, false if not.
    pub fn show_collapse(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_showcollapse(self.inner) != 0 }
    }

    /// Sets whether the collapse icon is enabled
    pub fn set_show_collapse(&mut self, flag: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_showcollapse(self.inner, flag as i32) }
    }

    /// Returns whether the root is shown
    pub fn show_root(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_showroot(self.inner) != 0 }
    }

    /// Sets whether the root is shown
    pub fn set_show_root(&mut self, flag: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_showroot(self.inner, flag as i32) }
    }

    /// Gets the connector style
    pub fn connector_style(&self) -> TreeConnectorStyle {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_connectorstyle(self.inner)) }
    }

    /// Sets the connector style
    pub fn set_connector_style(&mut self, val: TreeConnectorStyle) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_connectorstyle(self.inner, val as i32) }
    }

    /// Gets the sort order
    pub fn sort_order(&self) -> TreeSort {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_sortorder(self.inner)) }
    }

    /// Sets the sort order
    pub fn set_sort_order(&mut self, val: TreeSort) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_sortorder(self.inner, val as i32) }
    }

    /// Gets the select frame
    pub fn select_frame(&self) -> FrameType {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_selectbox(self.inner)) }
    }

    /// Sets the select frame
    pub fn set_select_frame(&mut self, val: FrameType) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_selectbox(self.inner, val as i32) }
    }

    /// Gets the Tree select mode
    pub fn select_mode(&self) -> TreeSelect {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_selectmode(self.inner)) }
    }

    /// Sets the Tree select mode
    pub fn set_select_mode(&mut self, val: TreeSelect) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_selectmode(self.inner, val as i32) }
    }

    /// Gets the Tree item's reselect mode
    pub fn item_reselect_mode(&self) -> TreeItemReselectMode {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_item_reselect_mode(self.inner)) }
    }

    /// Sets the Tree item's reselect mode
    pub fn set_item_reselect_mode(&mut self, mode: TreeItemReselectMode) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_item_reselect_mode(self.inner, mode as i32) }
    }

    /// Gets the Tree item's draw mode
    pub fn item_draw_mode(&self) -> TreeItemDrawMode {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_item_draw_mode(self.inner)) }
    }

    /// Sets the Tree item's draw mode
    pub fn set_item_draw_mode(&mut self, mode: TreeItemDrawMode) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_item_draw_mode(self.inner, mode as i32) }
    }

    /// Recalculate widget dimensions and scrollbar visibility, normally done automatically
    pub fn calc_dimensions(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_calc_dimensions(self.inner) }
    }

    /// Recalculates the tree's sizes and scrollbar visibility, normally done automatically
    pub fn calc_tree(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_calc_tree(self.inner) }
    }

    /// Recalculates the tree's sizes and scrollbar visibility, normally done automatically
    pub fn recalc_tree(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_recalc_tree(self.inner) }
    }

    /// Returns whether an item is displayed
    pub fn displayed(&mut self, item: &TreeItem) -> bool {
        assert!(!self.was_deleted());
        assert!(!item.inner.is_null());
        unsafe { Fl_Tree_displayed(self.inner, item.inner) != 0 }
    }

    /// Shows an item
    pub fn show_item(&mut self, item: &TreeItem, y_offset: i32) {
        assert!(!self.was_deleted());
        assert!(!item.inner.is_null());
        unsafe { Fl_Tree_show_item(self.inner, item.inner, y_offset) }
    }

    /// Adjust the vertical scrollbar so that `item` is visible
    pub fn show_item_top(&mut self, item: &TreeItem) {
        assert!(!self.was_deleted());
        assert!(!item.inner.is_null());
        unsafe { Fl_Tree_show_item_top(self.inner, item.inner) }
    }

    /// Adjust the vertical scrollbar so that `item` is in the middle of the display
    pub fn show_item_middle(&mut self, item: &TreeItem) {
        assert!(!self.was_deleted());
        assert!(!item.inner.is_null());
        unsafe { Fl_Tree_show_item_middle(self.inner, item.inner) }
    }

    /// Adjust the vertical scrollbar so that the is at the bottom of the display.
    pub fn show_item_bottom(&mut self, item: &TreeItem) {
        assert!(!self.was_deleted());
        assert!(!item.inner.is_null());
        unsafe { Fl_Tree_show_item_bottom(self.inner, item.inner) }
    }

    /// Display the item
    pub fn display(&mut self, item: &TreeItem) {
        assert!(!self.was_deleted());
        assert!(!item.inner.is_null());
        unsafe { Fl_Tree_display(self.inner, item.inner) }
    }

    /// Gets the vertical position of the item
    pub fn vposition(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_vposition(self.inner) }
    }

    /// Sets the vertical position of the item
    pub fn set_vposition(&mut self, pos: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_vposition(self.inner, pos) }
    }

    /// Gets the horizontal position of the item
    pub fn hposition(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_hposition(self.inner) }
    }

    /// Sets the horizontal position of the item
    pub fn set_hposition(&mut self, pos: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_hposition(self.inner, pos) }
    }

    /// Returns whether the widget is a scrollbar
    pub fn is_scrollbar<W: WidgetExt>(&mut self, w: &W) -> bool {
        assert!(!w.was_deleted());
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_is_scrollbar(self.inner, w.as_widget_ptr() as *mut Fl_Widget) != 0 }
    }

    /// Gets the scrollbar size
    pub fn scrollbar_size(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_scrollbar_size(self.inner) }
    }

    /// Sets the scrollbar size
    pub fn set_scrollbar_size(&mut self, sz: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_scrollbar_size(self.inner, sz as i32) }
    }

    /// Returns whether vertical scrolling is visible
    pub fn is_vscroll_visible(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_is_vscroll_visible(self.inner) != 0 }
    }

    /// Returns whether horizontal scrolling is visible
    pub fn is_hscroll_visible(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_is_hscroll_visible(self.inner) != 0 }
    }

    /// Set the callback item
    pub fn set_callback_item(&mut self, item: &TreeItem) {
        assert!(!self.was_deleted());
        assert!(!item.inner.is_null());
        unsafe { Fl_Tree_set_callback_item(self.inner, item.inner) }
    }

    /// Get the callback item
    pub fn callback_item(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_callback_item(self.inner)) }
    }

    /// Set the callback reason
    pub fn set_callback_reason(&mut self, reason: TreeReason) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_set_callback_reason(self.inner, reason as i32) }
    }

    /// Get the callback reason
    pub fn callback_reason(&self) -> TreeReason {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_callback_reason(self.inner)) }
    }

    /// Get an item's pathname
    pub fn item_pathname(&self, item: &TreeItem) -> Result<String, FltkError> {
        assert!(!self.was_deleted());
        let mut temp = vec![0u8; 256];
        unsafe {
            let ret = Fl_Tree_item_pathname(self.inner, temp.as_mut_ptr() as _, 256, item.inner);
            if ret == 0 {
                if let Some(pos) = temp.iter().position(|x| *x == 0) {
                    temp = temp.split_at(pos).0.to_vec();
                }
                Ok(String::from_utf8_lossy(&temp).to_string())
            } else {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            }
        }
    }
}

impl IntoIterator for Tree {
    type Item = TreeItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.get_items().map_or_else(
            || Vec::with_capacity(0).into_iter(),
            std::iter::IntoIterator::into_iter,
        )
    }
}

impl TreeItem {
    /// Create a `TreeItem` from a raw pointer
    /// # Safety
    /// The pointer must be valid
    pub unsafe fn from_raw(ptr: *mut Fl_Tree_Item) -> Option<TreeItem> {
        if ptr.is_null() {
            None
        } else {
            let inner = Fl_Tree_Item_tree(ptr) as *mut _;
            let tracker =
                fltk_sys::fl::Fl_Widget_Tracker_new(inner as *mut fltk_sys::fl::Fl_Widget);
            assert!(!tracker.is_null());
            let tree = Tree {
                inner,
                tracker,
                is_derived: false,
            };
            let parent = Fl_Tree_Item_parent(ptr);
            let is_root = Fl_Tree_Item_is_root(ptr) != 0;
            Some(TreeItem {
                inner: ptr,
                parent,
                tree,
                is_root,
                is_derived: false,
            })
        }
    }

    /// Creates a new TreeItem
    pub fn new(tree: &Tree, label: &str) -> Self {
        let label = CString::safe_new(label);
        unsafe {
            let ptr = Fl_Tree_Item_new(tree.inner, label.as_ptr());
            assert!(!ptr.is_null());
            Self {
                inner: ptr,
                parent: ptr,
                tree: tree.clone(),
                is_root: true,
                is_derived: true,
            }
        }
    }

    /**
    Overrides the draw_item_content method
    Example usage:
       ```rust,no_run
       use fltk::{draw, enums::*, tree};
       let mut tree = tree::Tree::default();
       let mut item = tree::TreeItem::new(&tree, "Hello");
       item.draw_item_content(|item, render| {
           // Our item's dimensions + text content
           let x = item.label_x();
           let y = item.label_y();
           let w = item.label_w();
           let h = item.label_h();
           let txt = if let Some(txt) = item.label() {
               txt
           } else {
               String::new()
           };
           if render {
               // Draw bg -- a filled rectangle
               draw::draw_rect_fill(x, y, w, h, item.label_bgcolor());
               // Draw label
               draw::set_font(Font::Helvetica, 14);
               draw::set_draw_color(Color::ForeGround); // use recommended fg color
               draw::draw_text2(&txt, x, y, w, h, Align::Left); // draw the item's label
           }
           // Rendered or not, we must calculate content's max X position
           let (lw, _) = draw::measure(&txt, true); // get width of label text
           return x + lw; // return X + label width
       });
       // Add our custom item to a path
       let _third = tree.add_item("first/second/thrid", &item).unwrap();
       ```
    */
    pub fn draw_item_content<F: FnMut(&mut Self, bool) -> i32>(&mut self, cb: F) {
        assert!(!self.was_deleted());
        assert!(self.is_derived);
        unsafe {
            unsafe extern "C" fn shim(
                item: *mut Fl_Tree_Item,
                render: i32,
                data: *mut raw::c_void,
            ) -> i32 {
                let mut item = TreeItem::from_raw(item).unwrap();
                let a = data as *mut Box<dyn FnMut(&mut TreeItem, bool) -> i32>;
                let f: &mut (dyn FnMut(&mut TreeItem, bool) -> i32) = &mut **a;
                if let Ok(ret) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f(&mut item, render != 0)
                })) {
                    ret
                } else {
                    0
                }
            }
            let a: *mut Box<dyn FnMut(&mut Self, bool) -> i32> =
                Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut raw::c_void = a as *mut raw::c_void;
            let callback: Option<
                unsafe extern "C" fn(
                    self_: *mut Fl_Tree_Item,
                    arg1: i32,
                    arg2: *mut raw::c_void,
                ) -> i32,
            > = Some(shim);
            Fl_Tree_Item_draw_item_content(self.inner, callback, data);
        }
    }

    /// Set the internal data of the tree item
    /// # Warning
    /// This method doesn't store the type information of the passed data
    pub fn set_user_data<T: Clone + 'static>(&mut self, data: T) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Tree_Item_set_user_data(self.inner, Box::into_raw(Box::from(data)) as _);
        }
    }

    /// Get the stored data in the tree item
    /// # Safety
    /// Setting the user data doesn't store type information, as such it's on the developer to maintain the correct type
    pub unsafe fn user_data<T: Clone + 'static>(&self) -> Option<T> {
        assert!(!self.was_deleted());
        let ptr = Fl_Tree_Item_user_data(self.inner);
        if ptr.is_null() {
            None
        } else {
            let data = ptr as *const _ as *mut T;
            Some((*data).clone())
        }
    }

    /// Gets the x position
    pub fn x(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_x(self.inner) }
    }

    /// Gets the y position
    pub fn y(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_y(self.inner) }
    }

    /// Gets the width
    pub fn w(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_w(self.inner) }
    }

    /// Gets the height
    pub fn h(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_h(self.inner) }
    }

    /// Gets the label's x position
    pub fn label_x(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_label_x(self.inner) }
    }

    /// Gets the label's y position
    pub fn label_y(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_label_y(self.inner) }
    }

    /// Gets the label's width
    pub fn label_w(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_label_w(self.inner) }
    }

    /// Gets the label's height
    pub fn label_h(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_label_h(self.inner) }
    }

    /// Shows the tree item
    pub fn show_self(&self, indent: &str) {
        assert!(!self.was_deleted());
        let indent = CString::safe_new(indent);
        unsafe { Fl_Tree_Item_show_self(self.inner, indent.as_ptr() as *mut raw::c_char) }
    }

    /// Sets the label of the tree item
    pub fn set_label(&mut self, val: &str) {
        assert!(!self.was_deleted());
        let val = CString::safe_new(val);
        unsafe { Fl_Tree_set_Item_label(self.inner, val.as_ptr() as *mut raw::c_char) }
    }

    /// Gets the label of the tree item
    pub fn label(&self) -> Option<String> {
        assert!(!self.was_deleted());
        unsafe {
            let x = Fl_Tree_Item_label(self.inner);
            if x.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(x as *mut raw::c_char)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }

    /// Sets the label's font
    pub fn set_label_font(&mut self, val: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_set_labelfont(self.inner, val.bits() as i32) }
    }

    /// Gets the label's font
    pub fn label_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_Item_labelfont(self.inner)) }
    }

    /// Sets the label's size
    pub fn set_label_size(&mut self, sz: i32) {
        assert!(!self.was_deleted());
        let sz = if sz < 1 { 1 } else { sz };
        unsafe { Fl_Tree_Item_set_labelsize(self.inner, sz) }
    }

    /// Gets the label's size
    pub fn label_size(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_labelsize(self.inner) }
    }

    /// Sets the label's foreground color
    pub fn set_label_fgcolor(&mut self, val: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_set_labelfgcolor(self.inner, val.bits() as u32) }
    }

    /// Gets the label's foreground color
    pub fn label_fgcolor(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_Item_labelfgcolor(self.inner)) }
    }

    #[deprecated(since = "1.2.19", note = "please use `set_label_fgcolor` instead")]
    /// Sets the label's foreground color
    pub fn set_label_fg_color(&mut self, val: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_set_labelfgcolor(self.inner, val.bits() as u32) }
    }

    #[deprecated(since = "1.2.19", note = "please use `label_fgcolor` instead")]
    /// Gets the label's foreground color
    pub fn label_fg_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_Item_labelfgcolor(self.inner)) }
    }

    /// Sets the label's color
    pub fn set_label_color(&mut self, val: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_set_labelcolor(self.inner, val.bits() as u32) }
    }

    /// Gets the label's color
    pub fn label_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_Item_labelcolor(self.inner)) }
    }

    /// Sets the label's background color
    pub fn set_label_bgcolor(&mut self, val: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_set_labelbgcolor(self.inner, val.bits() as u32) }
    }

    /// Gets the label's foreground color
    pub fn label_bgcolor(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tree_Item_labelbgcolor(self.inner)) }
    }

    /// Sets the item's associated widget
    pub fn set_widget<W: WidgetExt>(&mut self, val: &W) {
        assert!(!val.was_deleted());
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_set_widget(self.inner, val.as_widget_ptr() as *mut Fl_Widget) }
    }

    #[deprecated(since = "1.2.18", note = "please use `try_widget` instead")]
    /// Gets the item's associated widget
    pub fn widget(&self) -> Widget {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Tree_Item_widget(self.inner) as *mut fltk_sys::widget::Fl_Widget;
            assert!(!ptr.is_null());
            Widget::from_widget_ptr(ptr)
        }
    }

    /// Gets the item's associated widget
    pub fn try_widget(&self) -> Option<impl WidgetExt> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Tree_Item_widget(self.inner) as *mut fltk_sys::widget::Fl_Widget;
            if ptr.is_null() {
                None
            } else {
                Some(Widget::from_widget_ptr(ptr))
            }
        }
    }

    /// Gets the children count
    pub fn children(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_children(self.inner) as i32 }
    }

    /// Gets the child item at idx position
    pub fn child(&self, idx: i32) -> Option<TreeItem> {
        if idx < 0 || idx >= self.children() {
            return None;
        }
        assert!(!self.was_deleted());
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_child(self.inner, idx as i32) as *mut Fl_Tree_Item)
        }
    }

    /// Returns whether the item has children
    pub fn has_children(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_has_children(self.inner) != 0 }
    }

    /// Find a child using its name, returns index result
    /// # Errors
    /// Errors on failure to find child
    pub fn find_child(&mut self, name: &str) -> Result<i32, FltkError> {
        assert!(!self.was_deleted());
        let name = CString::safe_new(name);
        unsafe {
            let x = Fl_Tree_Item_find_child(self.inner, name.as_ptr());
            if x == -1 {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            } else {
                Ok(x as i32)
            }
        }
    }

    /// Remove child using its name
    /// # Errors
    /// Errors on failure to remove child
    pub fn remove_child(&mut self, new_label: &str) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        let new_label = CString::safe_new(new_label);
        unsafe {
            match Fl_Tree_Item_remove_child(self.inner, new_label.as_ptr()) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Remove all children
    pub fn clear_children(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_clear_children(self.inner) }
    }

    /// Swap children a and b
    /// # Errors
    /// Errors on failure to swap children
    pub fn swap_children(&mut self, a: &TreeItem, b: &TreeItem) -> Result<(), FltkError> {
        assert!(!self.was_deleted() && !a.was_deleted() && !b.was_deleted());
        unsafe {
            match Fl_Tree_Item_swap_children(self.inner, a.inner, b.inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Find child by name, returns option of the item
    pub fn find_child_item(&self, name: &str) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        let name = CString::safe_new(name);
        unsafe {
            TreeItem::from_raw(
                Fl_Tree_Item_find_child_item(self.inner, name.as_ptr()) as *mut Fl_Tree_Item
            )
        }
    }

    /// Replace a tree item
    pub fn replace(&mut self, new_item: &TreeItem) -> Option<TreeItem> {
        assert!(!self.was_deleted() && !new_item.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_Item_replace(self.inner, new_item.inner)) }
    }

    /// Replace a child
    pub fn replace_child(&mut self, old_item: &TreeItem, new_item: &TreeItem) -> Option<TreeItem> {
        assert!(!self.was_deleted() && !old_item.was_deleted() && !new_item.was_deleted());
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_replace_child(
                self.inner,
                old_item.inner,
                new_item.inner,
            ))
        }
    }

    /// Deparent a child by index
    pub fn deparent(&mut self, index: i32) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        if index < 0 || index >= self.children() {
            return None;
        }
        unsafe { TreeItem::from_raw(Fl_Tree_Item_deparent(self.inner, index as i32)) }
    }

    /// Reparent a child by index
    /// # Errors
    /// Errors on failure to move item   
    pub fn reparent(&mut self, new_child: &TreeItem, index: i32) -> Result<(), FltkError> {
        assert!(!self.was_deleted() && !new_child.was_deleted());
        if index < 0 || index >= self.children() {
            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
        }
        unsafe {
            match Fl_Tree_Item_reparent(self.inner, new_child.inner, index as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Move the item 'from' to sibling position of 'to'.
    /// # Errors
    /// Errors on failure to move item   
    pub fn move_item(&mut self, to: i32, from: i32) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Tree_Item_move(self.inner, to as i32, from as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Move the current item above the specified `item`
    /// # Errors
    /// Errors on failure to move item   
    pub fn move_above(&mut self, item: &TreeItem) -> Result<(), FltkError> {
        assert!(!self.was_deleted() && !item.was_deleted());
        unsafe {
            match Fl_Tree_Item_move_above(self.inner, item.inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Move the current item below the specified `item`
    /// # Errors
    /// Errors on failure to move item   
    pub fn move_below(&mut self, item: &TreeItem) -> Result<(), FltkError> {
        assert!(!self.was_deleted() && !item.was_deleted());
        unsafe {
            match Fl_Tree_Item_move_below(self.inner, item.inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Parent the current item as a child of the specified `item`.
    /// # Errors
    /// Errors on failure to move item    
    pub fn move_into(&mut self, item: &TreeItem, pos: i32) -> Result<(), FltkError> {
        assert!(!self.was_deleted() && !item.was_deleted());
        unsafe {
            match Fl_Tree_Item_move_into(self.inner, item.inner, pos as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Gets the depth of the item
    pub fn depth(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_depth(self.inner) as i32 }
    }

    /// Gets the previous item
    pub fn prev(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_Item_prev(self.inner)) }
    }

    /// Gets the next item
    pub fn next(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_Item_next(self.inner)) }
    }

    /// Gets the next sibling
    pub fn next_sibling(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_Item_next_sibling(self.inner)) }
    }

    /// Gets the previous sibling
    pub fn prev_sibling(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_Item_prev_sibling(self.inner)) }
    }

    /// Update surrounding siblings
    pub fn update_prev_next(&mut self, index: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_update_prev_next(self.inner, index as i32) }
    }

    /// Return the parent of the item
    pub fn parent(&self) -> Option<TreeItem> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_Item_parent(self.inner) as *mut Fl_Tree_Item) }
    }

    /// Set the parent of the item
    pub fn set_parent(&mut self, val: &TreeItem) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_set_parent(self.inner, val.inner) }
    }

    /// Return the tree of the item
    pub fn tree(&self) -> Option<Tree> {
        assert!(!self.was_deleted());
        unsafe { Tree::from_raw(Fl_Tree_Item_tree(self.inner) as *mut Fl_Tree) }
    }

    /// Open the item exposing all children
    pub fn open(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_open(self.inner) }
    }

    /// Close the item hiding all children
    pub fn close(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_close(self.inner) }
    }

    /// Returns whether an item is open
    pub fn is_open(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_is_open(self.inner) != 0 }
    }

    /// Returns whether an item is closed
    pub fn is_close(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_is_close(self.inner) != 0 }
    }

    /// Toggle the open state of the item
    pub fn open_toggle(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_open_toggle(self.inner) }
    }

    /// Select an item at index
    pub fn select(&mut self, index: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_select(self.inner, index as i32) }
    }

    /// Toggle the select state of an item
    pub fn select_toggle(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_select_toggle(self.inner) }
    }

    /// Select all subitems, returns number of selected items
    pub fn select_all(&mut self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_select_all(self.inner) as i32 }
    }

    /// Deselect an item
    pub fn deselect(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_deselect(self.inner) }
    }

    /// Deselect all subitems
    pub fn deselect_all(&mut self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_deselect_all(self.inner) as i32 }
    }

    /// Returns whether an item is root
    pub fn is_root(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_is_root(self.inner) != 0 }
    }

    /// Returns whether an item is visible
    pub fn is_visible(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_is_visible(self.inner) != 0 }
    }

    /// Returns whether an item is active
    pub fn is_active(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_is_active(self.inner) != 0 }
    }

    /// Returns whether an item is activated
    pub fn is_activated(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_is_activated(self.inner) != 0 }
    }

    /// Deactivate an item
    pub fn deactivate(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_deactivate(self.inner) }
    }

    /// Activate an item
    pub fn activate(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_activate(self.inner, val as i32) }
    }

    /// Returns whether an item is selected
    pub fn is_selected(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Tree_Item_is_selected(self.inner) != 0 }
    }

    /// Check if the tree item was deleted
    pub fn was_deleted(&self) -> bool {
        unsafe {
            let parent = self.parent;
            let is_root = self.is_root;
            if self.tree.was_deleted() {
                return true;
            }
            if is_root {
                self.tree.root().is_none() || self.inner.is_null()
            } else {
                Fl_Tree_Item_children(parent) == 0 || self.inner.is_null()
            }
        }
    }

    /// Gets the user icon
    pub fn user_icon(&self) -> Option<Box<dyn ImageExt>> {
        assert!(!self.was_deleted());
        unsafe {
            let image_ptr = Fl_Tree_Item_usericon(self.inner);
            if image_ptr.is_null() {
                None
            } else {
                Some(Box::new(Image::from_image_ptr(
                    image_ptr as *mut fltk_sys::image::Fl_Image,
                )))
            }
        }
    }

    /// Sets the user icon
    pub fn set_user_icon<Img: ImageExt>(&mut self, image: Option<Img>) {
        assert!(!self.was_deleted());
        if let Some(image) = image {
            assert!(!image.was_deleted());
            unsafe { Fl_Tree_Item_set_usericon(self.inner, image.as_image_ptr() as *mut _) }
        } else {
            unsafe { Fl_Tree_Item_set_usericon(self.inner, std::ptr::null_mut::<raw::c_void>()) }
        }
    }

    /// Return the internal pointer of the tree item
    pub fn as_ptr(&self) -> *mut Fl_Tree_Item {
        self.inner
    }
}

impl Iterator for TreeItem {
    type Item = TreeItem;
    /// Gets the next item
    fn next(&mut self) -> Option<Self::Item> {
        assert!(!self.was_deleted());
        unsafe { TreeItem::from_raw(Fl_Tree_Item_next(self.inner)) }
    }
}

impl TreeItemArray {
    /// Returns the len() of the array
    fn total(&self) -> i32 {
        unsafe { Fl_Tree_Item_Array_total(self.inner) as i32 }
    }

    /// Swaps children
    #[allow(dead_code)]
    fn swap(&mut self, ax: i32, bx: i32) {
        unsafe { Fl_Tree_Item_Array_swap(self.inner, ax, bx) }
    }

    /// Move items
    #[allow(dead_code)]
    fn move_item(&mut self, to: i32, from: i32) -> i32 {
        unsafe { Fl_Tree_Item_Array_move(self.inner, to, from) }
    }

    /// Deparent item
    #[allow(dead_code)]
    fn deparent(&mut self, pos: i32) -> i32 {
        unsafe { Fl_Tree_Item_Array_deparent(self.inner, pos) }
    }

    /// Reparent item
    #[allow(dead_code)]
    fn reparent(&mut self, item: &TreeItem, newparent: &TreeItem, pos: i32) -> i32 {
        unsafe { Fl_Tree_Item_Array_reparent(self.inner, item.inner, newparent.inner, pos) }
    }

    /// Clears the array
    #[allow(dead_code)]
    fn clear(&mut self) {
        unsafe { Fl_Tree_Item_Array_clear(self.inner) }
    }

    /// Adds an item to the array
    #[allow(dead_code)]
    fn add(&mut self, val: &TreeItem) {
        unsafe { Fl_Tree_Item_Array_add(self.inner, val.inner) }
    }

    /// Insert an item to the array at pos
    #[allow(dead_code)]
    fn insert(&mut self, pos: i32, new_item: &TreeItem) {
        unsafe { Fl_Tree_Item_Array_insert(self.inner, pos as i32, new_item.inner) }
    }

    /// Replace an item at pos
    #[allow(dead_code)]
    fn replace(&mut self, pos: i32, new_item: &TreeItem) {
        unsafe { Fl_Tree_Item_Array_replace(self.inner, pos, new_item.inner) }
    }

    /// Remove an item at pos
    #[allow(dead_code)]
    fn remove(&mut self, index: i32) {
        unsafe { Fl_Tree_Item_Array_remove(self.inner, index) }
    }

    /// Remove an item
    #[allow(dead_code)]
    fn remove_item(&mut self, item: &TreeItem) -> i32 {
        unsafe { Fl_Tree_Item_Array_remove_item(self.inner, item.inner) }
    }

    /// Gets the item at idx
    fn at(&self, idx: i32) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_Item_Array_at(self.inner, idx as i32)) }
    }

    /// Transforms the `TreeItemArray` into a vector
    fn into_vec(self) -> Option<Vec<TreeItem>> {
        let c = self.total();
        let mut v: Vec<TreeItem> = vec![];
        if c == 0 {
            None
        } else {
            for i in 0..c {
                let val = self.at(i);
                val.as_ref()?;
                v.push(val.unwrap());
            }
            Some(v)
        }
    }
}

impl IntoIterator for TreeItemArray {
    type Item = TreeItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().map_or_else(
            || Vec::with_capacity(0).into_iter(),
            std::iter::IntoIterator::into_iter,
        )
    }
}

unsafe impl Send for TreeItem {}

unsafe impl Sync for TreeItem {}

impl PartialEq for TreeItem {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for TreeItem {}

impl Drop for TreeItemArray {
    fn drop(&mut self) {
        unsafe { Fl_Tree_Item_Array_delete(self.inner) }
    }
}
