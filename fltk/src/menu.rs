use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::menu::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a menu bar
#[derive(WidgetBase, WidgetExt, MenuExt, Debug)]
pub struct MenuBar {
    _inner: *mut Fl_Menu_Bar,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a menu button
#[derive(WidgetBase, WidgetExt, MenuExt, Debug)]
pub struct MenuButton {
    _inner: *mut Fl_Menu_Button,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a menu choice
#[derive(WidgetBase, WidgetExt, MenuExt, Debug)]
pub struct Choice {
    _inner: *mut Fl_Choice,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a MacOS system menu bar
#[derive(WidgetBase, WidgetExt, MenuExt, Debug)]
pub struct SysMenuBar {
    _inner: *mut Fl_Sys_Menu_Bar,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    _inner: *mut Fl_Menu_Item,
}

/// Defines the menu flag for any added menu items using the add() method
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MenuFlag {
    /// Normal item
    Normal = 0,
    /// Inactive item
    Inactive = 1,
    /// Item is a checkbox toggle (shows checkbox for on/off state)
    Toggle = 2,
    /// The on/off state for checkbox/radio buttons (if set, state is 'on')
    Value = 4,
    /// Item is a radio button
    Radio = 8,
    /// Invisible item
    Invisible = 0x10,
    /// Indicates user_data() is a pointer to another menu array (unused with Rust)
    SubmenuPointer = 0x20,
    /// Menu item is a submenu
    Submenu = 0x40,
    /// Menu divider
    MenuDivider = 0x80,
    /// Horizontal menu (actually reserved for future use)
    MenuHorizontal = 0x100,
}

impl MenuItem {
    /// Initializes a new window, useful for popup menus
    pub fn new(choices: &[&str]) -> MenuItem {
        unsafe {
            let sz = choices.len();
            let mut temp: Vec<*mut raw::c_char> = vec![];
            for &choice in choices {
                let c = CString::safe_new(choice);
                temp.push(c.into_raw());
            }
            let item_ptr = Fl_Menu_Item_new(temp.as_ptr() as *mut *mut raw::c_char, sz as i32);
            assert!(!item_ptr.is_null());
            MenuItem { _inner: item_ptr }
        }
    }

    /// Creates a popup menu at the specified coordinates and returns its choice
    pub fn popup(&mut self, x: i32, y: i32) -> Option<MenuItem> {
        assert!(!self.was_deleted());
        unsafe {
            let item = Fl_Menu_Item_popup(self._inner, x, y);
            if item.is_null() {
                None
            } else {
                let item = MenuItem {
                    _inner: item as *mut Fl_Menu_Item,
                };
                Some(item)
            }
        }
    }

    /// Returns the label of the menu item
    pub fn label(&self) -> Option<String> {
        assert!(!self.was_deleted());
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
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Menu_Item_label(self._inner) as *mut raw::c_char;
            if !ptr.is_null() {
                let _ = CString::from_raw(ptr);
            }
            let txt = CString::safe_new(txt);
            Fl_Menu_Item_set_label(self._inner, txt.into_raw());
        }
    }

    /// Returns the label type of the menu item
    pub fn label_type(&self) -> LabelType {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Menu_Item_label_type(self._inner)) }
    }

    /// Sets the label type of the menu item
    pub fn set_label_type(&mut self, typ: LabelType) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Menu_Item_set_label_type(self._inner, typ as i32);
        }
    }

    /// Returns the label color of the menu item
    pub fn label_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Menu_Item_label_color(self._inner)) }
    }

    /// Sets the label color of the menu item
    pub fn set_label_color(&mut self, color: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_set_label_color(self._inner, color.bits() as u32) }
    }

    /// Returns the label font of the menu item
    pub fn label_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Menu_Item_label_font(self._inner)) }
    }

    /// Sets the label font of the menu item
    pub fn set_label_font(&mut self, font: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_set_label_font(self._inner, font.bits() as i32) }
    }

    /// Returns the label size of the menu item
    pub fn label_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_label_size(self._inner) as u32 }
    }

    /// Sets the label size of the menu item
    pub fn set_label_size(&mut self, sz: u32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_set_label_size(self._inner, sz as i32) }
    }

    /// Returns the value of the menu item
    pub fn value(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_value(self._inner) != 0 }
    }

    /// Sets the menu item
    pub fn set(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_set(self._inner) }
    }

    /// Clears the menu item
    pub fn clear(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_clear(self._inner) }
    }

    /// Returns whether the menu item is visible or not
    pub fn visible(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_visible(self._inner) != 0 }
    }

    /// Returns whether the menu item is active
    pub fn active(&mut self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_active(self._inner) != 0 }
    }

    /// Activates the menu item
    pub fn activate(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_activate(self._inner) }
    }

    /// Deactivates the menu item
    pub fn deactivate(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_deactivate(self._inner) }
    }

    /// Returns whether a menu item is a submenu
    pub fn is_submenu(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_submenu(self._inner) != 0 }
    }

    /// Returns whether a menu item is a checkbox
    pub fn is_checkbox(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_checkbox(self._inner) != 0 }
    }

    /// Returns whether a menu item is a radio item
    pub fn is_radio(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_radio(self._inner) != 0 }
    }

    /// Shows the menu item
    pub fn show(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_show(self._inner) }
    }

    /// Hides the menu item
    pub fn hide(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_hide(self._inner) }
    }

    /// Get the next menu item
    pub fn next(&mut self, idx: u32) -> Option<MenuItem> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Menu_Item_next(self._inner, idx as i32);
            if ptr.is_null() {
                None
            } else {
                Some(MenuItem { _inner: ptr })
            }
        }
    }

    /// Get the menu item at `idx`
    pub fn at(&self, idx: u32) -> Option<MenuItem> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Menu_Item_next(self._inner, idx as i32);
            if ptr.is_null() {
                None
            } else {
                Some(MenuItem { _inner: ptr })
            }
        }
    }

    /// Get the user data
    /// # Safety
    /// Can return multiple mutable instances of the user data, which has a different lifetime than the object
    pub unsafe fn user_data(&self) -> Option<Box<dyn FnMut()>> {
        let ptr = Fl_Menu_Item_user_data(self._inner);
        if ptr.is_null() {
            None
        } else {
            let x = ptr as *mut Box<dyn FnMut()>;
            let x = Box::from_raw(x);
            Fl_Menu_Item_set_callback(self._inner, None, std::ptr::null_mut());
            Some(*x)
        }
    }

    /// Set a callback for the menu item
    pub fn set_callback<F: FnMut() + 'static>(&mut self, cb: F) {
        assert!(!self.was_deleted());
        unsafe {
            unsafe extern "C" fn shim(
                _wid: *mut fltk_sys::menu::Fl_Widget,
                data: *mut raw::c_void,
            ) {
                let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
                let f: &mut (dyn FnMut()) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f()));
            }
            let _old_data = self.user_data();
            let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut raw::c_void = a as *mut std::ffi::c_void;
            let callback: fltk_sys::menu::Fl_Callback = Some(shim);
            Fl_Menu_Item_set_callback(self._inner, callback, data);
        }
    }

    /// Set a callback for the menu item
    pub fn set_callback2<F: FnMut(&mut Self) + 'static>(&mut self, cb: F) {
        assert!(!self.was_deleted());
        unsafe {
            unsafe extern "C" fn shim(wid: *mut fltk_sys::menu::Fl_Widget, data: *mut raw::c_void) {
                let mut wid = crate::widget::Widget::from_widget_ptr(wid as *mut _);
                let a: *mut Box<dyn FnMut(&mut crate::widget::Widget)> =
                    data as *mut Box<dyn FnMut(&mut crate::widget::Widget)>;
                let f: &mut (dyn FnMut(&mut crate::widget::Widget)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
            }
            let _old_data = self.user_data();
            let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut raw::c_void = a as *mut std::ffi::c_void;
            let callback: fltk_sys::menu::Fl_Callback = Some(shim);
            Fl_Menu_Item_set_callback(self._inner, callback, data);
        }
    }

    /// Use a sender to send a message during callback
    pub fn emit<T: 'static + Clone + Send + Sync>(
        &mut self,
        sender: crate::app::Sender<T>,
        msg: T,
    ) {
        self.set_callback(move || sender.send(msg.clone()));
    }

    /// Check if a menu item was deleted
    pub fn was_deleted(&self) -> bool {
        self._inner.is_null()
    }
}

/// Delete a menu item
/// # Safety
/// The wrapper can't assure use after free when manually deleting a menu item
pub unsafe fn delete_menu_item(item: MenuItem) {
    Fl_Menu_Item_delete(item._inner)
}

unsafe impl Send for MenuItem {}

unsafe impl Sync for MenuItem {}

impl IntoIterator for MenuItem {
    type Item = MenuItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        let mut v: Vec<MenuItem> = vec![];
        let mut i = 0;
        while let Some(item) = self.next(i) {
            v.push(item);
            i += 1;
        }
        v.into_iter()
    }
}
