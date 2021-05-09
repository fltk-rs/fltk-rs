## Contributing

Contributions are very welcome! Even if just for submitting bug fixes, improving the documentation, adding tests and/or examples.

### Contributing to the wrapper
The wrapper itself now lives in the cfltk [repo](https://github.com/MoAlyousef/cfltk) as a git submodule in the fltk-sys directory, it uses C89 for the headers (in the cfltk/include directory) and C++11 for the source files (in the cfltk/src directory). 
Bindgen is used on the header files using the bind.sh script. It's not added as dependency to this project since it depends on libclang and 
llvm and which add to the build complexity. As such, code added into the header files should be crossplatform and not depend on system pecularities or system headers (ifdefs are acceptable).  
C89 was chosen for the headers since bindgen works best with them. 
It's possible to contribute by directly modifying the .rs files in the fltk-sys directory. Or just running the bind.sh or the bind.sh line corresponding the header unit being modified, and all headers depending on it (for example cfl_window.h includes cfl_group.h).
For the C/C++ code, the supplied clang-format formatting is used.
FLTK's source code lives within cfltk/fltk also as a git submodule. The intention of the wrapper is to only expose the non-deprecated gui parts of the public api which lives in the fltk/FL directory. The reasoning is that the non-gui parts can be replaced by crates from the Rust ecosystem, and the deprecated parts aren't really needed since no Rust software depends on them. Also no modifications should be done there unless it's git pulling the latest master branch from the FLTK repo. 

### Contributing to the bindings
FLTK itself is quite conservative when using features, it does so to get the high portability its known for. As such, similarly the Rust code shouldn't use nightly or unstable features. The current MSRV (Minimum Supported Rust Version) is 1.39 (bound to change when necessary). Avoid pulling other dependencies except when necessary. The dependencies themselves need the same MSRV or lower and be crossplatform, or guarded behind conditional compilation config.
Doc comments would also be appreciated, the rustc warning for missing_docs is enabled by default. rustfmt is used for formatting. Pull requests automatically go to the github workflow in which 
the crate is built on the major desktop platforms, along with all the examples.
The bindings live in the fltk and fltk-derive directories. The fltk-derive provides derive macros which are intended to be internal to the fltk project. FLTK depends on inheritance to represent relationships between widgets, and these are easier done with derive macros in Rust, thus it's used to implement all widget trait methods. They also reduce repetition in code. 
The fltk directory contains a prelude which has all the traits exposed by the crate, as well as the error types. Widgets are grouped by their inheritance relations, the outliers being the utils (for utility functions) and misc modules. The misc module contain widgets which are either composite (containing multiple) widgets or widgets directly inheriting from Fl_Widget. 

### Contributing examples 
Each example should be in a single file, with no dependencies to other than this crate. This allows people to just copy them verbatim and have the examples run, it also avoids adding unnecessary dev-dependencies to the project which might increase build times. If you would like to contribute more complex examples, I invite you to contribute them to the fltk-rs/demos [repo](https://github.com/fltk-rs/demos). Also adding your project or examples/demos using fltk as a dependency or dev-dependency to the Project Showcase issue tracker is also very appreciated. 

### Major API changes
Please open an issue or a discussion for such changes before going ahead and doing them. It allows for discussion regarding the proposed changes. 
This crate tries to stay as close as possible to the FLTK api, which offers familiarity to people having used FLTK, 
and allows easier referral to the official documentation.

### Typical workflow
Given the above info, a normal workflow to add a method to a certain button widget for example would entail adding the method wrapper to the corresponding header file cfl_button.h and an implementation in the cpp file cfl_button.cpp (both in fltk-rs/fltk-sys/cfltk). Running `$ ./fltk-sys/bind.sh` or just the corresponding line to the header file `$ bindgen --use-core --ctypes-prefix libc fltk-sys/cfltk/include/cfl_button.h -o fltk-sys/src/button.rs`. If the method is part of the Fl_Button (which corresponds to the the interface implemented by all button widgets), the method is added to the ButtonExt trait, with the implementation added to the fltk-derive/src/button.rs impl_button_trait derive macro. Otherwise it's added directly in the fltk/src/button.rs in the impl of the corresponding button widget. If the method requires special handling or could benefit from an example, add an example to the fltk/examples directory or to the doc comment, showing a minimal example of using that method.
Running `cargo test` should test the added example whether it was added to the examples directory or the doc comment. 
