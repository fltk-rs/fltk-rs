pub use crate::prelude::*;
use fltk_sys::menu::*;
use std::{ffi, mem, os::raw, ptr};

#[derive(WidgetTrait, MenuTrait, Debug, Clone)]
pub struct MenuBar {
    _inner: *mut Fl_Menu_Bar,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, MenuTrait, Debug, Clone)]
pub struct MenuButton {
    _inner: *mut Fl_Menu_Button,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, MenuTrait, Debug, Clone)]
pub struct Choice {
    _inner: *mut Fl_Choice,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    _inner: *mut Fl_Menu_Item,
    _title: ffi::CString,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
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
    pub fn label(&self) -> String {
        unsafe {
            ffi::CStr::from_ptr(Fl_Menu_Item_label(self._inner))
                .to_str()
                .unwrap()
                .to_owned()
        }
    }
    pub fn set_label(&mut self, txt: &str) {
        unsafe {
            let txt = ffi::CString::new(txt).unwrap();
            Fl_Menu_Item_set_label(self._inner, txt.as_ptr() as *const raw::c_char);
        }
    }
    pub fn label_type<T: WidgetType>(&self) -> T {
        unsafe { T::from_i32(Fl_Menu_Item_label_type(self._inner)) }
    }
    pub fn set_label_type<T: WidgetType>(&mut self, typ: T) {
        unsafe {
            Fl_Menu_Item_set_label_type(self._inner, typ.to_int());
        }
    }
    pub fn label_color(&self) -> Color {
        unsafe { mem::transmute(Fl_Menu_Item_label_color(self._inner)) }
    }

    pub fn set_label_color(&mut self, color: Color) {
        unsafe { Fl_Menu_Item_set_label_color(self._inner, color as i32) }
    }

    pub fn label_font(&self) -> Font {
        unsafe { mem::transmute(Fl_Menu_Item_label_font(self._inner)) }
    }

    pub fn set_label_font(&mut self, font: Font) {
        unsafe { Fl_Menu_Item_set_label_color(self._inner, font as i32) }
    }

    pub fn label_size(&self) -> usize {
        unsafe { Fl_Menu_Item_label_size(self._inner) as usize }
    }

    pub fn set_label_size(&mut self, sz: usize) {
        unsafe { Fl_Menu_Item_set_label_size(self._inner, sz as i32) }
    }

    pub fn value(&self) -> bool {
        unsafe {
            match Fl_Menu_Item_value(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn set(&mut self) {
        unsafe { Fl_Menu_Item_set(self._inner) }
    }

    pub fn clear(&mut self) {
        unsafe { Fl_Menu_Item_clear(self._inner) }
    }

    pub fn visible(&self) -> bool {
        unsafe {
            match Fl_Menu_Item_visible(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn active(&mut self) {
        unsafe { Fl_Menu_Item_activate(self._inner) }
    }

    pub fn activate(&mut self) {
        unsafe { Fl_Menu_Item_activate(self._inner) }
    }

    pub fn deactivate(&mut self) {
        unsafe { Fl_Menu_Item_deactivate(self._inner) }
    }
    pub fn show(&mut self) {
        unsafe { Fl_Menu_Item_show(self._inner) }
    }

    pub fn hide(&mut self) {
        unsafe { Fl_Menu_Item_hide(self._inner) }
    }
}
