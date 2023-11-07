#[doc(hidden)]
#[macro_export]
/// Implements BrowserExt
macro_rules! impl_browser_ext {
    ($name:tt, $flname:tt) => {
        paste::paste! {
            unsafe impl BrowserExt for $name {
                fn remove(&mut self, line: i32) {
                    unsafe {
                        if line > 0 && line <= self.size() + 1 {
                            [<$flname _remove>](self.inner.widget() as _, line as i32)
                        }
                    }
                }

                fn add(&mut self, item: &str) {

                    let item = CString::safe_new(item);
                    unsafe { [<$flname _add>](self.inner.widget() as _, item.as_ptr()) }
                }

                fn add_with_data<T: Clone + 'static>(&mut self, item: &str, data: T) {
                    self.add(item);
                    self.set_data(self.size(), data);
                }

                fn insert(&mut self, line: i32, item: &str) {
                        let item = CString::safe_new(item);
                        unsafe { [<$flname _insert>](self.inner.widget() as _, line as i32, item.as_ptr()) }
                }

                fn insert_with_data<T: Clone + 'static>(&mut self, line: i32, item: &str, data: T) {
                    self.insert(line, item);
                    self.set_data(line, data);
                }

                fn move_item(&mut self, to: i32, from: i32) {

                    if to > 0 && to <= self.size() + 1 && from > 0 && from <= self.size() + 1 {
                    unsafe { [<$flname _move>](self.inner.widget() as _, to as i32, from as i32) }
                    }
                }

                fn swap(&mut self, a: i32, b: i32) {

                    if a > 0 && a <= self.size() + 1 && b > 0 && b <= self.size() + 1 {
                    unsafe { [<$flname _swap>](self.inner.widget() as _, a as i32, b as i32) }
                    }
                }

                fn clear(&mut self) {
                    unsafe {
                        [<$flname _clear>](self.inner.widget() as _)
                    }
                }

                fn size(&self) -> i32 {
                    unsafe {
                        [<$flname _size>](self.inner.widget() as _) as i32
                    }
                }

                fn select(&mut self, line: i32) {

                    if line > 0 && line <= self.size() + 1 {
                    unsafe {
                            [<$flname _select>](self.inner.widget() as _, line as i32);
                        }
                    }
                }

                fn selected(&self, line: i32) -> bool {

                    if line > 0 && line <= self.size() + 1 {
                    unsafe { [<$flname _selected>](self.inner.widget() as _, line as i32) != 0 }
                    } else {
                        false
                    }
                }

                fn text(&self, line: i32) -> Option<String> {

                    if line > 0 && line <= self.size() + 1 {
                    unsafe {
                            let text = [<$flname _text>](self.inner.widget() as _, line as i32);
                            if text.is_null() {
                                None
                            } else {
                                Some(
                                    CStr::from_ptr(text as *mut raw::c_char)
                                        .to_string_lossy()
                                        .to_string(),
                                )
                            }
                        }
                    } else {
                        None
                    }
                }

                fn selected_text(&self) -> Option<String> {
                    self.text(self.value())
                }

                fn set_text(&mut self, line: i32, txt: &str) {

                    if line > 0 && line <= self.size() + 1 {
                        let txt = CString::safe_new(txt);
                        unsafe { [<$flname _set_text>](self.inner.widget() as _, line as i32, txt.as_ptr()) }
                    }
                }

                fn load<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), FltkError> {

                    if !path.as_ref().exists() {
                        return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
                    }
                    let path = path
                        .as_ref()
                        .to_str()
                        .ok_or(FltkError::Unknown(String::from(
                            "Failed to convert path to string",
                        )))?;
                    let path = CString::new(path)?;
                    unsafe {
                        [<$flname _load_file>](self.inner.widget() as _, path.as_ptr());
                        Ok(())
                    }
                }

                fn text_size(&self) -> i32 {

                    unsafe { [<$flname _text_size>](self.inner.widget() as _) as i32 }
                }

                fn set_text_size(&mut self, c: i32) {
                    unsafe {
                        [<$flname _set_text_size>](self.inner.widget() as _, c as i32)
                    }
                }

                fn set_icon<Img: ImageExt>(&mut self, line: i32, image: Option<Img>) {

                    if line > 0 && line <= self.size() + 1 {
                        if let Some(image) = image {
                            assert!(!image.was_deleted());
                            unsafe {
                                [<$flname _set_icon>](
                                    self.inner.widget() as _,
                                    line as i32,
                                    image.as_image_ptr() as *mut _,
                                )
                            }
                        } else {
                            unsafe {
                                [<$flname _set_icon>](
                                    self.inner.widget() as _,
                                    line as i32,
                                    std::ptr::null_mut() as *mut raw::c_void,
                                )
                            }
                        }
                    }
                }

                fn icon(&self, line: i32) -> Option<Box<dyn ImageExt>> {
                    unsafe {
                        if line > 0 && line <= self.size() + 1 {
                            let image_ptr = [<$flname _icon>](self.inner.widget() as _, line as i32);
                            if image_ptr.is_null() {
                                None
                            } else {
                                let img =
                                $crate::image::Image::from_image_ptr(image_ptr as *mut fltk_sys::image::Fl_Image);
                                Some(Box::new(img))
                            }
                        } else {
                            None
                        }
                    }
                }

                fn remove_icon(&mut self, line: i32) {
                    unsafe {
                        if line > 0 && line <= self.size() + 1 {
                            [<$flname _remove_icon>](self.inner.widget() as _, line as i32)
                        }
                    }
                }

                fn top_line(&mut self, line: i32) {

                    if line > 0 && line <= self.size() + 1 {
                    unsafe { [<$flname _topline>](self.inner.widget() as _, line as i32) }
                    }
                }

                fn bottom_line(&mut self, line: i32) {

                    if line > 0 && line <= self.size() + 1 {
                    unsafe { [<$flname _bottomline>](self.inner.widget() as _, line as i32) }
                    }
                }

                fn middle_line(&mut self, line: i32) {

                    if line > 0 && line <= self.size() + 1 {
                    unsafe { [<$flname _middleline>](self.inner.widget() as _, line as i32) }
                    }
                }

                fn format_char(&self) -> char {

                    unsafe { [<$flname _format_char>](self.inner.widget() as _) as u8 as char }
                }

                fn set_format_char(&mut self, c: char) {

                    debug_assert!(c != 0 as char);
                    let c = if c as i32 > 128 { 128 as char } else { c };
                    unsafe { [<$flname _set_format_char>](self.inner.widget() as _, c as raw::c_char) }
                }

                fn column_char(&self) -> char {

                    unsafe { [<$flname _column_char>](self.inner.widget() as _) as u8 as char }
                }

                fn set_column_char(&mut self, c: char) {

                    debug_assert!(c != 0 as char);
                    let c = if c as i32 > 128 { 128 as char } else { c };
                    unsafe { [<$flname _set_column_char>](self.inner.widget() as _, c as raw::c_char) }
                }

                fn column_widths(&self) -> Vec<i32> {

                    unsafe {
                        let widths = [<$flname _column_widths>](self.inner.widget() as _);
                        // Should never throw
                        assert!(!widths.is_null());
                        let mut v: Vec<i32> = vec![];
                        let mut i = 0;
                        while (*widths.offset(i) != 0) {
                            v.push(*widths.offset(i));
                            i += 1;
                        }
                        v
                    }
                }

                fn set_column_widths(&mut self, arr: &[i32]) {

                    debug_assert!(!arr.contains(&0), "FLTK uses 0 as a sentinel value for the array/slice. To hide a column, use -1!");
                    unsafe {
                        let old = [<$flname _column_widths>](self.inner.widget() as _);
                        let mut v = arr.to_vec();
                        v.push(0);
                        let v = v.into_boxed_slice();
                        [<$flname _set_column_widths>](self.inner.widget() as _, Box::into_raw(v) as _);
                        if !old.is_null() && *old.offset(0) != 0 {
                            let _ = Box::from_raw(old as *mut i32);
                        }
                    }
                }

                fn displayed(&self, line: i32) -> bool {

                    if line > 0 && line <= self.size() + 1 {
                    unsafe { [<$flname _displayed>](self.inner.widget() as _, line as i32) != 0 }
                    } else {
                        false
                    }
                }

                fn make_visible(&mut self, line: i32) {

                    if line > 0 && line <= self.size() + 1 {
                    unsafe { [<$flname _make_visible>](self.inner.widget() as _, line as i32) }
                    }
                }

                fn position(&self) -> i32 {

                    unsafe { [<$flname _position>](self.inner.widget() as _) as i32 }
                }

                fn set_position(&mut self, pos: i32) {

                    unsafe { [<$flname _set_position>](self.inner.widget() as _, pos as i32) }
                }

                fn hposition(&self) -> i32 {

                    unsafe { [<$flname _hposition>](self.inner.widget() as _) as i32 }
                }

                fn set_hposition(&mut self, pos: i32) {

                    unsafe { [<$flname _set_hposition>](self.inner.widget() as _, pos as i32) }
                }

                fn has_scrollbar(&self) -> $crate::browser::BrowserScrollbar {

                    unsafe { std::mem::transmute([<$flname _has_scrollbar>](self.inner.widget() as _)) }
                }

                fn set_has_scrollbar(&mut self, mode: $crate::browser::BrowserScrollbar) {

                    unsafe {
                        [<$flname _set_has_scrollbar>](self.inner.widget() as _, mode as raw::c_uchar)
                    }
                }

                fn scrollbar_size(&self) -> i32 {

                    unsafe { [<$flname _scrollbar_size>](self.inner.widget() as _) as i32 }
                }

                fn set_scrollbar_size(&mut self, new_size: i32) {

                    unsafe { [<$flname _set_scrollbar_size>](self.inner.widget() as _, new_size as i32) }
                }

                fn sort(&mut self) {

                    unsafe { [<$flname _sort>](self.inner.widget() as _) }
                }

                fn scrollbar(&self) -> $crate::valuator::Scrollbar {

                    unsafe {
                        let ptr = [<$flname _scrollbar>](self.inner.widget() as _);
                        assert!(!ptr.is_null());
                        $crate::valuator::Scrollbar::from_widget_ptr(
                            ptr as *mut fltk_sys::widget::Fl_Widget,
                        )
                    }
                }

                fn hscrollbar(&self) -> $crate::valuator::Scrollbar {

                    unsafe {
                        let ptr = [<$flname _hscrollbar>](self.inner.widget() as _);
                        assert!(!ptr.is_null());
                        $crate::valuator::Scrollbar::from_widget_ptr(
                            ptr as *mut fltk_sys::widget::Fl_Widget,
                        )
                    }
                }

                fn value(&self) -> i32 {

                    unsafe { [<$flname _value>](self.inner.widget() as _) as i32 }
                }

                fn set_data<T: Clone + 'static>(&mut self, line: i32, data: T) {

                    if line > 0 && line <= self.size() + 1 {
                    unsafe {
                            [<$flname _set_data>](self.inner.widget() as _, line, Box::into_raw(Box::from(data)) as _);
                        }
                    }
                }

                unsafe fn data<T: Clone + 'static>(&self, line: i32) -> Option<T> {

                    if line > 0 && line <= self.size() + 1 {
                        let ptr = [<$flname _data>](self.inner.widget() as _, line);
                        if ptr.is_null() {
                            None
                        } else {
                            let data = ptr as *const _ as *mut T;
                            Some((*data).clone())
                        }
                    } else {
                        None
                    }
                }

                fn hide_line(&mut self, line: i32) {

                    if line > 0 && line <= self.size() + 1 {
                    unsafe { [<$flname _hide_line>](self.inner.widget() as _, line); }
                    }
                }

                fn selected_items(&self) -> Vec<i32> {
                    let mut temp = vec![];
                    if self.value() > 0 {
                        for i in 1..self.size() + 1 {
                            if self.selected(i) {
                                temp.push(i);
                            }
                        }
                    }
                    temp
                }
            }
        }
    };
}

pub use impl_browser_ext;
