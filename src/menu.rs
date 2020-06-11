use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::menu::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};


/// Creates a menu bar
#[derive(WidgetExt, MenuExt, Debug)]
pub struct MenuBar {
    _inner: *mut Fl_Menu_Bar,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a menu button
#[derive(WidgetExt, MenuExt, Debug)]
pub struct MenuButton {
    _inner: *mut Fl_Menu_Button,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a menu choice
#[derive(WidgetExt, MenuExt, Debug)]
pub struct Choice {
    _inner: *mut Fl_Choice,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    _inner: *mut Fl_Menu_Item,
    _parent: *const MenuBar,
    _alloc: bool,
}

/// Defines the menu flag for any added menu items using the add() method
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MenuFlag {
    Normal = 0,
    Inactive = 1,
    Toggle = 2,
    Value = 4,
    Radio = 8,
    Invisible = 0x10,
    SubmenuPointer = 0x20,
    Submenu = 0x40,
    MenuDivider = 0x80,
    MenuHorizontal = 0x100,
}

impl MenuItem {
    /// Initializes a new window, useful for popup menus
    pub fn new(choices: Vec<&str>) -> MenuItem {
        unsafe {
            let sz = choices.len();
            let mut temp: Vec<*mut raw::c_char> = vec![];
            for choice in choices {
                temp.push(CString::new(choice).unwrap().into_raw());
            }
            let item_ptr = Fl_Menu_Item_new(temp.as_ptr() as *mut *mut raw::c_char, sz as i32);
            assert!(!item_ptr.is_null());
            MenuItem {
                _inner: item_ptr,
                _parent: 0 as *const MenuBar,
                _alloc: true,
            }
        }
    }

    /// Creates a popup menu at the specified coordinates and returns its choice
    pub fn popup(&mut self, x: i32, y: i32) -> Option<MenuItem> {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe {
            let item = Fl_Menu_Item_popup(self._inner, x, y);
            if item.is_null() {
                None
            } else {
                let item = MenuItem {
                    _inner: item as *mut Fl_Menu_Item,
                    _parent: 0 as *const MenuBar,
                    _alloc: false,
                };
                Some(item)
            }
        }
    }

    /// Returns the label of the menu item
    pub fn label(&self) -> Option<String> {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe {
            let label_ptr = Fl_Menu_Item_label(self._inner);
            if label_ptr.is_null() {
                return None;
            }
            Some(
                CStr::from_ptr(label_ptr as *mut raw::c_char)
                    .to_string_lossy()
                    .to_string(),
            )
        }
    }

    /// Sets the label of the menu item
    pub fn set_label(&mut self, txt: &str) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe {
            let ptr = Fl_Menu_Item_label(self._inner) as *mut raw::c_char;
            if !ptr.is_null() {
                let _ = CString::from_raw(ptr);
            }
            let txt = CString::new(txt).unwrap();
            Fl_Menu_Item_set_label(self._inner, txt.into_raw());
        }
    }

    /// Returns the label type of the menu item
    pub fn label_type(&self) -> LabelType {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { mem::transmute(Fl_Menu_Item_label_type(self._inner)) }
    }

    /// Sets the label type of the menu item
    pub fn set_label_type(&mut self, typ: LabelType) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe {
            Fl_Menu_Item_set_label_type(self._inner, typ as i32);
        }
    }

    /// Returns the label color of the menu item
    pub fn label_color(&self) -> Color {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { mem::transmute(Fl_Menu_Item_label_color(self._inner)) }
    }

    /// Sets the label color of the menu item
    pub fn set_label_color(&mut self, color: Color) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { Fl_Menu_Item_set_label_color(self._inner, color as u32) }
    }

    /// Returns the label font of the menu item
    pub fn label_font(&self) -> Font {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { mem::transmute(Fl_Menu_Item_label_font(self._inner)) }
    }

    /// Sets the label font of the menu item
    pub fn set_label_font(&mut self, font: Font) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { Fl_Menu_Item_set_label_font(self._inner, font as i32) }
    }

    /// Returns the label size of the menu item
    pub fn label_size(&self) -> u32 {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { Fl_Menu_Item_label_size(self._inner) as u32 }
    }

    /// Sets the label size of the menu item
    pub fn set_label_size(&mut self, sz: u32) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { Fl_Menu_Item_set_label_size(self._inner, sz as i32) }
    }

    /// Returns the value of the menu item
    pub fn value(&self) -> bool {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe {
            match Fl_Menu_Item_value(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets the menu item
    pub fn set(&mut self) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { Fl_Menu_Item_set(self._inner) }
    }

    /// Clears the menu item
    pub fn clear(&mut self) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { Fl_Menu_Item_clear(self._inner) }
    }

    /// Returns whether the menu item is visible or not
    pub fn visible(&self) -> bool {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe {
            match Fl_Menu_Item_visible(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Returns whether the menu item is active
    pub fn active(&mut self) -> bool {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe {
            match Fl_Menu_Item_active(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Activates the menu item
    pub fn activate(&mut self) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { Fl_Menu_Item_activate(self._inner) }
    }

    /// Deactivates the menu item
    pub fn deactivate(&mut self) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { Fl_Menu_Item_deactivate(self._inner) }
    }

    /// Returns whether a menu item is a submenu
    pub fn is_submenu(&self) -> bool {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { Fl_Menu_Item_submenu(self._inner) != 0 }
    }

    /// Returns whether a menu item is a checkbox
    pub fn is_checkbox(&self) -> bool {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe {
            Fl_Menu_Item_checkbox(self._inner) != 0
        }
    }

    /// Returns whether a menu item is a radio item
    pub fn is_radio(&self) -> bool {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe {
            Fl_Menu_Item_radio(self._inner) != 0
        }
    }

    /// Shows the menu item
    pub fn show(&mut self) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { Fl_Menu_Item_show(self._inner) }
    }

    /// Hides the menu item
    pub fn hide(&mut self) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe { Fl_Menu_Item_hide(self._inner) }
    }

    /// Get the next menu item
    pub fn next(&mut self, idx: u32) -> Option<MenuItem> {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe {
            let ptr = Fl_Menu_Item_next(self._inner, idx as i32);
            if ptr.is_null() {
                None
            } else {
                Some(MenuItem {
                    _inner: ptr,
                    _parent: self._parent,
                    _alloc: self._alloc,
                })
            }
        }
    }

    /// Get the user data
    pub unsafe fn user_data(&self) -> Option<Box<dyn FnMut()>> {
        let ptr = Fl_Menu_Item_user_data(self._inner);
        if ptr.is_null() {
            None
        } else {
            let x = ptr as *mut Box<dyn FnMut()>;
            let x = Box::from_raw(x);
            Some(*x)
        }
    }

    /// Manually set the user data
    pub unsafe fn set_user_data(&mut self, data: *mut raw::c_void) {
        Fl_Menu_Item_set_user_data(self._inner, data)
    }

    /// Set a callback for the menu item
    pub fn set_callback(&mut self, cb: Box<dyn FnMut()>) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe {
            unsafe extern "C" fn shim(
                _wid: *mut fltk_sys::menu::Fl_Widget,
                data: *mut raw::c_void,
            ) {
                let a: *mut Box<dyn FnMut()> = mem::transmute(data);
                let f: &mut (dyn FnMut()) = &mut **a;
                f();
            }
            self.unset_callback();
            let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
            let data: *mut raw::c_void = mem::transmute(a);
            let callback: fltk_sys::menu::Fl_Callback = Some(shim);
            Fl_Menu_Item_callback(self._inner, callback, data);
        }
    }

    /// Use a sender to send a message during callback
    pub fn emit<T: 'static + Copy + Send + Sync>(&mut self, sender: crate::app::Sender<T>, msg: T) {
        self.set_callback(Box::new(move || sender.send(msg)));
    }

    /// Manually unset a callback
    pub unsafe fn unset_callback(&mut self) {
        let old_data = self.user_data();
        if old_data.is_some() {
            let _ = old_data.unwrap();
            self.set_user_data(0 as *mut raw::c_void);
        }
    }

    /// Delete the old callback and replace it with an empty one
    pub fn safe_unset_callback(&mut self) {
        assert!(!self.was_deleted() && !self._inner.is_null());
        unsafe {
            self.unset_callback();
        }
        self.set_callback(Box::new(move || { /* do nothing */ }));
    }

    /// Check if a menu item was deleted
    pub fn was_deleted(&self) -> bool {
        if !self._parent.is_null() {
            let parent = unsafe { Fl_Menu_Bar_menu((*self._parent)._inner) };
            if parent.is_null() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

unsafe impl Send for MenuItem {}

unsafe impl Sync for MenuItem {}

impl Drop for MenuItem {
    fn drop(&mut self) {
        if self._alloc {
            unsafe { Fl_Menu_Item_delete(self._inner) }
            self._inner = 0 as *mut Fl_Menu_Item;
        }
    }
}

#[cfg(test)]
mod menu {
    use super::*;
    #[test]
    fn label() {
        let mut menu = MenuBar::new(0, 0, 0, 0, "hello");
        menu.set_label("cloned");
    }
    #[test]
    fn tooltip() {
        let mut menu = MenuBar::new(0, 0, 0, 0, "hello");
        menu.set_tooltip("tooltip");
        assert!(menu.tooltip().unwrap() == "tooltip");
    }
}
