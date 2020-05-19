use crate::image::Image;
pub use crate::prelude::*;
use crate::widget::Widget;
use fltk_sys::tree::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Defines the Tree sort order
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TreeSort {
    None = 0,
    Ascending = 1,
    Descending = 2,
}

/// Defines the Tree's connector sort order
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TreeConnectorStyle {
    None = 0,
    Dotted = 1,
    Solid = 2,
}

/// Defines the Tree select mode
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TreeSelect {
    None = 0,
    Single = 1,
    Multi = 2,
    SingleDraggable = 3,
}

/// Defines the TreeItem's select mode
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TreeItemSelect {
    Deselect = 0,
    Select = 1,
    Toggle = 2,
}

/// Defines the Tree's callback reason
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TreeReason {
    None = 0,
    Selected,
    Deselected,
    Reselected,
    Opened,
    Closed,
    Dragged,
}

/// Defines the TreeItem's reselect mode
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TreeItemReselectMode {
    Once = 0,
    Always,
}

/// Defines the TreeItem's draw mode
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TreeItemDrawMode {
    Default = 0,
    LabelAndWidget = 1,
    HeightFromWidget = 2,
}

/// Defines a tree widget
#[derive(WidgetExt, Debug)]
pub struct Tree {
    _inner: *mut Fl_Tree,
}

/// Defines a tree item
#[derive(Debug)]
pub struct TreeItem {
    _inner: *mut Fl_Tree_Item,
}

/// Defines a tree item array
#[derive(Debug)]
struct TreeItemArray {
    _inner: *mut Fl_Tree_Item_Array,
}

impl Tree {
    /// Creates a Tree from a raw Fl_Tree pointer
    pub unsafe fn from_raw(ptr: *mut Fl_Tree) -> Option<Tree> {
        if ptr.is_null() {
            None
        } else {
            let x = Tree { _inner: ptr };
            Some(x)
        }
    }

    /// Begins the Tree widget
    pub fn begin(&self) {
        unsafe { Fl_Tree_begin(self._inner) }
    }

    /// Ends the Tree widget
    pub fn end(&self) {
        unsafe { Fl_Tree_end(self._inner) }
    }

    /// Shows the Tree widget
    pub fn show_self(&mut self) {
        unsafe { Fl_Tree_show_self(self._inner) }
    }

    /// Sets the root label
    pub fn set_root_label(&mut self, new_label: &str) {
        let new_label = CString::new(new_label).unwrap();
        unsafe { Fl_Tree_root_label(self._inner, new_label.into_raw() as *const raw::c_char) }
    }

    /// Gets the Tree's root
    pub fn root(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_root(self._inner)) }
    }

    /// Sets the Tree's root
    pub fn set_root(&mut self, new_item: Option<TreeItem>) {
        let ptr = match new_item {
            None => std::ptr::null_mut(),
            Some(item) => item._inner,
        };
        unsafe { Fl_Tree_set_root(self._inner, ptr) }
    }

    /// Adds a TreeItem
    pub fn add(&mut self, path: &str) -> Option<TreeItem> {
        let path = CString::new(path).unwrap();
        unsafe {
            let x = Fl_Tree_add(self._inner, path.into_raw() as *mut raw::c_char);
            TreeItem::from_raw(x)
        }
    }

    /// Inserts a TreeItem above another tree item
    pub fn insert_above(&mut self, above: TreeItem, name: &str) -> Option<TreeItem> {
        let name = CString::new(name).unwrap();
        unsafe {
            let x = Fl_Tree_insert_above(
                self._inner,
                above._inner,
                name.into_raw() as *mut raw::c_char,
            );
            TreeItem::from_raw(x)
        }
    }

    /// Inserts a TreeItem at a position ```pos```
    pub fn insert(&mut self, item: TreeItem, name: &str, pos: u32) -> Option<TreeItem> {
        debug_assert!(pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        let name = CString::new(name).unwrap();
        unsafe {
            let x = Fl_Tree_insert(
                self._inner,
                item._inner,
                name.into_raw() as *mut raw::c_char,
                pos as i32,
            );
            TreeItem::from_raw(x)
        }
    }

    /// Removes a TreeItem
    pub fn remove(&mut self, item: TreeItem) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_remove(self._inner, item._inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Clears a tree
    pub fn clear(&mut self) {
        unsafe { Fl_Tree_clear(self._inner) }
    }

    /// Clears all children
    pub fn clear_children(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_clear_children(self._inner as *mut Fl_Tree, item._inner) }
    }

    /// Finds a tree item
    pub fn find_item(&self, path: &str) -> Option<TreeItem> {
        let path = CString::new(path).unwrap();
        unsafe {
            let x = Fl_Tree_find_item(self._inner, path.into_raw() as *mut raw::c_char);
            if x.is_null() {
                None
            } else {
                TreeItem::from_raw(x as *mut Fl_Tree_Item)
            }
        }
    }

    /// finds a clicked item
    pub fn find_clicked(&self, yonly: bool) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_find_clicked(self._inner, yonly as i32) as *mut Fl_Tree_Item) }
    }

    /// Set the item that was last clicked.
    pub fn set_item_clicked(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_item_clicked(self._inner)) }
    }

    /// Gets the first tree item
    pub fn first(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_first(self._inner)) }
    }

    /// Gets the first visible tree item
    pub fn first_visible_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_first_visible_item(self._inner)) }
    }

    /// Gets the next tree item
    pub fn next(&mut self, item: TreeItem) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_next(self._inner, item._inner)) }
    }

    /// Gets the previous tree item
    pub fn prev(&mut self, item: TreeItem) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_prev(self._inner, item._inner)) }
    }

    /// Gets the last tree item
    pub fn last(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_last(self._inner)) }
    }

    /// Gets the last visible tree item
    pub fn last_visible_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_last_visible_item(self._inner)) }
    }

    /// Gets the next visible tree item
    pub fn next_visible_item(&mut self, start: TreeItem, direction_key: Key) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_next_visible_item(
                self._inner,
                start._inner,
                direction_key as i32,
            ))
        }
    }

    /// Gets the first selected tree item
    pub fn first_selected_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_first_selected_item(self._inner)) }
    }

    /// Gets the last selected tree item
    pub fn last_selected_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_last_selected_item(self._inner)) }
    }

    /// Gets the next tree item, direction_key is by default Key::Down
    pub fn next_item(&mut self, item: TreeItem, direction_key: Key, visible: bool) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_next_item(
                self._inner,
                item._inner,
                direction_key as i32,
                visible as i32,
            ))
        }
    }

    /// Gets the next selected tree item, direction_key is by default Key::Down
    pub fn next_selected_item(&mut self, item: TreeItem, direction_key: Key) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_next_selected_item(
                self._inner,
                item._inner,
                direction_key as i32,
            ))
        }
    }

    /// Gets the selected tree items
    pub fn get_selected_items(&mut self) -> Option<Vec<TreeItem>> {
        unsafe {
            let mut items = TreeItemArray {
                _inner: std::ptr::null_mut(),
            };
            let ret = Fl_Tree_get_selected_items(self._inner, &mut items._inner);
            if ret == 0 {
                None
            } else {
                items.into_vec()
            }
        }
    }

    /// Gets the all tree items
    pub fn get_items(&self) -> Option<Vec<TreeItem>> {
        unsafe {
            let mut items = TreeItemArray {
                _inner: std::ptr::null_mut(),
            };
            let ret = Fl_Tree_get_items(self._inner, &mut items._inner);
            if ret == 0 {
                None
            } else {
                items.into_vec()
            }
        }
    }

    /// Opens a tree item, causing the children to be shown
    pub fn open(&mut self, path: &str, do_callback: bool) -> Result<(), FltkError> {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_open(
                self._inner,
                path.into_raw() as *mut raw::c_char,
                do_callback as i32,
            ) {
                0 => Ok(()),
                1 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Toggle the open state
    pub fn open_toggle(&mut self, item: TreeItem, do_callback: bool) {
        unsafe { Fl_Tree_open_toggle(self._inner, item._inner, do_callback as i32) }
    }

    /// Close a tree item, causing the children to be hidden
    pub fn close(&mut self, path: &str, do_callback: bool) -> Result<(), FltkError> {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_close(
                self._inner,
                path.into_raw() as *mut raw::c_char,
                do_callback as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Returns whether an item is open
    pub fn is_open(&self, path: &str) -> bool {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_is_open(self._inner, path.into_raw() as *mut raw::c_char) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Returns whether an item is closed
    pub fn is_close(&self, path: &str) -> bool {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_is_close(self._inner, path.into_raw() as *mut raw::c_char) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Select a tree item
    pub fn select(&mut self, path: &str, do_callback: bool) -> Result<(), FltkError> {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_select(
                self._inner,
                path.into_raw() as *mut raw::c_char,
                do_callback as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Toggle the select state of the specified
    pub fn select_toggle(&mut self, item: TreeItem, do_callback: bool) {
        unsafe { Fl_Tree_select_toggle(self._inner, item._inner, do_callback as i32) }
    }

    pub fn deselect(&mut self, path: &str, do_callback: bool) -> Result<(), FltkError> {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_deselect(
                self._inner,
                path.into_raw() as *mut raw::c_char,
                do_callback as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Deselect all items
    pub fn deselect_all(&mut self, item: TreeItem, do_callback: bool) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_deselect_all(self._inner, item._inner, do_callback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Select only the specified item, deselecting all others that might be selected.
    pub fn select_only(&mut self, selected_item: TreeItem, do_callback: bool) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_select_only(self._inner, selected_item._inner, do_callback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Select all items
    pub fn select_all(&mut self, item: TreeItem, do_callback: bool) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_select_all(self._inner, item._inner, do_callback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Extend the selection between and including ```from``` and ```to``` in a certain direction
    pub fn extend_selection_dir(
        &mut self,
        from: TreeItem,
        to: TreeItem,
        direction_key: Key,
        val: TreeItemSelect,
        visible: bool,
    ) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_extend_selection_dir(
                self._inner,
                from._inner,
                to._inner,
                direction_key as i32,
                val as i32,
                visible as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Extend the selection between and including ```from``` and ```to```
    pub fn extend_selection(
        &mut self,
        from: TreeItem,
        to: TreeItem,
        val: TreeItemSelect,
        visible: bool,
    ) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_extend_selection(self._inner, from._inner, to._inner, val as i32, visible as i32)
            {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Set the item that currently should have keyboard focus
    pub fn set_item_focus(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_set_item_focus(self._inner, item._inner) }
    }

    /// Get the item that currently has keyboard focus
    pub fn get_item_focus(&self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_get_item_focus(self._inner)) }
    }

    /// Returns whether an item is selected
    pub fn is_selected(&self, path: &str) -> bool {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_is_selected(self._inner, path.into_raw() as *mut raw::c_char) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Gets the items' label font
    pub fn item_label_font(&self) -> Font {
        unsafe { mem::transmute(Fl_Tree_item_labelfont(self._inner)) }
    }

    /// Sets the items' label font
    pub fn set_item_label_font(&mut self, val: Font) {
        unsafe { Fl_Tree_set_item_labelfont(self._inner, val as i32) }
    }

    /// Gets the items' label size
    pub fn item_label_size(&self) -> u32 {
        unsafe { Fl_Tree_item_labelsize(self._inner) as u32 }
    }

    /// Sets the items' label size
    pub fn set_item_label_size(&mut self, val: u32) {
        debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_set_item_labelsize(self._inner, val as i32) }
    }

    /// Gets the items' foreground color
    pub fn item_label_fg_color(&self) -> Color {
        unsafe { mem::transmute(Fl_Tree_item_labelfgcolor(self._inner)) }
    }

    /// Sets the items' foreground color
    pub fn set_item_label_fg_color(&mut self, val: Color) {
        unsafe { Fl_Tree_set_item_labelfgcolor(self._inner, val as u32) }
    }

    /// Gets the items' background color
    pub fn item_label_bg_color(&self) -> Color {
        unsafe { mem::transmute(Fl_Tree_item_labelbgcolor(self._inner)) }
    }

    /// Sets the items' foreground color
    pub fn set_item_label_bg_color(&mut self, val: Color) {
        unsafe { Fl_Tree_set_item_labelbgcolor(self._inner, val as u32) }
    }

    /// Gets the items' connector color
    pub fn connector_color(&self) -> Color {
        unsafe { mem::transmute(Fl_Tree_connectorcolor(self._inner)) }
    }

    /// Sets the items' foreground color
    pub fn set_connector_color(&mut self, val: Color) {
        unsafe { Fl_Tree_set_connectorcolor(self._inner, val as u32) }
    }

    /// Gets the left margin
    pub fn margin_left(&self) -> u32 {
        unsafe { Fl_Tree_marginleft(self._inner) as u32 }
    }

    /// Sets the left margin
    pub fn set_margin_left(&mut self, val: u32) {
        debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_set_marginleft(self._inner, val as i32) }
    }

    /// Gets the top margin
    pub fn margin_top(&self) -> u32 {
        unsafe { Fl_Tree_margintop(self._inner) as u32 }
    }

    /// Sets the top margin
    pub fn set_margin_top(&mut self, val: u32) {
        debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_set_margintop(self._inner, val as i32) }
    }

    /// Gets the bottom margin
    pub fn margin_bottom(&self) -> u32 {
        unsafe { Fl_Tree_marginbottom(self._inner) as u32 }
    }

    /// Sets the bottom margin
    pub fn set_margin_bottom(&mut self, val: u32) {
        debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_set_marginbottom(self._inner, val as i32) }
    }

    /// Gets the line spacing
    pub fn line_spacing(&self) -> u32 {
        unsafe { Fl_Tree_linespacing(self._inner) as u32 }
    }

    /// Sets the line spacing
    pub fn set_line_spacing(&mut self, val: u32) {
        debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_set_linespacing(self._inner, val as i32) }
    }

    /// Gets the open child bottom margin
    pub fn open_child_margin_bottom(&self) -> u32 {
        unsafe { Fl_Tree_openchild_marginbottom(self._inner) as u32 }
    }

    /// Sets the open child bottom margin
    pub fn set_open_child_margin_bottom(&mut self, val: u32) {
        debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_set_openchild_marginbottom(self._inner, val as i32) }
    }

    /// Gets the user icon left margin
    pub fn user_icon_margin_left(&self) -> u32 {
        unsafe { Fl_Tree_usericonmarginleft(self._inner) as u32 }
    }

    /// Sets the user icon left margin
    pub fn set_user_icon_margin_left(&mut self, val: u32) {
        debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_set_usericonmarginleft(self._inner, val as i32) }
    }

    /// Gets the label's left margin
    pub fn label_margin_left(&self) -> u32 {
        unsafe { Fl_Tree_labelmarginleft(self._inner) as u32 }
    }

    /// Sets the label's left margin
    pub fn set_label_margin_left(&mut self, val: u32) {
        debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_set_labelmarginleft(self._inner, val as i32) }
    }

    /// Gets the widget's left margin
    pub fn widget_margin_left(&self) -> u32 {
        unsafe { Fl_Tree_widgetmarginleft(self._inner) as u32 }
    }

    /// Sets the widget's left margin
    pub fn set_widget_margin_left(&mut self, val: u32) {
        debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_set_widgetmarginleft(self._inner, val as i32) }
    }

    /// Gets the connector's width
    pub fn connector_width(&self) -> u32 {
        unsafe { Fl_Tree_connectorwidth(self._inner) as u32 }
    }

    /// Sets the connector's width
    pub fn set_connector_width(&mut self, val: u32) {
        debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_set_connectorwidth(self._inner, val as i32) }
    }

    /// Gets the user icon
    pub fn user_icon(&self) -> Option<Image> {
        unsafe {
            let x = Fl_Tree_usericon(self._inner);
            if x.is_null() {
                None
            } else {
                Some(Image::from_raw(x as *mut fltk_sys::image::Fl_Image))
            }
        }
    }

    /// Sets the user icon
    pub fn set_user_icon<Img: ImageExt>(&mut self, val: &Img) {
        unsafe { Fl_Tree_set_usericon(self._inner, val.as_ptr()) }
    }

    /// Gets the opne icon
    pub fn open_icon(&self) -> Option<Image> {
        unsafe {
            let x = Fl_Tree_openicon(self._inner);
            if x.is_null() {
                None
            } else {
                Some(Image::from_raw(x as *mut fltk_sys::image::Fl_Image))
            }
        }
    }

    /// Sets the opne icon
    pub fn set_open_icon<Img: ImageExt>(&mut self, val: &Img) {
        unsafe { Fl_Tree_set_openicon(self._inner, val.as_ptr()) }
    }

    /// Gets the close icon
    pub fn close_icon(&self) -> Option<Image> {
        unsafe {
            let x = Fl_Tree_closeicon(self._inner);
            if x.is_null() {
                None
            } else {
                Some(Image::from_raw(x as *mut fltk_sys::image::Fl_Image))
            }
        }
    }

    /// Sets the opne icon
    pub fn set_close_icon<Img: ImageExt>(&mut self, val: &Img) {
        unsafe { Fl_Tree_set_closeicon(self._inner, val.as_ptr()) }
    }

    /// Returns true if the collapse icon is enabled, false if not.
    pub fn show_collapse(&self) -> bool {
        unsafe {
            match Fl_Tree_showcollapse(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets whether the collapse icon is enabled
    pub fn set_show_collapse(&mut self, flag: bool) {
        unsafe { Fl_Tree_set_showcollapse(self._inner, flag as i32) }
    }

    /// Returs whether the root is shown
    pub fn show_root(&self) -> bool {
        unsafe {
            match Fl_Tree_showroot(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets whether the root is shown
    pub fn set_show_root(&mut self, flag: bool) {
        unsafe { Fl_Tree_set_showroot(self._inner, flag as i32) }
    }

    /// Gets the connector style
    pub fn connector_style(&self) -> TreeConnectorStyle {
        unsafe { mem::transmute(Fl_Tree_connectorstyle(self._inner)) }
    }

    /// Sets the connector style
    pub fn set_connector_style(&mut self, val: TreeConnectorStyle) {
        unsafe { Fl_Tree_set_connectorstyle(self._inner, val as i32) }
    }

    /// Gets the sort order
    pub fn sort_order(&self) -> TreeSort {
        unsafe { mem::transmute(Fl_Tree_sortorder(self._inner)) }
    }

    /// Sets the sort order
    pub fn set_sort_order(&mut self, val: TreeSort) {
        unsafe { Fl_Tree_set_sortorder(self._inner, val as i32) }
    }

    /// Gets the select frame(Fl_Box)
    pub fn select_frame(&self) -> FrameType {
        unsafe { mem::transmute(Fl_Tree_selectbox(self._inner)) }
    }

    /// Sets the select frame(Fl_Box)
    pub fn set_select_frame(&mut self, val: FrameType) {
        unsafe { Fl_Tree_set_selectbox(self._inner, val as i32) }
    }

    /// Gets the Tree select mode
    pub fn select_mode(&self) -> TreeSelect {
        unsafe { mem::transmute(Fl_Tree_selectmode(self._inner)) }
    }

    /// Sets the Tree select mode
    pub fn set_select_mode(&mut self, val: TreeSelect) {
        unsafe { Fl_Tree_set_selectmode(self._inner, val as i32) }
    }

    /// Gets the Tree item's reselect mode
    pub fn item_reselect_mode(&self) -> TreeItemReselectMode {
        unsafe { mem::transmute(Fl_Tree_item_reselect_mode(self._inner)) }
    }

    /// Sets the Tree item's reselect mode
    pub fn set_item_reselect_mode(&mut self, mode: TreeItemReselectMode) {
        unsafe { Fl_Tree_set_item_reselect_mode(self._inner, mode as i32) }
    }

    /// Gets the Tree item's draw mode
    pub fn item_draw_mode(&self) -> TreeItemDrawMode {
        unsafe { mem::transmute(Fl_Tree_item_draw_mode(self._inner)) }
    }

    /// Sets the Tree item's draw mode
    pub fn set_item_draw_mode(&mut self, mode: TreeItemDrawMode) {
        unsafe { Fl_Tree_set_item_draw_mode(self._inner, mode as i32) }
    }

    /// Recalculate widget dimensions and scrollbar visibility, normally done automatically
    pub fn calc_dimensions(&mut self) {
        unsafe { Fl_Tree_calc_dimensions(self._inner) }
    }

    /// Recalculates the tree's sizes and scrollbar visibility, normally done automatically
    pub fn calc_tree(&mut self) {
        unsafe { Fl_Tree_calc_tree(self._inner) }
    }

    /// Recalculates the tree's sizes and scrollbar visibility, normally done automatically
    pub fn recalc_tree(&mut self) {
        unsafe { Fl_Tree_recalc_tree(self._inner) }
    }

    /// Returns whether an item is displayed
    pub fn displayed(&mut self, item: TreeItem) -> bool {
        unsafe {
            match Fl_Tree_displayed(self._inner, item._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Shows an item
    pub fn show_item(&mut self, item: TreeItem, y_offset: i32) {
        unsafe { Fl_Tree_show_item(self._inner, item._inner, y_offset) }
    }

    /// Adjust the vertical scrollbar so that ```item``` is visible
    pub fn show_item_top(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_show_item_top(self._inner, item._inner) }
    }

    /// Adjust the vertical scrollbar so that ```item``` is in the middle of the display
    pub fn show_item_middle(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_show_item_middle(self._inner, item._inner) }
    }

    /// Adjust the vertical scrollbar so that the is at the bottom of the display.
    pub fn show_item_bottom(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_show_item_bottom(self._inner, item._inner) }
    }

    /// Display the item
    pub fn display(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_display(self._inner, item._inner) }
    }

    /// Gets the vertical position of the item
    pub fn vposition(&self) -> i32 {
        unsafe { Fl_Tree_vposition(self._inner) }
    }

    /// Sets the vertical position of the item
    pub fn set_vposition(&mut self, pos: i32) {
        unsafe { Fl_Tree_set_vposition(self._inner, pos) }
    }

    /// Gets the horizontal position of the item
    pub fn hposition(&self) -> i32 {
        unsafe { Fl_Tree_hposition(self._inner) }
    }

    /// Sets the horizontal position of the item
    pub fn set_hposition(&mut self, pos: i32) {
        unsafe { Fl_Tree_set_hposition(self._inner, pos) }
    }

    /// Returns whether the widget is a scrollbar
    pub fn is_scrollbar<W: WidgetExt>(&mut self, w: &W) -> bool {
        unsafe {
            match Fl_Tree_is_scrollbar(self._inner, w.as_widget_ptr() as *mut Fl_Widget) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Gets the scrollbar size
    pub fn scrollbar_size(&self) -> i32 {
        unsafe { Fl_Tree_scrollbar_size(self._inner) }
    }

    /// Sets the scrollbar size
    pub fn set_scrollbar_size(&mut self, sz: u32) {
        unsafe { Fl_Tree_set_scrollbar_size(self._inner, sz as i32) }
    }

    /// Returns whether vertical scrolling is visible
    pub fn is_vscroll_visible(&self) -> bool {
        unsafe {
            match Fl_Tree_is_vscroll_visible(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Returns whether horizontal scrolling is visible
    pub fn is_hscroll_visible(&self) -> bool {
        unsafe {
            match Fl_Tree_is_hscroll_visible(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Set the callback item
    pub fn set_callback_item(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_set_callback_item(self._inner, item._inner) }
    }

    /// Get the callback item
    pub fn callback_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_callback_item(self._inner)) }
    }

    /// Set the callback reason
    pub fn set_callback_reason(&mut self, reason: TreeReason) {
        unsafe { Fl_Tree_set_callback_reason(self._inner, reason as i32) }
    }

    /// Get the callback reason
    pub fn callback_reason(&self) -> TreeReason {
        unsafe { mem::transmute(Fl_Tree_callback_reason(self._inner)) }
    }
}

impl TreeItem {
    /// Create a TreeItem from a raw pointer
    pub unsafe fn from_raw(ptr: *mut Fl_Tree_Item) -> Option<TreeItem> {
        if ptr.is_null() {
            None
        } else {
            let x = TreeItem { _inner: ptr };
            Some(x)
        }
    }

    /// Gets the x position
    pub fn x(&self) -> i32 {
        unsafe { Fl_Tree_Item_x(self._inner) }
    }

    /// Gets the y position
    pub fn y(&self) -> i32 {
        unsafe { Fl_Tree_Item_y(self._inner) }
    }

    /// Gets the width
    pub fn w(&self) -> i32 {
        unsafe { Fl_Tree_Item_w(self._inner) }
    }

    /// Gets the height
    pub fn h(&self) -> i32 {
        unsafe { Fl_Tree_Item_h(self._inner) }
    }

    /// Gets the label's x position
    pub fn label_x(&self) -> i32 {
        unsafe { Fl_Tree_Item_label_x(self._inner) }
    }

    /// Gets the label's y position
    pub fn label_y(&self) -> i32 {
        unsafe { Fl_Tree_Item_label_y(self._inner) }
    }

    /// Gets the label's width
    pub fn label_w(&self) -> i32 {
        unsafe { Fl_Tree_Item_label_w(self._inner) }
    }

    /// Sets the label's width
    pub fn label_h(&self) -> i32 {
        unsafe { Fl_Tree_Item_label_h(self._inner) }
    }

    /// Shows the tree item
    pub fn show_self(&self, indent: &str) {
        let indent = CString::new(indent).unwrap();
        unsafe { Fl_Tree_Item_show_self(self._inner, indent.into_raw() as *mut raw::c_char) }
    }

    /// Sets the label of the tree item
    pub fn set_label(&mut self, val: &str) {
        let val = CString::new(val).unwrap();
        unsafe { Fl_Tree_set_Item_label(self._inner, val.into_raw() as *mut raw::c_char) }
    }

    /// Gets the label of the tree item
    pub fn label(&self) -> String {
        unsafe {
            let x = Fl_Tree_Item_label(self._inner);
            assert!(!x.is_null());
            CStr::from_ptr(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Sets the label's font
    pub fn set_label_font(&mut self, val: Font) {
        unsafe { Fl_Tree_Item_set_labelfont(self._inner, val as i32) }
    }

    /// Gets the label's font
    pub fn label_font(&self) -> Font {
        unsafe { mem::transmute(Fl_Tree_Item_labelfont(self._inner)) }
    }

    /// Sets the label's size
    pub fn set_label_size(&mut self, val: u32) {
        debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_Item_set_labelsize(self._inner, val as i32) }
    }

    /// Gets the label's size
    pub fn label_size(&self) -> u32 {
        unsafe { Fl_Tree_Item_labelsize(self._inner) as u32 }
    }

    /// Sets the label's foreground color
    pub fn set_label_fg_color(&mut self, val: Color) {
        unsafe { Fl_Tree_Item_set_labelfgcolor(self._inner, val as u32) }
    }

    /// Gets the label's foreground color
    pub fn label_fg_color(&self) -> Color {
        unsafe { mem::transmute(Fl_Tree_Item_labelfgcolor(self._inner)) }
    }

    /// Sets the label's color
    pub fn set_label_color(&mut self, val: Color) {
        unsafe { Fl_Tree_Item_set_labelcolor(self._inner, val as u32) }
    }

    /// Gets the label's color
    pub fn label_color(&self) -> Color {
        unsafe { mem::transmute(Fl_Tree_Item_labelcolor(self._inner)) }
    }

    /// Sets the label's background color
    pub fn set_label_bg_color(&mut self, val: Color) {
        unsafe { Fl_Tree_Item_set_labelbgcolor(self._inner, val as u32) }
    }

    /// Gets the label's foreground color
    pub fn label_bg_color(&self) -> Color {
        unsafe { mem::transmute(Fl_Tree_Item_labelbgcolor(self._inner)) }
    }

    /// Sets the item's associated widget
    pub fn set_widget<W: WidgetExt>(&mut self, val: &W) {
        unsafe { Fl_Tree_Item_set_widget(self._inner, val.as_widget_ptr() as *mut Fl_Widget) }
    }

    /// Gets the item's associated widget
    pub fn widget(&self) -> Widget {
        unsafe {
            Widget::from_raw(Fl_Tree_Item_widget(self._inner) as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Gets the children count
    pub fn children(&self) -> u32 {
        unsafe { Fl_Tree_Item_children(self._inner) as u32 }
    }

    /// Gets the child item at idx position
    pub fn child(&self, idx: u32) -> Option<TreeItem> {
        debug_assert!(idx <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { TreeItem::from_raw(Fl_Tree_Item_child(self._inner, idx as i32) as *mut Fl_Tree_Item) }
    }

    /// Returns whether the item has children
    pub fn has_children(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_has_children(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Find a child using its name, returns index result
    pub fn find_child(&mut self, name: &str) -> Result<u32, FltkError> {
        let name = CString::new(name).unwrap();
        unsafe {
            let x = Fl_Tree_Item_find_child(self._inner, name.into_raw());
            if x == -1 {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            } else {
                Ok(x as u32)
            }
        }
    }

    /// Remove child using its name
    pub fn remove_child(&mut self, new_label: &str) -> Result<(), FltkError> {
        let new_label = CString::new(new_label).unwrap();
        unsafe {
            match Fl_Tree_Item_remove_child(self._inner, new_label.into_raw()) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Remove all children
    pub fn clear_children(&mut self) {
        unsafe { Fl_Tree_Item_clear_children(self._inner) }
    }

    /// Swap children a and b
    pub fn swap_children(&mut self, a: TreeItem, b: TreeItem) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_Item_swap_children(self._inner, a._inner, b._inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Find child by name, returns option of the item
    pub fn find_child_item(&self, name: &str) -> Option<TreeItem> {
        let name = CString::new(name).unwrap();
        unsafe {
            TreeItem::from_raw(
                Fl_Tree_Item_find_child_item(self._inner, name.into_raw()) as *mut Fl_Tree_Item
            )
        }
    }

    /// Replace a tree item
    pub fn replace(&mut self, new_item: TreeItem) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_Item_replace(self._inner, new_item._inner)) }
    }

    /// Replace a child
    pub fn replace_child(&mut self, old_item: TreeItem, new_item: TreeItem) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_replace_child(
                self._inner,
                old_item._inner,
                new_item._inner,
            ))
        }
    }

    /// Deparent a child by index
    pub fn deparent(&mut self, index: u32) -> Option<TreeItem> {
        debug_assert!(index <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { TreeItem::from_raw(Fl_Tree_Item_deparent(self._inner, index as i32)) }
    }

    /// Reparent a child by index
    pub fn reparent(&mut self, newchild: TreeItem, index: u32) -> Result<(), FltkError> {
        debug_assert!(index <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe {
            match Fl_Tree_Item_reparent(self._inner, newchild._inner, index as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Move item 
    pub fn move_item(&mut self, to: u32, from: u32) -> Result<(), FltkError> {
        debug_assert!(to <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        debug_assert!(from <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe {
            match Fl_Tree_Item_move(self._inner, to as i32, from as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Move item above another item
    pub fn move_above(&mut self, item: TreeItem) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_Item_move_above(self._inner, item._inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Move item below another item
    pub fn move_below(&mut self, item: TreeItem) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_Item_move_below(self._inner, item._inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Move item into another item
    pub fn move_into(&mut self, item: TreeItem, pos: u32) -> Result<(), FltkError> {
        debug_assert!(pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe {
            match Fl_Tree_Item_move_into(self._inner, item._inner, pos as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    /// Gets the depth of the item
    pub fn depth(&self) -> u32 {
        unsafe { Fl_Tree_Item_depth(self._inner) as u32 }
    }

    /// Gets the previous item
    pub fn prev(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_Item_prev(self._inner)) }
    }

    /// Gets the next item
    pub fn next(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_Item_next(self._inner)) }
    }

    /// Gets the next sibling
    pub fn next_sibling(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_Item_next_sibling(self._inner)) }
    }

    /// Gets the previous sibling
    pub fn prev_sibling(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_Item_prev_sibling(self._inner)) }
    }

    /// Update surrounding siblings
    pub fn update_prev_next(&mut self, index: u32) {
        debug_assert!(index <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_Item_update_prev_next(self._inner, index as i32) }
    }

    /// Return the parent of the item
    pub fn parent(&self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_Item_parent(self._inner) as *mut Fl_Tree_Item) }
    }

    /// Set the parent of the item
    pub fn set_parent(&mut self, val: TreeItem) {
        unsafe { Fl_Tree_Item_set_parent(self._inner, val._inner) }
    }

    /// Return the tree of the item
    pub fn tree(&self) -> Option<Tree> {
        unsafe { Tree::from_raw(Fl_Tree_Item_tree(self._inner) as *mut Fl_Tree) }
    }

    /// Open the item exposing all children
    pub fn open(&mut self) {
        unsafe { Fl_Tree_Item_open(self._inner) }
    }

    /// Close the item hiding all children
    pub fn close(&mut self) {
        unsafe { Fl_Tree_Item_close(self._inner) }
    }

    /// Returns whether an item is open
    pub fn is_open(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_open(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Returns whether an item is closed
    pub fn is_close(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_close(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Toggle the open state of the item
    pub fn open_toggle(&mut self) {
        unsafe { Fl_Tree_Item_open_toggle(self._inner) }
    }

    /// Select an item at index
    pub fn select(&mut self, index: u32) {
        debug_assert!(index <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { Fl_Tree_Item_select(self._inner, index as i32) }
    }

    /// Toggle the select state of an item
    pub fn select_toggle(&mut self) {
        unsafe { Fl_Tree_Item_select_toggle(self._inner) }
    }

    /// Select all subitems, returns number of selected items
    pub fn select_all(&mut self) -> u32 {
        unsafe { Fl_Tree_Item_select_all(self._inner) as u32 }
    }

    /// Deselect an item
    pub fn deselect(&mut self) {
        unsafe { Fl_Tree_Item_deselect(self._inner) }
    }

    /// Deselect all subitems
    pub fn deselect_all(&mut self) -> u32 {
        unsafe { Fl_Tree_Item_deselect_all(self._inner) as u32 }
    }

    /// Returns whether an item is root
    pub fn is_root(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_root(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Returns whether an item is visible
    pub fn is_visible(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_visible(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Returns whether an item is active
    pub fn is_active(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_active(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Returns whether an item is activated
    pub fn is_activated(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_activated(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Deactivate an item
    pub fn deactivate(&mut self) {
        unsafe { Fl_Tree_Item_deactivate(self._inner) }
    }

    /// Activate an item
    pub fn activate(&mut self, val: bool) {
        unsafe { Fl_Tree_Item_activate(self._inner, val as i32) }
    }

    /// Returns whether an item is selected
    pub fn is_selected(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_selected(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }
}

impl TreeItemArray {
    /// Returns the len() of the array
    fn total(&self) -> u32 {
        unsafe { Fl_Tree_Item_Array_total(self._inner) as u32 }
    }

    /// Swaps children
    #[allow(dead_code)]
    fn swap(&mut self, ax: i32, bx: i32) {
        unsafe { Fl_Tree_Item_Array_swap(self._inner, ax, bx) }
    }

    /// Move items
    #[allow(dead_code)]
    fn move_item(&mut self, to: i32, from: i32) -> i32 {
        unsafe { Fl_Tree_Item_Array_move(self._inner, to, from) }
    }

    /// Deparent item
    #[allow(dead_code)]
    fn deparent(&mut self, pos: i32) -> i32 {
        unsafe { Fl_Tree_Item_Array_deparent(self._inner, pos) }
    }

    /// Reparent item
    #[allow(dead_code)]
    fn reparent(&mut self, item: TreeItem, newparent: TreeItem, pos: i32) -> i32 {
        unsafe { Fl_Tree_Item_Array_reparent(self._inner, item._inner, newparent._inner, pos) }
    }

    /// Clears the array
    #[allow(dead_code)]
    fn clear(&mut self) {
        unsafe { Fl_Tree_Item_Array_clear(self._inner) }
    }

    /// Adds an item to the array
    #[allow(dead_code)]
    fn add(&mut self, val: TreeItem) {
        unsafe { Fl_Tree_Item_Array_add(self._inner, val._inner) }
    }

    /// Insert an item to the array at pos
    #[allow(dead_code)]
    fn insert(&mut self, pos: u32, new_item: TreeItem) {
        unsafe { Fl_Tree_Item_Array_insert(self._inner, pos as i32, new_item._inner) }
    }

    /// Replace an item at pos
    #[allow(dead_code)]
    fn replace(&mut self, pos: i32, new_item: TreeItem) {
        unsafe { Fl_Tree_Item_Array_replace(self._inner, pos, new_item._inner) }
    }

    /// Remove an item at pos
    #[allow(dead_code)]
    fn remove(&mut self, index: i32) {
        unsafe { Fl_Tree_Item_Array_remove(self._inner, index) }
    }

    /// Remove an item
    #[allow(dead_code)]
    fn remove_item(&mut self, item: TreeItem) -> i32 {
        unsafe { Fl_Tree_Item_Array_remove_item(self._inner, item._inner) }
    }

    /// Gets the item at idx
    fn at(&self, idx: u32) -> TreeItem {
        debug_assert!(idx <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
        unsafe { TreeItem::from_raw(Fl_Tree_Item_Array_at(self._inner, idx as i32)).unwrap() }
    }

    /// Transforms the TreeItemArray into a vector
    fn into_vec(self) -> Option<Vec<TreeItem>> {
        let c = self.total();
        let mut v: Vec<TreeItem> = vec![];
        if c == 0 {
            None
        } else {
            for i in 0..c {
                v.push(self.at(i));
            }
            Some(v)
        }
    }
}

unsafe impl Send for TreeItem {}

unsafe impl Sync for TreeItem {}

impl Drop for TreeItemArray {
    fn drop(&mut self) {
        unsafe { Fl_Tree_Item_Array_delete(self._inner) }
    }
}
