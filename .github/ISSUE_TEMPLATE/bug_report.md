---
name: Bug report
about: Create a report to help us improve
title: "[BUG]"
labels: ''
assignees: ''

---

### Remember to search before filing a new report

## Describe the bug
A clear and concise description of what the bug is.

## To Reproduce
Please provide a minimal reproducible examples and the steps to reproduce the behavior.

## Expected behavior
A clear and concise description of what you expected to happen.

## Screenshots
If applicable, add screenshots to help explain your problem.

## Desktop info
 - OS: [e.g. Windows 10 x64, macOS 10.15, Ubuntu 18.04 x64]
 - Version [e.g. 0.4.2]

## Additional info
Add any other context about the problem here.

## Build failures
If you're having build difficulties, please check:
- The required dependencies for your platform, mentioned in the [README](https://github.com/fltk-rs/fltk-rs#dependencies).
- The [FAQ](https://github.com/fltk-rs/fltk-rs/blob/master/FAQ.md). 
- That you have a working CMake and C++11 compiler. At least CMake should be in your PATH. 
- That your Rust's target arch is similar to your C/C++ target arch. 
- The [Setup chapter](https://fltk-rs.github.io/fltk-book/Setup.html). 

Otherwise you can open a github discussion [here](https://github.com/fltk-rs/fltk-rs/discussions) describing your difficulties, along with the output of:
- `cargo install fltk-check-env && fltk-check-env`
- `cargo build -vv`

Please only open a bug report if the repo/crate used to build and currently fails. The github-actions CI already checks for the 3 major desktop platforms. Also include the outputs of:
 - `cargo build -vv`
 - `rustup toolchain list`
 - CMake error and log files

Also remember if you're git cloning the fltk-rs repo, to run `git submodule update --init --recursive` since this repo uses git submodules. 
