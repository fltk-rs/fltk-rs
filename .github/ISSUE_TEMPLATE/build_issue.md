---
name: Build failure
about: Create a report to help us improve
title: "[BUG]"
labels: ''
assignees: ''

---

### Remember to search before filing a new report

## Build failures
If you're having build difficulties, please check:
- The required dependencies for your platform, mentioned in the [README](https://github.com/fltk-rs/fltk-rs#dependencies).
- The [FAQ](https://github.com/fltk-rs/fltk-rs/blob/master/FAQ.md). 
- That you have a working CMake and C++17 compiler. At least CMake should be in your PATH. 
- That your Rust's target arch is similar to your C/C++ target arch. 
- The [Setup chapter](https://fltk-rs.github.io/fltk-book/Setup.html). 

Otherwise you can open a github discussion [here](https://github.com/fltk-rs/fltk-rs/discussions) describing your difficulties, along with the output of:
- `cargo install fltk-check-env && fltk-check-env`
- `cargo build -vv`

Please only open a bug report if the repo/crate used to build and currently fails. The github-actions CI already checks for the 3 major desktop platforms. Also include the outputs of:
 - `rustup toolchain list`
 - `cargo build -vv`
 - Any CMake error and log files

Also remember if you're git cloning the fltk-rs repo, to run `git submodule update --init --recursive` if you hadn't cloned with `--recurse-submodules` since this repo uses git submodules. 

## To Reproduce
Please provide the steps to reproduce the build failure. That means if you enable any features please list them, if you also use other dependencies, it would be helpful to list a minimal Cargo.toml file which exhibits the build failure. If you use some environment or containerization please also specify that you do. 

## Desktop info
 - OS: [e.g. Windows 10 x64, macOS 10.15, Ubuntu 18.04 x64]
 - Version [e.g. 1.4.2]
 - Display server on linux (wayland or x11)

## Additional info
Add any other context about the problem here.

## Missing info
Please verify that with the provided info here that the build failure can be easily reproduced. Providing incomplete info just leads to the wasting of dev time, is not appreciated, and will just lead to closing or even deleting the issue, since it adds no relevant indexable info. 
