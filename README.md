# fltk-rs

Rust bindings for the FLTK GUI library. Still in alpha.

The FLTK gui crate is a crossplatform lightweight library which can be linked to statically (LGPL) to produce small, self-contained and fast binaries.

[Documentation](https://docs.rs/fltk)

## Using in a project
Just add the following to your project's Cargo.toml file.
```toml
[dependencies]
fltk = "^0.1.14"
```
An example hello world application:
```rust
use fltk::window::*;

fn main() {
    let app = fl::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    wind.show();
    app.run().unwrap();
}
```
Please check the examples directory for more examples.

## Building

To build, just run:
```
$ git clone https://github.com/MoAlyousef/fltk-rs
$ cd fltk-rs
$ cargo build
```


## Dependencies

CMake and a C++ compiler need to be installed and in your PATH for a crossplatform build. After that on Windows and Mac OS X, normally no external dependencies are needed. For Linux, X11 development headers need to be installed for development. For Debian-based distribution, that means running:
```
$ sudo apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev
```


## Examples

To run the examples: 
```
$ cargo run --example editor
$ cargo run --example calculator
$ cargo run --example gallery
$ cargo run --example button
$ cargo run --example terminal
$ cargo run --example hello
```
![alt_test](screenshots/hello.jpg)

![alt_test](screenshots/gallery.jpg)
Setting the scheme to Gtk.

![alt_test](screenshots/calc.jpg)

![alt_test](screenshots/editor.jpg)
Setting the scheme to Gtk

![alt_test](screenshots/terminal.jpg)

## Currently implemented widgets

Most common widgets are implemented: 
- Images (BMP, JPEG, GIF, PNG, SVG)
- Button
- RadioButton
- ToggleButton
- RoundButton
- CheckButton
- LightButton
- RepeatButton
- Native FileDialog
- Frame (Fl_Box)
- Window
- DoubleWindow
- MenuWindow
- Group
- Pack
- Tabs
- Scroll
- Tile
- TextDisplay
- TextEditor
- Input
- IntInput
- FloatInput
- MultilineInput
- SecretInput
- FileInput
- Output
- MultilineOutput
- MenuBar
- MenuItem
- Choice (dropdown list)
- Slider
- ValueSlider
- Dial
- Counter
- Scrollbar
- Roller
- Adjuster
- ValueInput
- Browser
- SelectBrowser
- HoldBrowser
- MultiBrowser

## Todo

- Complete widget set
- Better documentation
- Better testing

## Contributions

Contributions are very welcome!
