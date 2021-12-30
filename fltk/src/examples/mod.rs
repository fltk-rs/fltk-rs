/*!
Documentation for the examples

Each file is a standalone executable with its own main function. Examples vary from basic hello world simplicity to some more complex examples. They may contain logic bugs, however their purpose is for demonstration and for testing UI elements. If you would like to contribute fixes, improvements or more examples, you're very welcome to do so.

To run the examples:
```
$ cargo run --example editor
$ cargo run --example calculator
$ cargo run --example calculator2
$ cargo run --example tabs
$ cargo run --example terminal
$ cargo run --example counter
$ cargo run --example hello
$ cargo run --example hello_button
$ cargo run --example paint
$ cargo run --example pong
$ cargo run --example fb
$ cargo run --example custom_widgets
$ cargo run --example <filename>
```

More interesting examples can be found in the fltk-rs-demos [repo](https://github.com/fltk-rs/demos).
Also a nice implementation of the 7guis tasks can be found [here](https://github.com/tdryer/7guis-fltk-rs).
Various advanced examples can also be found [here](https://github.com/wyhinton/FLTK-RS-Examples).
*/

pub mod animations;
pub mod calculator;
pub mod calculator2;
pub mod charts;
pub mod closable_tab;
pub mod composite_widgets;
pub mod counter;
pub mod counter2;
pub mod counter3;
pub mod counter4;
pub mod custom_choice;
pub mod custom_dial;
pub mod custom_popup;
pub mod custom_widgets;
pub mod defaults;
pub mod editor;
pub mod fb;
pub mod flex;
pub mod format_text;
pub mod frames;
pub mod gradients;
pub mod hello;
pub mod hello_button;
pub mod image;
pub mod menubutton;
pub mod messages;
pub mod paint;
pub mod pong;
pub mod popup_browser;
pub mod rgb;
pub mod rounded_images;
pub mod shapedwindow;
pub mod spreadsheet;
pub mod system_fonts;
pub mod table;
pub mod tabs;
pub mod temp_converter;
pub mod terminal;
pub mod threads_windows;
pub mod tree;
pub mod widget_table;
pub mod wizard;
