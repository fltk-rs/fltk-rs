use crate::image::Image;
pub use crate::prelude::*;
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

impl TreeItem {
    pub unsafe fn from_raw(ptr: *mut Fl_Tree_Item) -> Option<TreeItem> {
        if ptr.is_null() {
            None
        } else {
            let x = TreeItem { _inner: ptr };
            Some(x)
        }
    }
}

impl Tree {
    pub fn show_self(&mut self) {
        unsafe { fltk_sys::tree::show_self(self._inner) }
    }

    pub fn root_label(&mut self, new_label: &str) {
        let new_label = CString::new(new_label).unwrap();
        unsafe {
            fltk_sys::tree::root_label(self._inner, new_label.into_raw() as *const raw::c_char)
        }
    }

    pub fn root(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(fltk_sys::tree::root(self._inner)) }
    }

    pub fn set_root(&mut self, newitem: TreeItem) {
        unsafe { fltk_sys::tree::set_root(self._inner, newitem._inner) }
    }

    pub fn add(&mut self, parent_item: TreeItem, name: &str) -> Option<TreeItem> {
        let name = CString::new(name).unwrap();
        unsafe {
            let x = fltk_sys::tree::add(
                self._inner,
                parent_item._inner,
                name.into_raw() as *mut raw::c_char,
            );
            assert!(!x.is_null());
            TreeItem::from_raw(x)
        }
    }

    pub fn insert_above(&mut self, above: TreeItem, name: &str) -> Option<TreeItem> {
        let name = CString::new(name).unwrap();
        unsafe {
            let x = fltk_sys::tree::insert_above(
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
            let x = fltk_sys::tree::insert(
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
            match fltk_sys::tree::Fl_Tree_remove(self._inner, item._inner) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn clear(&mut self) {
        unsafe { fltk_sys::tree::clear(self._inner) }
    }

    pub fn clear_children(&mut self, item: TreeItem) {
        unsafe {
            fltk_sys::tree::clear_children(self._inner as *mut fltk_sys::tree::Fl_Tree, item._inner)
        }
    }

    pub fn find_clicked(&self, yonly: i32) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(fltk_sys::tree::find_clicked(self._inner, yonly)
                as *mut fltk_sys::tree::Fl_Tree_Item)
        }
    }

    pub fn item_clicked(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(fltk_sys::tree::item_clicked(self._inner)) }
    }

    pub fn first(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(fltk_sys::tree::first(self._inner)) }
    }

    pub fn first_visible_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(fltk_sys::tree::first_visible_item(self._inner)) }
    }

    pub fn next(&mut self, item: TreeItem) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(fltk_sys::tree::next(self._inner, item._inner)) }
    }

    pub fn prev(&mut self, item: TreeItem) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(fltk_sys::tree::prev(self._inner, item._inner)) }
    }

    pub fn last(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(fltk_sys::tree::last(self._inner)) }
    }

    pub fn last_visible_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(fltk_sys::tree::last_visible_item(self._inner)) }
    }

    pub fn next_visible_item(&mut self, start: TreeItem, dir: Key) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(fltk_sys::tree::next_visible_item(
                self._inner,
                start._inner,
                dir as i32,
            ))
        }
    }

    pub fn first_selected_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(fltk_sys::tree::first_selected_item(self._inner)) }
    }

    pub fn last_selected_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(fltk_sys::tree::last_selected_item(self._inner)) }
    }

    pub fn next_item(&mut self, item: TreeItem, dir: Key, visible: bool) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(fltk_sys::tree::next_item(
                self._inner,
                item._inner,
                dir as i32,
                visible as i32,
            ))
        }
    }

    pub fn next_selected_item(&mut self, item: TreeItem, dir: Key) -> Option<TreeItem> {
        unsafe {
            TreeItem::from_raw(fltk_sys::tree::next_selected_item(
                self._inner,
                item._inner,
                dir as i32,
            ))
        }
    }

    pub fn get_selected_items(&mut self) -> Vec<Option<TreeItem>> {
        unsafe {
            let mut items: Vec<*mut fltk_sys::tree::Fl_Tree_Item> = vec![];
            let cnt: *mut i32 = std::ptr::null_mut();
            fltk_sys::tree::get_selected_items(
                self._inner,
                items.as_mut_ptr() as *mut fltk_sys::tree::Fl_Tree_Item,
                cnt,
            );
            let mut items2: Vec<Option<TreeItem>> = Vec::with_capacity(cnt as usize);
            for item in items {
                items2.push(TreeItem::from_raw(item));
            }
            items2
        }
    }

    pub fn open(&mut self, item: TreeItem, docallback: bool) -> Result<(), FltkError> {
        unsafe {
            match fltk_sys::tree::open(self._inner, item._inner, docallback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn open_toggle(&mut self, item: TreeItem, docallback: bool) {
        unsafe { fltk_sys::tree::open_toggle(self._inner, item._inner, docallback as i32) }
    }

    pub fn close(&mut self, item: TreeItem, docallback: bool) -> Result<(), FltkError> {
        unsafe {
            match fltk_sys::tree::close(self._inner, item._inner, docallback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn is_open(&self, item: TreeItem) -> bool {
        unsafe {
            match fltk_sys::tree::is_open(self._inner, item._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn is_close(&self, item: TreeItem) -> bool {
        unsafe {
            match fltk_sys::tree::is_close(self._inner, item._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn select(&mut self, item: TreeItem, docallback: bool) -> Result<(), FltkError> {
        unsafe {
            match fltk_sys::tree::Fl_Tree_select(self._inner, item._inner, docallback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn select_toggle(&mut self, item: TreeItem, docallback: bool) {
        unsafe { fltk_sys::tree::select_toggle(self._inner, item._inner, docallback as i32) }
    }

    pub fn deselect(&mut self, item: TreeItem, docallback: bool) -> Result<(), FltkError> {
        unsafe {
            match fltk_sys::tree::deselect(self._inner, item._inner, docallback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn deselect_all(&mut self, item: TreeItem, docallback: bool) -> Result<(), FltkError> {
        unsafe {
            match fltk_sys::tree::deselect_all(self._inner, item._inner, docallback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn select_only(&mut self, selitem: TreeItem, docallback: bool) -> Result<(), FltkError> {
        unsafe {
            match fltk_sys::tree::select_only(self._inner, selitem._inner, docallback as i32) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            }
        }
    }

    pub fn select_all(&mut self, item: TreeItem, docallback: bool) -> Result<(), FltkError> {
        unsafe {
            match fltk_sys::tree::select_all(self._inner, item._inner, docallback as i32) {
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
            match fltk_sys::tree::extend_selection_dir(
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
            match fltk_sys::tree::extend_selection(
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
        unsafe { fltk_sys::tree::set_item_focus(self._inner, item._inner) }
    }

    pub fn get_item_focus(&self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(fltk_sys::tree::get_item_focus(self._inner)) }
    }

    pub fn is_selected(&self, item: TreeItem) -> bool {
        unsafe {
            match fltk_sys::tree::is_selected(self._inner, item._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn item_labelfont(&self) -> Font {
        unsafe { mem::transmute(fltk_sys::tree::item_labelfont(self._inner)) }
    }

    pub fn set_item_labelfont(&mut self, val: Font) {
        unsafe { fltk_sys::tree::set_item_labelfont(self._inner, val as i32) }
    }

    pub fn item_labelsize(&self) -> i32 {
        unsafe { fltk_sys::tree::item_labelsize(self._inner) }
    }

    pub fn set_item_labelsize(&mut self, val: i32) {
        unsafe { fltk_sys::tree::set_item_labelsize(self._inner, val) }
    }

    pub fn item_labelfgcolor(&self) -> Color {
        unsafe { mem::transmute(fltk_sys::tree::item_labelfgcolor(self._inner)) }
    }

    pub fn set_item_labelfgcolor(&mut self, val: Color) {
        unsafe { fltk_sys::tree::set_item_labelfgcolor(self._inner, val as u32) }
    }

    pub fn item_labelbgcolor(&self) -> Color {
        unsafe { mem::transmute(fltk_sys::tree::item_labelbgcolor(self._inner)) }
    }

    pub fn set_item_labelbgcolor(&mut self, val: Color) {
        unsafe { fltk_sys::tree::set_item_labelbgcolor(self._inner, val as u32) }
    }

    pub fn connectorcolor(&self) -> Color {
        unsafe { mem::transmute(fltk_sys::tree::connectorcolor(self._inner)) }
    }

    pub fn set_connectorcolor(&mut self, val: Color) {
        unsafe { fltk_sys::tree::set_connectorcolor(self._inner, val as u32) }
    }

    pub fn marginleft(&self) -> i32 {
        unsafe { fltk_sys::tree::marginleft(self._inner) }
    }

    pub fn set_marginleft(&mut self, val: i32) {
        unsafe { fltk_sys::tree::set_marginleft(self._inner, val) }
    }

    pub fn margintop(&self) -> i32 {
        unsafe { fltk_sys::tree::margintop(self._inner) }
    }

    pub fn set_margintop(&mut self, val: i32) {
        unsafe { fltk_sys::tree::set_margintop(self._inner, val) }
    }

    pub fn marginbottom(&self) -> i32 {
        unsafe { fltk_sys::tree::marginbottom(self._inner) }
    }

    pub fn set_marginbottom(&mut self, val: i32) {
        unsafe { fltk_sys::tree::set_marginbottom(self._inner, val) }
    }

    pub fn linespacing(&self) -> i32 {
        unsafe { fltk_sys::tree::linespacing(self._inner) }
    }

    pub fn set_linespacing(&mut self, val: i32) {
        unsafe { fltk_sys::tree::set_linespacing(self._inner, val) }
    }

    pub fn openchild_marginbottom(&self) -> i32 {
        unsafe { fltk_sys::tree::openchild_marginbottom(self._inner) }
    }

    pub fn set_openchild_marginbottom(&mut self, val: i32) {
        unsafe { fltk_sys::tree::set_openchild_marginbottom(self._inner, val) }
    }

    pub fn usericonmarginleft(&self) -> i32 {
        unsafe { fltk_sys::tree::usericonmarginleft(self._inner) }
    }

    pub fn set_usericonmarginleft(&mut self, val: i32) {
        unsafe { fltk_sys::tree::set_usericonmarginleft(self._inner, val) }
    }

    pub fn labelmarginleft(&self) -> i32 {
        unsafe { fltk_sys::tree::labelmarginleft(self._inner) }
    }

    pub fn set_labelmarginleft(&mut self, val: i32) {
        unsafe { fltk_sys::tree::set_labelmarginleft(self._inner, val) }
    }

    pub fn widgetmarginleft(&self) -> i32 {
        unsafe { fltk_sys::tree::widgetmarginleft(self._inner) }
    }

    pub fn set_widgetmarginleft(&mut self, val: i32) {
        unsafe { fltk_sys::tree::set_widgetmarginleft(self._inner, val) }
    }

    pub fn connectorwidth(&self) -> i32 {
        unsafe { fltk_sys::tree::connectorwidth(self._inner) }
    }

    pub fn set_connectorwidth(&mut self, val: i32) {
        unsafe { fltk_sys::tree::set_connectorwidth(self._inner, val) }
    }

    pub fn usericon(&self) -> Option<Image> {
        unsafe {
            let x = fltk_sys::tree::usericon(self._inner);
            if x.is_null() {
                None
            } else {
                Some(Image::from_raw(x as *mut fltk_sys::image::Fl_Image))
            }
        }
    }

    pub fn set_usericon<Img: ImageExt>(&mut self, val: Img) {
        unsafe { fltk_sys::tree::set_usericon(self._inner, val.as_ptr()) }
    }

    pub fn openicon(&self) -> Option<Image> {
        unsafe {
            let x = fltk_sys::tree::openicon(self._inner);
            if x.is_null() {
                None
            } else {
                Some(Image::from_raw(x as *mut fltk_sys::image::Fl_Image))
            }
        }
    }

    pub fn set_openicon<Img: ImageExt>(&mut self, val: Img) {
        unsafe { fltk_sys::tree::set_openicon(self._inner, val.as_ptr()) }
    }

    pub fn closeicon(&self) -> Option<Image> {
        unsafe {
            let x = fltk_sys::tree::closeicon(self._inner);
            if x.is_null() {
                None
            } else {
                Some(Image::from_raw(x as *mut fltk_sys::image::Fl_Image))
            }
        }
    }

    pub fn set_closeicon<Img: ImageExt>(&mut self, val: Img) {
        unsafe { fltk_sys::tree::set_closeicon(self._inner, val.as_ptr()) }
    }

    pub fn showcollapse(&self) -> bool {
        unsafe {
            match fltk_sys::tree::showcollapse(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn set_showcollapse(&mut self, flag: bool) {
        unsafe { fltk_sys::tree::set_showcollapse(self._inner, flag as i32) }
    }

    pub fn showroot(&self) -> bool {
        unsafe {
            match fltk_sys::tree::showroot(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn set_showroot(&mut self, flag: bool) {
        unsafe { fltk_sys::tree::set_showroot(self._inner, flag as i32) }
    }

    pub fn connectorstyle(&self) -> i32 {
        unsafe { fltk_sys::tree::connectorstyle(self._inner) }
    }

    pub fn set_connectorstyle(&mut self, val: i32) {
        unsafe { fltk_sys::tree::set_connectorstyle(self._inner, val as i32) }
    }

    pub fn sortorder(&self) -> TreeSort {
        unsafe { mem::transmute(fltk_sys::tree::sortorder(self._inner)) }
    }

    pub fn set_sortorder(&mut self, val: TreeSort) {
        unsafe { fltk_sys::tree::set_sortorder(self._inner, val as i32) }
    }

    pub fn select_frame(&self) -> FrameType {
        unsafe { mem::transmute(fltk_sys::tree::selectbox(self._inner)) }
    }

    pub fn set_select_frame(&mut self, val: FrameType) {
        unsafe { fltk_sys::tree::set_selectbox(self._inner, val as i32) }
    }

    pub fn selectmode(&self) -> TreeSelect {
        unsafe { mem::transmute(fltk_sys::tree::selectmode(self._inner)) }
    }

    pub fn set_selectmode(&mut self, val: TreeSelect) {
        unsafe { fltk_sys::tree::set_selectmode(self._inner, val as i32) }
    }

    pub fn item_reselect_mode(&self) -> TreeItemReselectMode {
        unsafe { mem::transmute(fltk_sys::tree::item_reselect_mode(self._inner)) }
    }

    pub fn set_item_reselect_mode(&mut self, mode: TreeItemReselectMode) {
        unsafe { fltk_sys::tree::set_item_reselect_mode(self._inner, mode as i32) }
    }

    pub fn item_draw_mode(&self) -> TreeItemDrawMode {
        unsafe { mem::transmute(fltk_sys::tree::item_draw_mode(self._inner)) }
    }

    pub fn set_item_draw_mode(&mut self, mode: TreeItemDrawMode) {
        unsafe { fltk_sys::tree::set_item_draw_mode(self._inner, mode as i32) }
    }

    pub fn calc_dimensions(&mut self) {
        unsafe { fltk_sys::tree::calc_dimensions(self._inner) }
    }

    pub fn calc_tree(&mut self) {
        unsafe { fltk_sys::tree::calc_tree(self._inner) }
    }

    pub fn recalc_tree(&mut self) {
        unsafe { fltk_sys::tree::recalc_tree(self._inner) }
    }

    pub fn displayed(&mut self, item: TreeItem) -> bool {
        unsafe {
            match fltk_sys::tree::displayed(self._inner, item._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn show_item(&mut self, item: TreeItem, yoff: i32) {
        unsafe { fltk_sys::tree::show_item(self._inner, item._inner, yoff) }
    }

    pub fn show_item_top(&mut self, item: TreeItem) {
        unsafe { fltk_sys::tree::show_item_top(self._inner, item._inner) }
    }

    pub fn show_item_middle(&mut self, item: TreeItem) {
        unsafe { fltk_sys::tree::show_item_middle(self._inner, item._inner) }
    }

    pub fn show_item_bottom(&mut self, item: TreeItem) {
        unsafe { fltk_sys::tree::show_item_bottom(self._inner, item._inner) }
    }

    pub fn display(&mut self, item: TreeItem) {
        unsafe { fltk_sys::tree::display(self._inner, item._inner) }
    }

    pub fn vposition(&self) -> i32 {
        unsafe { fltk_sys::tree::vposition(self._inner) }
    }

    pub fn set_vposition(&mut self, pos: i32) {
        unsafe { fltk_sys::tree::set_vposition(self._inner, pos) }
    }

    pub fn hposition(&self) -> i32 {
        unsafe { fltk_sys::tree::hposition(self._inner) }
    }

    pub fn set_hposition(&mut self, pos: i32) {
        unsafe { fltk_sys::tree::set_hposition(self._inner, pos) }
    }

    pub fn is_scrollbar<W: WidgetExt>(&mut self, w: W) -> bool {
        unsafe {
            match fltk_sys::tree::is_scrollbar(
                self._inner,
                w.as_widget_ptr() as *mut fltk_sys::tree::Fl_Widget,
            ) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn scrollbar_size(&self) -> i32 {
        unsafe { fltk_sys::tree::scrollbar_size(self._inner) }
    }

    pub fn set_scrollbar_size(&mut self, sz: i32) {
        unsafe { fltk_sys::tree::set_scrollbar_size(self._inner, sz) }
    }

    pub fn is_vscroll_visible(&self) -> bool {
        unsafe {
            match fltk_sys::tree::is_vscroll_visible(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn is_hscroll_visible(&self) -> bool {
        unsafe {
            match fltk_sys::tree::is_hscroll_visible(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn set_callback_item(&mut self, item: TreeItem) {
        unsafe { fltk_sys::tree::set_callback_item(self._inner, item._inner) }
    }

    pub fn callback_item(&mut self) -> Option<TreeItem> {
        unsafe { TreeItem::from_raw(fltk_sys::tree::callback_item(self._inner)) }
    }

    pub fn set_callback_reason(&mut self, reason: TreeReason) {
        unsafe { fltk_sys::tree::set_callback_reason(self._inner, reason as i32) }
    }

    pub fn callback_reason(&self) -> TreeReason {
        unsafe { mem::transmute(fltk_sys::tree::callback_reason(self._inner)) }
    }
}
