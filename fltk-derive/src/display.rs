#[macro_export]
macro_rules! impl_display_ext {
    ($name: tt, $flname: tt) => {
        unsafe impl DisplayExt for $name {
            fn buffer(&self) -> Option<TextBuffer> {
                unsafe {
                    assert!(!self.was_deleted());
                    let buffer = concat_idents!($flname, _get_buffer)(self.inner);
                    if buffer.is_null() {
                        None
                    } else {
                        let mut buf = TextBuffer::from_ptr(buffer);
                        Some(buf)
                    }
                }
            }

            fn set_buffer<B: Into<Option<crate::text::TextBuffer>>>(&mut self, buffer: B) {
                unsafe {
                    assert!(!self.was_deleted());
                    if let Some(buffer) = buffer.into() {
                        let _old_buf = self.buffer();
                        concat_idents!($flname, _set_buffer)(self.inner, buffer.as_ptr())
                    } else {
                        concat_idents!($flname, _set_buffer)(
                            self.inner,
                            std::ptr::null_mut() as *mut Fl_Text_Buffer,
                        )
                    }
                }
            }

            fn style_buffer(&self) -> Option<TextBuffer> {
                unsafe {
                    assert!(!self.was_deleted());
                    let buffer = concat_idents!($flname, _get_style_buffer)(self.inner);
                    if buffer.is_null() {
                        None
                    } else {
                        let mut buf = TextBuffer::from_ptr(buffer);
                        Some(buf)
                    }
                }
            }

            fn text_font(&self) -> Font {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { mem::transmute(concat_idents!($flname, _text_font)(self.inner)) }
            }

            fn set_text_font(&mut self, font: Font) {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { concat_idents!($flname, _set_text_font)(self.inner, font.bits() as i32) }
            }

            fn text_color(&self) -> Color {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { mem::transmute(concat_idents!($flname, _text_color)(self.inner)) }
            }

            fn set_text_color(&mut self, color: Color) {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { concat_idents!($flname, _set_text_color)(self.inner, color.bits() as u32) }
            }

            fn text_size(&self) -> i32 {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { concat_idents!($flname, _text_size)(self.inner) as i32 }
            }

            fn set_text_size(&mut self, sz: i32) {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { concat_idents!($flname, _set_text_size)(self.inner, sz as i32) }
            }

            fn scroll(&mut self, topLineNum: i32, horizOffset: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _scroll)(
                        self.inner,
                        topLineNum as i32,
                        horizOffset as i32,
                    )
                }
            }

            fn insert(&self, text: &str) {
                let text = CString::safe_new(text);
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _insert)(self.inner, text.as_ptr())
                }
            }

            fn set_insert_position(&mut self, newPos: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _set_insert_position)(self.inner, newPos as i32)
                }
            }

            fn insert_position(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _insert_position)(self.inner) as i32
                }
            }

            fn position_to_xy(&self, pos: i32) -> (i32, i32) {
                unsafe {
                    let mut x: i32 = 0;
                    let mut y: i32 = 0;
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _position_to_xy)(
                        self.inner, pos as i32, &mut x, &mut y,
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
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _count_lines)(self.inner, start as i32, end as i32, x)
                        as i32
                }
            }

            fn move_right(&mut self) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    let x = concat_idents!($flname, _move_right)(self.inner);
                    if x == 0 {
                        Err(FltkError::Internal(FltkErrorKind::FailedOperation))
                    } else {
                        Ok(())
                    }
                }
            }

            fn move_left(&mut self) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    let x = concat_idents!($flname, _move_left)(self.inner);
                    if x == 0 {
                        Err(FltkError::Internal(FltkErrorKind::FailedOperation))
                    } else {
                        Ok(())
                    }
                }
            }

            fn move_up(&mut self) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    let x = concat_idents!($flname, _move_up)(self.inner);
                    if x == 0 {
                        Err(FltkError::Internal(FltkErrorKind::FailedOperation))
                    } else {
                        Ok(())
                    }
                }
            }

            fn move_down(&mut self) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    let x = concat_idents!($flname, _move_down)(self.inner);
                    if x == 0 {
                        Err(FltkError::Internal(FltkErrorKind::FailedOperation))
                    } else {
                        Ok(())
                    }
                }
            }

            fn show_cursor(&mut self, val: bool) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _show_cursor)(self.inner, val as i32);
                }
            }

            fn set_highlight_data<B: Into<Option<TextBuffer>>>(
                &mut self,
                mut style_buffer: B,
                entries: Vec<StyleTableEntry>,
            ) {
                assert!(!self.was_deleted());
                assert!(entries.len() < 61);
                let entries = if entries.len() == 0 {
                    vec![StyleTableEntry {
                        color: Color::Black,
                        font: Font::Helvetica,
                        size: crate::app::font_size(),
                    }]
                } else {
                    entries
                };
                let mut colors: Vec<u32> = vec![];
                let mut fonts: Vec<i32> = vec![];
                let mut sizes: Vec<i32> = vec![];
                for entry in entries.iter() {
                    colors.push(entry.color.bits() as u32);
                    fonts.push(entry.font.bits() as i32);
                    sizes.push(entry.size as i32);
                }
                let style_buffer = style_buffer.into();
                if let Some(style_buffer) = style_buffer {
                    let _old_buf = self.style_buffer();
                    unsafe {
                        concat_idents!($flname, _set_highlight_data)(
                            self.inner,
                            style_buffer.as_ptr() as *mut raw::c_void,
                            colors.as_mut_ptr(),
                            fonts.as_mut_ptr(),
                            sizes.as_mut_ptr(),
                            entries.len() as i32,
                        )
                    }
                } else {
                    if let Some(buf) = self.style_buffer() {
                        unsafe {
                            concat_idents!($flname, _set_highlight_data)(
                                self.inner,
                                buf.as_ptr() as _,
                                colors.as_mut_ptr(),
                                fonts.as_mut_ptr(),
                                sizes.as_mut_ptr(),
                                1,
                            )
                        }
                    }
                }
            }

            fn unset_highlight_data(&mut self, style_buffer: TextBuffer) {
                assert!(!self.was_deleted());
                unsafe {
                    let mut colors = [Color::Black.bits()];
                    let mut fonts = [Font::Helvetica.bits()];
                    let mut sizes = [14];
                    concat_idents!($flname, _set_highlight_data)(
                        self.inner,
                        style_buffer.as_ptr() as *mut raw::c_void,
                        colors.as_mut_ptr(),
                        fonts.as_mut_ptr(),
                        sizes.as_mut_ptr(),
                        1,
                    )
                }
            }

            fn set_cursor_style(&mut self, style: crate::text::Cursor) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_cursor_style)(self.inner, style as i32)
                }
            }

            fn set_cursor_color(&mut self, color: Color) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_cursor_color)(self.inner, color.bits() as u32)
                }
            }

            fn set_scrollbar_size(&mut self, size: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_scrollbar_size)(self.inner, size as i32)
                }
            }

            fn set_scrollbar_align(&mut self, align: Align) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_scrollbar_align)(self.inner, align.bits() as i32)
                }
            }

            fn cursor_style(&self) -> crate::text::Cursor {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(concat_idents!($flname, _cursor_style)(self.inner))
                }
            }

            fn cursor_color(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(concat_idents!($flname, _cursor_color)(self.inner))
                }
            }

            fn scrollbar_size(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _scrollbar_size)(self.inner) as i32
                }
            }

            fn scrollbar_align(&self) -> Align {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(concat_idents!($flname, _scrollbar_align)(self.inner))
                }
            }

            fn line_start(&self, pos: i32) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _line_start)(self.inner, pos as i32) as i32
                }
            }

            fn line_end(&self, start_pos: i32, is_line_start: bool) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _line_end)(
                        self.inner,
                        start_pos as i32,
                        is_line_start as i32,
                    ) as i32
                }
            }

            fn skip_lines(&mut self, start_pos: i32, lines: i32, is_line_start: bool) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _skip_lines)(
                        self.inner,
                        start_pos as i32,
                        lines as i32,
                        is_line_start as i32,
                    ) as i32
                }
            }

            fn rewind_lines(&mut self, start_pos: i32, lines: i32) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _rewind_lines)(
                        self.inner,
                        start_pos as i32,
                        lines as i32,
                    ) as i32
                }
            }

            fn next_word(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _next_word)(self.inner)
                }
            }

            fn previous_word(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _previous_word)(self.inner)
                }
            }

            fn word_start(&self, pos: i32) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _word_start)(self.inner, pos as i32) as i32
                }
            }

            fn word_end(&self, pos: i32) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _word_end)(self.inner, pos as i32) as i32
                }
            }

            fn x_to_col(&self, x: f64) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _x_to_col)(self.inner, x)
                }
            }

            fn col_to_x(&self, col: f64) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _col_to_x)(self.inner, col)
                }
            }

            fn set_linenumber_width(&mut self, w: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_linenumber_width)(self.inner, w)
                }
            }

            fn linenumber_width(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _linenumber_width)(self.inner)
                }
            }

            fn set_linenumber_font(&mut self, font: Font) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_linenumber_font)(self.inner, font.bits() as i32)
                }
            }

            fn linenumber_font(&self) -> Font {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(concat_idents!($flname, _linenumber_font)(self.inner))
                }
            }

            fn set_linenumber_size(&mut self, size: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_linenumber_size)(self.inner, size as i32)
                }
            }

            fn linenumber_size(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _linenumber_size)(self.inner) as i32
                }
            }

            fn set_linenumber_fgcolor(&mut self, color: Color) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_linenumber_fgcolor)(
                        self.inner,
                        color.bits() as u32,
                    )
                }
            }

            fn linenumber_fgcolor(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(concat_idents!($flname, _linenumber_fgcolor)(self.inner))
                }
            }

            fn set_linenumber_bgcolor(&mut self, color: Color) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_linenumber_bgcolor)(
                        self.inner,
                        color.bits() as u32,
                    )
                }
            }

            fn linenumber_bgcolor(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(concat_idents!($flname, _linenumber_bgcolor)(self.inner))
                }
            }

            fn set_linenumber_align(&mut self, align: Align) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_linenumber_align)(self.inner, align.bits() as i32)
                }
            }

            fn linenumber_align(&self) -> Align {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(concat_idents!($flname, _linenumber_align)(self.inner))
                }
            }

            fn in_selection(&self, x: i32, y: i32) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    concat_idents!($flname, _in_selection)(self.inner, x, y) != 0
                }
            }

            fn wrap_mode(&mut self, wrap: crate::text::WrapMode, wrap_margin: i32) {
                assert!(!self.was_deleted());
                unsafe { concat_idents!($flname, _wrap_mode)(self.inner, wrap as i32, wrap_margin) }
            }

            fn wrapped_column(&self, row: i32, column: i32) -> i32 {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { concat_idents!($flname, _wrapped_column)(self.inner, row, column) }
            }

            fn wrapped_row(&self, row: i32) -> i32 {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { concat_idents!($flname, _wrapped_row)(self.inner, row) }
            }
        }
    };
}
