#![allow(clippy::missing_transmute_annotations)]

/// Test program for fltk-rs wrapper for Terminal widget
/// Intended to test the wrapper interface but not the underlying fltk functionality
/// Jonathan Griffitts, 11-2023
///
use fltk::{
    enums::{Color, Font, LabelType},
    menu::MenuBar,
    prelude::*,
    terminal::{
        Attrib, CharFlags, OutFlags, RedrawStyle, ScrollbarStyle, Terminal, Utf8Char, XtermColor,
    },
    window::{Window, WindowType},
};

const WIN_WIDTH: i32 = 900;
const WIN_HEIGHT: i32 = 600;

fn main() {
    let app = fltk::app::App::default();

    // Set panic handler for main thread (will become UI thread)
    std::panic::set_hook(Box::new({
        |e| {
            eprintln!("!!!!PANIC!!!!{:#?}", e);
            error_box(e.to_string()); // Only works from the UI thread
            std::process::exit(2);
        }
    }));

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

    let mut term = Terminal::new(0, 30, WIN_WIDTH, WIN_HEIGHT - 30, None);
    term.set_label("term");
    main_win.resizable(&term);
    term.set_label_type(LabelType::None);

    let idx = menu_bar.add_choice("Test&1");
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

    let idx = menu_bar.add_choice("Test&3");
    menu_bar.at(idx).unwrap().set_callback({
        let mut term1 = term.clone();
        move |c| mb_test3_cb(c, &mut term1)
    });
    menu_bar
        .at(idx)
        .unwrap()
        .set_shortcut(unsafe { std::mem::transmute(0x80033) }); // Alt-3

    let idx = menu_bar.add_choice("Test&4");
    menu_bar.at(idx).unwrap().set_callback({
        let mut term1 = term.clone();
        move |c| mb_test4_cb(c, &mut term1)
    });
    menu_bar
        .at(idx)
        .unwrap()
        .set_shortcut(unsafe { std::mem::transmute(0x80034) }); // Alt-4

    let idx = menu_bar.add_choice("Test&5");
    menu_bar.at(idx).unwrap().set_callback({
        let mut term1 = term.clone();
        move |c| mb_test5_cb(c, &mut term1)
    });
    menu_bar
        .at(idx)
        .unwrap()
        .set_shortcut(unsafe { std::mem::transmute(0x80035) }); // Alt-5

    menu_bar.end();

    main_win.end();
    main_win.show();

    // Worker thread that drives the startup tests
    let _worker_thread: std::thread::JoinHandle<_> = std::thread::spawn({
        let mut term = term.clone();
        move || {
            println!("Startup tests\n");
            term.append("Startup tests\n\n");
            term.append("<tmp>\n"); // This line will be overwritten later

            term.cursor_up(2, false);
            assert_eq!(term.text(false), "Startup tests\n\n"); // Ignores lines below cursor
            assert_eq!(term.text(true), "Startup tests\n\n<tmp>\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

            // Testing ansi() and set_ansi() methods
            assert!(term.ansi(), "Default ANSI mode should be ON at startup");
            term.append("ANSI mode is \x1b[4mON\x1b[0m\n");
            term.set_ansi(false);
            assert!(!term.ansi());
            term.append("ANSI mode is \x1b[4mOFF\x1b[0m\n");
            // append() method is already being used/tested. Test the u8, ascii, and utf8 variants
            term.append_u8(b"Appending u8 array\n");
            term.append_ascii("Appending ASCII array ↑ (up-arrow is dropped)\n");
            term.set_ansi(true); // Restore ANSI state

            // Play with the horizontal scrollbar
            assert_eq!(term.hscrollbar_style(), ScrollbarStyle::AUTO);
            term.set_hscrollbar_style(ScrollbarStyle::ON);
            assert_eq!(term.hscrollbar_style(), ScrollbarStyle::ON);

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
                Color::from_hex(0xff_ff_f0),
                "Default cursor fg at startup"
            );
            term.set_cursor_bg_color(Color::Red);
            assert_eq!(term.cursor_bg_color(), Color::Red);
            assert_eq!(term.cursor_fg_color(), Color::from_hex(0xff_ff_f0));
            term.set_cursor_fg_color(Color::Blue);
            assert_eq!(term.cursor_bg_color(), Color::Red);
            assert_eq!(term.cursor_fg_color(), Color::Blue);
            term.set_cursor_bg_color(Color::XtermGreen); // Restore the defaults
            term.set_cursor_fg_color(Color::from_hex(0xff_ff_f0));
            assert_eq!(term.cursor_bg_color(), Color::XtermGreen);
            assert_eq!(term.cursor_fg_color(), Color::from_hex(0xff_ff_f0));

            // The default display_rows() will derive from the window size
            let dr = term.display_rows();
            let height = term.height();
            assert_eq!(height, term.h());
            assert!(dr > 20, "Default display_rows at startup");
            term.resize(term.x(), term.y(), term.w(), height * 2);
            assert_eq!(term.h(), height * 2);
            assert_eq!(height * 2, term.h());
            assert!(term.display_rows() > dr);
            term.resize(term.x(), term.y(), term.w(), height);

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

            assert_eq!(term.history_rows(), 100, "Default history_rows at startup");
            term.set_history_rows(50);
            assert_eq!(term.history_rows(), 50);
            term.set_history_rows(100); // Set back to default
            assert_eq!(term.history_rows(), 100);

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

            // Sanity checks: enum values are implicitly assigned in the C++ code so could change unexpectedly
            assert_eq!(
                RedrawStyle::NoRedraw.bits(),
                0x0000,
                "RedrawStyle enum values have been reassigned"
            );
            assert_eq!(
                RedrawStyle::RateLimited.bits(),
                0x0001,
                "RedrawStyle enum values have been reassigned"
            );
            assert_eq!(
                RedrawStyle::PerWrite.bits(),
                0x0002,
                "RedrawStyle enum values have been reassigned"
            );

            let sb = term.scrollbar();
            let hsb = term.hscrollbar();
            // Both vertical and horizontal scrollbars are at zero
            assert_eq!(sb.value(), 0.0);
            assert_eq!(hsb.value(), 0.0);
            term.set_hscrollbar_style(ScrollbarStyle::AUTO);

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
            let r = term.h_to_row(100);
            let c = term.w_to_col(100);
            term.append(&format!(
                "Text size: {ts}, h_to_row(100): {r}, w_to_col(100): {c}\n"
            ));
            assert_eq!(ts, 14);
            term.set_text_size(30);
            assert_eq!(term.text_size(), 30);
            term.append(&format!(
                "Text size: {}, h_to_row(100): {}, w_to_col(100): {}\n",
                term.text_size(),
                term.h_to_row(100),
                term.w_to_col(100)
            ));
            term.set_text_size(ts);
            assert_eq!(term.text_size(), ts);
            term.append(&format!(
                "Text size: {}, h_to_row(100): {}, w_to_col(100): {}\n",
                term.text_size(),
                term.h_to_row(100),
                term.w_to_col(100)
            ));

            // Keyboard handler
            term.handle({
                move |term, e| {
                    match e {
                        fltk::enums::Event::KeyDown
                            if fltk::app::event_key() == fltk::enums::Key::Escape =>
                        {
                            // false to let FLTK handle ESC. true to hide ESC
                            false
                        }

                        fltk::enums::Event::KeyDown
                            if fltk::app::event_length() == 1 && fltk::app::is_event_ctrl() =>
                        {
                            // We handle control keystroke
                            let k = fltk::app::event_text();
                            term.append_utf8(&k);
                            true
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
                        // We can do this, or else ignore them (return false)
                        // fltk::enums::Event::Focus | fltk::enums::Event::Unfocus => {
                        //     term.redraw();
                        //     true
                        // }
                        _ => false, // Let FLTK handle everything else
                    }
                }
            });

            let attr_save = term.text_attrib();
            term.set_text_attrib(Attrib::Inverse | Attrib::Italic);
            term.append("\nStartup tests complete. Keyboard is live.\n");
            assert_eq!(term.text_attrib(), Attrib::Inverse | Attrib::Italic);
            term.set_text_attrib(attr_save);
            assert_eq!(term.text_attrib(), attr_save);
            term.redraw();
        }
    });

    app.run().unwrap();
}
//--------------------------------------------------------------------------------------
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
    term.plot_char('A', 0, 0);
    term.plot_char('B', 1, 1);
    term.plot_char('C', 2, 2);
    term.plot_char('D', 3, 3);
    term.plot_char('E', 4, 4);
    term.plot_char('F', 5, 5);
    term.set_text_fg_color(Color::XtermWhite);

    assert_eq!(term.cursor_row(), 11);
    assert_eq!(term.cursor_col(), 0);

    term.set_text_bg_color(Color::DarkBlue);
    term.plot_char_utf8('b', 8, 1);
    term.plot_char_utf8('↑', 9, 1);
    term.plot_char_utf8('c', 8, 2);
    term.plot_char_utf8('↑', 9, 2);
    term.plot_char_utf8('d', 8, 3);
    term.plot_char_utf8('↑', 9, 3);
    term.plot_char_utf8('e', 8, 4);
    term.plot_char_utf8('↑', 9, 4);
    term.plot_char_utf8('f', 8, 5);
    term.plot_char_utf8('↑', 9, 5);
    term.plot_char_utf8('g', 8, 6);
    term.plot_char_utf8('↑', 9, 6);
    term.set_text_bg_color(Color::TransparentBg);

    term.set_text_attrib(Attrib::Inverse | Attrib::Italic);
    term.append("Done!\n");
    term.set_text_attrib(Attrib::Normal);
}

//--------------------------------------------------------------------------------------
/// More tests that run when the menu bar button Test2 is clicked
fn mb_test2_cb(_choice: &mut fltk::menu::Choice, term: &mut Terminal) {
    term.take_focus().unwrap();
    term.reset_terminal();

    for i in 0..50 {
        term.append(&format!("{i}\n"));
    }
    assert_eq!(term.history_rows(), 100);

    term.clear_history();
    assert_eq!(term.history_use(), 0);

    term.set_text_attrib(Attrib::Inverse | Attrib::Italic);
    term.append("\nDone!\n");
    term.set_text_attrib(Attrib::Normal);
}

//--------------------------------------------------------------------------------------
/// Another set of tests that run when Test3 is clicked
fn mb_test3_cb(_choice: &mut fltk::menu::Choice, term: &mut Terminal) {
    term.take_focus().unwrap();
    term.reset_terminal();
    assert_eq!(term.text_bg_color_default(), Color::TransparentBg);

    assert_eq!(term.history_use(), 0);
    term.clear();
    assert_eq!(term.cursor_row(), 0);
    assert_eq!(term.history_use(), term.display_rows()); // A screenful of lines added to history

    term.append("Test\ntext\na\nb\nc\nd");
    assert_eq!(term.cursor_row(), 5);
    let hist = term.history_use();
    term.clear_screen_home(false);
    assert_eq!(term.cursor_row(), 0);
    assert_eq!(term.history_use(), hist); // History not changed

    term.append("Test\ntext\na\nb\nc\nd\ne");
    assert_eq!(term.cursor_row(), 6);
    term.clear_screen_home(true);
    assert_eq!(term.cursor_row(), 0);

    term.append("Test\ntext\na\nb\nc\n");
    assert_eq!(term.cursor_row(), 5);
    term.clear_to_color(Color::DarkBlue);
    assert_eq!(term.text_bg_color_default(), Color::TransparentBg);
    assert_eq!(term.text_bg_color(), Color::TransparentBg);
    assert_eq!(term.cursor_row(), 0);

    // Test cursor_home()
    term.append("Test\n\n\n\n\n\n\n\n\n\n");
    assert_eq!(term.cursor_row(), 10);
    term.cursor_home();
    assert_eq!(term.cursor_row(), 0);

    // Test the widget color
    assert_eq!(term.color(), Color::Black); // Default
    term.set_color(Color::DarkGreen);
    assert_eq!(term.color(), Color::DarkGreen);
    term.set_color(Color::Black);
    assert_eq!(term.color(), Color::Black);
    term.append(
        "This should be one line of white text on black, embedded into the top of a blue field.\n",
    );

    assert_eq!(term.output_translate(), OutFlags::LF_TO_CRLF); // default
    term.set_output_translate(OutFlags::OFF);
    assert_eq!(term.output_translate(), OutFlags::OFF);
    term.set_output_translate(OutFlags::LF_TO_CRLF); // restore default
    assert_eq!(term.output_translate(), OutFlags::LF_TO_CRLF);

    term.set_text_attrib(Attrib::Inverse | Attrib::Italic);
    term.append("\nDone!\n");
    term.set_text_attrib(Attrib::Normal);
}

//--------------------------------------------------------------------------------------
/// Another set of tests for the ring-buffer access methods
/// Note: these tests depend heavily on the low-level "protected" parts of the fltk library, which should be used with caution.
fn mb_test4_cb(_choice: &mut fltk::menu::Choice, term: &mut Terminal) {
    let sel_len = term.selection_text_len();
    let sel = term.selection_text();

    term.take_focus().unwrap();
    term.reset_terminal();
    // Test the Utf8Char primitive
    let uc = Utf8Char::new(b'Q');
    let uc1 = uc.text_utf8();
    assert_eq!(&uc1, b"Q");
    assert_eq!(&uc.attrib(), &Attrib::Normal);
    assert_eq!(
        &uc.charflags(),
        &(CharFlags::FG_XTERM | CharFlags::BG_XTERM)
    );
    assert_eq!(&uc.fgcolor(), &Color::XtermWhite);
    assert_eq!(&uc.bgcolor(), &Color::TransparentBg);

    let ring_rows = term.ring_rows();

    term.take_focus().unwrap();
    term.clear_history();
    assert_eq!(term.history_use(), 0);

    // Subtract row numbers, modulo `rows`
    fn row_diff(rows: i32, a: i32, b: i32) -> i32 {
        match a - b {
            n if n < 0 => n + rows,
            n => n,
        }
    }
    // disp_srow is always 1 greater than hist_erow, modulo (ring_rows+1)
    assert_eq!(row_diff(ring_rows, term.disp_srow(), term.hist_erow()), 1);
    assert!(term.disp_srow() >= 0);
    assert!(term.disp_erow() >= 0);
    assert!(term.hist_srow() >= 0);
    assert!(term.hist_erow() >= 0);
    assert!(term.offset() >= 0);
    assert!(term.disp_srow() <= ring_rows);
    assert!(term.disp_erow() <= ring_rows);
    assert!(term.hist_srow() <= ring_rows);
    assert!(term.hist_erow() <= ring_rows);
    assert!(term.offset() <= ring_rows);

    assert_eq!(term.ring_srow(), 0);
    assert_eq!(term.ring_erow(), ring_rows - 1);
    assert_eq!(
        row_diff(ring_rows, term.disp_erow(), term.disp_srow()) + 1,
        term.display_rows()
    );
    assert_eq!(
        row_diff(ring_rows, term.hist_erow(), term.hist_srow()) + 1,
        term.history_rows()
    );

    assert_eq!(term.ring_erow(), term.ring_rows() - 1);
    assert_eq!(term.ring_srow(), 0);

    /// Local function to read back all rows from the display into a long string.
    /// Does not include scrollback history.
    /// Trims trailing blanks on each line
    fn read_disp(term: &Terminal) -> String {
        let rows = term.display_rows();
        let mut text: Vec<u8> = Vec::with_capacity((rows * 64) as usize);
        for row in 0..rows {
            let r = term.u8c_disp_row(row).trim();
            // Iterate through a row, accumulating [u8]
            for c in r.iter() {
                // Note: Sometimes utf-8 length is > 1
                text.extend_from_slice(c.text_utf8());
            }
            text.extend_from_slice(b"\n");
        }
        // Return the result as a string
        std::str::from_utf8(&text).unwrap().to_string()
    }

    term.clear();
    term.append("Top line  ↑ (up-arrow)");
    term.set_text_attrib(Attrib::Underline);
    term.append("  ");
    term.set_text_attrib(Attrib::Normal);
    term.append("  \n");
    let mut text_out = read_disp(term);
    // Trim trailing empty lines
    text_out = text_out.trim_end_matches(&"\n").to_string();
    // The two plain blanks at the end will be trimmed, the two underlined blanks will be retained.

    assert_eq!(text_out, "Top line  ↑ (up-arrow)  ");
    let r = term.u8c_disp_row(0);
    assert_eq!(r.col(0).text_utf8(), b"T");
    assert_eq!(r.col(10).text_utf8(), b"\xe2\x86\x91"); // UTF-8 up-arrow
    assert_eq!(r.col(24).text_utf8(), b" "); // First blank after test text, NOT trimmed
    let r = term.u8c_disp_row(1);
    assert_eq!(r.col(0).text_utf8(), b" "); // Second row starts with blanks
    assert_eq!(r.col(1).text_utf8(), b" "); // Second row is full of blanks

    // Clear the screen again, then append test text, then read it back and compare
    let test_text = "The wind was a torrent of darkness among the gusty trees.
The moon was a ghostly galleon tossed upon cloudy seas.
The road was a ribbon of moonlight over the purple moor,
And the highwayman came riding—
            Riding—riding—
The highwayman came riding, up to the old inn-door.";

    term.clear_history();
    term.clear();
    let bg_save = term.text_bg_color();
    let fg_save = term.text_fg_color();
    term.set_text_bg_color(Color::DarkBlue); // Set spooky colors
    term.set_text_fg_color(Color::from_rgb(0x40, 0x40, 0xff));
    term.append(test_text);
    term.set_text_bg_color(bg_save);
    term.set_text_fg_color(fg_save);

    let mut text_out = read_disp(term);
    // Trim trailing empty lines
    text_out = text_out.trim_end_matches(&"\n").to_string();
    assert_eq!(test_text, text_out);

    assert_eq!(row_diff(ring_rows, term.disp_srow(), term.hist_erow()), 1);

    assert_eq!(term.ring_srow(), 0);
    assert_eq!(term.ring_erow(), ring_rows - 1);
    assert_eq!(
        row_diff(ring_rows, term.disp_erow(), term.disp_srow()) + 1,
        term.display_rows()
    );
    assert_eq!(
        row_diff(ring_rows, term.hist_erow(), term.hist_srow()) + 1,
        term.history_rows()
    );

    term.append(&format!(
        "\n\nScreen has {} rows of {} columns.\n",
        term.display_rows(),
        term.display_columns()
    ));

    term.append(&format!("Selection len: {sel_len}\nSelection: '{sel}'\n"));
}

//--------------------------------------------------------------------------------------
/// Yet another set of tests for misc cursor functions and other stuff
/// Note: these tests depend heavily on the low-level "protected" parts of the fltk library, which should be used with caution.
fn mb_test5_cb(_choice: &mut fltk::menu::Choice, term: &mut Terminal) {
    term.take_focus().unwrap();

    // Test the attr_fg_color and attr_bg_color methods.
    // Put a single character 'A' into the buffer and check it
    term.clear(); // No reset_terminal(), just clear() to preserve the mouse selection for later
    term.set_text_bg_color(Color::TransparentBg);
    term.set_text_fg_color(Color::XtermWhite);
    term.append("A");
    let r = &term.u8c_disp_row(0);
    let uc = r.col(0);
    assert_eq!(uc.text_utf8(), b"A");
    assert_eq!(&uc.attr_fgcolor(None), &Color::XtermWhite);
    assert_eq!(&uc.attr_bgcolor(None), &Color::TransparentBg);
    assert_eq!(&uc.attr_bgcolor(Some(term)), &Color::Black);
    assert_eq!(&uc.attr_fgcolor(Some(term)), &Color::XtermWhite);
    assert_eq!(&uc.attrib(), &Attrib::Normal);

    // Put a short string "BCD" into the first line of the buffer, with fg color change after the 'B' and bold after 'C'
    term.clear();
    term.set_text_fg_color_xterm(XtermColor::White);
    term.set_text_bg_color_xterm(XtermColor::Black);
    assert_eq!(term.text_attrib(), Attrib::Normal);

    assert!(term.ansi());
    term.append("B\x1b[32mC\x1b[1mD\n");

    let r = &term.u8c_disp_row(0);
    let uc = r.col(0);
    assert_eq!(uc.text_utf8(), b"B");
    assert!(uc.is_char(b'B'));
    assert!(!uc.is_char(b'A'));
    assert_eq!(&uc.fgcolor(), &Color::XtermWhite);
    assert_eq!(&uc.bgcolor(), &Color::XtermBlack);
    assert_eq!(&uc.attr_fgcolor(None), &Color::XtermWhite);
    assert_eq!(&uc.attr_bgcolor(None), &Color::XtermBlack);
    assert_eq!(
        &uc.charflags(),
        &(CharFlags::FG_XTERM | CharFlags::BG_XTERM)
    );

    let uc = r.col(1);
    assert_eq!(uc.text_utf8(), b"C");
    assert!(uc.is_char(b'C'));
    assert_eq!(&uc.fgcolor(), &Color::XtermGreen);
    assert_eq!(&uc.bgcolor(), &Color::XtermBlack);
    assert_eq!(&uc.attr_fgcolor(None), &Color::XtermGreen);
    assert_eq!(&uc.attr_bgcolor(None), &Color::XtermBlack);
    assert_eq!(
        &uc.charflags(),
        &(CharFlags::FG_XTERM | CharFlags::BG_XTERM)
    );

    let uc = r.col(2);
    assert_eq!(uc.text_utf8(), b"D");
    assert!(uc.is_char(b'D'));
    assert_eq!(&uc.fgcolor(), &Color::XtermGreen);
    assert_eq!(&uc.bgcolor(), &Color::XtermBlack);
    assert_eq!(&uc.attr_fgcolor(None), &Color::from_rgb(0x20, 0xf0, 0x20));
    assert_eq!(&uc.attr_bgcolor(None), &Color::from_rgb(0x20, 0x20, 0x20));
    assert_eq!(
        &uc.attr_fgcolor(Some(term)),
        &Color::from_rgb(0x20, 0xf0, 0x20)
    );
    assert_eq!(&uc.attr_bgcolor(None), &Color::from_rgb(0x20, 0x20, 0x20));
    assert_eq!(
        &uc.charflags(),
        &(CharFlags::FG_XTERM | CharFlags::BG_XTERM)
    );

    // Put a short string "BCDE" into the buffer, with fg color change after the 'B', bg change after 'C', and bold after 'D'
    term.clear();
    term.set_text_fg_color_xterm(XtermColor::White);
    term.set_text_bg_color_xterm(XtermColor::Black);
    term.set_text_attrib(Attrib::Normal);
    assert_eq!(term.text_attrib(), Attrib::Normal);

    assert!(term.ansi());
    term.append("B\x1b[37mC\x1b[44mD\x1b[1mE\n");

    let r = &term.u8c_disp_row(0);
    let uc = r.col(0);
    assert_eq!(uc.text_utf8(), b"B");
    assert!(uc.is_char(b'B'));
    assert!(!uc.is_char(b'A'));
    assert_eq!(&uc.fgcolor(), &Color::XtermWhite);
    assert_eq!(&uc.bgcolor(), &Color::XtermBlack);
    assert_eq!(&uc.attr_fgcolor(None), &Color::XtermWhite);
    assert_eq!(&uc.attr_bgcolor(None), &Color::XtermBlack);
    assert_eq!(
        &uc.charflags(),
        &(CharFlags::FG_XTERM | CharFlags::BG_XTERM)
    );

    let uc = r.col(1);
    assert_eq!(uc.text_utf8(), b"C");
    assert!(uc.is_char(b'C'));
    assert_eq!(&uc.fgcolor(), &Color::XtermWhite);
    assert_eq!(&uc.bgcolor(), &Color::XtermBlack);
    assert_eq!(&uc.attr_fgcolor(None), &Color::XtermWhite);
    assert_eq!(&uc.attr_bgcolor(None), &Color::XtermBlack);
    assert_eq!(
        &uc.charflags(),
        &(CharFlags::FG_XTERM | CharFlags::BG_XTERM)
    );

    let uc = r.col(2);
    assert_eq!(uc.text_utf8(), b"D");
    assert!(uc.is_char(b'D'));
    assert_eq!(&uc.fgcolor(), &Color::XtermWhite);
    assert_eq!(&uc.bgcolor(), &Color::XtermBgBlue);
    assert_eq!(&uc.attr_fgcolor(None), &Color::XtermWhite);
    assert_eq!(&uc.attr_bgcolor(None), &Color::XtermBgBlue);
    assert_eq!(
        &uc.charflags(),
        &(CharFlags::FG_XTERM | CharFlags::BG_XTERM)
    );

    let uc = r.col(3);
    assert_eq!(uc.text_utf8(), b"E");
    assert!(uc.is_char(b'E'));
    assert_eq!(&uc.fgcolor(), &Color::XtermWhite);
    assert_eq!(&uc.bgcolor(), &Color::XtermBgBlue);
    assert_eq!(&uc.attr_fgcolor(None), &Color::from_hex(0xf0f0f0));
    assert_eq!(&uc.attr_bgcolor(None), &Color::from_hex(0x2020e0));
    assert_eq!(
        &uc.charflags(),
        &(CharFlags::FG_XTERM | CharFlags::BG_XTERM)
    );

    // Test some miscellaneous Utf8 constants
    assert_eq!(uc.length(), 1);
    assert_eq!(uc.max_utf8(), 4);
    assert_eq!(uc.pwidth(), 9.0);
    assert_eq!(uc.pwidth_int(), 9);

    term.set_text_fg_color_xterm(XtermColor::White);
    term.set_text_bg_color_xterm(XtermColor::Black);
    term.clear();
    term.set_text_attrib(Attrib::Normal);

    // Mouse selection functions
    term.append(&format!("Mouse selection: {:?}\n", &term.get_selection()));
    term.clear_mouse_selection();
    assert_eq!(term.get_selection(), None);

    // Play with cursor position
    term.append("0123456789\n"); // Set up test pattern
    term.append("ABCDEFGHIJ\n");
    term.append("abcdefghij\n");

    term.set_cursor_row(1);
    assert_eq!(term.cursor_row(), 1);
    term.set_cursor_col(1);
    assert_eq!(term.cursor_col(), 1);
    assert_eq!(term.u8c_cursor().text_utf8(), b"1");

    term.append("----"); // Overwrites text at cursor and moves cursor forward
    assert_eq!(term.cursor_row(), 1);
    assert_eq!(term.cursor_col(), 5);
    assert_eq!(term.u8c_cursor().text_utf8(), b"5");
    term.set_cursor_col(1);
    assert_eq!(term.u8c_cursor().text_utf8(), b"-"); // Overwritten text

    term.cursor_up(1, false);
    assert_eq!(term.cursor_row(), 0);
    assert_eq!(term.cursor_col(), 1);
    assert_eq!(term.u8c_cursor().text_utf8(), b"o");

    // Hit top of screen, so nothing happens
    term.cursor_up(1, false);
    assert_eq!(term.cursor_row(), 0);
    assert_eq!(term.cursor_col(), 1);
    assert_eq!(term.u8c_cursor().text_utf8(), b"o");

    // Hit top of screen with scroll enabled. A blank line from history is scrolled in.
    term.cursor_up(1, true);
    assert_eq!(term.cursor_row(), 0);
    assert_eq!(term.cursor_col(), 1);
    assert_eq!(term.u8c_cursor().text_utf8(), b" ");

    // Go back down to the overwritten text
    term.cursor_down(2, false);
    assert_eq!(term.cursor_row(), 2);
    assert_eq!(term.cursor_col(), 1);
    assert_eq!(term.u8c_cursor().text_utf8(), b"-");

    // Go right past the overwritten text
    term.cursor_right(4, false);
    assert_eq!(term.cursor_row(), 2);
    assert_eq!(term.cursor_col(), 5);
    assert_eq!(term.u8c_cursor().text_utf8(), b"5");

    // Go left to the end of the overwritten text
    term.cursor_left(1);
    assert_eq!(term.cursor_row(), 2);
    assert_eq!(term.cursor_col(), 4);
    assert_eq!(term.u8c_cursor().text_utf8(), b"-");

    // Scroll back down, removing the blank line at the top.
    // Cursor stays in place, the text moves under it.
    term.scroll(1);
    assert_eq!(term.cursor_row(), 2);
    assert_eq!(term.cursor_col(), 4);
    assert_eq!(term.u8c_cursor().text_utf8(), b"E");

    // Clear from here to end-of-line
    term.clear_eol();
    assert_eq!(term.cursor_row(), 2);
    assert_eq!(term.cursor_col(), 4);
    assert_eq!(term.u8c_cursor().text_utf8(), b" ");

    // Now clear from here to start-of-line. Cursor does not move.
    term.clear_sol();
    assert_eq!(term.cursor_row(), 2);
    assert_eq!(term.cursor_col(), 4);
    assert_eq!(term.u8c_cursor().text_utf8(), b" ");
    term.cursor_left(1);
    assert_eq!(term.u8c_cursor().text_utf8(), b" ");
    term.set_cursor_col(0);
    assert_eq!(term.u8c_cursor().text_utf8(), b" ");

    // Clear some lines
    term.clear_line(1);
    assert_eq!(term.cursor_row(), 2);
    assert_eq!(term.cursor_col(), 0);
    term.set_cursor_row(1);
    assert_eq!(term.u8c_cursor().text_utf8(), b" ");
    term.set_cursor_row(3);
    term.clear_cur_line();
    assert_eq!(term.u8c_cursor().text_utf8(), b" ");
    assert_eq!(term.cursor_row(), 3);
    assert_eq!(term.cursor_col(), 0);

    term.append("Two lines above are intentionally left blank.\n");
    assert_eq!(term.cursor_row(), 4);
    assert_eq!(term.cursor_col(), 0);

    // Set up the test pattern again, then play with insert/delete
    term.append("0123456789\n");
    term.append("ABCDEFGHIJ\n");
    term.append("abcdefghij\n");
    assert_eq!(term.cursor_row(), 7);

    term.set_cursor_row(4);
    term.set_cursor_col(4);
    assert_eq!(term.u8c_cursor().text_utf8(), b"4");

    term.insert_char('x', 5); // Push this row right 5 chars starting at col 4
    assert_eq!(term.u8c_cursor().text_utf8(), b"x");
    term.cursor_right(5, false);
    assert_eq!(term.cursor_col(), 9);
    assert_eq!(term.u8c_cursor().text_utf8(), b"4");

    // Insert two blank rows above cursor. Cursor stays put.
    term.insert_rows(2);
    assert_eq!(term.cursor_row(), 4);
    assert_eq!(term.cursor_col(), 9);
    assert_eq!(term.u8c_cursor().text_utf8(), b" ");
    term.cursor_down(2, false); // Go down to find our text again
    assert_eq!(term.u8c_cursor().text_utf8(), b"4");

    // Go back to the beginning of the inserted 'x' characters and delete them.
    term.cursor_left(5);
    assert_eq!(term.u8c_cursor().text_utf8(), b"x");
    term.delete_cur_chars(5);
    assert_eq!(term.cursor_row(), 6);
    assert_eq!(term.cursor_col(), 4);
    assert_eq!(term.u8c_cursor().text_utf8(), b"4");

    term.delete_chars(7, 2, 2); // Delete "CD" from the next row
    term.cursor_down(1, false);
    term.cursor_left(2);
    assert_eq!(term.u8c_cursor().text_utf8(), b"E");

    term.delete_rows(1); // Middle row of pattern is gone, cursor stays put
    assert_eq!(term.u8c_cursor().text_utf8(), b"c");
    term.cursor_up(1, false);
    term.delete_rows(2); // Delete remains of test pattern

    term.set_text_attrib(Attrib::Bold);
    term.insert_char_eol('-', 3, 15, 20);
    term.set_cursor_row(3);
    term.set_cursor_col(15);
    assert_eq!(term.u8c_cursor().text_utf8(), b"-"); // Check the insertion
    assert_eq!(term.u8c_cursor().attrib(), Attrib::Bold);

    term.set_text_attrib(Attrib::Italic);
    term.append(" and all lines below");
    term.set_text_attrib(Attrib::Normal);
    term.cursor_down(1, false);

    let mut hsb = term.hscrollbar();
    let mut sb = term.scrollbar();
    hsb.set_value(100.0);
    assert_eq!(hsb.value(), 100.0);
    sb.set_value(50.0);
    assert_eq!(sb.value(), 50.0);
    hsb.set_value(0.0);
    assert_eq!(hsb.value(), 0.0);
    sb.set_value(0.0);
    assert_eq!(sb.value(), 0.0);
}

//--------------------------------------------------------------------------------------
/// Displays an error message.
/// **Note**: this does not work unless called from the UI thread.
/// If you need cross-thread error boxes, try using a non-fltk
/// dialog such as `native_dialog::MessageDialog` instead of `fltk::dialog`
fn error_box(msg: String) {
    fltk::app::lock().unwrap();
    fltk::dialog::message_title("Error");
    fltk::dialog::message_set_hotspot(true);
    fltk::dialog::message_icon_label("!");
    fltk::dialog::message_default(&msg);
    fltk::app::unlock();
}
