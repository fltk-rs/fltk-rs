# FAQ

## Build issues

### Build fails on windows, why can't CMake find my toolchain?
If you're building using the MSVC toolchain, make sure you run you're build (at least your initial build) using the Native Tools Command Prompt, which should appear once you start typing "native" in the start menu, choose the version corresponding to your installed Rust toolchain (x86 or x64). The Native Tools Command Prompt has all the environment variables set correctly for native development.

If you're building for the GNU toolchain, make sure that Make is also installed, which usually comes installed in MSYS2 and Cygwin.

### Why do I get a Link error while using the mingw toolchain on windows?
If the linking fails because of this issue: https://github.com/rust-lang/rust/issues/47048, it should work by using the fltk-shared feature. Which would also generate a dynamic library which would need to be deployed with your application.
```toml
[dependencies]
fltk = { version = "^0.7", features = ["fltk-shared"] }
```

### How do I force CMake to use a certain C++ compiler?
FLTK works with all 3 major compilers. If you would like to change the C++ compiler that's chosen by default by CMake, you can change the CXX environment variable before running the build:
```
$ export CXX=/usr/bin/clang++
$ cargo run
```
CMake caches the C++ compiler variable after it's first run, so if the above failed because of a previous run, you would have to run ```cargo clean``` or you can manually delete the CMakeCache.txt file in the build directory.

## Deployment

### How do I deploy my application?
Rust, by default, statically links your application. FLTK is built also for static linking. That means that the resulting executable can be directly deployed without the need to deploy other files along with it. If you want to create a WIN32 application, Mac OS Bundle or Linux AppImage, please check the question just below!

### Why do I get a console window whenever I start my GUI app?
This is the default behavior of the toolchain, and is helpful for debugging purposes. It can be turned off easily by adding ```#![windows_subsystem = "windows"]``` at the beginning of your main.rs file if you're on windows. For Mac OS and Linux, this is done by a post-build process to create a Mac OS Bundle or Linux AppImage respectively.

See [cargo-bundle](https://github.com/burtonageo/cargo-bundle) for an automated tool for creating Mac OS app bundles. 

See [here](https://docs.appimage.org/packaging-guide/overview.html#converting-existing-binary-packages) for directions on creating an AppImage for Linux.

### Why is the size of my resulting executable larger than I had expected?
FLTK is known for it's small applications. Make sure you're building in release, and make sure symbols are stripped using the strip command in Unix-like systems. On Windows it's unnecessary since symbols would end up in the pdb file (which shouldn't be deployed).

### Can I cross-compile my application to a mobile platform or WASM?
FLTK currently doesn't support WASM nor mobile platforms. It is focused on desktop applications.

## Licensing

### Can I use this crate in a commercial application?
Yes. This crate has an MIT license which requires acknowledgment. FLTK (the C++ library) is licensed under the LGPL license with an exception allowing static linking for commercial/closed-source use. You can find the full terms of both licenses here:
- https://github.com/fltk/fltk/blob/master/COPYING
- https://github.com/MoAlyousef/fltk-rs/blob/master/LICENSE

## Alignment

### Why can't I align input or output text to the right?
FLTK has some known issues with text alignment and right-to-left language support.

## Concurrency

### Do you plan on supporting multithreading or async/await?
FLTK supports multithreaded and concurrent applications. See the examples directory for examples on usage with threads, messages, async_std and tokio.

## Windowing

### Why does FLTK exit when I hit the escape key?
This is the default behavior in FLTK. You can easily override it by setting a callback for your main window:
```rust
    wind.set_callback(Box::new(move || {
        if fltk::app::event() == fltk::Event::Close {
            app.quit(); // Which would close using the close button. You can also assign other keys to close the application
        }
    }));
```

## Panics

### My app panics when I try to handle events, how can I fix it?
This is due to a debug_assert which checks that the involved widget and the window are capable of handling events. Although most events would be handled correctly, some events require that the aforementioned conditions be met. Thus it is advisable to place your event handling code after the main drawing is done, i.e after calling your main window's show() method. Another point is that event handling and drawing should be done in the main thread.

## Memory and unsafety

### How memory-safe is fltk-rs?
FLTK manages it's own memory. Any widget is automatically owned by a parent which does the book-keeping as well and deletion, this is the enclosing widget implementing GroupExt such as windws etc. This is done in the C++ FLTK library itself. Any constructed widget calls the current() method which detects the enclosing group widget, and calls its add() method rending ownership to the group widget. Upon destruction of the group widget, all owned widgets are freed. Also all widgets are wrapped in a mutex for all mutating methods, and their lifetimes are tracked using an Fl_Widget_Tracker, That means widgets have interior mutability as if wrapped in an Arc<Mutex<widget>>. Cloning a widget performs a memcpy of the underlying pointer and allows for interior mutability; it does not create a new widget.
SharedImages are reference-counted by FLTK and cloning an image does create a new image. All mutating methods are wrapped in locks, however images don't offer interior mutability, and would require manual wrapping with Arc<Mutex>> in the Rust side for multi-threaded applications.
This locking might lead to some performance degradation as compared to the original FLTK library, it does allow for multithreaded applications, and is necessary in an FLTK (C++) application if it also required threading.
So while FLTK widgets don't leak, this might create lifetime issues with certain widgets, namely the TextEditor and TextDisplay widgets. These 2 widgets require a TextBuffer, which might point to a file, and which might get destroyed/freed before the destruction of these widgets. As such the buffers require manual resource management if they happen to outlive the TextDisplay or TextEditor widgets, similarly, images might require explicit management if they happen to outlive the widget they were used in and are no longer needed in the program.
Overriding drawing methods will box data to be sent to the C++ library, so the data should optimally be limited to widgets or plain old data types to avoid unnecessary leaks if a custom drawn widget might be deleted during the lifetime of the program.
Deleting a widget or its parent in that callback's widget can be done safely using the safe method variants of delete() and clear(). If recursive deletion of capturing callbacks is needed, there are the unsafe variants unsafe_delete and unsafe_clear. It's preferable to use channels in this case when dealing with widgets that might be deleted since these are copy types and don't leak.
That said, fltk-rs is still in active development, and has not yet been fuzzed nor thouroughly tested for memory safety issues.
The 2 internal traits fltk-sys and fltk-derive are supposed to remain internal, and not be exposed into the public api, and are thus marked unsafe.

### Why is fltk-rs using so much unsafe code?
Interfacing with C++ or C code can't be reasoned about by the Rust compiler, so the unsafe keyword is needed.

## Contributing
Please refer to the [CONTRIBUTING](https://github.com/MoAlyousef/fltk-rs/blob/master/CONTRIBUTING.md) page for further information.
