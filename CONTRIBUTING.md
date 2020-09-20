## Contributing

Contributions are very welcome! Even if just for submitting bug fixes, improving the documentation, adding tests and/or examples.

### Contributing to the wrapper
The wrapper itself which can be found in the fltk-sys directory, it uses C89 for the headers and C++11 for the source files. 
Bindgen is used on the header files using bind.sh script. It's not added as dependency to this project since it depends on libclang and 
llvm which has some build issues on windows last time I tried it. 
C89 was chosen for the headers since bindgen works best with them. 
It's possible to contribute by directly modifying the .rs files in the fltk-sys directory. 
For the C/C++ code, the supplied clang-format formatting is used. 
Building the wrapper applies a patch to FLTK which disables building fluid and deletes the VERSION file which might throw-off the clang compiler on some platforms. Prior to creating a pull request, discard all changes done to the FLTK submodule.

### Contributing to the bindings
The Rust code shouldn't use nightly or unstable features. Avoid pulling other dependencies. 
Doc comments would also be appreciated. rustfmt is used for formatting. Pull requests automatically go to the github workflow in which 
the crate is built on the major desktop platforms, along with all the examples.

### Contributing examples
Each example should be in a single file, with no dependencies to other than this crate. This allows each example to be standalone.

### Major API changes
Please open an issue for such changes before going ahead and doing them. It allows for discussion regarding the proposed changes. 
This crate ties to stay as close as possible to the FLTK api, which offers familiarity to people having used FLTK, 
and allows easier referral to the official documentation.
