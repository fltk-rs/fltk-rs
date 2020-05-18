use crate::image::Image;
pub use crate::prelude::*;
use crate::widget::Widget;
use fltk_sys::tree::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TreeSort {
    None = 0,
    Ascending = 1,
    Descending = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TreeConnector {
    None = 0,
    Dotted = 1,
    Solid = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TreeSelect {
    None = 0,
    Single = 1,
    Multi = 2,

    SingleDraggable = 3,
}

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

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TreeItemReselectMode {
    Once = 0,
    Always,
}

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
    pub unsafe fn from_raw(ptr: *mut Fl_Tree) -> Option<Tree> {
        if ptr.is_null() {
            None
        } else {
            let x = Tree { _inner: ptr };
            Some(x)
        }
    }

    pub fn begin(&self) {
        unsafe { Fl_Tree_begin(self._inner) }
    }

    pub fn end(&self) {
        unsafe { Fl_Tree_end(self._inner) }
    }

    pub fn show_self(&mut self) {
        unsafe { Fl_Tree_show_self(self._inner) }
    }

    pub fn set_root_label(&mut self, new_label: &str) {
        let new_label = CString::new(new_label).unwrap();
        unsafe {
            Fl_Tree_root_label(self._inner, new_label.into_raw() as *const raw::c_char)
        }
    }

    pub fn root(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_root(self._inner)) }
    }

    pub fn set_root(&mut self, newitem: Option<TreeItem>) {
        let ptr = match newitem {
            None => std::ptr::null_mut(),
            Some(item) => item._inner,
        };
        unsafe { Fl_Tree_set_root(self._inner, ptr) }
    }

    pub fn add(&mut self, path: &str) -> Option<TreeItem> {
        let path = CString::new(path).unwrap();
        unsafe {
            let x = Fl_Tree_add(self._inner, path.into_raw() as *mut raw::c_char);
            assert!(!x.is_null());
            TreeItem::from_raw(x)
        }
    }

    pub fn insert_above(&mut self, above: TreeItem, name: &str) -> Option<TreeItem> {
        let name = CString::new(name).unwrap();
        unsafe {
            let x = Fl_Tree_insert_above(
                self._inner,
                above._inner,
                name.into_raw() as *mut raw::c_char,
            );
            assert!(!x.is_null());
            TreeItem::from_raw(x)
        }
    }

    pub fn insert(&mut self, item: TreeItem, name: &str, pos: i32) -> Option<TreeItem> {
        let name = CString::new(name).unwrap();
        unsafe {
            let x = Fl_Tree_insert(
                self._inner,
                item._inner,
                name.into_raw() as *mut raw::c_char,
                pos,
            );
            assert!(!x.is_null());
            TreeItem::from_raw(x)
        }
    }

    pub fn remove(&mut self, item: TreeItem) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_remove(self._inner, item._inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn clear(&mut self) {
        unsafe { Fl_Tree_clear(self._inner) }
    }

    pub fn clear_children(&mut self, item: TreeItem) {
        unsafe {
            Fl_Tree_clear_children(self._inner as *mut Fl_Tree, item._inner)
        }
    }

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

    pub fn find_clicked(&self, yonly: i32) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_find_clicked(self._inner, yonly)
                as *mut Fl_Tree_Item)
        }
    }

    pub fn item_clicked(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_item_clicked(self._inner)) }
    }

    pub fn first(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_first(self._inner)) }
    }

    pub fn first_visible_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_first_visible_item(self._inner)) }
    }

    pub fn next(&mut self, item: TreeItem) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_next(self._inner, item._inner)) }
    }

    pub fn prev(&mut self, item: TreeItem) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_prev(self._inner, item._inner)) }
    }

    pub fn last(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_last(self._inner)) }
    }

    pub fn last_visible_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_last_visible_item(self._inner)) }
    }

    pub fn next_visible_item(&mut self, start: TreeItem, dir: Key) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_next_visible_item(
                self._inner,
                start._inner,
                dir as i32,
            ))
        }
    }

    pub fn first_selected_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_first_selected_item(self._inner)) }
    }

    pub fn last_selected_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_last_selected_item(self._inner)) }
    }

    pub fn next_item(&mut self, item: TreeItem, dir: Key, visible: bool) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_next_item(
                self._inner,
                item._inner,
                dir as i32,
                visible as i32,
            ))
        }
    }

    pub fn next_selected_item(&mut self, item: TreeItem, dir: Key) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_next_selected_item(
                self._inner,
                item._inner,
                dir as i32,
            ))
        }
    }

    pub fn get_selected_items(&mut self) -> Option<Vec<TreeItem>> {
        unsafe {
            let mut items = TreeItemArray { _inner: std::ptr::null_mut(), };
            let ret = Fl_Tree_get_selected_items(self._inner, &mut items._inner);
            if ret == 0 {
                None
            } else {
                items.into_vec()
            }
        }
    }

    pub fn open(&mut self, path: &str, docallback: bool) -> Result<(), FltkError> {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_open(
                self._inner,
                path.into_raw() as *mut raw::c_char,
                docallback as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn open_toggle(&mut self, item: TreeItem, docallback: bool) {
        unsafe { Fl_Tree_open_toggle(self._inner, item._inner, docallback as i32) }
    }

    pub fn close(&mut self, path: &str, docallback: bool) -> Result<(), FltkError> {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_close(
                self._inner,
                path.into_raw() as *mut raw::c_char,
                docallback as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    // pub fn close_path(&mut self, path: &str) {
    //     let path = CString::new(path).unwrap();
    //     unsafe {
    //         Fl_Tree_close_path(self._inner, path.into_raw() as *mut raw::c_char, 1)
    //     }
    // }

    pub fn is_open(&self, path: &str) -> bool {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_is_open(self._inner, path.into_raw() as *mut raw::c_char) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn is_close(&self, path: &str) -> bool {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_is_close(self._inner, path.into_raw() as *mut raw::c_char) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn select(&mut self, path: &str, docallback: bool) -> Result<(), FltkError> {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_select(
                self._inner,
                path.into_raw() as *mut raw::c_char,
                docallback as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn select_toggle(&mut self, item: TreeItem, docallback: bool) {
        unsafe { Fl_Tree_select_toggle(self._inner, item._inner, docallback as i32) }
    }

    pub fn deselect(&mut self, path: &str, docallback: bool) -> Result<(), FltkError> {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_deselect(
                self._inner,
                path.into_raw() as *mut raw::c_char,
                docallback as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn deselect_all(&mut self, item: TreeItem, docallback: bool) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_deselect_all(self._inner, item._inner, docallback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn select_only(&mut self, selitem: TreeItem, docallback: bool) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_select_only(self._inner, selitem._inner, docallback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn select_all(&mut self, item: TreeItem, docallback: bool) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_select_all(self._inner, item._inner, docallback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn extend_selection_dir(
        &mut self,
        from: TreeItem,
        to: TreeItem,
        dir: i32,
        val: i32,
        visible: bool,
    ) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_extend_selection_dir(
                self._inner,
                from._inner,
                to._inner,
                dir,
                val,
                visible as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn extend_selection(
        &mut self,
        from: TreeItem,
        to: TreeItem,
        val: i32,
        visible: bool,
    ) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_extend_selection(
                self._inner,
                from._inner,
                to._inner,
                val,
                visible as i32,
            ) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn set_item_focus(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_set_item_focus(self._inner, item._inner) }
    }

    pub fn get_item_focus(&self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_get_item_focus(self._inner)) }
    }

    pub fn is_selected(&self, path: &str) -> bool {
        let path = CString::new(path).unwrap();
        unsafe {
            match Fl_Tree_is_selected(self._inner, path.into_raw() as *mut raw::c_char) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn item_labelfont(&self) -> Font {
        unsafe { mem::transmute(Fl_Tree_item_labelfont(self._inner)) }
    }

    pub fn set_item_labelfont(&mut self, val: Font) {
        unsafe { Fl_Tree_set_item_labelfont(self._inner, val as i32) }
    }

    pub fn item_labelsize(&self) -> i32 {
        unsafe { Fl_Tree_item_labelsize(self._inner) }
    }

    pub fn set_item_labelsize(&mut self, val: i32) {
        unsafe { Fl_Tree_set_item_labelsize(self._inner, val) }
    }

    pub fn item_labelfgcolor(&self) -> Color {
        unsafe { mem::transmute(Fl_Tree_item_labelfgcolor(self._inner)) }
    }

    pub fn set_item_labelfgcolor(&mut self, val: Color) {
        unsafe { Fl_Tree_set_item_labelfgcolor(self._inner, val as u32) }
    }

    pub fn item_labelbgcolor(&self) -> Color {
        unsafe { mem::transmute(Fl_Tree_item_labelbgcolor(self._inner)) }
    }

    pub fn set_item_labelbgcolor(&mut self, val: Color) {
        unsafe { Fl_Tree_set_item_labelbgcolor(self._inner, val as u32) }
    }

    pub fn connectorcolor(&self) -> Color {
        unsafe { mem::transmute(Fl_Tree_connectorcolor(self._inner)) }
    }

    pub fn set_connectorcolor(&mut self, val: Color) {
        unsafe { Fl_Tree_set_connectorcolor(self._inner, val as u32) }
    }

    pub fn marginleft(&self) -> i32 {
        unsafe { Fl_Tree_marginleft(self._inner) }
    }

    pub fn set_marginleft(&mut self, val: i32) {
        unsafe { Fl_Tree_set_marginleft(self._inner, val) }
    }

    pub fn margintop(&self) -> i32 {
        unsafe { Fl_Tree_margintop(self._inner) }
    }

    pub fn set_margintop(&mut self, val: i32) {
        unsafe { Fl_Tree_set_margintop(self._inner, val) }
    }

    pub fn marginbottom(&self) -> i32 {
        unsafe { Fl_Tree_marginbottom(self._inner) }
    }

    pub fn set_marginbottom(&mut self, val: i32) {
        unsafe { Fl_Tree_set_marginbottom(self._inner, val) }
    }

    pub fn linespacing(&self) -> i32 {
        unsafe { Fl_Tree_linespacing(self._inner) }
    }

    pub fn set_linespacing(&mut self, val: i32) {
        unsafe { Fl_Tree_set_linespacing(self._inner, val) }
    }

    pub fn openchild_marginbottom(&self) -> i32 {
        unsafe { Fl_Tree_openchild_marginbottom(self._inner) }
    }

    pub fn set_openchild_marginbottom(&mut self, val: i32) {
        unsafe { Fl_Tree_set_openchild_marginbottom(self._inner, val) }
    }

    pub fn usericonmarginleft(&self) -> i32 {
        unsafe { Fl_Tree_usericonmarginleft(self._inner) }
    }

    pub fn set_usericonmarginleft(&mut self, val: i32) {
        unsafe { Fl_Tree_set_usericonmarginleft(self._inner, val) }
    }

    pub fn labelmarginleft(&self) -> i32 {
        unsafe { Fl_Tree_labelmarginleft(self._inner) }
    }

    pub fn set_labelmarginleft(&mut self, val: i32) {
        unsafe { Fl_Tree_set_labelmarginleft(self._inner, val) }
    }

    pub fn widgetmarginleft(&self) -> i32 {
        unsafe { Fl_Tree_widgetmarginleft(self._inner) }
    }

    pub fn set_widgetmarginleft(&mut self, val: i32) {
        unsafe { Fl_Tree_set_widgetmarginleft(self._inner, val) }
    }

    pub fn connectorwidth(&self) -> i32 {
        unsafe { Fl_Tree_connectorwidth(self._inner) }
    }

    pub fn set_connectorwidth(&mut self, val: i32) {
        unsafe { Fl_Tree_set_connectorwidth(self._inner, val) }
    }

    pub fn usericon(&self) -> Option<Image> {
        unsafe {
            let x = Fl_Tree_usericon(self._inner);
            if x.is_null() {
                None
            } else {
                Some(Image::from_raw(x as *mut fltk_sys::image::Fl_Image))
            }
        }
    }

    pub fn set_usericon<Img: ImageExt>(&mut self, val: Img) {
        unsafe { Fl_Tree_set_usericon(self._inner, val.as_ptr()) }
    }

    pub fn openicon(&self) -> Option<Image> {
        unsafe {
            let x = Fl_Tree_openicon(self._inner);
            if x.is_null() {
                None
            } else {
                Some(Image::from_raw(x as *mut fltk_sys::image::Fl_Image))
            }
        }
    }

    pub fn set_openicon<Img: ImageExt>(&mut self, val: Img) {
        unsafe { Fl_Tree_set_openicon(self._inner, val.as_ptr()) }
    }

    pub fn closeicon(&self) -> Option<Image> {
        unsafe {
            let x = Fl_Tree_closeicon(self._inner);
            if x.is_null() {
                None
            } else {
                Some(Image::from_raw(x as *mut fltk_sys::image::Fl_Image))
            }
        }
    }

    pub fn set_closeicon<Img: ImageExt>(&mut self, val: Img) {
        unsafe { Fl_Tree_set_closeicon(self._inner, val.as_ptr()) }
    }

    pub fn showcollapse(&self) -> bool {
        unsafe {
            match Fl_Tree_showcollapse(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn set_showcollapse(&mut self, flag: bool) {
        unsafe { Fl_Tree_set_showcollapse(self._inner, flag as i32) }
    }

    pub fn showroot(&self) -> bool {
        unsafe {
            match Fl_Tree_showroot(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn set_showroot(&mut self, flag: bool) {
        unsafe { Fl_Tree_set_showroot(self._inner, flag as i32) }
    }

    pub fn connectorstyle(&self) -> i32 {
        unsafe { Fl_Tree_connectorstyle(self._inner) }
    }

    pub fn set_connectorstyle(&mut self, val: i32) {
        unsafe { Fl_Tree_set_connectorstyle(self._inner, val as i32) }
    }

    pub fn sortorder(&self) -> TreeSort {
        unsafe { mem::transmute(Fl_Tree_sortorder(self._inner)) }
    }

    pub fn set_sortorder(&mut self, val: TreeSort) {
        unsafe { Fl_Tree_set_sortorder(self._inner, val as i32) }
    }

    pub fn select_frame(&self) -> FrameType {
        unsafe { mem::transmute(Fl_Tree_selectbox(self._inner)) }
    }

    pub fn set_select_frame(&mut self, val: FrameType) {
        unsafe { Fl_Tree_set_selectbox(self._inner, val as i32) }
    }

    pub fn selectmode(&self) -> TreeSelect {
        unsafe { mem::transmute(Fl_Tree_selectmode(self._inner)) }
    }

    pub fn set_selectmode(&mut self, val: TreeSelect) {
        unsafe { Fl_Tree_set_selectmode(self._inner, val as i32) }
    }

    pub fn item_reselect_mode(&self) -> TreeItemReselectMode {
        unsafe { mem::transmute(Fl_Tree_item_reselect_mode(self._inner)) }
    }

    pub fn set_item_reselect_mode(&mut self, mode: TreeItemReselectMode) {
        unsafe { Fl_Tree_set_item_reselect_mode(self._inner, mode as i32) }
    }

    pub fn item_draw_mode(&self) -> TreeItemDrawMode {
        unsafe { mem::transmute(Fl_Tree_item_draw_mode(self._inner)) }
    }

    pub fn set_item_draw_mode(&mut self, mode: TreeItemDrawMode) {
        unsafe { Fl_Tree_set_item_draw_mode(self._inner, mode as i32) }
    }

    pub fn calc_dimensions(&mut self) {
        unsafe { Fl_Tree_calc_dimensions(self._inner) }
    }

    pub fn calc_tree(&mut self) {
        unsafe { Fl_Tree_calc_tree(self._inner) }
    }

    pub fn recalc_tree(&mut self) {
        unsafe { Fl_Tree_recalc_tree(self._inner) }
    }

    pub fn displayed(&mut self, item: TreeItem) -> bool {
        unsafe {
            match Fl_Tree_displayed(self._inner, item._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn show_item(&mut self, item: TreeItem, yoff: i32) {
        unsafe { Fl_Tree_show_item(self._inner, item._inner, yoff) }
    }

    pub fn show_item_top(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_show_item_top(self._inner, item._inner) }
    }

    pub fn show_item_middle(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_show_item_middle(self._inner, item._inner) }
    }

    pub fn show_item_bottom(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_show_item_bottom(self._inner, item._inner) }
    }

    pub fn display(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_display(self._inner, item._inner) }
    }

    pub fn vposition(&self) -> i32 {
        unsafe { Fl_Tree_vposition(self._inner) }
    }

    pub fn set_vposition(&mut self, pos: i32) {
        unsafe { Fl_Tree_set_vposition(self._inner, pos) }
    }

    pub fn hposition(&self) -> i32 {
        unsafe { Fl_Tree_hposition(self._inner) }
    }

    pub fn set_hposition(&mut self, pos: i32) {
        unsafe { Fl_Tree_set_hposition(self._inner, pos) }
    }

    pub fn is_scrollbar<W: WidgetExt>(&mut self, w: W) -> bool {
        unsafe {
            match Fl_Tree_is_scrollbar(
                self._inner,
                w.as_widget_ptr() as *mut Fl_Widget,
            ) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn scrollbar_size(&self) -> i32 {
        unsafe { Fl_Tree_scrollbar_size(self._inner) }
    }

    pub fn set_scrollbar_size(&mut self, sz: i32) {
        unsafe { Fl_Tree_set_scrollbar_size(self._inner, sz) }
    }

    pub fn is_vscroll_visible(&self) -> bool {
        unsafe {
            match Fl_Tree_is_vscroll_visible(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn is_hscroll_visible(&self) -> bool {
        unsafe {
            match Fl_Tree_is_hscroll_visible(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn set_callback_item(&mut self, item: TreeItem) {
        unsafe { Fl_Tree_set_callback_item(self._inner, item._inner) }
    }

    pub fn callback_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(Fl_Tree_callback_item(self._inner)) }
    }

    pub fn set_callback_reason(&mut self, reason: TreeReason) {
        unsafe { Fl_Tree_set_callback_reason(self._inner, reason as i32) }
    }

    pub fn callback_reason(&self) -> TreeReason {
        unsafe { mem::transmute(Fl_Tree_callback_reason(self._inner)) }
    }
}

impl TreeItem {
    pub unsafe fn from_raw(ptr: *mut Fl_Tree_Item) -> Option<TreeItem> {
        if ptr.is_null() {
            None
        } else {
            let x = TreeItem { _inner: ptr };
            Some(x)
        }
    }
    pub fn x(&self) -> i32 {
        unsafe {
            Fl_Tree_Item_x(self._inner)
        }
    }

    pub fn y(&self) -> i32 {
        unsafe {
            Fl_Tree_Item_y(self._inner)
        }
    }

    pub fn w(&self) -> i32 {
        unsafe {
            Fl_Tree_Item_w(self._inner)
        }
    }

    pub fn h(&self) -> i32 {
        unsafe {
            Fl_Tree_Item_h(self._inner)
        }
    }

    pub fn label_x(&self) -> i32 {
        unsafe {
            Fl_Tree_Item_label_x(self._inner)
        }
    }

    pub fn label_y(&self) -> i32 {
        unsafe {
            Fl_Tree_Item_label_y(self._inner)
        }
    }

    pub fn label_w(&self) -> i32 {
        unsafe {
            Fl_Tree_Item_label_w(self._inner)
        }
    }

    pub fn label_h(&self) -> i32 {
        unsafe {
            Fl_Tree_Item_label_h(self._inner)
        }
    }

    pub fn show_self(&self, indent: &str) {
        let indent = CString::new(indent).unwrap();
        unsafe {
            Fl_Tree_Item_show_self(self._inner, indent.into_raw() as *mut raw::c_char)
        }
    }

    pub fn set_label(&mut self, val: &str) {
        let val = CString::new(val).unwrap();
        unsafe {
            Fl_Tree_set_Item_label(self._inner, val.into_raw() as *mut raw::c_char)
        }
    }

    pub fn label(&self) -> String {
        unsafe {
            CStr::from_ptr(Fl_Tree_Item_label(self._inner) as *mut raw::c_char).to_string_lossy().to_string()
        }
    }

    pub fn set_labelfont(&mut self, val: Font) {
        unsafe {
            Fl_Tree_Item_set_labelfont(self._inner, val as i32)
        }
    }

    pub fn labelfont(&self) -> Font {
        unsafe {
            mem::transmute(Fl_Tree_Item_labelfont(self._inner))
        }
    }

    pub fn set_labelsize(&mut self, val: i32) {
        unsafe {
            Fl_Tree_Item_set_labelsize(self._inner, val)
        }
    }

    pub fn labelsize(&self) -> i32 {
        unsafe {
            Fl_Tree_Item_labelsize(self._inner)
        }
    }

    pub fn set_labelfgcolor(&mut self, val: Color) {
        unsafe {
            Fl_Tree_Item_set_labelfgcolor(self._inner, val as u32)
        }
    }

    pub fn labelfgcolor(&self) -> Color {
        unsafe {
            mem::transmute(Fl_Tree_Item_labelfgcolor(self._inner))
        }
    }

    pub fn set_labelcolor(&mut self, val: Color) {
        unsafe {
            Fl_Tree_Item_set_labelcolor(self._inner, val as u32)
        }
    }

    pub fn labelcolor(&self) -> Color {
        unsafe {
            mem::transmute(Fl_Tree_Item_labelcolor(self._inner))
        }
    }

    pub fn set_labelbgcolor(&mut self, val: Color) {
        unsafe {
            Fl_Tree_Item_set_labelbgcolor(self._inner, val as u32)
        }
    }

    pub fn labelbgcolor(&self) -> Color {
        unsafe {
            mem::transmute(Fl_Tree_Item_labelbgcolor(self._inner))
        }
    }

    pub fn set_widget<W: WidgetExt>(&mut self, val: W) {
        unsafe {
            Fl_Tree_Item_set_widget(self._inner, val.as_widget_ptr() as *mut Fl_Widget)
        }
    }

    pub fn widget(&self) -> Widget {
        unsafe {
            Widget::from_raw(Fl_Tree_Item_widget(self._inner) as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    pub fn children(&self) -> i32 {
        unsafe {
            Fl_Tree_Item_children(self._inner)
        }
    }

    pub fn child( &self, t: i32) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_child(self._inner, t) as *mut Fl_Tree_Item)
        }
    }

    pub fn has_children(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_has_children(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn find_child( &mut self, name: &str) -> Result<u32, FltkError> {
        let name = CString::new(name).unwrap();
        unsafe {
            let x =  Fl_Tree_Item_find_child(self._inner, name.into_raw());
            if x == -1 {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            } else {
                Ok(x as u32)
            }
        }
    }

    pub fn remove_child(&mut self, new_label: &str) -> Result<(), FltkError> {
        let new_label = CString::new(new_label).unwrap();
        unsafe {
            match Fl_Tree_Item_remove_child(self._inner, new_label.into_raw()) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn clear_children(&mut self) {
        unsafe {
            Fl_Tree_Item_clear_children(self._inner)
        }
    }

    pub fn swap_children( &mut self, a: TreeItem, b: TreeItem) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_Item_swap_children(self._inner, a._inner, b._inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn find_child_item( &self, name: &str) -> Option<TreeItem> {
        let name = CString::new(name).unwrap();
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_find_child_item(self._inner, name.into_raw()) as *mut Fl_Tree_Item)
        }
    }

    pub fn replace( &mut self, new_item: TreeItem) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_replace(self._inner, new_item._inner))
        }
    }

    pub fn replace_child( &mut self, olditem: TreeItem, newitem: TreeItem) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_replace_child(self._inner, olditem._inner, newitem._inner))
        }
    }

    pub fn deparent( &mut self, index: i32) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_deparent(self._inner, index))
        }
    }

    pub fn reparent( &mut self, newchild: TreeItem, index: i32) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_Item_reparent(self._inner, newchild._inner, index) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn move_item( &mut self, to: i32, from: i32) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_Item_move(self._inner, to, from) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn move_above( &mut self, item: TreeItem) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_Item_move_above(self._inner, item._inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn move_below( &mut self, item: TreeItem) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_Item_move_below(self._inner, item._inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn move_into( &mut self, item: TreeItem, pos: i32) -> Result<(), FltkError> {
        unsafe {
            match Fl_Tree_Item_move_into(self._inner, item._inner, pos) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn depth(&self) -> i32 {
        unsafe {
            Fl_Tree_Item_depth(self._inner)
        }
    }

    pub fn prev(&mut self) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_prev(self._inner))
        }
    }

    pub fn next(&mut self) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_next(self._inner))
        }
    }

    pub fn next_sibling(&mut self) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_next_sibling(self._inner))
        }
    }

    pub fn prev_sibling(&mut self) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_prev_sibling(self._inner))
        }
    }

    pub fn update_prev_next(&mut self, index: u32) {
        unsafe {
            Fl_Tree_Item_update_prev_next(self._inner, index as i32)
        }
    }

    pub fn parent(&self) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_parent(self._inner) as *mut Fl_Tree_Item)
        }
    }

    pub fn set_parent(&mut self, val: TreeItem) {
        unsafe {
            Fl_Tree_Item_set_parent(self._inner, val._inner)
        }
    }

    pub fn tree(&self) -> Option<Tree> {
        unsafe {
            Tree::from_raw(Fl_Tree_Item_tree(self._inner) as *mut Fl_Tree)
        }
    }

    pub fn open(&mut self) {
        unsafe {
            Fl_Tree_Item_open(self._inner)
        }
    }

    pub fn close(&mut self) {
        unsafe {
            Fl_Tree_Item_close(self._inner)
        }
    }

    pub fn is_open(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_open(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn is_close(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_close(self._inner) {
                0 => false,
                _ => true,  
            }
        }
    }

    pub fn open_toggle(&mut self) {
        unsafe {
            Fl_Tree_Item_open_toggle(self._inner)
        }
    }

    pub fn select(&mut self, val: u32) {
        unsafe {
            Fl_Tree_Item_select(self._inner, val as i32)
        }
    }

    pub fn select_toggle(&mut self) {
        unsafe {
            Fl_Tree_Item_select_toggle(self._inner)
        }
    }

    pub fn select_all(&mut self) -> u32 {
        unsafe {
            Fl_Tree_Item_select_all(self._inner) as u32
        }
    }

    pub fn deselect(&mut self) {
        unsafe {
            Fl_Tree_Item_deselect(self._inner)
        }
    }

    pub fn deselect_all(&mut self) -> u32 {
        unsafe {
            Fl_Tree_Item_deselect_all(self._inner) as u32
        }
    }

    pub fn is_root(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_root(self._inner) {
                0 => false,
                _ => true,  
            }
        }
    }

    pub fn is_visible(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_visible(self._inner) {
                0 => false,
                _ => true,  
            }
        }
    }

    pub fn is_active(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_active(self._inner) {
                0 => false,
                _ => true,  
            }
        }
    }

    pub fn is_activated(&self) -> bool {
        unsafe {
            match Fl_Tree_Item_is_activated(self._inner) {
                0 => false,
                _ => true,  
            }
        }
    }

    pub fn deactivate(&mut self) {
        unsafe {
            Fl_Tree_Item_deactivate(self._inner)
        }
    }

    pub fn activate(&mut self, val: bool) {
        unsafe {
            Fl_Tree_Item_activate(self._inner, val as i32)
        }
    }

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
    fn total(&self) -> i32 {
        unsafe {
            Fl_Tree_Item_Array_total(self._inner) 
        }
    }
    
    // #[allow(dead_code)]
    // fn swap( &mut self, ax: i32, bx: i32,) {
    //     unsafe {
    //         Fl_Tree_Item_Array_swap( self._inner, ax, bx,)
    //     }
    // }
    
    // #[allow(dead_code)]
    // fn move_item( &mut self, to: i32, from: i32,) -> i32 {
    //     unsafe {
    //         Fl_Tree_Item_Array_move( self._inner, to, from,) 
    //     }
    // }
    
    // #[allow(dead_code)]
    // fn deparent( &mut self, pos: i32,) -> i32 {
    //     unsafe {
    //         Fl_Tree_Item_Array_deparent( self._inner, pos,) 
    //     }
    // }
    
    // #[allow(dead_code)]
    // fn reparent( &mut self, item: TreeItem, newparent: TreeItem, pos: i32,) -> i32 {
    //     unsafe {
    //         Fl_Tree_Item_Array_reparent( self._inner, item._inner, newparent._inner, pos,) 
    //     }
    // }
    
    // #[allow(dead_code)]
    // fn clear(&mut self) {
    //     unsafe {
    //         Fl_Tree_Item_Array_clear(self._inner)
    //     }
    // }
    
    // #[allow(dead_code)]
    // fn add(&mut self, val: TreeItem) {
    //     unsafe {
    //         Fl_Tree_Item_Array_add(self._inner, val._inner)
    //     }
    // }
    
    // #[allow(dead_code)]
    // fn insert( &mut self, pos: i32, new_item: TreeItem,) {
    //     unsafe {
    //         Fl_Tree_Item_Array_insert( self._inner, pos, new_item._inner,)
    //     }
    // }
    
    // #[allow(dead_code)]
    // fn replace( &mut self, pos: i32, new_item: TreeItem,) {
    //     unsafe {
    //         Fl_Tree_Item_Array_replace( self._inner, pos, new_item._inner,)
    //     }
    // }
    
    // #[allow(dead_code)]
    // fn remove(&mut self, index: i32) {
    //     unsafe {
    //         Fl_Tree_Item_Array_remove(self._inner, index)
    //     }
    // }
    
    // #[allow(dead_code)]
    // fn remove_item( &mut self, item: TreeItem,) -> i32 {
    //     unsafe {
    //         Fl_Tree_Item_Array_remove_item( self._inner, item._inner,)
    //     }
    // }

    fn at(&self, idx: i32) -> TreeItem {
        unsafe {
            TreeItem::from_raw(Fl_Tree_Item_Array_at(self._inner, idx)).unwrap()
        }
    }

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

impl Drop for TreeItemArray {
    fn drop(&mut self) {
        unsafe {
            Fl_Tree_Item_Array_delete(self._inner)
        }
    }
}
