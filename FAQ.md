# FAQ

## Build issues

### Why does the build fails when I follow one of the tutorials?
The first tutorial uses the fltk-bundled feature flag, which is only supported for certain platforms since these are built using the Github Actions CI, namely:
- Windows 10 x64 (msvc and gnu).
- MacOS 10.15 x64.
- Ubuntu 18.04 or later, x64.

If you're not running one of the aforementioned platforms, you'll have to remove the fltk-bundled feature flag in your Cargo.toml file:
```toml
[dependencies]
fltk = "^1"
```
Furthermore, the fltk-bundled flag assumes you have curl and tar installed (for Windows, they're available in the Native Tools Command Prompt).

### Build fails on windows, why can't CMake find my toolchain?
If you're building using the MSVC toolchain, make sure you run you're build (at least your initial build) using the Native Tools Command Prompt, which should appear once you start typing "native" in the start menu, choose the version corresponding to your installed Rust toolchain (x86 or x64). The Native Tools Command Prompt has all the environment variables set correctly for native development.

If you're building for the GNU toolchain, make sure that Make is also installed, which usually comes installed in MSYS2 and Cygwin.

### Build fails on MacOS 11 with an Apple M1 chip, what can I do?
If you're getting "file too small to be an archive" error, you might be hitting these issues: https://github.com/rust-lang/cargo/issues/8875, https://github.com/rust-lang/rust/issues/50220. MacOS's native C/C++ toolchain shouldn't have this issue, and can be installed by running `xcode-select --install` or by installing XCode. Make sure the corresponding Rust toolchain (aarch64-apple-darwin) is installed as well. You can uninstall other Rust apple-darwin toolchains or use cargo-lipo instead if you need universal/fat binaries.

### Why do I get a Link error while using the mingw toolchain on windows?
If the linking fails because of this issue: https://github.com/rust-lang/rust/issues/47048 with older toolchains, it should work by using the fltk-shared feature (an issue with older compilers). Which would also generate a dynamic library which would need to be deployed with your application.
```toml
[dependencies]
fltk = { version = "^1", features = ["fltk-shared"] }
```

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
You can use the "use-ninja" feature flag if you have ninja installed. Or you can set the NUM_JOBS environment variable, which the cmake crate picks up and tries to parallelize the build.

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

### Can I cross-compile my application to a mobile platform or WASM?
FLTK currently doesn't support WASM nor mobile platforms. It is focused on desktop applications.

## Licensing

### Can I use this crate in a commercial application?
Yes. This crate has an MIT license which requires acknowledgment. FLTK (the C++ library) is licensed under the LGPL license with an exception allowing static linking for commercial/closed-source use. You can find the full terms of both licenses here:
- https://github.com/fltk/fltk/blob/master/COPYING
- https://github.com/fltk-rs/fltk-rs/blob/master/LICENSE

## Alignment

### Why can't I align input or output text to the right?
FLTK has some known issues with text alignment.

## Concurrency

### Do you plan on supporting multithreading or async/await?
FLTK supports multithreaded and concurrent applications. See the examples directory for examples on usage with threads, messages, async_std and tokio.

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

### My app crashes on certain linux distros when I use a NativeFileDialog, how can I fix it?
FLTK vendors libpng by default, sometimes version mismatches can cause problems if the Gtk native dialog expects a different version. In that case, you can use the feature "system-libpng" which would link to the distro's libpng.

## Memory and unsafety

### How memory-safe is fltk-rs?
The callback mechanism consists of a closure as a void pointer with a shim which dereferences the void pointer into a function pointer and calls the function. This is technically undefined behavior, however most implementations permit it and it's the method used by most wrappers to handle callbacks across FFI boundaries.
As stated before, panics accross FFI boundaries are undefined behavior, as such, the C++ wrapper never throws. Furthermore, all panics which might arise in callbacks are caught on the Rust side using catch_unwind.

FLTK manages it's own memory. Any widget is automatically owned by a parent which does the book-keeping as well and deletion, this is the enclosing widget implementing GroupExt such as windws etc. This is done in the C++ FLTK library itself. Any constructed widget calls the current() method which detects the enclosing group widget, and calls its add() method rending ownership to the group widget. Upon destruction of the group widget, all owned widgets are freed. Also all widgets are wrapped in a mutex for all mutating methods, and their lifetimes are tracked using an Fl_Widget_Tracker, That means widgets have interior mutability as if wrapped in an Arc<Mutex<widget>> and have a tracking pointer to detect deletion. Cloning a widget performs a memcpy of the underlying pointer and allows for interior mutability; it does not create a new widget.
Images are reference-counted. All mutating methods are wrapped in locks.
This locking might lead to some performance degradation as compared to the original FLTK library, it does allow for multithreaded applications, and is necessary in an FLTK (C++) application if it also required threading.

Overriding drawing methods will box data to be sent to the C++ library, so the data should optimally be limited to widgets or plain old data types to avoid unnecessary leaks if a custom drawn widget might be deleted during the lifetime of the program.

That said, fltk-rs is still in active development, and has not yet been fuzzed nor thouroughly tested for memory safety issues.

### Why is fltk-rs using so much unsafe code?
Interfacing with C++ or C code can't be reasoned about by the Rust compiler, so the unsafe keyword is needed.

### Is fltk-rs panic/exception-safe?
FLTK (C++) doesn't throw exceptions, neither do the C wrapper (cfltk) nor the fltk-sys crate. The higher level fltk crate, which wraps fltk-sys, is not exception-safe since it uses asserts internally after various operations to ensure memory-safety. An example is a widget constructor which checks that the returned pointer (from the C++ side) is not null from allocation failure. It also asserts all widget reads/writes are happening on valid (not deleted) widgets.
Also any function sending a string across FFI is checked for interal null bytes. For such functions, the developer can perform a sanity check on passed strings to make sure they're valid UTF-8 strings, or check that a widget was not deleted prior to accessing a widget. That said, all functions passed as callbacks to be handled by the C++ side are exception-safe.

## Contributing
Please refer to the [CONTRIBUTING](https://github.com/fltk-rs/fltk-rs/blob/master/CONTRIBUTING.md) page for further information.
