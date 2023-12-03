/// Test program for fltk-rs wrapper for Terminal widget
/// Intended to test the wrapper interface but not the underlying fltk functionality
/// Jonathan Griffitts, 11-2023
///
use fltk::{
    enums::{Color, Font, LabelType},
    group::experimental::{Attrib, RedrawStyle, Terminal},
    menu::MenuBar,
    // *,
    prelude::*,
    window::{Window, WindowType},
};

const WIN_WIDTH: i32 = 900;
const WIN_HEIGHT: i32 = 600;

fn main() {
    let app = fltk::app::App::default();

    let mut main_win = Window::new(
        2285,
        180,
        WIN_WIDTH,
        WIN_HEIGHT,
        "FLTK/Terminal Rust wrapper test",
    );
    main_win.set_type(WindowType::Double);
    main_win.make_resizable(true);

    let mut menu_bar = MenuBar::new(0, 0, WIN_WIDTH, 30, None);
    let idx = menu_bar.add_choice("Test&1");

    let mut term = Terminal::new(0, 30, WIN_WIDTH, WIN_HEIGHT - 30, None);
    term.set_label("term");
    main_win.resizable(&term);
    term.set_label_type(LabelType::None);

    menu_bar.at(idx).unwrap().set_callback({
        let mut term1 = term.clone();
        move |c| mb_test1_cb(c, &mut term1)
    });
    menu_bar
        .at(idx)
        .unwrap()
        .set_shortcut(unsafe { std::mem::transmute(0x80031) }); // Alt-1
    let idx = menu_bar.add_choice("Test&2");
    menu_bar.at(idx).unwrap().set_callback({
        let mut term1 = term.clone();
        move |c| mb_test2_cb(c, &mut term1)
    });
    menu_bar
        .at(idx)
        .unwrap()
        .set_shortcut(unsafe { std::mem::transmute(0x80032) }); // Alt-2
    menu_bar.end();

    main_win.end();
    main_win.show();

    // Worker thread that drives the startup tests
    let _worker_thread: std::thread::JoinHandle<_> = std::thread::spawn({
        let mut term = term.clone();
        move || {
            println!("Startup tests\n");
            term.append("Startup tests\n\n");

            // Testing ansi() and set_ansi() methods
            assert!(term.ansi(), "Default ANSI mode should be ON at startup");
            term.append("ANSI mode is \x1b[4mON\x1b[0m\n");
            term.set_ansi(false);
            assert!(!term.ansi());
            term.append("ANSI mode is \x1b[4mOFF\x1b[0m\n");
            // append() method is already being used/tested. Test the u8, ascii, and utf8 variants
            term.append_u8(b"Appending u8 array\n");
            term.append_ascii("Appending ASCII array ↑ (up-arrow is dropped)\n");

            // Test show_unknown() as incidental part of testing append methods
            term.set_show_unknown(true);
            assert!(term.show_unknown());
            term.append_ascii(
                "Appending ASCII array with show_unknown() ↑ (up-arrow is three unknown bytes)\n",
            );
            term.set_show_unknown(false);
            assert!(!term.show_unknown());

            term.append_utf8("Appending UTF8 array ↑ (up-arrow is visible)\n");
            term.append_utf8_u8(b"Appending UTF8 array as u8 \xe2\x86\x91 (up-arrow is visible)\n");

            // term.append(format!("Box type {:?}\n", term.Box())); // todo: box doesn't work

            let r = term.cursor_row();
            assert_eq!(term.cursor_col(), 0);
            term.append(&format!("Testing cursor row/col {r}"));
            assert_eq!(term.cursor_col(), 24);
            assert_eq!(term.cursor_row(), r);

            // Test cursor color methods
            assert_eq!(
                term.cursor_bg_color(),
                Color::XtermGreen,
                "Default cursor bg at startup"
            );
            assert_eq!(
                term.cursor_fg_color(),
                Color::from_hex(0xfffff0),
                "Default cursor fg at startup"
            );
            term.set_cursor_bg_color(Color::Red);
            assert_eq!(term.cursor_bg_color(), Color::Red);
            assert_eq!(term.cursor_fg_color(), Color::from_hex(0xfffff0));
            term.set_cursor_fg_color(Color::Blue);
            assert_eq!(term.cursor_bg_color(), Color::Red);
            assert_eq!(term.cursor_fg_color(), Color::Blue);
            term.set_cursor_bg_color(Color::XtermGreen); // Restore the defaults
            term.set_cursor_fg_color(Color::from_hex(0xfffff0));
            assert_eq!(term.cursor_bg_color(), Color::XtermGreen);
            assert_eq!(term.cursor_fg_color(), Color::from_hex(0xfffff0));

            // The default display_rows() will derive from the window size
            let dr = term.display_rows();
            assert!(dr > 20, "Default display_rows at startup");
            term.set_display_rows(60);
            assert_eq!(term.display_rows(), 60);
            term.set_display_rows(dr); // Set back to default
            assert_eq!(term.display_rows(), dr);

            // The default display_columns() will derive from the window size
            let dc = term.display_columns();
            assert!(dc > 80, "Default display_rows at startup");
            term.set_display_columns(200);
            assert_eq!(term.display_columns(), 200);
            term.append("\n         1         2         3         4         5         6         7         8         9");
            term.append("\n123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890");
            term.append("[This text should be truncated by display_columns() call below.]\n"); // We shouldn't see this on screen
            term.set_display_columns(90);
            assert_eq!(term.display_columns(), 90);
            term.set_display_columns(dc); // Set back to default
            assert_eq!(term.display_columns(), dc);

            let hl = term.history_lines();
            assert_eq!(hl, 100, "Default history_lines at startup");
            term.set_history_lines(60);
            assert_eq!(term.history_lines(), 60);
            term.set_history_lines(hl); // Set back to default
            assert_eq!(term.history_lines(), hl);

            // Is history_rows() an alias for history_lines()?
            assert_eq!(term.history_rows(), 100, "Default history_rows at startup");
            term.set_history_rows(50);
            assert_eq!(term.history_rows(), 50);
            term.set_history_lines(100); // Set back to default
            assert_eq!(term.history_lines(), 100);

            let hu = term.history_use();
            term.append(&format!(
                "history_use = {hu} (it's not clear what this means)\n"
            ));
            // assert_eq!(term.history_use(), hu+1);

            term.append(&format!(
                "margins = b:{} l:{} r:{} t{}\n",
                term.margin_bottom(),
                term.margin_left(),
                term.margin_right(),
                term.margin_top()
            ));
            assert_eq!(term.margin_bottom(), 3);
            assert_eq!(term.margin_left(), 3);
            assert_eq!(term.margin_right(), 3);
            assert_eq!(term.margin_top(), 3);

            term.set_margin_bottom(5);
            term.set_margin_left(10);
            term.set_margin_right(15);
            term.set_margin_top(20);
            assert_eq!(term.margin_bottom(), 5);
            assert_eq!(term.margin_left(), 10);
            assert_eq!(term.margin_right(), 15);
            assert_eq!(term.margin_top(), 20);

            term.append("Single character: '");
            term.print_char('X');
            term.append("', single UTF-8 character: '");
            term.print_char_utf8('↑');
            term.append("'\n");

            let rr = term.redraw_rate();
            assert_eq!(rr, 0.1, "Default redraw rate at startup");
            term.append(&format!("Redraw rate {rr}\n"));
            term.set_redraw_rate(1.0);
            assert_eq!(term.redraw_rate(), 1.0);
            term.set_redraw_rate(rr);
            assert_eq!(term.redraw_rate(), rr);

            let rs = term.redraw_style();
            term.append(&format!("Redraw style {rs:?}\n"));
            assert_eq!(
                rs,
                RedrawStyle::RateLimited,
                "Default redraw style at startup"
            );
            term.set_redraw_style(RedrawStyle::NoRedraw);
            assert_eq!(term.redraw_style(), RedrawStyle::NoRedraw);
            term.set_redraw_style(rs);
            assert_eq!(term.redraw_style(), rs);

            term.append(&format!(
                "Scrollbar actual size {}\n",
                term.scrollbar_actual_size()
            ));
            assert_eq!(term.scrollbar_actual_size(), 16);
            term.append(&format!("Scrollbar size {}\n", term.scrollbar_size()));
            assert_eq!(
                term.scrollbar_size(),
                0,
                "Default scrollbar size at startup"
            );
            term.set_scrollbar_size(40);
            assert_eq!(term.scrollbar_size(), 40);
            assert_eq!(term.scrollbar_actual_size(), 40);
            term.append(&format!(
                "Scrollbar actual size {}\n",
                term.scrollbar_actual_size()
            ));
            term.set_scrollbar_size(0); // Restore default
            assert_eq!(term.scrollbar_size(), 0);
            assert_eq!(term.scrollbar_actual_size(), 16);

            let sfc = term.selection_fg_color();
            let sbc = term.selection_bg_color();
            assert_eq!(sfc, Color::Black);
            assert_eq!(sbc, Color::White);
            term.append(&format!("Selection colors: {sfc} {sbc}\n"));
            term.set_selection_fg_color(Color::Green);
            term.set_selection_bg_color(Color::DarkBlue);
            assert_eq!(term.selection_fg_color(), Color::Green);
            assert_eq!(term.selection_bg_color(), Color::DarkBlue);
            term.set_selection_fg_color(sfc);
            term.set_selection_bg_color(sbc);
            assert_eq!(term.selection_fg_color(), Color::Black);
            assert_eq!(term.selection_bg_color(), Color::White);

            let tfcd = term.text_fg_color_default();
            let tbcd = term.text_bg_color_default();
            assert_eq!(tfcd, Color::XtermWhite);
            assert_eq!(tbcd, Color::TransparentBg);
            term.append(&format!("Default text colors: {sfc} {sbc}\n"));
            term.set_text_fg_color_default(Color::Green);
            term.set_text_bg_color_default(Color::DarkBlue);
            assert_eq!(term.text_fg_color_default(), Color::Green);
            assert_eq!(term.text_bg_color_default(), Color::DarkBlue);
            term.set_text_fg_color_default(tfcd);
            term.set_text_bg_color_default(tbcd);
            assert_eq!(term.text_fg_color_default(), Color::XtermWhite);
            assert_eq!(term.text_bg_color_default(), Color::TransparentBg);

            let tfc = term.text_fg_color();
            let tbc = term.text_bg_color();
            assert_eq!(tfc, Color::XtermWhite);
            assert_eq!(tbc, Color::TransparentBg);
            term.append(&format!("Text colors: {sfc} {sbc}\n"));
            term.set_text_fg_color(Color::Green);
            term.set_text_bg_color(Color::DarkBlue);
            assert_eq!(term.text_fg_color(), Color::Green);
            assert_eq!(term.text_bg_color(), Color::DarkBlue);
            term.set_text_fg_color(tfc);
            term.set_text_bg_color(tbc);
            assert_eq!(term.text_fg_color(), Color::XtermWhite);
            assert_eq!(term.text_bg_color(), Color::TransparentBg);

            let tf = term.text_font();
            term.append(&format!("Text font: {tf:?}\n"));
            assert_eq!(tf, Font::Courier);
            term.set_text_font(Font::Screen);
            assert_eq!(term.text_font(), Font::Screen);
            term.set_text_font(tf);
            assert_eq!(term.text_font(), Font::Courier);

            let ts = term.text_size();
            term.append(&format!("Text size: {ts}\n"));
            assert_eq!(ts, 14);
            term.set_text_size(30);
            assert_eq!(term.text_size(), 30);
            term.set_text_size(ts);
            assert_eq!(term.text_size(), ts);

            // Keyboard handler
            term.handle({
                let mut term = term.clone();
                move |_kc, e| {
                    match e {
                        fltk::enums::Event::KeyDown
                            if fltk::app::event_key() == fltk::enums::Key::Escape =>
                        {
                            // let FLTK handle ESC
                            false
                        }

                        fltk::enums::Event::KeyDown
                            if fltk::app::event_length() == 1 && !fltk::app::is_event_alt() =>
                        {
                            // We handle normal printable keystroke
                            let k = fltk::app::event_text();
                            term.take_focus().unwrap();
                            term.append(&k);
                            true
                        }

                        // fltk docs say that keyboard handler should always claim Focus and Unfocus events
                        fltk::enums::Event::Focus => true,
                        fltk::enums::Event::Unfocus => true,

                        _ => false, // Let FLTK handle everything else
                    }
                }
            });

            term.set_text_attrib(Attrib::Inverse | Attrib::Italic);
            term.append("\nStartup tests complete. Keyboard is live.\n");
            term.set_text_attrib(Attrib::Normal);
            term.redraw();
        }
    });

    app.run().unwrap();
}

/// More tests that run when the menu bar Test1 is clicked
fn mb_test1_cb(_choice: &mut fltk::menu::Choice, term: &mut Terminal) {
    term.take_focus().unwrap();
    term.reset_terminal();
    term.append("0123456789 0\n");
    term.append("0123456789 1\n");
    term.append("0123456789 2\n");
    term.append("0123456789 3\n");
    term.append("0123456789 4\n");
    term.append("0123456789 5\n");
    term.append("0123456789 6\n");
    term.append("0123456789 7\n");
    term.append("0123456789 8\n");
    term.append("0123456789 9\n");
    term.append("------------\n");

    term.set_text_fg_color(Color::Green);
    term.put_char('A', 0, 0);
    term.put_char('B', 1, 1);
    term.put_char('C', 2, 2);
    term.put_char('D', 3, 3);
    term.put_char('E', 4, 4);
    term.put_char('F', 5, 5);
    term.set_text_fg_color(Color::XtermWhite);

    assert_eq!(term.cursor_row(), 11);
    assert_eq!(term.cursor_col(), 0);

    term.set_text_bg_color(Color::DarkBlue);
    term.put_char_utf8('b', 8, 1);
    term.put_char_utf8('↑', 9, 1);
    term.put_char_utf8('c', 8, 2);
    term.put_char_utf8('↑', 9, 2);
    term.put_char_utf8('d', 8, 3);
    term.put_char_utf8('↑', 9, 3);
    term.put_char_utf8('e', 8, 4);
    term.put_char_utf8('↑', 9, 4);
    term.put_char_utf8('f', 8, 5);
    term.put_char_utf8('↑', 9, 5);
    term.put_char_utf8('g', 8, 6);
    term.put_char_utf8('↑', 9, 6);
    term.set_text_bg_color(Color::Black);

    term.set_text_attrib(Attrib::Inverse | Attrib::Italic);
    term.append("Done!\n");
    term.set_text_attrib(Attrib::Normal);
}

/// More tests that run when the menu bar button Test2 is clicked
fn mb_test2_cb(_choice: &mut fltk::menu::Choice, term: &mut Terminal) {
    term.take_focus().unwrap();
    term.reset_terminal();

    for i in 0..50 {
        term.append(&format!("{i}\n"));
    }
    assert_eq!(term.cursor_row(), 30);
    assert_eq!(term.display_rows(), 31);
    assert_eq!(term.history_rows(), 100);
    assert_eq!(term.history_lines(), 100);
    assert_eq!(term.history_use(), 20);

    term.clear_history();
    assert_eq!(term.history_use(), 0);

    term.set_text_attrib(Attrib::Inverse | Attrib::Italic);
    term.append("\nDone!\n");
    term.set_text_attrib(Attrib::Normal);
}
