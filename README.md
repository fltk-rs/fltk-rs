# fltk-rs

Rust bindings for the FLTK Graphical User Interface library. Still in alpha.

The FLTK crate is a crossplatform lightweight gui library which can be statically linked to produce small ( < 1mb for a hello world application after stripping), self-contained (no dependencies) and fast gui applications. 

[Documentation](https://docs.rs/fltk)

## Usage
Just add the following to your project's Cargo.toml file:

```toml
[dependencies]
fltk = "^0.1.20"
```
An example hello world application:

```rust
use fltk::{app::*, window::*};

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    wind.show();
    app.run().unwrap();
}
```
Please check the examples directory for more examples.
You will notice that all widgets are instantiated with a new() method, taking the x and y coordinates, as well as the width and height of the widget. Most widgets, except the TextDisplay and TextEditor, also take a label which can be left blank if needed. Another way to initialize a widget is using the builder pattern: (The following buttons are equivalent)

```rust
let but1 = Button::new(10, 10, 80, 40, "Button 1");

let but2 = Button::default()
    .with_pos(10, 10)
    .with_size(80, 40)
    .with_label("Button 2");
```

## Dependencies

CMake and a C++ compiler need to be installed and in your PATH for a crossplatform build. 
- Windows: None.
- MacOs: None.
- Linux: X11 development headers need to be installed for development. For Debian-based distribution, that means running:
```
$ sudo apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev
```

## Building

To build, just run:
```
$ git clone https://github.com/MoAlyousef/fltk-rs
$ cd fltk-rs
$ cargo build
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
- SimpleTerminal
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
