//! # fltk-rs
//!
//! [![Documentation](https://docs.rs/fltk/badge.svg)](https://docs.rs/fltk)
//! [![Crates.io](https://img.shields.io/crates/v/fltk.svg)](https://crates.io/crates/fltk)
//! [![License](https://img.shields.io/crates/l/fltk.svg)](https://github.com/MoAlyousef/fltk-rs/blob/master/LICENSE)
//! [![Build](https://github.com/MoAlyousef/fltk-rs/workflows/Build/badge.svg?branch=master)](https://github.com/MoAlyousef/fltk-rs/actions)
//!
//!
//!
//! Rust bindings for the FLTK Graphical User Interface library. 
//!
//! The FLTK crate is a crossplatform lightweight gui library which can be statically linked to produce small, self-contained (no dependencies) and fast gui applications.
//!
//! This crate is still in active development and is not production ready. However, you can still try it out and give valuable feedback.
//!
//! Here is a [list](https://en.wikipedia.org/wiki/FLTK#Use) of software using FLTK.
//!
//! - [Link](https://github.com/fltk/fltk) to the official FLTK repository.
//! - [Link](https://www.fltk.org/doc-1.3/index.html) to the official documentation.
//!
//! ## Usage
//!
//! Just add the following to your project's Cargo.toml file:
//!
//! ```toml
//! [dependencies]
//! fltk = "^0.4"
//! ```
//! The library is automatically statically linked to your binary. If however you would prefer dynamic linking, you can use the fltk-shared feature:
//! ```toml
//! [dependencies.fltk]
//! version = "^0.4"
//! features = ["fltk-shared"]
//! # or
//! [dependencies]
//! fltk = { version = "^0.4", features = ["fltk-shared"] }
//! ```
//! You can also enable ninja builds for a faster build of the C++ source using the "use-ninja" feature. Or if you have fltk already installed, you can use the fltk-system feature.
//!
//! To use the master branch in your project, you can use:
//! ```toml
//! [dependencies]
//! fltk = { git = "https://github.com/MoAlyousef/fltk-rs" }
//! ```
//!
//! An example hello world application:
//!
//! ```rust
//! use fltk::{app::*, window::*};
//!
//! fn main() {
//!     let app = App::default();
//!     let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
//!     wind.end();
//!     wind.show();
//!     app.run().unwrap();
//! }
//! ```
//!
//! Another example showing the basic callback functionality:
//! ```rust
//! use fltk::{app::*, button::*, frame::*, window::*};

//! fn main() {
//!     let app = App::default();
//!     let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
//!     let mut frame = Frame::new(0, 0, 400, 200, "");
//!     let mut but = Button::new(160, 210, 80, 40, "Click me!");
//!     wind.end();
//!     wind.show();
//!     but.set_callback(Box::new(move || frame.set_label("Hello World!")));
//!     app.run().unwrap();
//! }
//! ```
//! Please check the examples directory for more examples.
//! You will notice that all widgets are instantiated with a new() method, taking the x and y coordinates, as well as the width and height of the widget. Most widgets, except the TextDisplay and TextEditor, also take a label which can be left blank if needed. Another way to initialize a widget is using the builder pattern: (The following buttons are equivalent)
//!
//! ```rust
//! let but1 = Button::new(10, 10, 80, 40, "Button 1");
//!
//! let but2 = Button::default()
//!     .with_pos(10, 10)
//!     .with_size(80, 40)
//!     .with_label("Button 2");
//! ```
//!
//! An example of a counter showing use of the builder pattern:
//! ```rust
//! fn main() {
//!     let app = app::App::default();
//!     let mut wind = Window::default()
//!         .with_size(160, 200)
//!         .center_screen()
//!         .with_label("Counter");
//!     let mut frame = Frame::default()
//!         .with_size(100, 40)
//!         .center_of(&wind)
//!         .with_label("0");
//!     let mut but_inc = Button::default()
//!         .size_of(&frame)
//!         .above_of(&frame, 0)
//!         .with_label("+");
//!     let mut but_dec = Button::default()
//!         .size_of(&frame)
//!         .below_of(&frame, 0)
//!         .with_label("-");
//!     wind.make_resizable(true);
//!     wind.end();
//!     wind.show();
//!     /* Event handling */
//! }
//! ```
//!
//! ### Events
//!
//! Events can be handled using the set_callback method (as above) or the available fltk::app::set_callback() free function, which will handle the default trigger of each widget(like clicks for buttons):
//! ```rust
//!     /* previous hello world code */
//!     but.set_callback(Box::new(move || frame.set_label("Hello World!")));
//!     app.run().unwrap();
//! ```
//! Another way is to use message passing:
//! ```rust
//!     /* previous counter code */
//!     let (s, r) = app::channel::<Message>();
//!
//!     but_inc.set_callback(Box::new(move || s.send(Message::Increment)));
//!     but_dec.set_callback(Box::new(move || s.send(Message::Decrement)));
//!
//!     while app.wait() {
//!         let label: i32 = frame.label().parse().unwrap();
//!         match r.recv() {
//!             Some(Message::Increment) => frame.set_label(&(label + 1).to_string()),
//!             Some(Message::Decrement) => frame.set_label(&(label - 1).to_string()),
//!             None => (),
//!         }
//!     }
//! ```
//!
//! For custom event handling, the handle() method can be used:
//! ```rust
//!     some_widget.handle(Box::new(move |ev: app::Event| {
//!         match ev {
//!             /* handle ev */
//!         }
//!     }));
//! ```
//! Handled or ignored events using the handle method should return true, unhandled events should return false. 
//! More examples are available in the examples directory.
//!
//! ### Theming
//!
//! FLTK offers 4 application themes (called schemes):
//! - Base
//! - Gtk
//! - Gleam
//! - Plastic
//!
//! These can be set using the App::set_scheme() function.
//! Themes of individual widgets can be optionally modified using the provided methods in the WidgetExt trait, 
//! such as set_color(), set_label_font(), set_frame_type() etc:
//! ```rust
//!     some_button.set_color(Color::Light1); //! You can use one of the provided colors in the fltk enums
//!     some_button.set_color(Color::from_rgb(255, 0, 0)); //! Or you can specify a color by rgb or hex/u32 value
//!     some_button.set_color(Color::from_u32(0xffebee));
//!     some_button.set_frame(FrameType::RoundUpBox);
//!     some_button.set_font(Font::TimesItalic);
//! ```
//!
//! ## Features
//!
//! The following are the features offered by the crate:
//! - fltk-shared: Builds a shared lib of fltk
//! - use-ninja:  If you have ninja build installed, it builds faster than make or VS
//! - system-fltk: If you would like to use the installed fltk library, should be FLTK 1.4
//! - system-libpng: Uses the system libpng
//! - system-libjpeg: Uses the system libjpeg
//! - system-zlib: Uses the system zlib
//! - legacy-opengl: Support of Lagacy OpenGL, the crate uses GLVND by default
//! - gl-window: Support for GlWindow
//!
//! ## FAQ
//!
//! please check the [FAQ](https://github.com/MoAlyousef/fltk-rs/blob/master/FAQ.md) page for frequently asked questions, encountered issues, guides on deployment, and contribution.

pub mod app;
pub mod browser;
pub mod button;
pub mod dialog;
pub mod draw;
pub mod enums;
pub mod frame;
pub mod group;
pub mod image;
pub mod input;
pub mod menu;
pub mod misc;
pub mod output;
pub mod prelude;
pub mod table;
pub mod text;
pub mod tree;
pub mod valuator;
pub mod widget;
pub mod window;

#[macro_use]
extern crate fltk_derive;
