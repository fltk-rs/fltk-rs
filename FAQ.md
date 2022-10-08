# FAQ

## Build issues

### Why does the build fails when I follow one of the tutorials?
The first tutorial uses the fltk-bundled feature flag, which is only supported for certain platforms since these are built using the Github Actions CI, namely:
- Windows 10 x64 (msvc and gnu).
- MacOS 12 x64 and aarch64.
- Ubuntu 20.04 or later, x64 and aarch64.

If you're not running one of the aforementioned platforms, you'll have to remove the fltk-bundled feature flag in your Cargo.toml file:
```toml
[dependencies]
fltk = "^1.3"
```
Furthermore, the fltk-bundled flag assumes you have curl and tar installed (for Windows, they're available in the Native Tools Command Prompt).

### Build fails on windows, why can't CMake find my toolchain?
If you're building using the MSVC toolchain, make sure you run your build (at least your initial build) using the Native Tools Command Prompt, which should appear once you start typing "native" in the start menu, choose the version corresponding to your installed Rust toolchain (x86 or x64). The Native Tools Command Prompt has all the environment variables set correctly for native development. [cmake-rs](https://github.com/alexcrichton/cmake-rs) which the bindings use might not be able to find the Visual Studio 2022 generator, in which case, you can try to use the fltk-bundled feature, or use ninja via the use-ninja feature. This requires installing [Ninja](https://github.com/ninja-build/ninja/wiki/Pre-built-Ninja-packages) which can be installed with Chocolatey, Scoop or manually.

If you're building for the GNU toolchain, make sure that Make is also installed, which usually comes installed in mingw64 toolchain.


### Build fails on MacOS 11 with an Apple M1 chip, what can I do?
If you're getting "file too small to be an archive" error, you might be hitting this [issues](https://github.com/rust-lang/cargo/issues/8875) or this [issue](https://github.com/rust-lang/rust/issues/50220). MacOS's native C/C++ toolchain shouldn't have this issue, and can be installed by running `xcode-select --install` or by installing XCode. Make sure the corresponding Rust toolchain (aarch64-apple-darwin) is installed as well. You can uninstall other Rust apple-darwin toolchains or use cargo-lipo instead if you need universal/fat binaries.

### Why do I get a Link error while using the mingw toolchain on windows?
If the linking fails because of this [issue](https://github.com/rust-lang/rust/issues/47048) with older toolchains, it should work by using the fltk-shared feature (an issue with older compilers). Which would also generate a dynamic library which would need to be deployed with your application.
```toml
[dependencies]
fltk = { version = "^1.3", features = ["fltk-shared"] }
```

### Why does my msys2 mingw built fltk app using, fltk-bundled, isn't self-contained and requires several dlls?
If you have installed libgdiplus via pacman, it would require those dependencies on other systems. If you're using the windows sdk-provided libgdiplus, it shouldn't require extra dlls. You can either uninstall libgdiplus that was installed via pacman, or or you can build using the feature flag: `no-gdiplus`.

### Why do I get link errors when I use the system-fltk feature?
This crate targets FLTK 1.4, while currently most distros distribute an older version of FLTK (1.3.5). You can try to install FLTK (C++) by building from source.

### Build fails on Arch linux because of pango or cairo?
Pango changed its include paths which caused build failures across many projects. There are 2 solutions:
- Use the no-pango feature. Downsides: loss of rtl and cjk language support.
- Set the CFLAGS and CXXFLAGS to correct the global include paths.
```
$ export CFLAGS="-isystem /usr/include/harfbuzz -isystem /usr/include/cairo"
$ export CXXFLAGS="-isystem /usr/include/harfbuzz -isystem /usr/include/cairo"
```

### How do I force CMake to use a certain C++ compiler?
FLTK works with all 3 major compilers. If you would like to change the C++ compiler that's chosen by default by CMake, you can change the CXX environment variable before running the build:
```
$ export CXX=/usr/bin/clang++
$ cargo run
```
CMake caches the C++ compiler variable after it's first run, so if the above failed because of a previous run, you would have to run ```cargo clean``` or you can manually delete the CMakeCache.txt file in the build directory.

### Can I accelerate the build speed?
You can use the "use-ninja" feature flag if you have ninja installed. 

### Can I cache a previous build of the FLTK library?
You can use the fltk-bundled feature and use either the CFLTK_BUNDLE_DIR or CFLTK_BUNDLE_URL to point to the location of your cached cfltk and fltk libraries.

## Deployment

### How do I deploy my application?
Rust, by default, statically links your application. FLTK is built also for static linking. That means that the resulting executable can be directly deployed without the need to deploy other files along with it. If you want to create a WIN32 application, Mac OS Bundle or Linux AppImage, please check the question just below!

### Why do I get a console window whenever I start my GUI app?
This is the default behavior of the toolchain, and is helpful for debugging purposes. It can be turned off easily by adding ```#![windows_subsystem = "windows"]``` at the beginning of your main.rs file if you're on windows. 
If you would like to keep the console window on debug builds, but not on release builds, you can use ```#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]``` instead.

For Mac OS and Linux, this is done by a post-build process to create a Mac OS Bundle or Linux AppImage respectively.

See [cargo-bundle](https://github.com/burtonageo/cargo-bundle) for an automated tool for creating Mac OS app bundles. 

See [here](https://docs.appimage.org/packaging-guide/overview.html#converting-existing-binary-packages) for directions on creating an AppImage for Linux.

### Why is the size of my resulting executable larger than I had expected?
FLTK is known for it's small applications. Make sure you're building in release, and make sure symbols are stripped using the strip command in Unix-like systems. On Windows it's unnecessary since symbols would end up in the pdb file (which shouldn't be deployed).

If you need an even smaller size, try using opt-level="z":
```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
```

Newer versions of cargo (>1.46) support automatically stripping binaries in the post-build phase:
```toml
cargo-features = ["strip"]

[profile.release]
strip = true
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
```

Furthermore, you can build Rust's stdlib optimized for size (it comes optimized for speed by default). More info on that [here](https://github.com/johnthagen/min-sized-rust)

### Can I cross-compile my application to a mobile platform or WASM?
FLTK currently doesn't support WASM nor iOS. It has experimental support for Android (YMMV). It is focused on desktop applications.

## Licensing

### Can I use this crate in a commercial application?
Yes. This crate has an MIT license which requires acknowledgment. FLTK (the C++ library) is licensed under the LGPL license with an exception allowing static linking for commercial/closed-source use. You can find the full terms of both licenses here:
- [COPYING](https://github.com/fltk/fltk/blob/master/COPYING)
- [LICENSE](https://github.com/fltk-rs/fltk-rs/blob/master/LICENSE)

## Alignment

### Why can't I align input or output text to the right?
FLTK has some known issues with text alignment.

## Concurrency

### Do you plan on supporting multithreading or async/await?
FLTK supports multithreaded and concurrent applications. See the examples dir and the [fltk-rs demos repo](https://github.com/fltk-rs/demos) for examples on usage with threads, messages, async_std and tokio (web-todo examples).

### Should I explicitly call app::lock() and app::unlock()?
fltk-rs surrounds all mutating calls to widgets with a lock on the C++ wrapper side. Normally you wouldn't have to call app::lock() and app::unlock(). 
This depends however on the support of recursive mutexes in your system. 
If you notice haning in multithreaded applications, you might have to initialize threads (like xlib threads) by calling app::lock() once in your main thread. 
In that case, you can wrap widgets in an Arc<Mutex> or surround widget-mutating functions/methods with an app::lock and app::unlock. 
But that should rarely be required.

## Windowing

### Why does FLTK exit when I hit the escape key?
This is the default behavior in FLTK. You can easily override it by setting a callback for your main window:
```rust
    wind.set_callback(|_| {
        if fltk::app::event() == fltk::enums::Event::Close {
            app::quit(); // Which would close using the close button. You can also assign other keys to close the application
        }
    });
```

## Panics/Crashes

### My app panics when I try to handle events, how can I fix it?
This is due to a debug_assert which checks that the involved widget and the window are capable of handling events. Although most events would be handled correctly, some events require that the aforementioned conditions be met. Thus it is advisable to place your event handling code after the main drawing is done, i.e after calling your main window's show() method. Another point is that event handling and drawing should be done in the main thread. Panics accross FFI boundaries are undefined behavior, as such, the wrapper never throws. Furthermore, all panics which might arise in callbacks are caught on the Rust side using catch_unwind.

## Memory and unsafety

### How memory-safe is fltk-rs?
The callback mechanism consists of a closure as a void pointer with a shim which dereferences the void pointer into a function pointer and calls the function. This is technically undefined behavior, however most implementations permit it and it's the method used by most wrappers to handle callbacks across FFI boundaries. [link](https://rust-lang.github.io/unsafe-code-guidelines/layout/function-pointers.html#representation)

As stated before, panics accross FFI boundaries are undefined behavior, as such, the C++ wrapper never throws. Furthermore, all panics which might arise in callbacks are caught on the Rust side using catch_unwind.

FLTK manages it's own memory. Any widget is automatically owned by a parent which does the book-keeping as well and deletion, this is the enclosing widget implementing GroupExt such as windws etc. This is done in the C++ FLTK library itself. Any constructed widget calls the current() method which detects the enclosing group widget, and calls its add() method rending ownership to the group widget. Upon destruction of the group widget, all owned widgets are freed. Also all widgets are wrapped in a mutex for all mutating methods, and their lifetimes are tracked using an Fl_Widget_Tracker, That means widgets have interior mutability as if wrapped in an Arc<Mutex<widget>> and have a tracking pointer to detect deletion. Cloning a widget performs a memcpy of the underlying pointer and allows for interior mutability; it does not create a new widget.
Images are reference-counted. All mutating methods are wrapped in locks.
This locking might lead to some performance degradation as compared to the original FLTK library, it does allow for multithreaded applications, and is necessary in an FLTK (C++) application if it also required threading.

Overriding drawing methods will box data to be sent to the C++ library, so the data should optimally be limited to widgets or plain old data types to avoid unnecessary leaks if a custom drawn widget might be deleted during the lifetime of the program.

### Can I get memory leaks with fltk-rs?
Non-parented widgets that can no longer be accessed are a memory leak. Otherwise, as mentioned in the previous section all parented widgets lifetimes' are managed by the parent.
An example of a leaking widget:
```rust
fn main() {
    let a = app::App::default();
    let mut win = window::Window::default();
    win.end();
    win.show();

    {
        button::Button::default(); // this leaks since it's not parented by the window, and has no handle in main
    }
}
```

A more subtle cause of leaks, is removing a widget from a group, then the scope ends without it being added to another group or deleted:
```rust
fn main() {
    let a = app::App::default();
    let mut win = window::Window::default();
    {
        button::Button::default(); // This doesn't leak since the parent is the window
    }
    win.end();
    win.show();

    {
        win.remove_by_index(0); // the button leaks here since it's removed and we no longer have access to it
    }
}
```

### Why is fltk-rs using so much unsafe code?
Interfacing with C++ or C code can't be reasoned about by the Rust compiler, so the unsafe keyword is needed.

### Is fltk-rs panic/exception-safe?
FLTK (C++) doesn't throw exceptions, neither do the C wrapper (cfltk) nor the fltk-sys crate. The higher level fltk crate, which wraps fltk-sys, is not exception-safe since it uses asserts internally after various operations to ensure memory-safety. An example is a widget constructor which checks that the returned pointer (from the C++ side) is not null from allocation failure. It also asserts all widget reads/writes are happening on valid (not deleted) widgets.
Also any function sending a string across FFI is checked for interal null bytes. For such functions, the developer can perform a sanity check on passed strings to make sure they're valid UTF-8 strings, or check that a widget was not deleted prior to accessing a widget. That said, all functions passed as callbacks to be handled by the C++ side are exception-safe.

### Are there any environment variables which can affect the build or behavior?
- `CFLTK_TOOLCHAIN=<path>` allows passing the path to a CMake file acting as a CMAKE_TOOLCHAIN_FILE, this allows passing extra info to cmake if needed.
- `CFLTK_WAYLAND_ONLY=<1 or 0>` allows building for wayland only without directly linking X11 libs nor relying on their headers for the build process. This only works with the `use-wayland` feature flag.
- `CFLTK_BUNDLE_DIR=<path>` allows passing a path of prebuilt cfltk and fltk static libs, useful for when a customized build of fltk is needed, or for targetting other arches when building with the `fltk-bundled` flag.
- `CFLTK_BUNDLE_URL=<url>` similar to above but allows passing a url which will directs the build script to download from the passed url.
- `FLTK_BACKEND=<x11 or wayland>` allows choosing the backend of your hybrid X11/wayland FLTK app. This only works for apps built with `use-wayland` feature flag. 

## Contributing
Please refer to the [CONTRIBUTING](https://github.com/fltk-rs/fltk-rs/blob/master/CONTRIBUTING.md) page for further information.
