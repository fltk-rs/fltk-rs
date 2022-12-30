/*!
# fltk-rs

[![Documentation](https://docs.rs/fltk/badge.svg)](https://docs.rs/fltk)
[![Crates.io](https://img.shields.io/crates/v/fltk.svg)](https://crates.io/crates/fltk)
[![License](https://img.shields.io/crates/l/fltk.svg)](https://github.com/fltk-rs/fltk-rs/blob/master/LICENSE)
[![Build](https://github.com/fltk-rs/fltk-rs/workflows/Build/badge.svg?branch=master)](https://github.com/fltk-rs/fltk-rs/actions)



Rust bindings for the FLTK Graphical User Interface library.

The fltk crate is a cross-platform lightweight gui library which can be statically linked to produce small, self-contained (no external dependencies) and fast gui applications.

Why choose FLTK?
- Lightweight. Small binary, around 1mb after stripping. [Small memory footprint](https://szibele.com/memory-footprint-of-gui-toolkits/).
- Speed. Fast to install, fast to build, fast at startup and fast at runtime.
- Single executable. No DLLs to deploy.
- Supports old architectures.
- FLTK's permissive license which allows static linking for closed-source applications.
- Themeability (5 supported schemes: Base, GTK, Plastic, Gleam and Oxy), and additional theming using [fltk-theme](https://crates.io/crates/fltk-theme).
- Provides around 80 customizable widgets.
- Has inbuilt image support.

Resources:
- [Book](https://fltk-rs.github.io/fltk-book/)
- [本書的中文翻譯](https://flatig.vip/fltk-book-zh)
- [Documentation](https://docs.rs/fltk)
- [Videos](https://github.com/fltk-rs/fltk-rs#tutorials)
- [Discussions](https://github.com/fltk-rs/fltk-rs/discussions)
- [Examples](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples)
- [Demos](https://github.com/fltk-rs/demos)
- [7guis-fltk-rs](https://github.com/tdryer/7guis-fltk-rs)
- [FLTK-RS-Examples](https://github.com/wyhinton/FLTK-RS-Examples)
- Erco's FLTK cheat [page](http://seriss.com/people/erco/fltk/), which is an excellent FLTK C++ reference.

Here is a [list](https://en.wikipedia.org/wiki/FLTK#Use) of software using FLTK. For software using fltk-rs, check [here](https://github.com/fltk-rs/fltk-rs/issues/418).

- [Link](https://github.com/fltk/fltk) to the official FLTK repository.
- [Link](https://www.fltk.org/doc-1.4/index.html) to the official documentation.

## Usage

Just add the following to your project's Cargo.toml file:

```toml
[dependencies]
fltk = "^1.2"
```
To use the latest changes in the repo:
```toml
[dependencies]
fltk = { version = "^1.2", git = "https://github.com/fltk-rs/fltk-rs" }
```

To use the bundled libs (available for x64 windows (msvc & gnu (msys2)), x64 linux & macos):
```toml
[dependencies]
fltk = { version = "^1.2", features = ["fltk-bundled"] }
```

The library is automatically built and statically linked to your binary.

An example hello world application:

```rust,no_run
use fltk::{app, prelude::*, window::Window};
fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    wind.end();
    wind.show();
    app.run().unwrap();
}
```

Another example showing the basic callback functionality:
```rust,no_run
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();
    but.set_callback(move |_| frame.set_label("Hello World!")); // the closure capture is mutable borrow to our button
    app.run().unwrap();
}
```
Please check the examples directory for more examples.
You will notice that all widgets are instantiated with a new() method, taking the x and y coordinates, the width and height of the widget, as well as a label which can be left blank if needed. Another way to initialize a widget is using the builder pattern: (The following buttons are equivalent)

```rust,no_run
use fltk::{button::Button, prelude::*};
let but1 = Button::new(10, 10, 80, 40, "Button 1");

let but2 = Button::default()
    .with_pos(10, 10)
    .with_size(80, 40)
    .with_label("Button 2");
```

An example of a counter showing use of the builder pattern:
```rust,no_run
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
fn main() {
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
    app.run().unwrap();
}
```
Alternatively, you can use Pack, Flex (for flexbox layouts) or [Grid](https://github.com/fltk-rs/fltk-grid):
```rust,no_run
use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};
fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(160, 200).with_label("Counter");
    let mut flex = Flex::default().with_size(120, 140).center_of_parent().column();
    let mut but_inc = Button::default().with_label("+");
    let mut frame = Frame::default().with_label("0");
    let mut but_dec = Button::default().with_label("-");
    flex.end();
    wind.end();
    wind.show();
    app.run().unwrap();
}
```

### Events

Events can be handled using the `set_callback` method (as above) or the available `fltk::app::set_callback()` free function, which will handle the default trigger of each widget(like clicks for buttons):
```rust,ignore
    /* previous hello world code */
    but.set_callback(move |_| frame.set_label("Hello World!"));
    another_but.set_callback(|this_button| this_button.set_label("Works"));
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
            Event::Push => {
                println!("Pushed!");
                true
            },
            /* other events to be handled */
            _ => false,
        }
    });
```
Handled or ignored events using the handle method should return true, unhandled events should return false.
More examples are available in the examples directory.

For an alternative event handling mechanism using `on_<event>` methods, check the [fltk-evented crate](https://crates.io/crates/fltk-evented).

### Theming

FLTK offers 5 application schemes:
- Base
- Gtk
- Gleam
- Plastic
- Oxy

(Additional theming can be found in the [fltk-theme](https://crates.io/crates/fltk-theme) crate)

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
For default application colors, fltk-rs provides `app::background()`, `app::background2()` and `app::foreground()`. You can also specify the default application selection/inactive colors, font, label size, frame type, scrollbar size, menu line-spacing. Additionally the [fltk-theme](https://crates.io/crates/fltk-theme) crate offers some other predefined color maps (dark theme, tan etc) and widget themes which can be loaded into your application.

## Dependencies

Rust (version > 1.45), CMake (version > 3.11), Git and a C++11 compiler need to be installed and in your PATH for a cross-platform build from source. [Ninja](https://github.com/ninja-build/ninja) is recommended, but not required. This crate also offers a bundled form of fltk on selected x86_64 platforms (Windows (msvc and gnu), MacOS, Linux), this can be enabled using the fltk-bundled feature flag as mentioned in the usage section (this requires curl and tar to download and unpack the bundled libraries).

- Windows:
    - MSVC: Windows SDK
    - Gnu: No dependencies
- MacOS: No dependencies.
- Linux/BSD: X11 and OpenGL development headers need to be installed for development. The libraries themselves are normally available on linux distros with a graphical user interface.

For Debian-based GUI distributions, that means running:
```ignore
$ sudo apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev
```
For RHEL-based GUI distributions, that means running:
```ignore
$ sudo yum groupinstall "X Software Development" && yum install pango-devel libXinerama-devel libstdc++-static
```
For Arch-based GUI distributions, that means running:
```ignore
$ sudo pacman -S libx11 libxext libxft libxinerama libxcursor libxrender libxfixes pango cairo libgl mesa --needed
```
For Alpine linux:
```ignore
$ apk add pango-dev fontconfig-dev libxinerama-dev libxfixes-dev libxcursor-dev mesa-gl
```
For NixOS (Linux distribution) this `nix-shell` environment can be used:
```ignore
$ nix-shell --packages rustc cmake git gcc xorg.libXext xorg.libXft xorg.libXinerama xorg.libXcursor xorg.libXrender xorg.libXfixes libcerf pango cairo libGL mesa pkg-config
```

## Features

The following are the features offered by the crate:
- use-ninja: Uses the ninja build system if available for a faster build, especially on Windows.
- fltk-bundled: Support for bundled versions of cfltk and fltk on selected platforms (requires curl and tar)
- no-pango: Build without pango support on Linux/BSD.
- enable-glwindow: Support for drawing using OpenGL functions.
- system-libpng: Uses the system libpng
- system-libjpeg: Uses the system libjpeg
- system-zlib: Uses the system zlib

## FAQ

please check the [FAQ](https://github.com/fltk-rs/fltk-rs/blob/master/FAQ.md) page for frequently asked questions, encountered issues, guides on deployment, and contribution.
*/

#![allow(non_upper_case_globals)]
#![allow(clippy::needless_doctest_main)]
#![warn(missing_docs)]
#![allow(clippy::type_complexity)]

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

pub mod examples;

/// Basic fltk box/frame widget
pub mod frame;

/// Group widgets
pub mod group;

/// Image types supported by fltk
pub mod image;

/// Input widgets
pub mod input;

/// mod macros;
pub mod macros;

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

pub mod widget;

pub mod window;

/// Printing related functions
#[cfg(not(target_os = "android"))]
pub mod printer;
