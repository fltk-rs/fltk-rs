#[doc(hidden)]
#[macro_export]
/// Implements DisplayExt
macro_rules! impl_display_ext {
    ($name: ident, $flname: ident) => {
        paste::paste! {
            unsafe impl DisplayExt for $name {
                fn buffer(&self) -> Option<$crate::text::TextBuffer> {
                    unsafe {
                        let buffer = [<$flname _get_buffer>](self.inner.widget() as _);
                        if buffer.is_null() {
                            None
                        } else {
                            let buf = $crate::text::TextBuffer::from_ptr(buffer);
                            Some(buf)
                        }
                    }
                }

                fn has_buffer(&self) -> bool {
                    unsafe {
                        let buffer = [<$flname _get_buffer>](self.inner.widget() as _);
                        !buffer.is_null()
                    }
                }

                fn set_buffer<B: Into<Option<$crate::text::TextBuffer>>>(&mut self, buffer: B) {
                    unsafe {
                        if let Some(buffer) = buffer.into() {
                            [<$flname _set_buffer>](self.inner.widget() as _, buffer.as_ptr())
                        } else {
                            [<$flname _set_buffer>](
                                self.inner.widget() as _,
                                std::ptr::null_mut() as *mut Fl_Text_Buffer,
                            )
                        }
                    }
                }

                fn style_buffer(&self) -> Option<$crate::text::TextBuffer> {
                    unsafe {
                        let buffer = [<$flname _get_style_buffer>](self.inner.widget() as _);
                        if buffer.is_null() {
                            None
                        } else {
                            let buf = $crate::text::TextBuffer::from_ptr(buffer);
                            Some(buf)
                        }
                    }
                }

                fn text_font(&self) -> $crate::enums::Font {

                    assert!(self.has_buffer());
                    unsafe { std::mem::transmute([<$flname _text_font>](self.inner.widget() as _)) }
                }

                fn set_text_font(&mut self, font: $crate::enums::Font) {

                    unsafe { [<$flname _set_text_font>](self.inner.widget() as _, font.bits() as i32) }
                }

                fn text_color(&self) -> $crate::enums::Color {

                    assert!(self.has_buffer());
                    unsafe { std::mem::transmute([<$flname _text_color>](self.inner.widget() as _)) }
                }

                fn set_text_color(&mut self, color: $crate::enums::Color) {

                    unsafe { [<$flname _set_text_color>](self.inner.widget() as _, color.bits() as u32) }
                }

                fn text_size(&self) -> i32 {

                    assert!(self.has_buffer());
                    unsafe { [<$flname _text_size>](self.inner.widget() as _) as i32 }
                }

                fn set_text_size(&mut self, sz: i32) {

                    unsafe { [<$flname _set_text_size>](self.inner.widget() as _, sz as i32) }
                }

                fn scroll(&mut self, top_line_num: i32, h_offset: i32) {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _scroll>](
                            self.inner.widget() as _,
                            top_line_num as i32,
                            h_offset as i32,
                        )
                    }
                }

                fn insert(&self, text: &str) {
                    let text = CString::safe_new(text);
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _insert>](self.inner.widget() as _, text.as_ptr())
                    }
                }

                fn set_insert_position(&mut self, new_pos: i32) {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _set_insert_position>](self.inner.widget() as _, new_pos as i32)
                    }
                }

                fn insert_position(&self) -> i32 {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _insert_position>](self.inner.widget() as _) as i32
                    }
                }

                fn position_to_xy(&self, pos: i32) -> (i32, i32) {
                    unsafe {
                        let mut x: i32 = 0;
                        let mut y: i32 = 0;

                        assert!(self.has_buffer());
                        [<$flname _position_to_xy>](
                            self.inner.widget() as _, pos as i32, &mut x, &mut y,
                        );
                        (x as i32, y as i32)
                    }
                }

                fn count_lines(&self, start: i32, end: i32, is_line_start: bool) -> i32 {
                    let x = match is_line_start {
                        true => 1,
                        false => 0,
                    };
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _count_lines>](self.inner.widget() as _, start as i32, end as i32, x)
                            as i32
                    }
                }

                fn move_right(&mut self) -> Result<(), FltkError> {
                    unsafe {
                        assert!(self.has_buffer());
                        let x = [<$flname _move_right>](self.inner.widget() as _);
                        if x == 0 {
                            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
                        } else {
                            Ok(())
                        }
                    }
                }

                fn move_left(&mut self) -> Result<(), FltkError> {
                    unsafe {
                        assert!(self.has_buffer());
                        let x = [<$flname _move_left>](self.inner.widget() as _);
                        if x == 0 {
                            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
                        } else {
                            Ok(())
                        }
                    }
                }

                fn move_up(&mut self) -> Result<(), FltkError> {
                    unsafe {
                        assert!(self.has_buffer());
                        let x = [<$flname _move_up>](self.inner.widget() as _);
                        if x == 0 {
                            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
                        } else {
                            Ok(())
                        }
                    }
                }

                fn move_down(&mut self) -> Result<(), FltkError> {
                    unsafe {
                        assert!(self.has_buffer());
                        let x = [<$flname _move_down>](self.inner.widget() as _);
                        if x == 0 {
                            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
                        } else {
                            Ok(())
                        }
                    }
                }

                fn show_cursor(&mut self, val: bool) {
                    unsafe {
                        [<$flname _show_cursor>](self.inner.widget() as _, val as i32);
                    }
                }

                fn set_highlight_data<B: Into<Option<$crate::text::TextBuffer>>, E: Into<Vec<$crate::text::StyleTableEntry>>>(
                    &mut self,
                    style_buffer: B,
                    entries: E,
                ) {
                    let entries = entries.into();

                    assert!(entries.len() < 127);
                    let entries = if entries.len() == 0 {
                        vec![$crate::text::StyleTableEntry {
                            color: $crate::enums::Color::Black,
                            font: $crate::enums::Font::Helvetica,
                            size: $crate::app::font_size(),
                        }]
                    } else {
                        entries
                    };
                    let mut colors: Vec<u32> = vec![];
                    let mut fonts: Vec<i32> = vec![];
                    let mut sizes: Vec<i32> = vec![];
                    let mut attrs: Vec<u32> = vec![];
                    let mut bgcols: Vec<u32> = vec![];
                    for entry in entries.iter() {
                        colors.push(entry.color.bits() as u32);
                        fonts.push(entry.font.bits() as i32);
                        sizes.push(entry.size as i32);
                        attrs.push(0);
                        bgcols.push(0);
                    }
                    let style_buffer = style_buffer.into();
                    if let Some(style_buffer) = style_buffer {
                        let _old_buf = self.style_buffer();
                        unsafe {
                            [<$flname _set_highlight_data>](
                                self.inner.widget() as _,
                                style_buffer.as_ptr() as *mut raw::c_void,
                                colors.as_mut_ptr(),
                                fonts.as_mut_ptr(),
                                sizes.as_mut_ptr(),
                                attrs.as_mut_ptr(),
                                bgcols.as_mut_ptr(),
                                entries.len() as i32,
                            )
                        }
                    } else {
                        if let Some(buf) = self.style_buffer() {
                            unsafe {
                                [<$flname _set_highlight_data>](
                                    self.inner.widget() as _,
                                    buf.as_ptr() as _,
                                    colors.as_mut_ptr(),
                                    fonts.as_mut_ptr(),
                                    sizes.as_mut_ptr(),
                                    attrs.as_mut_ptr(),
                                    bgcols.as_mut_ptr(),
                                    1,
                                )
                            }
                        }
                    }
                }

                fn set_highlight_data_ext<B: Into<Option<$crate::text::TextBuffer>>, E: Into<Vec<$crate::text::StyleTableEntryExt>>>(
                    &mut self,
                    style_buffer: B,
                    entries: E,
                ) {
                    let entries = entries.into();

                    assert!(entries.len() < 127);
                    let entries = if entries.len() == 0 {
                        vec![$crate::text::StyleTableEntryExt {
                            color: $crate::enums::Color::Black,
                            font: $crate::enums::Font::Helvetica,
                            size: $crate::app::font_size(),
                            attr: $crate::text::TextAttr::None,
                            bgcolor: $crate::enums::Color::Black,
                        }]
                    } else {
                        entries
                    };
                    let mut colors: Vec<u32> = vec![];
                    let mut fonts: Vec<i32> = vec![];
                    let mut sizes: Vec<i32> = vec![];
                    let mut attrs: Vec<u32> = vec![];
                    let mut bgcols: Vec<u32> = vec![];
                    for entry in entries.iter() {
                        colors.push(entry.color.bits() as u32);
                        fonts.push(entry.font.bits() as i32);
                        sizes.push(entry.size as i32);
                        attrs.push(entry.attr as u32);
                        bgcols.push(entry.bgcolor.bits() as u32);
                    }
                    let style_buffer = style_buffer.into();
                    if let Some(style_buffer) = style_buffer {
                        let _old_buf = self.style_buffer();
                        unsafe {
                            [<$flname _set_highlight_data>](
                                self.inner.widget() as _,
                                style_buffer.as_ptr() as *mut raw::c_void,
                                colors.as_mut_ptr(),
                                fonts.as_mut_ptr(),
                                sizes.as_mut_ptr(),
                                attrs.as_mut_ptr(),
                                bgcols.as_mut_ptr(),
                                entries.len() as i32,
                            )
                        }
                    } else {
                        if let Some(buf) = self.style_buffer() {
                            unsafe {
                                [<$flname _set_highlight_data>](
                                    self.inner.widget() as _,
                                    buf.as_ptr() as _,
                                    colors.as_mut_ptr(),
                                    fonts.as_mut_ptr(),
                                    sizes.as_mut_ptr(),
                                    attrs.as_mut_ptr(),
                                    bgcols.as_mut_ptr(),
                                    1,
                                )
                            }
                        }
                    }
                }

                fn unset_highlight_data<B: Into<Option<$crate::text::TextBuffer>>>(&mut self, style_buffer: B) {

                    unsafe {
                        let mut colors = [$crate::enums::Color::Black.bits()];
                        let mut fonts = [$crate::enums::Font::Helvetica.bits()];
                        let mut sizes = [14];
                        let mut attrs = [0];
                        let mut bgcols = [0];
                        if let Some(style_buffer) = style_buffer.into() {
                            [<$flname _set_highlight_data>](
                                self.inner.widget() as _,
                                style_buffer.as_ptr() as *mut raw::c_void,
                                colors.as_mut_ptr(),
                                fonts.as_mut_ptr(),
                                sizes.as_mut_ptr(),
                                attrs.as_mut_ptr(),
                                bgcols.as_mut_ptr(),
                                1,
                            )
                        }
                    }
                }

                fn set_cursor_style(&mut self, style: $crate::text::Cursor) {
                    unsafe {
                        [<$flname _set_cursor_style>](self.inner.widget() as _, style as i32)
                    }
                }

                fn set_cursor_color(&mut self, color: $crate::enums::Color) {
                    unsafe {
                        [<$flname _set_cursor_color>](self.inner.widget() as _, color.bits() as u32)
                    }
                }

                fn set_scrollbar_size(&mut self, size: i32) {
                    unsafe {
                        [<$flname _set_scrollbar_size>](self.inner.widget() as _, size as i32)
                    }
                }

                fn set_scrollbar_align(&mut self, align: $crate::enums::Align) {
                    unsafe {
                        [<$flname _set_scrollbar_align>](self.inner.widget() as _, align.bits() as i32)
                    }
                }

                fn cursor_style(&self) -> $crate::text::Cursor {
                    unsafe {
                        std::mem::transmute([<$flname _cursor_style>](self.inner.widget() as _))
                    }
                }

                fn cursor_color(&self) -> $crate::enums::Color {
                    unsafe {
                        std::mem::transmute([<$flname _cursor_color>](self.inner.widget() as _))
                    }
                }

                fn scrollbar_size(&self) -> i32 {
                    unsafe {
                        [<$flname _scrollbar_size>](self.inner.widget() as _) as i32
                    }
                }

                fn scrollbar_align(&self) -> $crate::enums::Align {
                    unsafe {
                        std::mem::transmute([<$flname _scrollbar_align>](self.inner.widget() as _))
                    }
                }

                fn line_start(&self, pos: i32) -> i32 {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _line_start>](self.inner.widget() as _, pos as i32) as i32
                    }
                }

                fn line_end(&self, start_pos: i32, is_line_start: bool) -> i32 {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _line_end>](
                            self.inner.widget() as _,
                            start_pos as i32,
                            is_line_start as i32,
                        ) as i32
                    }
                }

                fn skip_lines(&mut self, start_pos: i32, lines: i32, is_line_start: bool) -> i32 {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _skip_lines>](
                            self.inner.widget() as _,
                            start_pos as i32,
                            lines as i32,
                            is_line_start as i32,
                        ) as i32
                    }
                }

                fn rewind_lines(&mut self, start_pos: i32, lines: i32) -> i32 {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _rewind_lines>](
                            self.inner.widget() as _,
                            start_pos as i32,
                            lines as i32,
                        ) as i32
                    }
                }

                fn next_word(&mut self) {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _next_word>](self.inner.widget() as _)
                    }
                }

                fn previous_word(&mut self) {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _previous_word>](self.inner.widget() as _)
                    }
                }

                fn word_start(&self, pos: i32) -> i32 {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _word_start>](self.inner.widget() as _, pos as i32) as i32
                    }
                }

                fn word_end(&self, pos: i32) -> i32 {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _word_end>](self.inner.widget() as _, pos as i32) as i32
                    }
                }

                fn x_to_col(&self, x: f64) -> f64 {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _x_to_col>](self.inner.widget() as _, x)
                    }
                }

                fn col_to_x(&self, col: f64) -> f64 {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _col_to_x>](self.inner.widget() as _, col)
                    }
                }

                fn set_linenumber_width(&mut self, w: i32) {
                    unsafe {
                        [<$flname _set_linenumber_width>](self.inner.widget() as _, w)
                    }
                }

                fn linenumber_width(&self) -> i32 {
                    unsafe {
                        [<$flname _linenumber_width>](self.inner.widget() as _)
                    }
                }

                fn set_linenumber_font(&mut self, font: $crate::enums::Font) {
                    unsafe {
                        [<$flname _set_linenumber_font>](self.inner.widget() as _, font.bits() as i32)
                    }
                }

                fn linenumber_font(&self) -> $crate::enums::Font {
                    unsafe {
                        std::mem::transmute([<$flname _linenumber_font>](self.inner.widget() as _))
                    }
                }

                fn set_linenumber_size(&mut self, size: i32) {
                    unsafe {
                        [<$flname _set_linenumber_size>](self.inner.widget() as _, size as i32)
                    }
                }

                fn linenumber_size(&self) -> i32 {
                    unsafe {
                        [<$flname _linenumber_size>](self.inner.widget() as _) as i32
                    }
                }

                fn set_linenumber_fgcolor(&mut self, color: $crate::enums::Color) {
                    unsafe {
                        [<$flname _set_linenumber_fgcolor>](
                            self.inner.widget() as _,
                            color.bits() as u32,
                        )
                    }
                }

                fn linenumber_fgcolor(&self) -> $crate::enums::Color {
                    unsafe {
                        std::mem::transmute([<$flname _linenumber_fgcolor>](self.inner.widget() as _))
                    }
                }

                fn set_linenumber_bgcolor(&mut self, color: $crate::enums::Color) {
                    unsafe {
                        [<$flname _set_linenumber_bgcolor>](
                            self.inner.widget() as _,
                            color.bits() as u32,
                        )
                    }
                }

                fn linenumber_bgcolor(&self) -> $crate::enums::Color {
                    unsafe {
                        std::mem::transmute([<$flname _linenumber_bgcolor>](self.inner.widget() as _))
                    }
                }

                fn set_linenumber_align(&mut self, align: $crate::enums::Align) {
                    unsafe {
                        [<$flname _set_linenumber_align>](self.inner.widget() as _, align.bits() as i32)
                    }
                }

                fn linenumber_align(&self) -> $crate::enums::Align {
                    unsafe {
                        std::mem::transmute([<$flname _linenumber_align>](self.inner.widget() as _))
                    }
                }

                fn in_selection(&self, x: i32, y: i32) -> bool {
                    unsafe {
                        assert!(self.has_buffer());
                        [<$flname _in_selection>](self.inner.widget() as _, x, y) != 0
                    }
                }

                fn wrap_mode(&mut self, wrap: $crate::text::WrapMode, wrap_margin: i32) {

                    unsafe { [<$flname _wrap_mode>](self.inner.widget() as _, wrap as i32, wrap_margin) }
                }

                fn wrapped_column(&self, row: i32, column: i32) -> i32 {

                    assert!(self.has_buffer());
                    unsafe { [<$flname _wrapped_column>](self.inner.widget() as _, row, column) }
                }

                fn wrapped_row(&self, row: i32) -> i32 {

                    assert!(self.has_buffer());
                    unsafe { [<$flname _wrapped_row>](self.inner.widget() as _, row) }
                }

                fn set_grammar_underline_color(&mut self, color: $crate::enums::Color) {

                    unsafe {
                        [<$flname _set_grammar_underline_color>](self.inner.widget() as _, color.bits() as u32)
                    }
                }

                fn grammar_underline_color(&self) -> $crate::enums::Color {

                    unsafe {
                        std::mem::transmute([<$flname _grammar_underline_color>](self.inner.widget() as _))
                    }
                }

                fn set_spelling_underline_color(&mut self, color: $crate::enums::Color) {

                    unsafe {
                        [<$flname _set_spelling_underline_color>](self.inner.widget() as _, color.bits() as u32)
                    }
                }

                fn spelling_underline_color(&self) -> $crate::enums::Color {

                    unsafe {
                        std::mem::transmute([<$flname _spelling_underline_color>](self.inner.widget() as _))
                    }
                }

                fn set_secondary_selection_color(&mut self, color: $crate::enums::Color) {

                    unsafe {
                        [<$flname _set_secondary_selection_color>](self.inner.widget() as _, color.bits() as u32)
                    }
                }

                fn secondary_selection_color(&self) -> $crate::enums::Color {

                    unsafe {
                        std::mem::transmute([<$flname _secondary_selection_color>](self.inner.widget() as _))
                    }
                }
                fn show_insert_position(&mut self) {

                    unsafe {
                        [<$flname _show_insert_position>](self.inner.widget() as _);
                    }
                }
            }
        }
    };
}

pub use impl_display_ext;
