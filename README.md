# fltk-rs

[![Documentation](https://docs.rs/fltk/badge.svg)](https://docs.rs/fltk)
[![Crates.io](https://img.shields.io/crates/v/fltk.svg)](https://crates.io/crates/fltk)
[![License](https://img.shields.io/crates/l/fltk.svg)](https://github.com/fltk-rs/fltk-rs/blob/master/LICENSE)
[![Build](https://github.com/fltk-rs/fltk-rs/workflows/Build/badge.svg?branch=master)](https://github.com/fltk-rs/fltk-rs/actions)


Rust bindings for the FLTK 1.5 Graphical User Interface library. 

The fltk crate is a cross-platform lightweight gui library which can be statically linked to produce small, self-contained and fast gui applications.

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

Why choose FLTK?
- Lightweight. Small binary, around 1mb after stripping. [Small memory footprint](https://szibele.com/memory-footprint-of-gui-toolkits/).
- Speed. Fast to install, fast to build, fast at startup and fast at runtime. 
- Single executable. No DLLs to deploy.
- Supports old architectures. 
- FLTK's permissive license which allows static linking for closed-source applications.
- Themeability (5 supported schemes: Base, GTK, Plastic, Gleam and Oxy), and additional theming using [fltk-theme](https://crates.io/crates/fltk-theme).
- Provides around 80 customizable widgets. 
- Has inbuilt image support.

Here is a [list](https://en.wikipedia.org/wiki/FLTK#Use) of software using FLTK. For software using fltk-rs, check [here](https://github.com/fltk-rs/fltk-rs/issues/418).

- [Link](https://github.com/fltk/fltk) to the official FLTK repository.
- [Link](https://www.fltk.org/doc-1.4/index.html) to the official documentation.

## Usage

Just add the following to your project's Cargo.toml file:

```toml
[dependencies]
fltk = "^0.20"
```
To use the latest changes in the repo:
```toml
[dependencies]
fltk = { version = "^0.20", git = "https://github.com/fltk-rs/fltk-rs", branch = "dev" }
```
Or if you have other depenendencies which depend on fltk-rs:
```toml
[dependencies]
fltk = "^0.20"

[patch.crates-io]
fltk = { git = "https://github.com/fltk-rs/fltk-rs", branch = "dev" }
```

To use the bundled libs (available for x64 windows (msvc & gnu (msys2-mingw)), x64 & aarch64 linux & macos):
```toml
[dependencies]
fltk = { version = "^1.5", features = ["fltk-bundled"] }
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
Please check the [examples](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples) directory for more examples.
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

Alternatively, you can use Flex (for flexbox layouts), Pack or Grid:
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

Another example:
```rust,no_run
use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300);
    let mut col = Flex::default_fill().column();
    col.set_margins(120, 80, 120, 80);
    let mut frame = Frame::default();
    let mut but = Button::default().with_label("Click me!");
    col.fixed(&but, 40);
    col.end();
    wind.end();
    wind.show();

    but.set_callback(move |_| frame.set_label("Hello world"));

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
        let label: i32 = frame.label().unwrap().parse().unwrap();
        if let Some(msg) = r.recv() {
            match msg {
                Message::Increment => frame.set_label(&(label + 1).to_string()),
                Message::Decrement => frame.set_label(&(label - 1).to_string()),
            }
        }
    }
```
For the remainder of the code, check the full example [here](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/counter2.rs).

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
Handled or ignored events using the handle method should return true, unhandled events should return false. More examples are available in the fltk/examples directory.

For an alternative event handling mechanism using an immediate-mode approach, check the [fltk-evented crate](https://crates.io/crates/fltk-evented).

### Theming

FLTK offers 5 application schemes:
- Base
- Gtk
- Gleam
- Plastic
- Oxy

(Additional theming can be found in the [fltk-theme](https://crates.io/crates/fltk-theme) crate)

These can be set using the `App::with_scheme()` method.
```rust,ignore
let app = app::App::default().with_scheme(app::Scheme::Gleam);
```
Themes of individual widgets can be optionally modified using the provided methods in the `WidgetExt` trait, such as `set_color()`, `set_label_font()`, `set_frame()` etc:
```rust,ignore
    some_button.set_color(Color::Light1); // You can use one of the provided colors in the fltk enums
    some_button.set_color(Color::from_rgb(255, 0, 0)); // Or you can specify a color by rgb or hex/u32 value
    some_button.set_color(Color::from_u32(0xffebee));
    some_button.set_frame(FrameType::RoundUpBox);
    some_button.set_font(Font::TimesItalic);
```
For default application colors, fltk-rs provides `app::background()`, `app::background2()` and `app::foreground()`. You can also specify the default application selection/inactive colors, font, label size, frame type, scrollbar size, menu line-spacing. Additionally the [fltk-theme](https://crates.io/crates/fltk-theme) crate offers some other predefined color maps (dark theme, tan etc) and widget themes which can be loaded into your application.

## Build Dependencies

Rust (version > 1.63), CMake (version > 3.15), Git and a C++17 compiler need to be installed and in your PATH for a cross-platform build from source. [Ninja](https://github.com/ninja-build/ninja) is recommended, but not required. This crate also offers a bundled form of fltk on selected x86_64 and aarch64 platforms (Windows (msvc and gnu), MacOS, Linux), this can be enabled using the fltk-bundled feature flag as mentioned in the usage section (this requires curl and tar to download and unpack the bundled libraries).

- Windows: 
    - MSVC: Windows SDK
    - Gnu: No dependencies
- MacOS: MacOS SDK (installed as part of xcode or the xcode command line tools).
- Linux/BSD: X11 (and wayland for if `no-wayland` feature flag isn't used) and OpenGL development headers need to be installed for development. The libraries themselves are normally available on linux/bsd distros with a graphical user interface.

For Debian-based GUI distributions, that means running:
```bash
sudo apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev
```
For RHEL-based GUI distributions, that means running:
```bash
sudo yum groupinstall "X Software Development" && sudo yum install pango-devel libXinerama-devel libstdc++-static
```
For Arch-based GUI distributions, that means running:
```bash
sudo pacman -S libx11 libxext libxft libxinerama libxcursor libxrender libxfixes pango cairo libgl mesa --needed
```
For Alpine linux:
```bash
apk add pango-dev fontconfig-dev libxinerama-dev libxfixes-dev libxcursor-dev mesa-gl
```
For NixOS (Linux distribution) this `nix-shell` environment can be used:
```bash
nix-shell --packages rustc cmake git gcc xorg.libXext xorg.libXft xorg.libXinerama xorg.libXcursor xorg.libXrender xorg.libXfixes libcerf pango cairo libGL mesa pkg-config
```
For Freebsd:
```bash
pkg install -y cairo pango fontconfig freetype2 libX11 libXext libXfixes mesa-libs
# building might require setting `CPATH=/usr/local/include` and `LIBRARY_PATH=$LIBRARY_PATH:/usr/local/lib
```

## Runtime Dependencies
- Windows: None
- MacOS: None
- Linux: You need X11 libraries, as well as pango and cairo for drawing (and OpenGL if you want to enable the enable-glwindow feature):
```bash
apt-get install -qq --no-install-recommends libx11-6 libxinerama1 libxft2 libxext6 libxcursor1 libxrender1 libxfixes3 libcairo2 libpango-1.0-0 libpangocairo-1.0-0 libpangoxft-1.0-0 libglib2.0-0 libfontconfig1 libglu1-mesa libgl1
```
Note that if you installed the build dependencies, it will also install the runtime dependencies automatically as well.

Also note that most graphical desktop environments already have these libs already installed. This list can be useful if you want to test your already built package in CI/docker (where there is no graphical user interface).

## Features

The following are the features offered by the crate:
- use-ninja: Uses the ninja build system if available for a faster build, especially on Windows.
- no-pango-cairo: Build without pango and cairo support on Linux/BSD, if rtl/cjk font support is not needed.
- fltk-bundled: Support for bundled versions of cfltk and fltk on selected platforms (requires curl and tar)
- enable-glwindow: Support for drawing using OpenGL functions.
- system-libpng: Uses the system libpng
- system-libjpeg: Uses the system libjpeg
- system-zlib: Uses the system zlib
- no-wayland: Disables FLTK's wayland hybrid backend (runs on wayland when present, and on X11 when not present). Requires libwayland-dev, wayland-protocols, libdbus-1-dev, libxkbcommon-dev, libgtk-3-dev (optional, for the GTK-style titlebar), in addition to the X11 development packages. Sample [CI](https://github.com/MoAlyousef/test_wayland/blob/main/.github/workflows/rust.yml).
- fltk-config: Uses an already installed FLTK's fltk-config to build this crate against. This still requires FLTK 1.5. Useful for reducing build times, testing against a locally built FLTK and doesn't need to invoke neither git nor cmake. 

## FAQ

please check the [FAQ](FAQ.md) page for frequently asked questions, encountered issues, guides on deployment, and contribution.

## Building

To build, just run:
```bash
git clone https://github.com/fltk-rs/fltk-rs --recurse-submodules
cd fltk-rs
cargo build
```

## Currently implemented types:

### Image types:
- SharedImage
- BmpImage
- JpegImage
- GifImage
- AnimGifImage
- PngImage
- SvgImage
- Pixmap
- RgbImage
- XpmImage
- XbmImage
- PnmImage
- TiledImage

### Widgets:
- Buttons
    - Button
    - RadioButton
    - ToggleButton
    - RoundButton
    - CheckButton
    - LightButton
    - RepeatButton
    - RadioLightButton
    - RadioRoundButton
    - ReturnButton
    - ShortcutButton
- Dialogs
    - Native FileDialog
    - FileChooser
    - HelpDialog
    - Message dialog
    - Alert dialog
    - Password dialog
    - Choice dialog
    - Input dialog
    - ColorChooser dialog
- Frame (Fl_Box)
- Windows
    - Window
    - SingleWindow (single buffered)
    - DoubleWindow (double buffered)
    - MenuWindow
    - OverlayWindow
    - GlWindow (requires the "enable-glwindow" flag)
    - Experimental GlWidgetWindow (requires the "enable-glwindow" flag)
- Groups
    - Group
    - Pack (Horizontal and Vertical)
    - Tabs
    - Scroll
    - Tile
    - Wizard
    - ColorChooser
    - Flex (Column and Row)
    - Grid
- Text display widgets
    - TextDisplay
    - TextEditor
- Input widgets
    - Input
    - IntInput
    - FloatInput
    - MultilineInput
    - SecretInput
    - FileInput
- Output widgets
    - Output
    - MultilineOutput
- Menu widgets
    - MenuBar
    - MenuItem
    - Choice (dropdown list)
    - SysMenuBar (MacOS menu bar which appears at the top of the screen)
    - MenuButton
- Valuator widgets
    - Slider
    - NiceSlider
    - ValueSlider
    - Dial
    - LineDial
    - Counter
    - Scrollbar
    - Roller
    - Adjuster
    - ValueInput
    - ValueOutput
    - FillSlider
    - FillDial
    - HorSlider (Horizontal slider)
    - HorFillSlider
    - HorNiceSlider
    - HorValueSlider
- Browsing widgets
    - Browser
    - SelectBrowser
    - HoldBrowser
    - MultiBrowser
    - FileBrowser
    - CheckBrowser
- Miscelaneous widgets
    - Spinner
    - Clock (Round and Square)
    - Chart (several chart types are available)
    - Progress (progress bar)
    - Tooltip
    - InputChoice
    - HelpView
- Table widgets
    - Table
    - TableRow
    - SmartTable (via the [fltk-table crate](https://crates.io/crates/fltk-table))
- Terminal Widget
    - Terminal
- Trees
    - Tree
    - TreeItem

### Drawing primitives
(In the draw module)
### Surface types:
- Printer.
- ImageSurface.
- SvgFileSurface.

### GUI designer

fltk-rs supports FLUID, the RAD wysiwyg designer for FLTK. 
Checkout the [fl2rust crate](https://github.com/fltk-rs/fl2rust) and [fl2rust template](https://github.com/fltk-rs/fl2rust-template).

- [FLTK Rust: Latest FLUID, fl2rust and fltk-rs](https://www.youtube.com/watch?v=33NdaW08fP8)

## Examples

To run the [examples](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples): 
```bash
cargo run --example editor
cargo run --example calculator
cargo run --example calculator2
cargo run --example counter
cargo run --example hello_svg
cargo run --example hello_button
cargo run --example fb
cargo run --example pong
cargo run --example custom_widgets
cargo run --example custom_dial
...
```

Using custom theming and also FLTK provided default schemes like Gtk:

- [hello_svg](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/hello_svg.rs)

- ![alt_test](https://github.com/fltk-rs/fltk-rs/raw/master/screenshots/hello.jpg)

- [calculator2](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/calculator2.rs)

- ![alt_test](https://github.com/fltk-rs/fltk-rs/raw/master/screenshots/calc2.jpg)

- [counter3](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/counter3.rs)

- ![alt_test](https://github.com/fltk-rs/fltk-rs/raw/master/screenshots/flutter_like.jpg)

- [custom_dial](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/custom_dial.rs)

- ![alt_test](https://github.com/fltk-rs/fltk-rs/raw/master/screenshots/dial.jpg)

- [calculator](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/calculator.rs)

- ![alt_test](https://github.com/fltk-rs/fltk-rs/raw/master/screenshots/calc.jpg)

- [tabs](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/tabs.rs)

- ![alt_test](https://github.com/fltk-rs/fltk-rs/raw/master/screenshots/tabs.jpg)

- [counter](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/counter.rs)

- ![alt_test](https://github.com/fltk-rs/fltk-rs/raw/master/screenshots/counter.jpg)

- [editor](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/editor.rs)

- ![alt_test](https://github.com/fltk-rs/fltk-rs/raw/master/screenshots/editor.jpg)

- [table](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/table.rs)

- ![alt_test](https://github.com/fltk-rs/fltk-rs/raw/master/screenshots/table.jpg)

- [charts](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/charts.rs)

- ![alt_test](https://github.com/fltk-rs/fltk-rs/raw/master/screenshots/charts.jpg)

- [pong](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/pong.rs)

- ![alt_test](https://github.com/fltk-rs/fltk-rs/raw/master/screenshots/pong.gif)

- [frames](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples/frames.rs)

- ![alt_test](https://github.com/fltk-rs/fltk-rs/raw/master/screenshots/frames.jpg)

Different frame types which can be used with many different widgets such as Frame, Button widgets, In/Output widgets...etc.

More interesting examples can be found in the fltk-rs-demos [repo](https://github.com/fltk-rs/demos).
Also a nice implementation of the 7guis tasks can be found [here](https://github.com/tdryer/7guis-fltk-rs).
Various advanced examples can also be found [here](https://github.com/wyhinton/FLTK-RS-Examples).


### Themes

Additional themes can be found in the [fltk-theme crate](https://github.com/fltk-rs/fltk-theme).

- ![screenshots/aero.jpg](https://github.com/fltk-rs/fltk-theme/blob/dfd1e97b62c94d7af9d615cab0ab809226957a3f/screenshots/aero.jpg)

- ![screenshots/black.jpg](https://github.com/fltk-rs/fltk-theme/blob/dfd1e97b62c94d7af9d615cab0ab809226957a3f/screenshots/black.jpg)

And more...

### Extra widgets

This crate exposes FLTK's set of widgets, which are all customizable. Additional custom widgets can be found in the [fltk-extras crate](https://github.com/fltk-rs/fltk-extras).

![image](https://user-images.githubusercontent.com/37966791/212541355-91062d78-5c5d-4b7a-aa6d-e1be49cff340.png)

![image](https://user-images.githubusercontent.com/37966791/212541425-f594a7bc-d7bc-49e5-90f3-03f52d437cce.png)

![ss](https://github.com/fltk-rs/fltk-extras/assets/37966791/a03e1912-7658-48be-a354-2b588b417fd8)

![image](https://user-images.githubusercontent.com/37966791/212541392-2cd4fb08-4152-484a-86da-64b2bc476a0e.png)

## Tutorials

- [Basics](https://www.youtube.com/watch?v=ygP4egJtmzw)
- [New basics](https://youtu.be/S1NSsHZs6hI) (Uses fltk post 1.0)
- [User input](https://youtu.be/rIq2O4vg9fQ)
- [Client-side web todo app](https://youtu.be/tdfFXi4-Yrw)
- [Create a media player using the vlc crate](https://youtu.be/enxqU3bhCEs)
- [Custom dialogs](https://youtu.be/tXeXHoKG6-I)
- [Add drag and drop to the editor example](https://www.youtube.com/watch?v=qp5hnRvSxAg)
- [Drawing things with fltk](https://www.youtube.com/watch?v=r9MOpvfBPWs)
- [Working with images](https://www.youtube.com/watch?v=Rn2sjfAX4WI)
- [Audio player with custom widgets](https://www.youtube.com/watch?v=okdFx6tv7ds)
- [Use FLUID (RAD tool) with Rust](https://www.youtube.com/watch?v=k_P0wG3-dNk)
- [multiple windows and embedding windows](https://www.youtube.com/watch?v=qEPYx1Lw7fY)
- [FLTK Rust tutorial: Improve FLTK's toggle button appearance!](https://www.youtube.com/watch?v=WCTbPKHXR-o)
- [FLTK Rust: Customizing your app and widgets](https://www.youtube.com/watch?v=uCZl0PuMVGo)
- [FLTK Rust: fltk-table, a boilerplate-less table creating crate](https://www.youtube.com/watch?v=pVJ8Yq1kDGs)
- [FLTK Rust: intro into the fltk-evented crate](https://www.youtube.com/watch?v=rAVHBl3W9W8)
- [FLTK Rust: Latest FLUID, fl2rust and fltk-rs](https://www.youtube.com/watch?v=33NdaW08fP8)

More videos in the playlist [here](https://www.youtube.com/playlist?list=PLHqrrowPLkDu9U-uk60sGM-YWLOJFfLoE).
Some of the demo projects can be found [here](https://github.com/fltk-rs/demos).
