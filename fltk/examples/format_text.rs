// Contributed by @mvasi90 https://github.com/mvasi90

use fltk::{
    app::{App, Scheme},
    button::Button,
    enums::{Color, Event, Font},
    group::*,
    menu::Choice,
    misc::Spinner,
    prelude::*,
    text::{StyleTableEntryExt, TextAttr, TextBuffer, TextEditor},
    window::Window,
};
use std::{cell::RefCell, char, rc::Rc};

struct Style {
    style_table: Vec<StyleTableEntryExt>,
}

impl Style {
    fn new() -> Style {
        Style {
            style_table: Vec::<StyleTableEntryExt>::new(),
        }
    }

    // Apply the given style to given text editor.
    // If the style not exists, then it is created
    // Any unused style is removed from the style table to keep it compact
    #[allow(clippy::too_many_arguments)]
    fn apply_style(
        &mut self,
        pos: Option<i32>,
        ins_items: Option<i32>,
        del_items: Option<i32>,
        repl_start: Option<i32>,
        repl_end: Option<i32>,
        font: Font,
        size: i32,
        color: Color,
        attr: TextAttr,
        text_editor: &mut TextEditor,
    ) {
        let mut style_buffer = text_editor.style_buffer().unwrap_or_default();

        // get existent style or create new one
        let style_char =
            match self.style_table.iter().position(|s| {
                s.font == font && s.size == size && s.color == color && s.attr == attr
            }) {
                Some(i) => ((i + 65) as u8 as char).to_string(),
                None => {
                    self.style_table.push(StyleTableEntryExt {
                        color,
                        font,
                        size,
                        attr,
                        bgcolor: Color::Black,
                    });
                    ((self.style_table.len() + 64) as u8 as char).to_string()
                }
            };

        // insert, delete or replace char index style to the style_buffer
        match ins_items {
            Some(n) if n > 0 => {
                // insert items with style
                style_buffer.insert(pos.unwrap(), style_char.repeat(n as usize).as_str());
            }
            _ => match del_items {
                Some(n) if n > 0 => {
                    // delete items with style
                    style_buffer.remove(pos.unwrap(), pos.unwrap() + n);
                }
                _ => match repl_end {
                    Some(n) if n > 0 => {
                        // replace items style
                        style_buffer.replace(
                            repl_start.unwrap(),
                            repl_end.unwrap(),
                            style_char
                                .repeat((repl_end.unwrap() - repl_start.unwrap()) as usize)
                                .as_str(),
                        );
                    }
                    _ => {}
                },
            },
        }

        // compact styles on the buffer and reorganize the char index on the style_buffer
        let mut style_index = style_buffer
            .text()
            .chars()
            .map(|c| (c as usize) - 65)
            .collect::<Vec<usize>>();
        style_index.sort_unstable();
        style_index.dedup();
        for (i, &v) in style_index.iter().enumerate() {
            self.style_table.swap(i, v);
            style_buffer.set_text(
                style_buffer
                    .text()
                    .replace(
                        (v + 65) as u8 as char,
                        ((i + 65) as u8 as char).to_string().as_str(),
                    )
                    .as_str(),
            );
        }

        // remove unused indexes
        //self.style_table = self.style_table.drain(in_buff.len()..).collect();
        self.style_table.truncate(style_index.len());
        text_editor.set_highlight_data_ext(style_buffer, self.style_table.to_owned());

        // uncomment this line to see that the style_table is compact
        // println!("total styles: {}", self.style_table.len());
    }
}

fn main() {
    let style = Rc::from(RefCell::from(Style::new()));

    let app = App::default().with_scheme(Scheme::Gleam);
    let mut wind = Window::default()
        .with_size(500, 200)
        .with_label("Highlight");
    let mut vpack = Pack::new(4, 4, 492, 192, "");
    vpack.set_spacing(4);
    let mut text_editor = TextEditor::default().with_size(492, 163);

    let mut hpack = Pack::new(4, 4, 492, 25, "").with_type(PackType::Horizontal);
    hpack.set_spacing(8);
    let mut font = Choice::default().with_size(130, 25);
    let mut choice = Choice::default().with_size(130, 25);
    let mut size = Spinner::default().with_size(60, 25);

    let mut color = Choice::default().with_size(100, 25);
    let mut btn_clear = Button::default().with_size(40, 25).with_label("X");
    hpack.end();

    vpack.end();
    wind.end();
    wind.show();

    text_editor.wrap_mode(fltk::text::WrapMode::AtBounds, 0);
    text_editor.set_buffer(TextBuffer::default());

    font.add_choice("Courier|Helvetica|Times");
    font.set_value(0);
    font.set_tooltip("Font");

    choice.add_choice("Normal|Underline|Strike");
    choice.set_value(0);

    size.set_value(18.0);
    size.set_step(1.0);
    size.set_range(12.0, 28.0);
    size.set_tooltip("Size");

    color.set_tooltip("Color");
    color.add_choice("#000000|#ff0000|#00ff00|#0000ff|#ffff00|#00ffff");
    color.set_value(0);

    btn_clear.set_label_color(Color::Red);
    btn_clear.set_tooltip("Clear style");

    // set colors
    for mut item in color.clone() {
        if let Some(lbl) = item.label() {
            item.set_label_color(Color::from_u32(
                u32::from_str_radix(lbl.trim().strip_prefix('#').unwrap(), 16)
                    .ok()
                    .unwrap(),
            ));
        }
    }

    let style_rc1 = Rc::clone(&style);

    text_editor.buffer().unwrap().add_modify_callback({
        let mut text_editor1 = text_editor.clone();
        let font1 = font.clone();
        let size1 = size.clone();
        let color1 = color.clone();
        let choice1 = choice.clone();
        move |pos: i32, ins_items: i32, del_items: i32, _: i32, _: &str| {
            let attr = if choice1.value() == 1 {
                TextAttr::Underline
            } else if choice1.value() == 2 {
                TextAttr::StrikeThrough
            } else {
                TextAttr::None
            };
            if ins_items > 0 || del_items > 0 {
                let mut style = style_rc1.borrow_mut();
                let color = Color::from_u32(
                    u32::from_str_radix(
                        color1
                            .text(color1.value())
                            .unwrap()
                            .trim()
                            .strip_prefix('#')
                            .unwrap(),
                        16,
                    )
                    .ok()
                    .unwrap(),
                );
                style.apply_style(
                    Some(pos),
                    Some(ins_items),
                    Some(del_items),
                    None,
                    None,
                    Font::by_name(font1.text(font1.value()).unwrap().trim()),
                    size1.value() as i32,
                    color,
                    attr,
                    &mut text_editor1,
                );
            }
        }
    });

    color.set_callback({
        let size = size.clone();
        let font = font.clone();
        let choice = choice.clone();
        let mut text_editor = text_editor.clone();
        let style_rc1 = Rc::clone(&style);
        move |color| {
            let attr = match choice.value() {
                0 => TextAttr::None,
                1 => TextAttr::Underline,
                2 => TextAttr::StrikeThrough,
                _ => unreachable!(),
            };
            if let Some(buf) = text_editor.buffer() {
                if let Some((s, e)) = buf.selection_position() {
                    let mut style = style_rc1.borrow_mut();
                    let color = Color::from_u32(
                        u32::from_str_radix(
                            color
                                .text(color.value())
                                .unwrap()
                                .trim()
                                .strip_prefix('#')
                                .unwrap(),
                            16,
                        )
                        .ok()
                        .unwrap(),
                    );
                    style.apply_style(
                        None,
                        None,
                        None,
                        Some(s),
                        Some(e),
                        Font::by_name(font.text(font.value()).unwrap().trim()),
                        size.value() as i32,
                        color,
                        attr,
                        &mut text_editor,
                    );
                }
            }
        }
    });

    // get the style from the current cursor position
    text_editor.handle({
        let style_rc1 = Rc::clone(&style);
        let mut font1 = font.clone();
        let mut size1 = size.clone();
        let mut color1 = color.clone();
        move |te, e| match e {
            Event::KeyUp | Event::Released => {
                if let Some(buff) = te.style_buffer() {
                    let i = te.insert_position();
                    if let Some(t) = buff.text_range(i, i + 1) {
                        if !t.is_empty() {
                            let style = style_rc1.borrow_mut();
                            if let Some(i) = t.chars().next().map(|c| (c as usize - 65)) {
                                if let Some(style) = style.style_table.get(i) {
                                    if let Some(mn) = font1.find_item(&format!("{:?}", style.font))
                                    {
                                        font1.set_item(&mn);
                                    }
                                    size1.set_value(style.size as f64);
                                    let (r, g, b) = style.color.to_rgb();
                                    if let Some(mn) =
                                        color1.find_item(format!("{r:02x}{g:02x}{b:02x}").as_str())
                                    {
                                        color1.set_item(&mn);
                                    }
                                }
                            }
                        }
                    }
                }
                true
            }
            _ => false,
        }
    });

    choice.set_callback({
        let mut color1 = color.clone();
        move |_| color1.do_callback()
    });

    font.set_callback({
        let mut color1 = color.clone();
        move |_| color1.do_callback()
    });

    size.set_callback({
        let mut color1 = color.clone();
        move |_| color1.do_callback()
    });

    // clear style of the current selection or, if no text is selected, clear all text style
    btn_clear.set_callback({
        let style_rc1 = Rc::clone(&style);
        let text_editor1 = text_editor.clone();
        move |_| {
            match text_editor1.buffer().unwrap().selection_position() {
                Some((_, _)) => {
                    font.set_value(0);
                    size.set_value(18.0);
                    color.set_value(0);
                    choice.set_value(0);
                    color.do_callback();
                }
                None => {
                    font.set_value(0);
                    size.set_value(18.0);
                    color.set_value(0);
                    style_rc1.borrow_mut().apply_style(
                        None,
                        None,
                        None,
                        Some(0),
                        Some(text_editor1.buffer().unwrap().length()),
                        Font::Courier,
                        16,
                        Color::Black,
                        TextAttr::None,
                        &mut text_editor,
                    );
                }
            };
        }
    });

    app.run().unwrap();
}
