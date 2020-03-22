# fltk-rs

Rust bindings for the FLTK GUI library.
Still very barebones, undocumented, untested and not at all production ready!

The FLTK gui library is a crossplatform lightweight C++ library which can be linked to statically (LGPL) to produce small, self-contained and fast binaries. 

## Using in a project
Just add the following to your project's Cargo.toml file.
```
[dependencies]
fltk = "0.1.4"
```
An example hello world application:
```rust
use fltk::window::*;

fn main() {
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    wind.show();
    fl::run();
}
```
Please check the examples directory for more examples.

## Building

To build, just run:
```
$ cargo build
```


## Dependencies

CMake and Ninja need to be installed and in your PATH for a crossplatform build. After that on Windows and Mac OS X, normally no external dependencies are needed. For Linux, X11 development headers need to be installed for development. For Debian-based distrobution, that means running:
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
$ cargo run --example hello
```

![alt_test](screenshots/gallery.jpg)
![alt_test](screenshots/calc.jpg)
![alt_test](screenshots/editor.jpg)


## Currently implemented widgets

Most common widgets are implemented: 
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
- Group
- Pack
- Tabs
- Scroll
- Tile
- TextDisplay (needs more work)
- TextEditor (needs more work)
- Input, IntInput, FloatInput, MultilineInput
- Output, MultilineOutput
- MenuBar
- MenuItem
- Choice (dropdown list)
- Slider, ValueSlider
- Dial
- Counter
- Scrollbar
- Roller
- Images

The implementation isn't complete no less. Customized event handling is not implemented.

## Todo

- Support customized event handling
- Complete widget set
- Better documentation
- Better testing

## Contributions

Contributions are very welcome!
