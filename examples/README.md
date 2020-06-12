# Examples

Each file is standalone executable with a main function. Examples vary from basic hello world simplicity to some more complex things. They may contain logic bugs, however their purpose is for demonstration and testing UI elements only. If you would like to contribute fixes, improvements or more example, you're welcome to do so.
Some drawing primitives necessitate the usage of Quartz 2D drawing on MacOS and might not work there, such as the paint or pong examples.

To run the examples: 
```
$ cargo run --example editor
$ cargo run --example calculator
$ cargo run --example gallery
$ cargo run --example terminal
$ cargo run --example counter
$ cargo run --example hello
$ cargo run --example hello_button
$ cargo run --example paint
$ cargo run --example glwindow
$ cargo run --example pong
$ cargo run --example <filename>
```