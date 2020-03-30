pub fn set_styly_table_entry(&mut self, entries: &mut Vec<StyleTableEntry>) {
    let mut colors: Vec<i32> = vec![];
    let mut fonts: Vec<i32> = vec![];
    let mut sizes: Vec<i32> = vec![];
    for entry in entries.iter() {
        colors.push(entry.color as i32);
        fonts.push(entry.font as i32);
        sizes.push(entry.size as i32);
    }
    unsafe {
        Fl_Text_Display_set_style_table_entry(self._inner, &colors[0], &fonts[0], &sizes[0], entries.len() as i32);
    }
}

void Fl_Text_Display_set_style_table_entry(Fl_Text_Display *self, const int* color, const int* font, const int* fontsz, int sz) {
    Fl_Text_Display::Style_Table_Entry *stable = new Fl_Text_Display::Style_Table_Entry[sz];
    for (int i = 0; i < sz; ++i) {
      stable[i] = {color[i], font[i], fontsz[i]};
    }
    Fl_Text_Buffer *sbuff = new Fl_Text_Buffer();
    self->highlight_data(sbuff, stable, sz, 'A', 0, 0);
  }