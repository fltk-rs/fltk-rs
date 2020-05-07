# Changelog

## Unreleased
### Changes

## [0.4.0] - 2020-05-07
### Changes
- Reinstated hard requirement of of Copy for sent messages.
- Change Image::new to Image::load when it involves getting an image from the filesystem.
- Make methods involved in loading file return Result.

## [0.3.12] - 2020-05-05
### Changes
- Removed hard requirement of of Copy for sent messages.
- Fix some methods taking an immutable reference to widgets

## [0.3.11] - 2020-05-03
### Changes
- Made app::awake_msg and app::thread_msg private, and are only exposed via app::channel<T>(), where T must implement Copy.
- Added async_std and tokio examples in the examples directory.
- Added CHANGELOG.md to the repository.
- Added FAQ.md to the repository.

## [0.3.10] - 2020-05-02
### Changes
- Added app::add_handler() to handle unhandled events, this takes a non-capturing closure.
- Removed deprecated description() method in FltkError.
- Added app::channel<T>() as well as app::awake_msg and app::thread_msg.
- Added clear and clear_submenu to MenuExt

## [0.3.9] - 2020-04-29
### Changes
- Completed ButtonExt methods and checks.
- Added impl methods for certain button checks (is_checked, is_toggled...etc).