# FAQ

## Build issues

### Build fails on windows, why can't CMake find my toolchain?
If you're building using the MSVC toolchain, make sure you run you're build (at least your initial build) using the Native Tools Command Prompt, which should appear once you start typing "native" in the start menu, choose the version corresponding to your installed Rust toolchain (x86 or x64). The Native Tools Command Prompt has all the environment variables set correctly for native development.

If you're building for the GNU toolchain, make sure that Make is also installed, which usually comes installed in MSYS2 and Cygwin.

### Why do I get a Link error while using the mingw toolchain on windows?
If the linking fails because of this issue: https://github.com/rust-lang/rust/issues/47048, it should work by using the fltk-shared feature. Which would also generate a dynamic library which would need to be deployed with your application.
```toml
[dependencies]
fltk = { version = "^0.4", features = ["fltk-shared"] }
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

## Memory and unsafety

### How memory safe is fltk-rs?
FLTK manages it's own memory. Any widget is automatically owned by a parent, which is the enclosing widget implementing GroupExt such as windws etc. This is done in the C++ FLTK library itself. Any constructed widget calls the current() method which detects the enclosing group widget, and calls its add() method rending ownership to the group widget. Upon destruction of the group widget, all owned widgets are freed. So while FLTK widgets don't leak, this might create lifetime issues with certain widgets, namely the TextEditor and TextDisplay widgets. These 2 widgets require a TextBuffer which might get destroyed/freed before the destruction of these widgets. So the crate's approach is to tend to naive use. That means TextBuffer currently leaks, this avoids memory unsafety issues and segfaults in favor of a memory leak. It does however offer an unsafe delete() method for manual memory management if necessary. 
That said, fltk-rs is still in active development, and has not yet been fuzzed nor thouroughly tested for memory safety issues.

### Why is fltk-rs using so much unsafe code?
Interfacing with C++ or C code can't be reasoned about by the Rust compiler, so the unsafe keyword is needed.

## Contributing

### How can I contribute?
Contributions are very welcome! Even if just for submitting bug fixes, improving the documentation, adding tests and/or examples.
The wrapper itself which can be found in the fltk-sys directory, uses C89 for the headers and C++11 for the source files. Bindgen is used on the header files using bind.sh script. It's not added as dependency to this project since it depends on libclang and llvm which has some build issues on windows last time I tried it. C89 was chosen for the headers since bindgen works best with them. It's possible to contribute by directly modifying the .rs files in the fltk-sys directory. For the C/C++ code, the default clang-format formatting is used. For Rust, rustfmt is used. The Rust code shouldn't use nightly/unstable features. Avoid pulling in heavy or unnecessary dependencies. Doc comments would also be appreciated.

### I disagree with a current api, how can I change it?
If you would like to change the api in some way, I propose opening an issue first so that it can be discussed before putting a large amount of work on it.
