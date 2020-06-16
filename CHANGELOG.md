# Changelog

## Unreleased
### Changes
- RgbImage takes ownership of the underlying data.
- Fix linkage of gl_start and gl_finish.

## [0.6.5] - 2020-06-16
### Changes
- Fix cmake invocation on Windows to build for release.
- Add most ImageExt methods.
- Change signature of draw::write_<image>_file to accept ImageExt trait.
- SharedImage::from_image() can't take ownership of the image.
- Add BMP, Jpeg and PNG image conversion methods.
- Update FLTK to commit 46b8968 which fixes Fl_SVG_Image to window icon issue.
- Add damage related methods to WidgetExt.
- Add gl_start and gl_finish to enable gl drawing within widgets.

## [0.6.4] - 2020-06-15
### Changes
- Rename SharedImage::from_rgb to from_image.
- Add count method to ImageExt.
- Modify visibility of internal image methods.
- ImageExt::to_rgb_data marked as unsafe.
- Added ImageExt::to_rgb_image.
- Update FLTK to commit b831848.
- Add assert for WindowExt::set_icon() supported image types.

## [0.6.3] - 2020-06-13
### Changes
- Fix memory leak with set_image and set_icon methods.
- Mark Image::into<ImageExt>() as unsafe.

## [0.6.2] - 2020-06-12
### Changes
- Mark Widget::into<WidgetExt>() as unsafe.
- Add App::windows() method which returs an optional Vector of the application's windows.
- Widget deletion calls app::delete_widget instead of raw delete.
- Add add_emit and insert_emit to MenuExt, also emit to MenuItem.
- Add support for systems without OpenGL using the no-opengl feature flag.

## [0.6.1] - 2020-06-10
### Changes
- Initialize internal FLTK styles on application start.
- Fix use of uninitialized data.
- Remove deprecate set_style_table_entry method, which was replaced by set_highlight_data.
- Impl Copy for the TextBuffer.
- Add WindowExt border and set_border methods.
- Add is_radio and is_checkbox methods to MenuItem.
- Remove WidgetType trait from LabelType and FrameType.

## [0.5.12] - 2020-06-08
### Changes
- Add TextBuffer::unset_buffer() method.
- Deprecated DisplayExt::set_style_table_entry, use set_highlight_data instead.

## [0.5.11] - 2020-06-04
### Changes
- DisplayExt::buffer just returns a manually managed buffer.
- Automatically export fltk::prelude and fltk::enums into the fltk namespace.

## [0.5.10] - 2020-06-02
### Changes
- Fixed typos in method names.
- Fixed use-after-free of TextDisplay StyleTableEntry.
- Enable manual memory management of the style table entry when necessary.
- Add clean way to quit application using App::quit().
- Revert the DisplayExt buffer method to returning a reference.

## [0.5.9] - 2020-06-02 -- YANKED
### Changes
- Fixed typos in method names.
- Fixed use-after-free of TextDisplay StyleTableEntry.
- Make TextBuffer semantics clearer using ManuallyDrop for all methods taking and returning a TextBuffer.
- Enable manual memory management of the style table entry when necessary.
- Add clean way to quit application using App::quit().
- Revert the DisplayExt buffer method to returning a reference.

## [0.5.8] - 2020-06-02 -- YANKED
### Changes
- Fixed typos in method names.
- Fixed use-after-free of TextDisplay StyleTableEntry.
- Make TextBuffer semantics clearer using ManuallyDrop for all methods taking and returning a TextBuffer.
- Enable manual memory management of the style table entry when necessary.
- Add clean way to quit application using App::quit().

## [0.5.7] - 2020-06-01
### Changes
- Add support for windows-gnu fltk-bundled builds.
- Update to latest FLTK.
- Add positions to common dialogs which are now supported by FLTK.

## [0.5.6] - 2020-05-31
### Changes
- Complete methods for the SimpleTerminal struct.
- Add proper cleanup for draw callbacks

## [0.5.5] - 2020-05-30
### Changes
- Add checks for menu clear and clear_submenu operations.
- Add checks for tree clear operations.
- Add MenuExt::remove.
- Important fix to menu item set_label method.

## [0.5.4] - 2020-05-30
### Changes
- Add more checks for widget deletion.
- Add cleanup after menu deletion.
- Remove redundant delete_widget function.
- Add method to iterate menus.
- Add a safe variant to delete and unset callbacks.
- Calls to widget::clear will automatically delete allocated user_data.
- Mark clear methods not acting on FLTK widgets as unsafe.

## [0.5.3] - 2020-05-29
### Changes
- Add a method to unset callbacks.
- Fix double free after acquiring user_data.
- Add a tracker pointer to all widgets.
- Check for use after widget deletion for all operations on widgets.

## [0.5.2] - 2020-05-28
### Changes
- Add Fl_Scroll methods.
- Mark GroupExt clear method as unsafe since it could invalidate underlying widgets.
- Add WidgetExt unsafe methods to acquire user_data and manually delete widgets.
- Add WidgetExt focus methods.

## [0.5.1] - 2020-05-26
### Changes
- Added several methods to MenuExt and Pack.
- Relax callback restrictions.
- Give some more descriptive names for draw methods.
- Revert back to C++11 for the wrapper.
- Allow for custom widget creation, along with example.

## [0.5.0] - 2020-05-26
### Changes
- Image constructors return a Result instead of an Option for all ops.
- Impl Clone for widgets, since they have interior mutability (basically Arc<Mutex<widget>> on the C++ side).
- Systematically return Error when FLTK (C++) returns Error code.
- Add emit method for widgets which take a app::Sender and a message to allow for message passing and shorted callback function. 
- Added the rest of the horizontal valuator widgets.
- Methods return raw pointers are marked as unsafe.
- Impl Error for FltkError.

## [0.4.13] - 2020-05-25
### Changes
- Add support for bundled fltk via the fltk-bundled feature flag.
- Several fixes to Offscreen and GlWindow methods.

## [0.4.12] - 2020-05-24
### Changes
- Translate gl functions exposed via fltk.
- Add mode and set_mode to GlWindow.
- Prefer platfrom gl header.

## [0.4.11] - 2020-05-23
### Changes
- Prefix all draw methods with draw_.
- Add a SharedImage struct.
- Add shown() and raw_handle() methods to WindowExt trait.

## [0.4.10] - 2020-05-22
### Changes
- Remove the necessity for the gl-window feature flag, however on Linux distros, OpenGL headers must be installed.
- Add several handler methods (delete_widget, event_inside_widget, event_inside).
- Add app::delete_widget() for safe widget deletion during event handling.
- Require a C++17 compiler to deal with msvc issues.
- Fix GlWindow conditional compilation.
- Fix GLU linkage on Linux.
- Fix memory leaks from CString.
- Remove unintentional debug message.

## [0.4.9] - 2020-05-21 (YANKED)
### Changes
- Remove the necessity for the gl-window feature flag, however on Linux distros, OpenGL headers must be installed.
- Add several handler methods (delete_widget, event_inside_widget, event_inside).
- Add app::delete_widget() for safe widget deletion during event handling.
- Require a C++17 compiler to deal with msvc issues.
- Fix GlWindow conditional compilation.
- Fix GLU linkage on Linux.

## [0.4.8] - 2020-05-20 (YANKED)
### Changes
- Add several handler methods (delete_widget, event_inside_widget, event_inside).
- Add app::delete_widget() for safe widget deletion during event handling.
- Require a C++17 compiler to deal with msvc issues.
- Fix GlWindow conditional compilation.
- Fix GLU linkage on Linux.

## [0.4.7] - 2020-05-20
### Changes
- Add GlWindow under gl-window feature flag.
- Add draw::show_colormap.
- Add draw methods.
- Add set_cursor method for window types.

## [0.4.6] - 2020-05-20
### Changes
- Add get_items() method to Tree widget.
- Add support for loading images from data/memory using the from_data() method.
- impl Send and Sync for TreeItem and MenuItem.
- Renamed ImageExt to_bytes to to_rgb which is more approprate.
- Added more button and valuator widgets.
- Add scale method to images.

## [0.4.5] - 2020-05-19
### Changes
- Require Rust 1.38 or higher.
- Use type hashing for channels.
- Add support for Trees and TreeItems.
- Add feature to support legacy OpenGL.

## [0.4.4] - 2020-05-15
### Changes
- Added wrapper for program_should_quit signal.
- Free user data after widget destruction.
- Added beeps.
- Require channel messages to also be Send and Sync.

## [0.4.3] - 2020-05-13
### Changes
- Mark internal traits as unsafe.
- Fix operator widget*().
- Add excludes to Cargo.toml.

## [0.4.2] - 2020-05-10
### Changes
- Added timeout callback functions to the app module.
- Added features to use system libjpeg, libpng and zlib instead of bundled ones.
- Added initial support for Drag and Drop operations

## [0.4.1] - 2020-05-07
### Changes
- Fix documentation

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
