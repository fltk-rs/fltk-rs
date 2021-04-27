use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_display_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let get_buffer = Ident::new(
        format!("{}_{}", name_str, "get_buffer").as_str(),
        name.span(),
    );
    let set_buffer = Ident::new(
        format!("{}_{}", name_str, "set_buffer").as_str(),
        name.span(),
    );
    let get_style_buffer = Ident::new(
        format!("{}_{}", name_str, "get_style_buffer").as_str(),
        name.span(),
    );
    let text_font = Ident::new(
        format!("{}_{}", name_str, "text_font").as_str(),
        name.span(),
    );
    let set_text_font = Ident::new(
        format!("{}_{}", name_str, "set_text_font").as_str(),
        name.span(),
    );
    let text_font = Ident::new(
        format!("{}_{}", name_str, "text_font").as_str(),
        name.span(),
    );
    let set_text_color = Ident::new(
        format!("{}_{}", name_str, "set_text_color").as_str(),
        name.span(),
    );
    let text_color = Ident::new(
        format!("{}_{}", name_str, "text_color").as_str(),
        name.span(),
    );
    let set_text_size = Ident::new(
        format!("{}_{}", name_str, "set_text_size").as_str(),
        name.span(),
    );
    let text_size = Ident::new(
        format!("{}_{}", name_str, "text_size").as_str(),
        name.span(),
    );
    let scroll = Ident::new(format!("{}_{}", name_str, "scroll").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let set_insert_position = Ident::new(
        format!("{}_{}", name_str, "set_insert_position").as_str(),
        name.span(),
    );
    let insert_position = Ident::new(
        format!("{}_{}", name_str, "insert_position").as_str(),
        name.span(),
    );
    let position_to_xy = Ident::new(
        format!("{}_{}", name_str, "position_to_xy").as_str(),
        name.span(),
    );
    let count_lines = Ident::new(
        format!("{}_{}", name_str, "count_lines").as_str(),
        name.span(),
    );
    let move_right = Ident::new(
        format!("{}_{}", name_str, "move_right").as_str(),
        name.span(),
    );
    let move_left = Ident::new(
        format!("{}_{}", name_str, "move_left").as_str(),
        name.span(),
    );
    let move_up = Ident::new(format!("{}_{}", name_str, "move_up").as_str(), name.span());
    let move_down = Ident::new(
        format!("{}_{}", name_str, "move_down").as_str(),
        name.span(),
    );
    let show_cursor = Ident::new(
        format!("{}_{}", name_str, "show_cursor").as_str(),
        name.span(),
    );
    let set_highlight_data = Ident::new(
        format!("{}_{}", name_str, "set_highlight_data").as_str(),
        name.span(),
    );
    let set_cursor_style = Ident::new(
        format!("{}_{}", name_str, "set_cursor_style").as_str(),
        name.span(),
    );
    let set_cursor_color = Ident::new(
        format!("{}_{}", name_str, "set_cursor_color").as_str(),
        name.span(),
    );
    let set_scrollbar_size = Ident::new(
        format!("{}_{}", name_str, "set_scrollbar_size").as_str(),
        name.span(),
    );
    let set_scrollbar_align = Ident::new(
        format!("{}_{}", name_str, "set_scrollbar_align").as_str(),
        name.span(),
    );
    let cursor_style = Ident::new(
        format!("{}_{}", name_str, "cursor_style").as_str(),
        name.span(),
    );
    let cursor_color = Ident::new(
        format!("{}_{}", name_str, "cursor_color").as_str(),
        name.span(),
    );
    let scrollbar_size = Ident::new(
        format!("{}_{}", name_str, "scrollbar_size").as_str(),
        name.span(),
    );
    let scrollbar_align = Ident::new(
        format!("{}_{}", name_str, "scrollbar_align").as_str(),
        name.span(),
    );
    let line_start = Ident::new(
        format!("{}_{}", name_str, "line_start").as_str(),
        name.span(),
    );
    let line_end = Ident::new(format!("{}_{}", name_str, "line_end").as_str(), name.span());
    let skip_lines = Ident::new(
        format!("{}_{}", name_str, "skip_lines").as_str(),
        name.span(),
    );
    let rewind_lines = Ident::new(
        format!("{}_{}", name_str, "rewind_lines").as_str(),
        name.span(),
    );
    let next_word = Ident::new(
        format!("{}_{}", name_str, "next_word").as_str(),
        name.span(),
    );
    let previous_word = Ident::new(
        format!("{}_{}", name_str, "previous_word").as_str(),
        name.span(),
    );
    let word_start = Ident::new(
        format!("{}_{}", name_str, "word_start").as_str(),
        name.span(),
    );
    let word_end = Ident::new(format!("{}_{}", name_str, "word_end").as_str(), name.span());
    let x_to_col = Ident::new(format!("{}_{}", name_str, "x_to_col").as_str(), name.span());
    let col_to_x = Ident::new(format!("{}_{}", name_str, "col_to_x").as_str(), name.span());
    let set_linenumber_width = Ident::new(
        format!("{}_{}", name_str, "set_linenumber_width").as_str(),
        name.span(),
    );
    let linenumber_width = Ident::new(
        format!("{}_{}", name_str, "linenumber_width").as_str(),
        name.span(),
    );
    let set_linenumber_font = Ident::new(
        format!("{}_{}", name_str, "set_linenumber_font").as_str(),
        name.span(),
    );
    let linenumber_font = Ident::new(
        format!("{}_{}", name_str, "linenumber_font").as_str(),
        name.span(),
    );
    let set_linenumber_size = Ident::new(
        format!("{}_{}", name_str, "set_linenumber_size").as_str(),
        name.span(),
    );
    let linenumber_size = Ident::new(
        format!("{}_{}", name_str, "linenumber_size").as_str(),
        name.span(),
    );
    let set_linenumber_fgcolor = Ident::new(
        format!("{}_{}", name_str, "set_linenumber_fgcolor").as_str(),
        name.span(),
    );
    let linenumber_fgcolor = Ident::new(
        format!("{}_{}", name_str, "linenumber_fgcolor").as_str(),
        name.span(),
    );
    let set_linenumber_bgcolor = Ident::new(
        format!("{}_{}", name_str, "set_linenumber_bgcolor").as_str(),
        name.span(),
    );
    let linenumber_bgcolor = Ident::new(
        format!("{}_{}", name_str, "linenumber_bgcolor").as_str(),
        name.span(),
    );
    let set_linenumber_align = Ident::new(
        format!("{}_{}", name_str, "set_linenumber_align").as_str(),
        name.span(),
    );
    let linenumber_align = Ident::new(
        format!("{}_{}", name_str, "linenumber_align").as_str(),
        name.span(),
    );
    let in_selection = Ident::new(
        format!("{}_{}", name_str, "in_selection").as_str(),
        name.span(),
    );
    let wrap_mode = Ident::new(
        format!("{}_{}", name_str, "wrap_mode").as_str(),
        name.span(),
    );
    let wrapped_column = Ident::new(
        format!("{}_{}", name_str, "wrapped_column").as_str(),
        name.span(),
    );
    let wrapped_row = Ident::new(
        format!("{}_{}", name_str, "wrapped_row").as_str(),
        name.span(),
    );

    let gen = quote! {
        unsafe impl DisplayExt for #name {
            fn buffer(&self) -> Option<TextBuffer> {
                unsafe {
                    assert!(!self.was_deleted());
                    let buffer = #get_buffer(self.inner);
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
                        #set_buffer(self.inner, buffer.as_ptr())
                    } else {
                        #set_buffer(self.inner, std::ptr::null_mut() as *mut Fl_Text_Buffer)
                    }
                }
            }

            fn style_buffer(&self) -> Option<TextBuffer> {
                unsafe {
                    assert!(!self.was_deleted());
                    let buffer = #get_style_buffer(self.inner);
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
                unsafe { mem::transmute(#text_font(self.inner)) }
            }

            fn set_text_font(&mut self, font: Font) {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { #set_text_font(self.inner, font.bits() as i32) }
            }

            fn text_color(&self) -> Color{
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { mem::transmute(#text_color(self.inner)) }
            }

            fn set_text_color(&mut self, color: Color){
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { #set_text_color(self.inner, color.bits() as u32) }
            }

            fn text_size(&self) -> i32{
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { #text_size(self.inner) as i32 }
            }

            fn set_text_size(&mut self, sz: i32) {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe { #set_text_size(self.inner, sz as i32) }
            }

            fn scroll(&mut self, topLineNum: i32, horizOffset: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #scroll(self.inner, topLineNum as i32, horizOffset as i32)
                }
            }

            fn insert(&self, text: &str) {
                let text = CString::safe_new(text);
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #insert(self.inner, text.as_ptr())
                }
            }

            fn set_insert_position(&mut self, newPos: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #set_insert_position(self.inner, newPos as i32)
                }
            }

            fn insert_position(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #insert_position(self.inner) as i32
                }
            }

            fn position_to_xy(&self, pos: i32) -> (i32, i32) {
                unsafe {
                    let mut x: i32 = 0;
                    let mut y: i32 = 0;
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #position_to_xy(self.inner, pos as i32, &mut x, &mut y);
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
                    #count_lines(self.inner, start as i32, end as i32, x) as i32
                }
            }

            fn move_right(&mut self) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    let x = #move_right(self.inner);
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
                    let x = #move_left(self.inner);
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
                    let x = #move_up(self.inner);
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
                    let x = #move_down(self.inner);
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
                    #show_cursor(self.inner, val as i32);
                }
            }

            fn set_highlight_data<B: Into<Option<TextBuffer>>>(&mut self, mut style_buffer: B, entries: Vec<StyleTableEntry>) {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                debug_assert!(entries.len() < 29);
                if entries.len() == 0 { return; }
                if let Some(style_buffer) = style_buffer.into() {
                    let _old_buf = self.style_buffer();
                    let mut colors: Vec<u32> = vec![];
                    let mut fonts: Vec<i32> = vec![];
                    let mut sizes: Vec<i32> = vec![];
                    for entry in entries.iter() {
                        colors.push(entry.color.bits() as u32);
                        fonts.push(entry.font.bits() as i32);
                        sizes.push(entry.size as i32);
                    }
                    unsafe {
                        #set_highlight_data(self.inner, style_buffer.as_ptr() as *mut raw::c_void, &mut colors[0], &mut fonts[0], &mut sizes[0], entries.len() as i32)
                    }
                }
            }

            fn set_cursor_style(&mut self, style: crate::text::Cursor) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_cursor_style(self.inner, style as i32)
                }
            }

            fn set_cursor_color(&mut self, color: Color){
                unsafe {
                    assert!(!self.was_deleted());
                    #set_cursor_color(self.inner, color.bits() as u32)
                }
            }

            fn set_scrollbar_size(&mut self, size: i32){
                unsafe {
                    assert!(!self.was_deleted());
                    #set_scrollbar_size(self.inner, size as i32)
                }
            }

            fn set_scrollbar_align(&mut self, align: Align){
                unsafe {
                    assert!(!self.was_deleted());
                    #set_scrollbar_align(self.inner, align.bits() as i32)
                }
            }

            fn cursor_style(&self) -> crate::text::Cursor {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#cursor_style(self.inner))
                }
            }

            fn cursor_color(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#cursor_color(self.inner))
                }
            }

            fn scrollbar_size(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #scrollbar_size(self.inner) as i32
                }
            }

            fn scrollbar_align(&self) -> Align {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#scrollbar_align(self.inner))
                }
            }

            fn line_start(&self, pos: i32) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #line_start(self.inner, pos as i32) as i32
                }
            }

            fn line_end(&self, start_pos: i32, is_line_start: bool) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #line_end(self.inner, start_pos as i32, is_line_start as i32) as i32
                }
            }

            fn skip_lines(&mut self, start_pos: i32, lines: i32, is_line_start: bool) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #skip_lines(self.inner, start_pos as i32, lines as i32, is_line_start as i32) as i32
                }
            }

            fn rewind_lines(&mut self, start_pos: i32, lines: i32) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #rewind_lines(self.inner, start_pos as i32, lines as i32) as i32
                }
            }

            fn next_word(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #next_word(self.inner)
                }
            }

            fn previous_word(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #previous_word(self.inner)
                }
            }

            fn word_start(&self, pos: i32) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #word_start(self.inner, pos as i32) as i32
                }
            }

            fn word_end(&self, pos: i32) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #word_end(self.inner, pos as i32) as i32
                }
            }

            fn x_to_col(&self, x: f64) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #x_to_col(self.inner, x)
                }
            }

            fn col_to_x(&self, col: f64) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #col_to_x(self.inner, col)
                }
            }

            fn set_linenumber_width(&mut self, w: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_linenumber_width(self.inner, w)
                }
            }

            fn linenumber_width(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #linenumber_width(self.inner)
                }
            }

            fn set_linenumber_font(&mut self, font: Font) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_linenumber_font(self.inner, font.bits() as i32)
                }
            }

            fn linenumber_font(&self) -> Font {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#linenumber_font(self.inner))
                }
            }

            fn set_linenumber_size(&mut self, size: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_linenumber_size(self.inner, size as i32)
                }
            }

            fn linenumber_size(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #linenumber_size(self.inner) as i32
                }
            }

            fn set_linenumber_fgcolor(&mut self, color: Color) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_linenumber_fgcolor(self.inner, color.bits() as u32)
                }
            }

            fn linenumber_fgcolor(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#linenumber_fgcolor(self.inner))
                }
            }

            fn set_linenumber_bgcolor(&mut self, color: Color) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_linenumber_bgcolor(self.inner, color.bits() as u32)
                }
            }

            fn linenumber_bgcolor(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#linenumber_bgcolor(self.inner))
                }
            }

            fn set_linenumber_align(&mut self, align: Align) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_linenumber_align(self.inner, align.bits() as i32)
                }
            }

            fn linenumber_align(&self) -> Align {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#linenumber_align(self.inner))
                }
            }

            fn in_selection(&self, x: i32, y: i32) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(self.buffer().is_some());
                    #in_selection(self.inner, x, y)  != 0
                }
            }

            fn wrap_mode(&mut self, wrap: crate::text::WrapMode, wrap_margin: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    #wrap_mode(self.inner, wrap as i32, wrap_margin)
                }
            }

            fn wrapped_column(&self, row: i32, column: i32) -> i32 {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe {
                    #wrapped_column(self.inner, row, column)
                }
            }

            fn wrapped_row(&self, row: i32) -> i32 {
                assert!(!self.was_deleted());
                assert!(self.buffer().is_some());
                unsafe {
                    #wrapped_row(self.inner, row)
                }
            }
        }
    };
    gen.into()
}
