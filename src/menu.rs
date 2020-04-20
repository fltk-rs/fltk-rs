use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::menu::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a menu bar
#[derive(WidgetExt, MenuExt, Clone, Debug)]
pub struct MenuBar {
    _inner: *mut Fl_Menu_Bar,
}

/// Creates a menu button
#[derive(WidgetExt, MenuExt, Clone, Debug)]
pub struct MenuButton {
    _inner: *mut Fl_Menu_Button,
}

/// Creates a menu choice
#[derive(WidgetExt, MenuExt, Clone, Debug)]
pub struct Choice {
    _inner: *mut Fl_Choice,
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
                temp.push(CString::new(choice).unwrap().into_raw() as *mut raw::c_char);
            }
            let item_ptr = Fl_Menu_Item_new(temp.as_ptr() as *mut *mut raw::c_char, sz as i32);
            assert!(!item_ptr.is_null());
            MenuItem { _inner: item_ptr }
        }
    }
    /// Creates a popup menu at the specified coordinates and returns its choice
    pub fn popup(&mut self, x: i32, y: i32) -> Option<MenuItem> {
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
    pub fn label(&self) -> String {
        unsafe {
            let label_ptr = Fl_Menu_Item_label(self._inner);
            assert!(!label_ptr.is_null(), "Failed to get menu item label!");
            CStr::from_ptr(label_ptr as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Sets the label of the menu item
    pub fn set_label(&mut self, txt: &str) {
        unsafe {
            let txt = CString::new(txt).unwrap();
            Fl_Menu_Item_set_label(self._inner, txt.into_raw() as *const raw::c_char);
        }
    }

    /// Returns the label type of the menu item
    pub fn label_type<T: WidgetType>(&self) -> T {
        unsafe { T::from_i32(Fl_Menu_Item_label_type(self._inner)) }
    }

    /// Sets the label type of the menu item
    pub fn set_label_type<T: WidgetType>(&mut self, typ: T) {
        unsafe {
            Fl_Menu_Item_set_label_type(self._inner, typ.to_int());
        }
    }

    /// Returns the label color of the menu item
    pub fn label_color(&self) -> Color {
        unsafe { mem::transmute(Fl_Menu_Item_label_color(self._inner)) }
    }

    /// Sets the label color of the menu item
    pub fn set_label_color(&mut self, color: Color) {
        unsafe { Fl_Menu_Item_set_label_color(self._inner, color as u32) }
    }

    /// Returns the label font of the menu item
    pub fn label_font(&self) -> Font {
        unsafe { mem::transmute(Fl_Menu_Item_label_font(self._inner)) }
    }

    /// Sets the label font of the menu item
    pub fn set_label_font(&mut self, font: Font) {
        unsafe { Fl_Menu_Item_set_label_font(self._inner, font as i32) }
    }

    /// Returns the label size of the menu item
    pub fn label_size(&self) -> u32 {
        unsafe { Fl_Menu_Item_label_size(self._inner) as u32 }
    }

    /// Sets the label size of the menu item
    pub fn set_label_size(&mut self, sz: u32) {
        unsafe { Fl_Menu_Item_set_label_size(self._inner, sz as i32) }
    }

    /// Returns the value of the menu item
    pub fn value(&self) -> bool {
        unsafe {
            match Fl_Menu_Item_value(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets the menu item
    pub fn set(&mut self) {
        unsafe { Fl_Menu_Item_set(self._inner) }
    }

    /// Clears the menu item
    pub fn clear(&mut self) {
        unsafe { Fl_Menu_Item_clear(self._inner) }
    }

    /// Returns whether the menu item is visible or not
    pub fn visible(&self) -> bool {
        unsafe {
            match Fl_Menu_Item_visible(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Returns whether the menu item is active
    pub fn active(&mut self) -> bool {
        unsafe {
            match Fl_Menu_Item_active(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Activates the menu item
    pub fn activate(&mut self) {
        unsafe { Fl_Menu_Item_activate(self._inner) }
    }

    /// Deactivates the menu item
    pub fn deactivate(&mut self) {
        unsafe { Fl_Menu_Item_deactivate(self._inner) }
    }

    /// Shows the menu item
    pub fn show(&mut self) {
        unsafe { Fl_Menu_Item_show(self._inner) }
    }

    /// Hides the menu item
    pub fn hide(&mut self) {
        unsafe { Fl_Menu_Item_hide(self._inner) }
    }
}

#[cfg(test)]
mod menu {
    use super::*;
    #[test]
    fn label() {
        let mut menu = MenuBar::new(0, 0, 0, 0, "hello");
        let menu2 = menu.clone();
        menu.set_label("cloned");
        assert!(menu2.label() == "cloned");
    }
    #[test]
    fn tooltip() {
        let mut menu = MenuBar::new(0, 0, 0, 0, "hello");
        menu.set_tooltip("tooltip");
        assert!(menu.tooltip().unwrap() == "tooltip");
        let menu2 = menu.clone();
        assert!(menu2.tooltip().unwrap() == "tooltip");
    }
    #[test]
    fn add_items() {
        let mut wind = crate::window::Window::new(10, 10, 0, 0, "");
        let mut menu = MenuBar::new(0, 0, 0, 0, "hello");
        wind.end();
        wind.show();
        let mut add_items = || {
            menu.add(
                "Edit/Cut",
                Shortcut::Ctrl + 'x',
                MenuFlag::Normal,
                Box::new(|| dbg!()),
            );
            let mut menu2 = menu.clone();
            menu2.add(
                "Edit/copy",
                Shortcut::Ctrl + 'x',
                MenuFlag::Normal,
                Box::new(|| dbg!()),
            );
            menu2.do_callback();
        };
        add_items();
    }
    #[test]
    fn cloning() {
        let mut wind = crate::window::Window::new(10, 10, 0, 0, "");
        let mut menu = MenuBar::new(0, 0, 0, 0, "hello");
        wind.end();
        wind.show();
        let add_items = || {
            menu.clone().add(
                "Edit/Cut",
                Shortcut::Ctrl + 'x',
                MenuFlag::Normal,
                Box::new(|| dbg!()),
            );
            let mut menu2 = menu.clone();
            menu2.clone().add(
                "Edit/copy",
                Shortcut::Ctrl + 'x',
                MenuFlag::Normal,
                Box::new(|| dbg!()),
            );
            menu2.do_callback();
        };
        add_items();
        menu.do_callback();
    }
    #[test]
    fn ownership() {
        pub struct Owner {
            pub menu: MenuBar,
        }
        impl Owner {
            pub fn new() -> Owner {
                Owner {
                    menu: MenuBar::new(0, 0, 0, 0, "hello"),
                }
            }
            pub fn init(&mut self) {
                self.menu.clone().add(
                    "Edit/Cut",
                    Shortcut::Ctrl + 'x',
                    MenuFlag::Normal,
                    Box::new(|| self.menu.set_label("h")),
                );
            }
        }
        let mut wind = crate::window::Window::new(0, 0, 0, 0, "");
        let mut o = Owner::new();
        wind.end();
        wind.show();
        o.init();
        o.menu.do_callback();
        assert!(o.menu.label() == "h");
    }
}
