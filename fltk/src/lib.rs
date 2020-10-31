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
//! fltk = "^0.10"
//! ```
//! The library is automatically built and statically linked to your binary.
//!
//! You can also enable ninja builds for a faster build of the C++ source using the "use-ninja" feature.
//!
//! An example hello world application:
//!
//! ```rust
//!     use fltk::{app::*, window::*};
//!
//!     let app = App::default();
//!     let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
//!     wind.end();
//!     wind.show();
//!     app.run().unwrap();
//! ```
//!
//! Another example showing the basic callback functionality:
//! ```rust
//!     use fltk::{app::*, button::*, frame::*, window::*};
//!
//!     let app = App::default();
//!     let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
//!     let mut frame = Frame::new(0, 0, 400, 200, "");
//!     let mut but = Button::new(160, 210, 80, 40, "Click me!");
//!     wind.end();
//!     wind.show();
//!     but.set_callback(move || frame.set_label("Hello World!"));
//!     app.run().unwrap();
//! ```
//! Please check the examples directory for more examples.
//! You will notice that all widgets are instantiated with a new() method, taking the x and y coordinates, the width and height of the widget, as well as a label which can be left blank if needed. Another way to initialize a widget is using the builder pattern: (The following buttons are equivalent)
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
//! ```
//!
//! ### Events
//!
//! Events can be handled using the set_callback method (as above) or the available fltk::app::set_callback() free function, which will handle the default trigger of each widget(like clicks for buttons):
//! ```rust
//!     /* previous hello world code */
//!     but.set_callback(move || frame.set_label("Hello World!"));
//!     app.run().unwrap();
//! ```
//! Another way is to use message passing:
//! ```rust
//!     /* previous counter code */
//!     let (s, r) = app::channel::<Message>();
//!
//!     but_inc.emit(s, Message::Increment);
//!     but_dec.emit(s, Message::Decrement);
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
//! For the remainder of the code, check the full example here:
//! https://github.com/MoAlyousef/fltk-rs/blob/master/examples/counter2.rs
//!
//! For custom event handling, the handle() method can be used:
//! ```rust
//!     some_widget.handle(move |ev: Event| {
//!         match ev {
//!             /* handle ev */
//!         }
//!     });
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
//! These can be set using the App::with_scheme() function.
//! ```rust
//! let app = App::default().with_scheme(AppScheme::Gleam);
//! ```
//! Themes of individual widgets can be optionally modified using the provided methods in the WidgetExt trait,
//! such as set_color(), set_label_font(), set_frame() etc:
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
//! - use-ninja:  If you have ninja build installed, it builds faster than make or VS
//! - system-libpng: Uses the system libpng
//! - system-libjpeg: Uses the system libjpeg
//! - system-zlib: Uses the system zlib
//! - fltk-bundled: Support for bundled versions of cfltk and fltk on selected platforms (requires curl and tar)
//! - enable-glwindow: Support for drawing using OpenGL functions.
//!
//! ## Dependencies
//! 
//! Rust (version > 1.38), CMake (version > 3.0), Git and a C++11 compiler need to be installed and in your PATH for a crossplatform build from source. This crate also offers a bundled form of fltk on selected platforms, this can be enabled using the fltk-bundled feature flag (which requires curl and tar to download and unpack the bundled libraries).
//! 
//! - Windows: No dependencies.
//! - MacOS: No dependencies.
//! - Linux: X11 and OpenGL development headers need to be installed for development. The libraries themselves are available on linux distros with a graphical user interface.
//! 
//! For Debian-based GUI distributions, that means running:
//! ```
//! $ sudo apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev
//! ```
//! For RHEL-based GUI distributions, that means running:
//! ```
//! $ sudo yum groupinstall "X Software Development" 
//! ```
//! For Arch-based GUI distributions, that means running:
//! ```
//! $ sudo pacman -S libx11 libxext libxft libxinerama libxcursor libxrender libxfixes pango libgl mesa --needed
//! ```
//! For Alpine linux:
//! ```
//! $ apk add pango-dev fontconfig-dev libxinerama-dev libxfixes-dev libxcursor-dev
//! ```
//! If you have ninja-build installed, you can enable it using the "use-ninja" feature. This should accelerate build times significantly.
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
pub mod surface;
pub(crate) mod utils;

pub use enums::*;
pub use prelude::*;

#[macro_use]
extern crate fltk_derive;
