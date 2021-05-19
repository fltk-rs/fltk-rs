/*!
# fltk-rs

[![Documentation](https://docs.rs/fltk/badge.svg)](https://docs.rs/fltk)
[![Crates.io](https://img.shields.io/crates/v/fltk.svg)](https://crates.io/crates/fltk)
[![License](https://img.shields.io/crates/l/fltk.svg)](https://github.com/fltk-rs/fltk-rs/blob/master/LICENSE)
[![Build](https://github.com/fltk-rs/fltk-rs/workflows/Build/badge.svg?branch=master)](https://github.com/fltk-rs/fltk-rs/actions)



Rust bindings for the FLTK Graphical User Interface library.

The fltk crate is a crossplatform lightweight gui library which can be statically linked to produce small, self-contained (no dependencies) and fast gui applications.

Tutorials:
- [Video](https://github.com/fltk-rs/fltk-rs#tutorials)
- [Written](https://github.com/fltk-rs/fltk-rs/wiki)

Here is a [list](https://en.wikipedia.org/wiki/FLTK#Use) of software using FLTK. For software using fltk-rs, check [here](https://github.com/fltk-rs/fltk-rs/issues/418).

- [Link](https://github.com/fltk/fltk) to the official FLTK repository.
- [Link](https://www.fltk.org/doc-1.3/index.html) to the official documentation.

## Usage

Just add the following to your project's Cargo.toml file:

```toml
[dependencies]
fltk = "^1"
```
To use the latest changes in the repo:
```toml
[dependencies]
fltk = { version = "^1", git = "https://github.com/fltk-rs/fltk-rs" }
```

The library is automatically built and statically linked to your binary.

For faster builds you can enable ninja builds for the C++ source using the "use-ninja" feature.

An example hello world application:

```no_run
use fltk::{app, prelude::*, window::Window};
let app = app::App::default();
let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
wind.end();
wind.show();
app.run().unwrap();
```

Another example showing the basic callback functionality:
```no_run
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
let app = app::App::default();
let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
let mut frame = Frame::new(0, 0, 400, 200, "");
let mut but = Button::new(160, 210, 80, 40, "Click me!");
wind.end();
wind.show();
but.set_callback(move |_| frame.set_label("Hello World!"));
app.run().unwrap();
```
Please check the examples directory for more examples.
You will notice that all widgets are instantiated with a new() method, taking the x and y coordinates, the width and height of the widget, as well as a label which can be left blank if needed. Another way to initialize a widget is using the builder pattern: (The following buttons are equivalent)

```no_run
use fltk::{button::*, prelude::*};
let but1 = Button::new(10, 10, 80, 40, "Button 1");

let but2 = Button::default()
    .with_pos(10, 10)
    .with_size(80, 40)
    .with_label("Button 2");
```

An example of a counter showing use of the builder pattern:
```no_run
use fltk::{app, button::*, frame::*, prelude::*, window::*};
let app = app::App::default();
let mut wind = Window::default()
    .with_size(160, 200)
    .center_screen()
    .with_label("Counter");
let mut frame = Frame::default()
    .with_size(100, 40)
    .center_of(&wind)
    .with_label("0");
let mut but_inc = Button::default()
    .size_of(&frame)
    .above_of(&frame, 0)
    .with_label("+");
let mut but_dec = Button::default()
    .size_of(&frame)
    .below_of(&frame, 0)
    .with_label("-");
wind.make_resizable(true);
wind.end();
wind.show();
/* Event handling */
```
Alternatively, you can use packs to layout your widgets:
```no_run
use fltk::{button::*, frame::*, group::*, prelude::*, window::*};
let mut wind = Window::default().with_size(160, 200).with_label("Counter");
// Vertical is default. You can choose horizontal using pack.set_type(PackType::Horizontal);
let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
pack.set_spacing(10);
let mut but_inc = Button::default().with_size(0, 40).with_label("+");
let mut frame = Frame::default().with_size(0, 40).with_label("0");
let mut but_dec = Button::default().with_size(0, 40).with_label("-");
pack.end();
```

### Events

Events can be handled using the `set_callback` method (as above) or the available `fltk::app::set_callback()` free function, which will handle the default trigger of each widget(like clicks for buttons):
```rust,ignore
    /* previous hello world code */
    but.set_callback(move |_| frame.set_label("Hello World!"));
    app.run().unwrap();
```
Another way is to use message passing:
```rust,ignore
    /* previous counter code */
    let (s, r) = app::channel::<Message>();

    but_inc.emit(s, Message::Increment);
    but_dec.emit(s, Message::Decrement);

    while app.wait() {
        let label: i32 = frame.label().parse().unwrap();
        if let Some(msg) = r.recv() {
            match msg {
                Message::Increment => frame.set_label(&(label + 1).to_string()),
                Message::Decrement => frame.set_label(&(label - 1).to_string()),
            }
        }
    }
```
For the remainder of the code, check the full example [here](https://github.com/fltk-rs/fltk-rs/blob/master/fltk/examples/counter2.rs).

For custom event handling, the handle() method can be used:
```rust,ignore
    some_widget.handle(move |widget, ev: Event| {
        match ev {
            /* handle ev */
        }
    });
```
Handled or ignored events using the handle method should return true, unhandled events should return false.
More examples are available in the examples directory.

### Theming

FLTK offers 4 application themes (called schemes):
- Base
- Gtk
- Gleam
- Plastic

These can be set using the `App::with_scheme()` function.
```rust,ignore
let app = app::App::default().with_scheme(app::Scheme::Gleam);
```
Themes of individual widgets can be optionally modified using the provided methods in the `WidgetExt` trait,
such as `set_color()`, `set_label_font()`, `set_frame()` etc:
```rust,ignore
    some_button.set_color(Color::Light1); //! You can use one of the provided colors in the fltk enums
    some_button.set_color(Color::from_rgb(255, 0, 0)); //! Or you can specify a color by rgb or hex/u32 value
    some_button.set_color(Color::from_u32(0xffebee));
    some_button.set_frame(FrameType::RoundUpBox);
    some_button.set_font(Font::TimesItalic);
```

## Features

The following are the features offered by the crate:
- use-ninja:  If you have ninja build installed, it builds faster than make or VS
- system-libpng: Uses the system libpng
- system-libjpeg: Uses the system libjpeg
- system-zlib: Uses the system zlib
- fltk-bundled: Support for bundled versions of cfltk and fltk on selected platforms (requires curl and tar)
- no-pango: Build without pango support on Linux/BSD.
- enable-glwindow: Support for drawing using OpenGL functions.

## Dependencies

Rust (version > 1.38), CMake (version > 3.0), Git and a C++11 compiler need to be installed and in your PATH for a crossplatform build from source. This crate also offers a bundled form of fltk on selected platforms, this can be enabled using the fltk-bundled feature flag (which requires curl and tar to download and unpack the bundled libraries).

- Windows: No dependencies.
- MacOS: No dependencies.
- Linux/BSD: X11 and OpenGL development headers need to be installed for development. The libraries themselves are available on linux distros with a graphical user interface.

For Debian-based GUI distributions, that means running:
```ignore
$ sudo apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libpng-dev libgl1-mesa-dev libglu1-mesa-dev
```
For RHEL-based GUI distributions, that means running:
```ignore
$ sudo yum groupinstall "X Software Development" && yum install pango-devel libXinerama-devel libpng-devel
```
For Arch-based GUI distributions, that means running:
```ignore
$ sudo pacman -S libx11 libxext libxft libxinerama libxcursor libxrender libxfixes libpng pango cairo libgl mesa --needed
```
For Alpine linux:
```ignore
$ apk add pango-dev fontconfig-dev libxinerama-dev libxfixes-dev libxcursor-dev libpng-dev mesa-gl
```
For NixOS (Linux distribution) this `nix-shell` environment can be used:
```ignore
$ nix-shell --packages rustc cmake git gcc xorg.libXext xorg.libXft xorg.libXinerama xorg.libXcursor xorg.libXrender xorg.libXfixes libpng libcerf pango cairo libGL mesa pkg-config
```
- Android(experimental): Android Studio, Android Sdk, Android Ndk.

## FAQ

please check the [FAQ](https://github.com/fltk-rs/fltk-rs/blob/master/FAQ.md) page for frequently asked questions, encountered issues, guides on deployment, and contribution.
*/

#![allow(non_upper_case_globals)]
#![warn(missing_docs)]
#![warn(broken_intra_doc_links)]

/// Application related methods and functions
pub mod app;
/// Browser widgets
pub mod browser;
/// Button widgets
pub mod button;
/// Dialog widgets
pub mod dialog;
/// Drawing primitives
pub mod draw;
/// Fltk defined enums: Color, Font, `CallbackTrigger` etc
pub mod enums;
/// Basic fltk box/frame widget
pub mod frame;
/// Group widgets
pub mod group;
/// Image types supported by fltk
pub mod image;
/// Input widgets
pub mod input;
/// Menu widgets
pub mod menu;
/// Miscellaneous widgets not fitting a certain group
pub mod misc;
/// Output widgets
pub mod output;
/// All fltk widget traits and flt error types
pub mod prelude;
/// Widget surface to image functions
pub mod surface;
/// Table widgets
pub mod table;
/// Text display widgets
pub mod text;
/// Tree widgets
pub mod tree;
/// General utility functions
pub mod utils;
/// Valuator widgets
pub mod valuator;
/// Basic empty widget
pub mod widget;
/// Window widgets
pub mod window;

/// Printing related functions
#[cfg(not(target_os = "android"))]
pub mod printer;

#[macro_use]
extern crate fltk_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
