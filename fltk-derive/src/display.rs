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
    let set_style_table_entry = Ident::new(
        format!("{}_{}", name_str, "set_style_table_entry").as_str(),
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
    let set_scrollbar_width = Ident::new(
        format!("{}_{}", name_str, "set_scrollbar_width").as_str(),
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
    let scrollbar_width = Ident::new(
        format!("{}_{}", name_str, "scrollbar_width").as_str(),
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

    let gen = quote! {
        unsafe impl DisplayExt for #name {
            fn buffer(&self) -> TextBuffer {
                unsafe {
                    assert!(!self.was_deleted());
                    let buffer = #get_buffer(self._inner);
                    assert!(!buffer.is_null());
                    TextBuffer::from_ptr(buffer)
                }
            }

            fn set_buffer(&mut self, mut buffer: &mut TextBuffer) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_buffer(self._inner, buffer.as_ptr())
                }
            }

            fn text_font(&self) -> Font {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#text_font(self._inner)) }
            }

            fn set_text_font(&mut self, font: Font) {
                assert!(!self.was_deleted());
                unsafe { #set_text_font(self._inner, font as i32) }
            }

            fn text_color(&self) -> Color{
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#text_color(self._inner)) }
            }

            fn set_text_color(&mut self, color: Color){
                assert!(!self.was_deleted());
                unsafe { #set_text_color(self._inner, color as u32) }
            }

            fn text_size(&self) -> u32{
                assert!(!self.was_deleted());
                unsafe { #text_size(self._inner) as u32 }
            }

            fn set_text_size(&mut self, sz: u32) {
                debug_assert!(sz <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                assert!(!self.was_deleted());
                unsafe { #set_text_size(self._inner, sz as i32) }
            }

            fn scroll(&mut self, topLineNum: u32, horizOffset: u32) {
                unsafe {
                    debug_assert!(topLineNum <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    debug_assert!(horizOffset <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    assert!(!self.was_deleted());
                    #scroll(self._inner, topLineNum as i32, horizOffset as i32)
                }
            }

            fn insert(&self, text: &str) {
                let text = CString::new(text).unwrap();
                unsafe {
                    assert!(!self.was_deleted());
                    #insert(self._inner, text.as_ptr())
                }
            }

            fn set_insert_position(&mut self, newPos: u32) {
                unsafe {
                    debug_assert!(newPos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    assert!(!self.was_deleted());
                    #set_insert_position(self._inner, newPos as i32)
                }
            }

            fn insert_position(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #insert_position(self._inner) as u32
                }
            }

            fn position_to_xy(&self, pos: u32) -> (u32, u32) {
                debug_assert!(pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    let mut x: i32 = 0;
                    let mut y: i32 = 0;
                    assert!(!self.was_deleted());
                    #position_to_xy(self._inner, pos as i32, &mut x, &mut y);
                    (x as u32, y as u32)
                }
            }

            fn count_lines(&self, start: u32, end: u32, is_line_start: bool) -> u32 {
                let x = match is_line_start {
                    true => 1,
                    false => 0,
                };
                debug_assert!(start <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                debug_assert!(end <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #count_lines(self._inner, start as i32, end as i32, x) as u32
                }
            }

            fn move_right(&mut self) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #move_right(self._inner);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn move_left(&mut self) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #move_left(self._inner);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn move_up(&mut self) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #move_up(self._inner);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn move_down(&mut self) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #move_down(self._inner);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn show_cursor(&mut self, val: bool) {
                unsafe {
                    assert!(!self.was_deleted());
                    #show_cursor(self._inner, val as i32);
                }
            }

            fn set_style_table_entry(&mut self, mut style_buffer: &mut TextBuffer, entries: Vec<StyleTableEntry>) -> crate::text::StyleTables {
                let mut colors: Vec<u32> = vec![];
                let mut fonts: Vec<i32> = vec![];
                let mut sizes: Vec<i32> = vec![];
                for entry in entries.iter() {
                    colors.push(entry.color as u32);
                    fonts.push(entry.font as i32);
                    sizes.push(entry.size as i32);
                }
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #set_style_table_entry(self._inner, style_buffer.as_ptr() as *mut raw::c_void, &mut colors[0], &mut fonts[0], &mut sizes[0], entries.len() as i32);
                    StyleTables { _inner: x }
                }
            }

            fn set_cursor_style(&mut self, style: CursorStyle) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_cursor_style(self._inner, style as i32)
                }
            }

            fn set_cursor_color(&mut self, color: Color){
                unsafe {
                    assert!(!self.was_deleted());
                    #set_cursor_color(self._inner, color as u32)
                }
            }

            fn set_scrollbar_width(&mut self, width: i32){
                unsafe {
                    assert!(!self.was_deleted());
                    #set_scrollbar_width(self._inner, width)
                }
            }

            fn set_scrollbar_size(&mut self, size: u32){
                debug_assert!(size <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #set_scrollbar_size(self._inner, size as i32)
                }
            }

            fn set_scrollbar_align(&mut self, align: Align){
                unsafe {
                    assert!(!self.was_deleted());
                    #set_scrollbar_align(self._inner, align as i32)
                }
            }

            fn cursor_style(&self) -> CursorStyle {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#cursor_style(self._inner))
                }
            }

            fn cursor_color(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#cursor_color(self._inner))
                }
            }

            fn scrollbar_width(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #scrollbar_width(self._inner) as u32
                }
            }

            fn scrollbar_size(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #scrollbar_size(self._inner) as u32
                }
            }

            fn scrollbar_align(&self) -> Align {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#scrollbar_align(self._inner))
                }
            }

            fn line_start(&self, pos: u32) -> u32 {
                debug_assert!(pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #line_start(self._inner, pos as i32) as u32
                }
            }

            fn line_end(&self, start_pos: u32, is_line_start: bool) -> u32 {
                debug_assert!(start_pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #line_end(self._inner, start_pos as i32, is_line_start as i32) as u32
                }
            }

            fn skip_lines(&mut self, start_pos: u32, lines: u32, is_line_start: bool) -> u32 {
                debug_assert!(start_pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                debug_assert!(lines <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #skip_lines(self._inner, start_pos as i32, lines as i32, is_line_start as i32) as u32
                }
            }

            fn rewind_lines(&mut self, start_pos: u32, lines: u32) -> u32 {
                debug_assert!(start_pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                debug_assert!(lines <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #rewind_lines(self._inner, start_pos as i32, lines as i32) as u32
                }
            }

            fn next_word(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    #next_word(self._inner)
                }
            }

            fn previous_word(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    #previous_word(self._inner)
                }
            }

            fn word_start(&self, pos: u32) -> u32 {
                debug_assert!(pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #word_start(self._inner, pos as i32) as u32
                }
            }

            fn word_end(&self, pos: u32) -> u32 {
                debug_assert!(pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #word_end(self._inner, pos as i32) as u32
                }
            }

            fn x_to_col(&self, x: f64) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    #x_to_col(self._inner, x)
                }
            }

            fn col_to_x(&self, col: f64) -> f64 {
                unsafe {
                    assert!(!self.was_deleted());
                    #col_to_x(self._inner, col)
                }
            }

            fn set_linenumber_width(&mut self, w: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_linenumber_width(self._inner, w)
                }
            }

            fn linenumber_width(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #linenumber_width(self._inner)
                }
            }

            fn set_linenumber_font(&mut self, font: Font) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_linenumber_font(self._inner, font as i32)
                }
            }

            fn linenumber_font(&self) -> Font {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#linenumber_font(self._inner))
                }
            }

            fn set_linenumber_size(&mut self, size: u32) {
                debug_assert!(size <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #set_linenumber_size(self._inner, size as i32)
                }
            }

            fn linenumber_size(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #linenumber_size(self._inner) as u32
                }
            }

            fn set_linenumber_fgcolor(&mut self, color: Color) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_linenumber_fgcolor(self._inner, color as u32)
                }
            }

            fn linenumber_fgcolor(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#linenumber_fgcolor(self._inner))
                }
            }

            fn set_linenumber_bgcolor(&mut self, color: Color) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_linenumber_bgcolor(self._inner, color as u32)
                }
            }

            fn linenumber_bgcolor(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#linenumber_bgcolor(self._inner))
                }
            }

            fn set_linenumber_align(&mut self, align: Align) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_linenumber_align(self._inner, align as i32)
                }
            }

            fn linenumber_align(&self) -> Align {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#linenumber_align(self._inner))
                }
            }

            fn in_selection(&self, x: i32, y: i32) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    match #in_selection(self._inner, x, y) {
                        0 => false,
                        _ => true,
                    }
                }
            }
        }
    };
    gen.into()
}
