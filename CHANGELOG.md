# Changelog


### [1.3.25] - 2022-12-25
- Add `WidgetBase::from_dyn_widget() ad from_dyn_widget_ptr()`.
- Add `utils::is<WidgetExt>(w) and is_ptr_of()`.
- Add `WidgetExt::image_mut() and deimage_mut()`.
- Mark `WidgetExt::as_widget_ptr() and ImageExt::as_image_ptr()` as safe.
- Pull FLTK fixes.

### [1.3.24] - 2022-12-15
- Add image::IcoImage.
- Use Fl_Widget::bind_image and bind_deimage internally.
- Relax GlobalState callback. (Fn to FnMut)
- Add WidgetExt::as_base_widget.
- Pull FLTK fixes.

## [1.3.23] - 2022-12-02
- Fix docs.rs build!

## [1.3.22] - 2022-12-02
- Adds newly added Oxy scheme.
- Add MenuExt::item_pathname.
- Add draw_rounded_rect and draw_rounded_rectf.
- Add app::WidgetId trait.
- Pull FLTK fixes.

## [1.3.21] - 2022-11-24
- Impl HasRawDisplayHandle for windows.
- Fix MenuItem::set_callback().
- Add MenuItem::set_shortcut and set_flag.
- Use bitflags for MenuFlag.

## [1.3.20] - 2022-11-13
- Add ColorChooser::set_tuple_rgb by @ConsoleC137.
- Add a grabbed() check for MenuButton::clear().
- Use FLTK's own Flex widget (newly added to FLTK).
- Update FLTK.

## [1.3.19] - 2022-11-01
- Add ColorChooser::set_rgb().
- Add check in MenuExt::text() for empty menus by @purrie.
- Add other similar checks for other MenuExt methods.
- Impl PartialEq, Eq and Default for StyleTableEntry and StyleTableEntryExt.
- Pull FLTK fixes.

## [1.3.18] - 2022-10-21
- Fix build on Windows when enabling the raw-window-handle feature.

## [1.3.17] - 2022-10-21
- Several improvements to the tree example, by @peter-scholtens. 
- Add support for raw-window-handle version 0.5 under the rwh05 feature flag.
- Add draw::draw_rbox().
- Add MenuButton::clear().
- Update README and FAQ.
- Pull FLTK fixes.

## [1.3.16] - 2022-10-05
- Default to drawing with Cairo on Linux/BSD
- Fix building with both features use-wayland and enable-glwindow while adding widgets to the glwindow.
- Pull upstream support for adding widgets to GlWindow.
- Pull upstream fixes.
- Remove once_cell.
- Update ttf-parser to version 0.17.

## [1.3.15] - 2022-09-11
- Revert breaking change with raw-window-handle's update.
- Support CFLTK_WAYLAND_ONLY env variable which allows building a wayland non-hybrid FLTK app.
- Fix some clippy lints.
- Pull FLTK's pure wayland backend.

## [1.3.14] - 2022-09-06
- impl std::default::Default for TextBuffer by @ConsoleC137.
- Avoid calling git submodule update in build script.
- Update raw-window-handle dep to 0.5.
- Pull FLTK's hybrid wayland backend.
- Pull FLTK fixes.

## [1.3.13] - 2022-08-09
- Add BrowserExt::hide_line().
- Replace lazy_static with once_cell.
- Remove assert from Offscreen::is_valid().
- Pull FLTK fixes.

## [1.3.12] - 2022-07-26
- Explicitly check darwin arch in fltk-sys' build script.
- Add index check with GroupExt::insert
- Pull upstream FLTK fixes.

## [1.3.11] - 2022-07-10
- Add WindowType::Single as alias to WindowType::Normal.
- Add Window::set_on_top() which modifies the Window's level on macOS platforms to be NSMainMenuWindowLevel + 2.
- Fix Flex::recalc to not change the current group.
- Expose Group::set_current.
- Pull FLTK fixes.

## [1.3.10] - 2022-07-01
- Fix Flex::clear.
- Fix build using musl on Alpine Linux. 
- Add app::modal().
- Revert change where app::channel::recv() requires Clone.
- Pull FLTK fixes.

## [1.3.9] - 2022-06-23
- Pull FLTK fixes.
- Fix opacity issue on wayland.

## [1.3.8] - 2022-06-06
- Add app::raw_open_callback.
- Add WindowExt::icon_label and set_icon_label.
- Pull fixes from FLTK.

## [1.3.7] - 2022-05-19
- Add app::GlobalState.
- Add DisplayExt::show_insert_position by @degaart
- Relax some DisplayExt method asserts checking for an existing buffer by @degaart.
- Pull fixes from FLTK.

## [1.3.6] - 2022-04-23
- Re-enable doc example scraping after upstream nightly rustdoc fix.

## [1.3.5] - 2022-04-17
- Add BrowserExt::data(), set_data(), add_with_data() and insert_with_data() methods.
- Fix SharedImage::from_image() refcount.

## [1.3.4] - 2022-04-10
- Pull FLTK fixes.
- Some refactoring.

## [1.3.2] - 2022-03-31
- Add Screen type as well as related screen functions and screens_info example, by @joseluis.
- Add Rect and new Coordinates type, by @joseluis.
- Add Size and Cell types.
- Add link to fltk-grid.
- Add CFLTK_BUNDLE_PREFIX_URL by @aaronps.
- Check curl and tar error codes when using fltk-bundled feature flags and properly report them, by @aaronps.

## [1.3.1] - 2022-03-22
- Pull updated Wayland backend code from FLTK.
- Fix HasRawWindowHandle implementation on Wayland.
- Improve TreeItem::move* docs.
- Deprecate dialog::choice() and choice_default() in favor of choice2() and choice2_default().
- Fix BrowserExt::set_icon() leak after calling BrowserExt::clear().

## [1.3.0] - 2022-03-06
- Pull FLTK dialog [fix](https://github.com/fltk/fltk/issues/401).
- Pull FLTK's wayland support.
- Enable building using the wayland backend, requires the use-wayland feature flag.

## [1.2.32] - 2022-02-26
- Free singleshot heap-allocated trait-object(callbacks) in app::awake_callback() and add_timeout() by @mosolovsa.
- Fix TiledImage refcount.
- Fix MenuItem::add() and insert().
- Add experimental GlWidgetWindow, gated under the enable-glwindow feature.
- Add app::set_color_with_alpha(), gated under the enable-glwindow feature.
- Update ttf-parser to 0.15.

## [1.2.31] - 2022-02-19
- Add dialog::message_icon().
- Add dialog::FileChooser size and position methods.
- Add dialog::message_set_hotspot(), message_hotspot() and set_font().
- Fix FileChooser::set_label() and set_ok_label().

## [1.2.30] - 2022-02-10
- Add MenuItem::add, add_emit, insert and insert_emit.
- Add details to channel docs by @peter-scholtens.
- Derive PartialEq for TextAttr.
- Update patch to detect Event::Enter for windows on Windows.
- Update FLTK.

## [1.2.29] - 2022-01-31
- Add TextEditor::AnyState const value.
- Add TextEditor::add_key_binding() and remove_key_binding().
- Add TextBuffer methods of secondary selection.
- Fix for unicode fonts.
- Add DisplayExt::set_highlight_data_ext() and text::StyleTableEntryExt (supports underline, strikethrough, background color...etc).
- Add setters & getters for secondary selection, spelling and grammar colors.
- Pull FLTK menu fixes.
- Update FLTK (removes experimental drivers).

## [1.2.28] - 2022-01-22
- Expose TreeItem user_data.
- Documentation fix by @AshfordN.
- Further improvement to the editor example (updates title bar with status of the file) by @peter-scholtens.
- Check image size in ImageExt::scale().
- Add debug assert checking if browser column widths contains 0.

## [1.2.27] - 2022-01-14
- Add custom impl of Clone for app::Sender & Receiver by @KillTheMule.
- Remove dependency on stb_truetype in favor of ttf-parser.
- Add Spinner::set_wrap() and wrap().
- Improve editor example, #1105 by @peter-scholtens.
- Pull FLTK InputChoice changes and other fixes.

## [1.2.26] - 2022-01-05
- Add timeout3, idle3 functions which properly handle removal of callbacks, by @AshfordN.
- Deprecate app::release() in favor of app::set_grab(None), by @AshfordN.
- Pull FLTK fixes.

## [1.2.25] - 2021-12-30
- Add WindowExt::set_override() by @AshfordN.
- Add draw::wrap_measure() by @AshfordN.
- Add TableExt::cursor2rowcol().
- Add enum table::TableResizeFlag.
- Add WidgetBase::assume_derived().
- Relax derive requirement for TableExt::scrollbar().
- Removed use of function-like macros in doc attribute which requires Rust 1.54.

## [1.2.24] - 2021-12-27
- Improve calculator examples. Thanks @peter-scholtens.
- Add TableExt::find_cell(), TableExt::try_get_selection() and try_visible_cells().
- Fix FileChooser filter docs. FileChooser also accepts Paths in addition to strings.
- Use crossbeam-channel internally to avoid conflicts with app::awake().

## [1.2.23] - 2021-12-21
- Update raw-window-handle to 0.4.
- Make Offscreen aware of single-threaded feature.
- Fix visibility of is_ui_thread().
- Update FLTK.

## [1.2.22] - 2021-12-19
- Add doc comments and example comments. Thanks @KillTheMule.
- Fix doc typos. Thanks @ctrl-81.
- Fix doc typos. Thanks @AshfordN.
- Guard against division by zero in Flex code.
- Check that certain calls are only done in the main thread.
- Add app::is_initialized().
- Use ninja only via feature flag.

## [1.2.21] - 2021-12-14
- Fix BmpImage::from_data() not passing the correct len.
- Fix GroupExt::remove_by_index() virtual call.
- Fix documentation typo. Thanks @mark-summerfield.
- Add HelpDialog::shown().
- Add Window::clear_modal_states().
- Add Window::force_position().
- Improve font documentation.
- Add hack to find a font's name based on the file name when stb fails to get the name.
- Improve radio button docs.
- Pull FLTK IME fixes.
- Deprecate Tree::set_item_clicked().
- Relax mutability requirements for several methods.

## [1.2.20] - 2021-12-09
- Cleanup macros and surrounding api. Thanks @AshfordN.
- Update doc comment on draw::set_line_style().
- Add Window xclass and default_xclass setter and getter. (Changes the XA_WM_CLASS property of the window)
- Add `Color::from_hex_str(&str)` and `to_hex_str()`.
- Optimize Color::from_rgba_tuple().
- Add draw::text_extents().
- Properly `cfltk_` prefix extern Cocoa wrapper functions.
- Fix build on 32-bit X11 systems (introduced in 1.2.11 by Window::platform_hide() and platform_show()).

## [1.2.19] - 2021-12-03
- Fix docs for dialog::message_title_default(). Thanks @hannesbraun.
- Add TreeItem::set_label_fgcolor() and label_fgcolor(), deprecate the older names. Thanks @AshfordN.
- Add TreeItem::as_ptr().
- Add Tree::item_pathname().
- Add enums::Key::F1 to F12, Key::is_fn_key() and Key::fn_key(i32).

## [1.2.18] - 2021-11-27
- Add menu::mac_set_about(). Thanks @hannesbraun.
- Add TreeItem::try_widget() and Wizard::try_current_widget().
- Add app::version_str().
- Add dialog::message_title() and message_title_default().
- Shift experimental Flow widget to its own [crate](https://github.com/MoAlyousef/fltk-flow).
- Add TreeItem::draw_item_content.
- Expose several macros that assist in implementing some traits (WidgetExt, WidgetBase, GroupExt), useful for creating custom widgets.
- Deprecate TreeItem::widget(), wizard::current_widget() and Group::current().

## [1.2.17] - 2021-11-21
- Add draw::draw_check().
- Expose trait macros.
- Link to FLTK bundled libpng on Linux.
- Add SimpleTerminal::append2.
- Build thin binaries on MacOS 11 M1 since cargo/rust still don't support universal binaries.

## [1.2.16] - 2021-11-15
- Add WidgetExt::center_x() and center_y().
- Allow scraping examples for docs.rs.
- Add Fl_Flex::remove to C wrapper.
- Expose WidgetExt::raw_user_data() and set_raw_user_data().
- Pull fixes from FLTK.

## [1.2.15] - 2021-11-11
- Fix android build (break introduced with version 1.2.11). Thanks @Ar37-rs. 
- Don't assert that window is shown with set_opacity.

## [1.2.14] - 2021-11-09
- Mark Window::platform_hide() and platform_show() as safe.
- Add WindowExt::wait_for_expose().
- Fix div by zero in Flex orphaned space.
- Flex::set_size() no longer needs a mutable widget ref.
- Flex::end() and recalc() no longer need mutable ref to self.
- Pull FLTK fixes.
- Update README with tutorial links and reference to fltk-table and fltk-evented.
- Cleanup and reorganize macro ordering to be after structs.
- Formatting and lints.

## [1.2.13] - 2021-11-01
- Unlink C++ std lib to enable cross-compiling to Windows.
- Fix draw::Offscreen::rescale().
- Update FLTK device code.

## [1.2.12] - 2021-10-30
- Fix 32bit Windows broken build in version 1.2.11, by changing ShowWindow extern calling convention.

## [1.2.11] - 2021-10-30
- Pull FLTK focus box fix.
- Fix Flex::Row set_pad().
- Add experimental Flow widget.
- Add Window::platform_show() and platform_hide() which calls the platforms show and hide routines.

## [1.2.10] - 2021-10-23
- Add utils::char_len() and content_view().
- Remove asserts in WindowExt::set_cursor_image() and just rescale if image had been modified.

## [1.2.9] - 2021-10-19
- Fix image links in crates.io.
- Adds image::RgbScaling enum.
- Adds RgbImage::scaling_algorithm() and set_scaling_algorithm().
- Add ImageExt::to_rgb_image().
- Add RgbImage::from_pixmap().
- Assert image size in WindowExt::set_cursor_image().
- Scale returned RgbImage in ImageExt::to_rgb().
- Update FLTK.

## [1.2.8] - 2021-10-11
- Fix floating point exception in Flex inner vector not properly resizing with set_size().
- Fix crates.io links by using absolute urls.

## [1.2.7] - 2021-10-07
- Add Group::try_current().
- Add Color::from_rgba_tuple().
- Constify more enums::Color and Shortcut methods.
- Constify utils functions.
- Formatting and clippy.
- Update FLTK and cfltk.

## [1.2.6] - 2021-10-05
- Move macros into fltk and remove dependency on syn & quote.
- Make raw-window-handle an optional dependency.
- Constify Key, Event and Shortcut constructors.
- Add const Key::to_char().
- Add const Color::from_rgbi().

## [1.2.5] - 2021-10-02
- Add app::font_size() getter.
- Handle None in DisplayExt::set_highlight_data().
- Alias GlutWindow as GlWindow since they are basically the same!
- Remove wrong implementation of draw::scale_x().
- Add WidgetExt::handle_event().
- Relax app::handle() event requirements, and use `Into<Event>`.
- Pull FLTK Gif fixes.

## [1.2.4] - 2021-09-29
- Pull image fixes from FLTK.
- Add app::compose, compose_state, reset_marked_text and insertion_point_location.
- Add app::copy.
- Check the size of drawn text.
- Get scaling factor on MacOS without requiring info.plist.
- Add copy and paste overloads for selection buffers.
- Add app::add_clipboard_notify() and remove_clipboard_notify().

## [1.2.3] - 2021-09-24
- Add a new group::Flex widget wrapping [Fl_Flex](https://github.com/osen/FL_Flex).
- Use fork of FL_Flex to avoid patching, then linking libstd++.
- Add Flex's margin and pad properties getter and setter.
- Overload Flex::add to recalculate layout.
- Add WidgetBase::resize_callback().
- Add WidgetBase::default_fill() to construct a widget with the size of its parent.
- Improve some docs.
- Derive Default for Column, Row and Grid widgets.
- Fix cargo doc build issue on docs.rs.

## [1.1.19] - 2021-09-18
- Fix doc comments about FileDialog and NativeFileChooser.
- Using fltk-bundled automatically enables glwindow. Note, other user-built bundles can be cached and used via the CFLTK_BUNDLE_DIR or CFLTK_BUNDLE_URL env variables.
- Remove unnecessary references to use-ninja in the README and main docs since it will be used if found.
- Update ROADMAP.

## [1.1.18] - 2021-09-15
- Open display for certain draw functions when appropriate.
- Add TextBuffer::search_forward(), search_backward(), find_char_forward() and find_char_backward().
- Add DisplayExt::unset_highlight_data() convenience method.
- Update FLTK to pull nanosvg fixes.

## [1.1.17] - 2021-09-10
- Fix image type asserts in Window::set_shape().
- Add draw::gl_start() and gl_finish(). As well as app::set_gl_visual(Mode).
- Copy shared lib when building with fltk-shared feature to the target dir.
- Update docs regarding WidgetExt::trigger(), set_trigger() and enums::CallbackTrigger explaining their provenance.

## [1.1.16] - 2021-09-04
- Fix typo in function name (my_getContentView).
- Enable building FLTK and cfltk in MinSizeRel profile when using opt-level="s" or "z".
- Add a single-threaded feature. This allows linking X11-based libs not built with threading support. As a side-effect, this disables accessing/modifying widgets from other threads, it also might improve performance. It should not be the default.
- Update fltk and syn.

## [1.1.15] - 2021-08-31
- Pull FLTK tooltip fix for issue #797.
- Fix `fn WidgetExt::callback() -> Option<Box<dyn FnMut()>>` (callback getter) to only work for FLTK types with default callbacks.
- Properly export the widget_extends macro.
- Fix undefined symbol on macos.

## [1.1.13] - 2021-08-27
- Check for ImageExt::count() when calling ImageExt::to_rgb().
- Remove unnecessary deps.
- Refactor cfltk to remove code duplication and reduce build times.

## [1.1.12] - 2021-08-21
- Update libc, cmake-rs, bitflags and syn.
- Fix fltk-shared feature on X11 systems.

## [1.1.11] - 2021-08-12
- Remove redundant macro expansion.
- Update deps.

## [1.1.10] - 2021-08-09
- Add output::OutputType.
- Add format_text example. Contributed by @mvasi90.
- Fix some clippy lints.
- Add Font::get_name().
- Add image::RgbImage::convert().
- Add ColorDepth::from_u8().
- Add a widget_extends! macro for custom widgets, which basically implements Deref/DerefMut and adds convenience constructor and anchoring methods.
- (Revert a breaking change introduced in 1.1.8)

## [1.1.7] - 2021-08-04
- Fix invalid cast in TextBuffer::selection_position().
- Update FLTK.
- Add draw::scale().
- Add app::add_system_handler and remove_system_handler.

## [1.1.6] - 2021-08-02
- Fix high cpu usage due to spurious awakes during app::wait().
- Add Color::to_rgb().
- Add frame_color() and set_frame_color().
- Add menubutton example, this should be preferable over MenuItem for right click menus.

## [1.1.5] - 2021-07-31
- Add Color::contrast, grayscale and rgb_color.
- Add doc comment that SvgFileSurface can panic on creation.
- Add WidgetExt::has_focus().

## [1.1.4] - 2021-07-23
- Add app::set_frame_type_cb() and set_frame2().
- Add Color::gray_ramp and color_average().
- Add app::draw_frame_active().
- Fix doc of draw::draw_frame and draw_frame2 and add assert.
- Update Syn. 

## [1.1.3] - 2021-07-21
- Add WidgetExt::with_type().
- Add MenuExt::menu and set_menu().
- Add ImageExt::draw_ext() which allows passing the offset to the image.
- Add app::frame_shadow_width setter and getter.
- Add app::frame_border_radius_max setter and getter.
- Add app::set_color().
- Add enums::Color::darker() and lighter().

## [1.1.2] - 2021-07-19
- Fix app::event_clicks() and add overload for event_clicks_num() for number of clicks.
- Add widget_resize() for simple resizing without class-specific resize functionality.
- Add overloads for `app::*_idle` and `app::*_timeout` which accept simple function pointers.
- Add utils::filename_expand().
- Support `::new` and `::end` calls for Column and Row widgets.
- Support `::end` calls for VGrid and HGrid widgets.
- Update libc and FLTK.

## [1.1.0] - 2021-07-10
- Largely simplify getting concrete types, ex. Scroll::scrollbar() now returns a concrete Scrollbar instead of a `Box<dyn ValuatorExt>`, and parent() returns an `Option<Group>` instead of an `Option<Box<dyn GroupExt>>`.
- Add Window::opacity() and set_opacity() to support window transparency/opacity.
- Add MenuItem::add_image() to add icons to menu items.
- MenuExt::add(), add_emit(), insert() and insert_emit() now return the index of the menu item.
- Add WidgetExt::set_image_scaled() and set_deimage_scaled().
- Add callback getter.
- Fix MSRV, turns out it's 1.46 when using if in const context!
- Add TableExt::scrollbar() and hscrollbar().

## [1.0.22] - 2021-07-05
- Add `fn WidgetExt::is_same<W: WidgetExt>(&self, other: &W)`.
- Add WidgetExt::active and active_r.
- Add app::ready, check, release, reload_scheme, menu_line_spacing and set_menu_line_spacing.
- Fix double fclose in SvgFileSurface drop impl. Thanks @DBLouis.
- Pull MouseWheel coordinate in subwindows fix from FLTK.
- impl PartialEq for several FLTK types.

## [1.0.21] - 2021-07-01
- Fix MenuItem::set_label() on Windows.
- Add into_group() method to GroupExt.
- Add MenuButton types enum MenuButtonType.
- Allow DisplayExt::set_highlight_data() to be set even in the absence of a text buffer.
- Pull fixes from FLTK and cfltk.

## [1.0.20] - 2021-06-26
- Add app::event_original_key().
- Add GroupExt::init_sizes(), bounds() and add_resizable().
- Add alias OFlatBox to OFlatFrame.
- Add feature `no-gdiplus` to disable gdi+ drawing on Windows.
- Fix dependabot config.
- Update dependencies.

## [1.0.19] - 2021-06-17
- Fixed spreadsheet example, Thanks @eadf.
- Fixed calculator example. Thanks @eadf.
- Add several clipboard event handling functions into the app module. app::event_clipboard().
- Add app::clipboard_contains().
- Add app::paste_text() and paste_image().
- Add app::event_dispatch to intercept events before they reach widgets’ handlers.
- Add app::handle_raw for forwarding events from event_dispatch.

## [1.0.18] - 2021-06-08
- Fix Font::set_font(), FLTK expects a long lived string.

## [1.0.17] - 2021-06-07
- Update app::set_callback to reflect WidgetExt::set_callback.
- Add app::swap_frame_type() to swap the default frame type with a new one.
- Add app::frame_type() to get the default frame type.
- Add enums::FrameType::swap_frames() to swap frame types.
- Free WindowExt::raw_handle() memory on error.
- Use full font name instead of family name when handling external truetype fonts.
- Add convenience functions to enums::Font for loading and replacing fonts.

## [1.0.16] - 2021-06-02
- Update to latest FLTK with gdi+ support on Windows (for anti-aliased oblique lines and curves).
- Fix GroupExt::clear().
- Add `impl From<i32> for Event`.
- Use bitflags for Event.
- Impl fmt::Display for Event.

## [1.0.14] - 2021-05-23
- Update FLTK and cfltk.
- Link to system libpng on linux to avoid conflicting with FLTK's bundled libpng.
- Add pixels_per_unit() and pixel_h pixel_w for SingleWindow and DoubleWindow.
- Add WindowExt::screen_num() and set_screen_num().

## [1.0.13] - 2021-05-20
- Fix link in main docs.
- Relax FileChooser::shown() to accept immutable borrow.
- Update fltk-derive.

## [1.0.12] - 2021-05-11
- Pass FileChooser title as long-lived string.
- Fix GroupExt::clear to account for empty groups.

## [1.0.11] - 2021-05-11
- Add open_display call before surface::ImageSurface::new.
- Update FAQ to use 1.0 code . Thanks @sportfloh
- Add GroupExt::unsafe_clear for faster clearing of group widgets.
- Update FLTK & cfltk.

## [1.0.9] - 2021-05-09
- Add RgbImage constructors which take line data as param.
- Add draw::draw_image2 which takes line data as param.
- Update FLTK & syn.

## [1.0.8] - 2021-05-03
- Transfer repo and update LICENSE files.
- Fix links to releases and demos.

## [1.0.7] - 2021-05-02
- Add missing Shortcut/EventState values.
- Add missing screen functions.
- Add SvgImage::normalize().
- Re-revert WindowExt::set_icon().

## [1.0.6] - 2021-05-01
- Fix WindowExt::set_icon potential leak.
- Update FLTK.

## [1.0.5] - 2021-04-28
- Revert WindowExt::set_icon to previous behavior.
- Add null checks to new image code.

## [1.0.4] - 2021-04-28 (Yanked)
- Add Meta, Command & Control Shortcuts (helpful for working with MacOS). #640. Thanks @hannesbraun
- Fix WidgetExt::set_image() memory leak when called with an already set image.
- Update deps: Syn, libc and FLTK.

## [1.0.3] - 2021-04-27 (Yanked)
- Grab correct image for correct drops.
- Fix `enable-glwindow` builds.

## [1.0.2] - 2021-04-26
- Reorganize build script.
- Remove redundant functions in the bindings.
- Update FLTK, cfltk, syn.
- Base on cfltk main branch.

## [1.0.1] - 2021-04-18
- Add MenuItem::children().
- Relax mutability requirement for MenuItem::next() and at().
- Add check on WidgetExt::label().

## [1.0.0] - 2021-04-18
- [BREAKING] Importing fltk::* no longer auto-imports the prelude nor enums modules.
- [BREAKING] Importing widgets no longer auto-imports the prelude nor enums modules.
- [BREAKING] Replace set_callback, handle, draw, draw_cell with their overloads.
- [BREAKING] Widgets take a `&'static str` for a label when initialized. To use dynamic labels, use set_label or with_label.
- [BREAKING] Replace TableExt::visible_cells and get_selection with their easier overloads.
- [BREAKING] Rename InputChoice::set_value2 to set_value_index.
- [BREAKING] Take i32 where FLTK expects i32.
- [BREAKING] Rename WidgetType::to_int() to to_i32().
- [BREAKING] app::event_dx() and event_dy() return an app::MouseWheel instead of i32.
- [BREAKING] enums::Mouse moved to app::MouseButton.
- [BREAKING] Move enums::TextCursor to text::Cursor.
- Add a Column and Row widgets which support auto_layout by default, but require that widgets be added using add().
- Add ValueInput::soft and set_soft methods.
- Add WindowExt::set_cursor_image() and default_cursor().

## [0.16.5] - 2021-04-10
### Changes
- Update FLTK, syn and libc.
- Add assert of widget validity when cloning.

## [0.16.4] - 2021-04-01
### Changes
- Add GroupExt::clip_children() and set_clip_children().
- Add GroupExt::draw_child(), update_child, draw_children and draw_outside_label.
- Fix FileBrowser::set_filter doc comment.
- Update libc and syn.
- Update FLTK and cfltk.

## [0.16.3] - 2021-03-27
### Changes
- Add app::set_selection_color() and set_inactive_color().
- Update readme on events.
- Update deps.
- Try to use ninja when available.

## [0.16.2] - 2021-03-24
### Changes
- Update FLTK cfltk and libc.
- Add BrowserExt::selected_text().
- Remove commented code.
- Fix refcount checks.
- Fix typo in docs.
- Update FAQ.
- Add ValuatorExt::set_step() zero checks.

## [0.16.1] - 2021-03-22
### Changes
- [BREAKING] Remove image conversion code.
- Fix Fl_RGB_Image allocation.
- Refactor build script.
- Enable the fltk-bundled feature on a system path using the CFLTK_BUNDLE_DIR env variable.
- Enable the fltk-bundled feature on a user-defined url using CFLTK_BUNDLE_URL env var.
- Add app::set_font_size() to set the global font size of the app.
- Add Slider (and Scrollbar) slider_size setter and getter.
- Add Slider (and Scrollbar) slider_frame setter and getter.

## [0.15.15] - 2021-03-21
### Changes
- Specify 'static lifetime for Printer labels.
- Add assert on image's color depth prior to writing to a file.
- Add second check in GlWindow get_proc_address.

## [0.15.14] - 2021-03-19
### Changes
- Fix print support on linux/bsd and add proper error handling.
- Pull Gif fix from FLTK.
- Fix system-fltk feature.

## [0.15.13] - 2021-03-18
### Changes
- Update deps.
- Add ImageExt::w() and h().
- Add MenuExt::global(). 
- Add MenuItem::measure(), draw() and set_image().
- Add WindowExt::shap() and set_shape().
- Fix take_focus() error value.
- Add WindowExt::x_root() and y_root().
- Add FrameType::dx(), dy(), dw() and dh().

## [0.15.12] - 2021-03-17
### Changes
- Add WidgetExt::center_of_parent().
- Add ButtonExt::value() and set_value().
- Add comment on supported image formats in WindowExt::set_icon().
- Remove redundant set_size() definitions in Chart and BrowserExt.
- Add WidgetExt::w() and WidgetExt::h().

## [0.15.11] - 2021-03-14
### Changes
- Add Spinner value setter and getter.

## [0.15.10] - 2021-03-13
### Changes
- Remove unnecessary clone in RgbImage::new().
- Add doc comments regarding SysMenuBar and enums::Event::Resize.

## [0.15.9] - 2021-03-10
### Changes
- Clone data in RgbImage::new().
- Fix typo in EngravedBox name.
- Fix memory leak in RgbImage::new() and other functions calling it.
- Add enums::Damage.
- Add WidgetExt::damage_type() and set_damage_type().
- Add WidgetExt::size_of_parent().

## [0.15.7] - 2021-03-08
### Changes
- Fix typo in README. Thanks @kainjow.
- Update deps.
- Update README & FAQ.

## [0.15.6] - 2021-03-03
### Changes
- Add screen scale getters and setters in the app module.
- add WidgetExt::visible() and visible_r() (recursive) methods.
- Add GroupExt::remove_by_index().
- MenuItem::popup() no longer requires mut self.
- Update deps.

## [0.15.5] - 2021-03-01
### Changes
- Check for 0 children with Pack::auto_layout(). Thanks @dheijl.

## [0.15.4] - 2021-02-26
### Changes
- Add utils::rgba2hex().
- Pass u32 to utils::hex2rgba().
- Fix docs and intra doc links.
- Update FLTK and cfltk.

## [0.15.3] - 2021-02-24
### Changes
- Mark app::handle and handle_main as safe.
- Add methods to get and set Dial angles.
- Add app::remove_idle, has_idle, remove_timeout.

## [0.15.2] - 2021-02-22
### Changes
- [BREAKING] RgbImage::new() and draw::draw_image() take a ColorDepth enum instead of an int.
- [BREAKING] Remove app::delay().
- [BREAKING] Remove image related LabelType values.
- [BREAKING] Change iconsize and set_iconsize to icon_size and set_icon_size.
- [BREAKING] BrowserExt::topline, middleline and bottomline converted to snake case.
- [BREAKING] app::wait_for returns a Result<bool, FltkError>.
- Fix temp file creation when lacking TMPDIR env variable on some systems.
- Fix doc tests.
- Add tests directory.
- Update FLTK and cfltk.

## [0.14.13] - 2021-02-19
### Changes
- Clean up examples.
- Move example usage to docs.
- Mark Pixmap constructor as unsafe.
- Refactoring.

## [0.14.12] - 2021-02-18
### Changes
- Update FLTK and cfltk.
- cleanup fltk.patch.
- Fix broken image::Pixmap.
- Add asserts for supported window icon images.

## [0.14.11] - 2021-02-17
### Changes
- Fix android typo causing failed android builds.
- Fix draw::draw_image().

## [0.14.10] - 2021-02-16
### Changes
- Mark draw::draw_image() as unsafe.
- Add docs regarding label's special symbols.
- Add enable-glwindow to docs.rs metadata.

## [0.14.9] - 2021-02-14
### Changes
- Add draw::draw_image().
- Add OverlayWindow widget.
- Relax requirement of static str for get_proc_address.
- Update deps.

## [0.14.8] - 2021-02-13
### Changes
- Add TabelExt::get_selection2 and visible_cells2 variants which return tuples.
- Add GlutWindow widget.

## [0.14.7] - 2021-02-12
### Changes
- Fix size of window::RawHandle on 32-bit Xlib systems (#502). Thanks @CaseyB.

## [0.14.6] - 2021-02-11
### Changes
- Add BrowserExt::value(). Thanks @tdryer.
- Refactoring derive code.
- Update cfltk.

## [0.14.5] - 2021-02-10
### Changes
- Remove unnecessary field in MenuItem.

## [0.14.4] - 2021-02-09
### Changes
- Add app::event_mouse_button();
- Add enums::Mouse.
- Fix docs.
- impl IntoIter for MenuItem.

## [0.14.3] - 2021-02-08
### Changes
- Add app::add_idle().
- Add app::sleep().
- Add note on WidgetBase::draw() regarding drawing on MacOS.
- Update cfltk.

## [0.14.2] - 2021-02-07
### Changes
- Add InputChoice::input() which returns the underlying input widget. Thanks @tdryer
- Update docs.

## [0.14.1] - 2021-02-06
### Changes
- [BREAKING] Rename old app::awake<F: FnMut()>(cb: F) to app::awake_callback.
- [BREAKING] Remove redundant/unnecessary methods from the App struct.
- Add app::event_key_down(Key) which takes a key and returns true if the given key was held down (or pressed) during the last event. Thanks @CaseyB.
- Add framebuffer drawing functions: draw::draw_rgba(), draw_rgba_nocopy(), draw_rgb() and draw_rgb_nocopy().
- Add utils::hex2rgba().
- Add app::awake().
- Add custom std::fmt::Debug impl for Event to account for custom events.
- Update libc dependency.

## [0.13.15] - 2021-02-04
### Changes
- Add app::handle_main() for sending events to the main window.
- Add i32 conversion methods for enums::Event.

## [0.13.14] - 2021-02-03
### Changes
- Lock messages.
- Enable detection of window resizing (added enums::Event::Resize).
- Added unsafe app::handle which enables sending messages to app windows.
- Expose unsafe functions app::awake_msg and thread_msg.
- Update FLTK and cfltk.

## [0.13.13] - 2021-02-02
### Changes
- Add optional gl_loader dependency for interop with other OpenGL crates.
- Add raw-window-handle dependency for interop with other renderers and windowing systems.
(both are lightweight)
- Pass contentView to RawWindowHandle for compat with wgpu.

## [0.13.11] - 2021-01-29
### Changes
- Add GlWindow::get_proc_address().
- Use bitflags for draw::LineStyle.
- Open display before calls to draw.
- Add default variants of message boxes (not requiring coordinates)

## [0.13.10] - 2021-01-25
### Changes
- Add missing docs.
- Fix clippy warnings.
- Update syn crate.

## [0.13.9] - 2021-01-22
### Changes
- Fix GlWindow::set_mode to account for change of enums::Mode now using bitfalgs.
- Fix linkage with feature enable-glwindow.
- Add missing assert.
- Update cfltk.

## [0.13.7] - 2021-01-18
### Changes
- Add debug_assert that char != 0 for BrowserExt::set_format_char and set_column_char.
- Fix rustdoc failing tests.
- Add syntactic sugar for DisplayExt::set_buffer to accept Into-Option, same for set_highlight_data.

## [0.13.5] - 2021-01-15
### Changes
- Change dialog::color_chooser's default to white instead of black.
- Add dialog::color_chooser_with_default().

## [0.13.4] - 2021-01-13
### Changes
- Fix premature drop in app::load_font.
- Add build instructions for nixOS. Thanks @legendofa

## [0.13.3] - 2021-01-12
### Changes
- [BREAKING] Remove deprecated calls app::set_color and calls to scrollbar_width (should be replaced by scrollbar_size).
- [BREAKING] Change Color::BackGround to BackGround2 to better reflect the color of input/output widgets.
- [BREAKING] TableRowSelectMode enum values were trimmed to remove Select prefix.
- [BREAKING] DragType enum values were trimmed to remove Drag prefix.
- [BREAKING] Change text methods to snake case in dialog and browser modules for consistency.
- FAQ: Add info on keeping the console window only for debug builds on Windows.Thanks @dheijl
- Add proper Default impl for use with fl2rust.
- Add Color::by_index(u8).
- Add WindowExt::hotspot().
- Add Group::current().
- Add ButtonExt and MenuExt down_box().
- Add HelpView and InputChoice widgets in the misc module.
- Add CheckBrowser in the browser module.
- Add app::get_system_colors().
- Update dependencies.
- Add missing docs.

## [0.12.8] - 2021-01-06
### Changes
- Add WindowExt::size_range which sets min/max width/height.

## [0.12.7] - 2021-01-05
### Changes
- Relax Copy trait bound to Clone as message requirement in widget emit calls.
- Update syn version.

## [0.12.6] - 2021-01-03
### Changes
- Add MenuExt::end, a stub needed for fluid.
- Add TableExt::callback_col, callback_row, callback_context.
- Build fluid with fltk.

## [0.12.5] - 2020-12-30
### Changes
- Widget deletion is relegated to WidgetBase.
- Add app::background, background2 and foreground to set default app colors.
- Deprecate app::set_color.

## [0.12.4] - YANKED
### Changes

## [0.12.3] - 2020-12-28
### Changes
- Add IntoIterator impls for GroupExt widgets, MenuExt widgets and the Tree widget.

## [0.12.2] - 2020-12-26
### Changes
- Fix logic bug in the BrowserExt::select method. Thanks @ThiNei2l.

## [0.12.1] - 2020-12-26
### Changes
- [BREAKING] TableRow::select_row and select_all_rows also take a flag to specify selection/deselection/toggling existing state.
- [BREAKING] Mark WidgetExt::into_widget as unsafe since it allows casting into incompatible widget types.
- Update docs.
- Update dependencies.
- Refactoring.

## [0.12.0] - 2020-12-23 - YANKED
### Changes

## [0.11.6] - 2020-12-13
### Changes
- Update docs and README.
- Dissociate versions of fltk from both fltk-derive and fltk-sys.

## [0.11.5] - 2020-12-09
### Changes
- Add wiki.
- Update dependencies.
- Update README and lib.rs.


## [0.11.4] - 2020-11-30
### Changes
- [BREAKING] Remove enums::Color::to_rgb and to_u32.
- [BREAKING] WidgetExt::trigger returns a CallbackTrigger instead of an int.
- Add custom Display implementation for Color.
- Use lazy_static for globals.
- Use bitflags for several enum types.
- Add utils::rgb2hex and utils::hex2rgb.
- Add draw::set_draw_rgb_color() and set_draw_hex_color().
- Relax app::Message requirements to only be Send + Sync.
- Add app::set_font(Font) to set global app font.
- Add app::set_color(Color) to set the global app's background color.
- Add app::visible_focus and set_visible_focus.
- Fix WidgetExt::center_of() to account for special positioning within windows.
- WidgetExt::as_window and as_group only require immutable ref to self.
- Default Window to a DoubleWindow instead of single window for better performance.

## [0.10.15] - 2020-11-27
### Changes
- Add more fields to enums::Align.
- Add impl BitOr for Shortcut | Shortcut.
- Update dependencies.

## [0.10.14] - 2020-11-23
### Changes
- Pin fltk crate versions to each other.
- update fltk and cfltk.

## [0.10.13] - 2020-11-21
### Changes

- Fix WidgetExt::right_of() by @dheatovwil
- Fix WidgetExt::center_of().
- Add WidgetExt::trigger() getter.
- Trigger rebuild on env change of CC or CXX.
- Update cfltk.

## [0.10.12] - 2020-11-20
### Changes
- Write to temp files in TEMP/TMPDIR.
- Add surface::SvgFileSurface to draw widgets as svg images.
- Add surface::ImageSurface::draw_decorated_window().
- Add WindowExt::decorated_w() and decorated_h().

## [0.10.11] - 2020-11-19
### Changes
- Added doc comment on the limitations of SimpleTerminal.
- Added a Dockerfile, because why not!
- Update cfltk and fltk.

## [0.10.10] - 2020-11-17
### Changes
- Clean up build script.
- Account for spacing with auto_layout.
- Add custom widgets: group::HGrid and group::VGrid.
- Add lib search paths for Mageia 7 by @nsalguero.
- Fix docs on special characters. 
- Add feature: no-pango if rtl and cjk unicode support is not needed on linux/bsd.s

## [0.10.9] - 2020-11-15
### Changes
- Add Pack::auto_layout.
- GroupExt::resizable() takes a &WidgetExt instead of a &mut WidgetExt.

## [0.10.8] - 2020-11-13
### Changes
- WindowExt::make_resizable moved to GroupExt::make_resizable.
- WidgetExt::parent returns a boxed GroupExt instead of a WidgetExt.
- Follow cfltk stable branch instead of main.

## [0.10.7] - 2020-11-11
### Changes
- Separate the C api "cfltk" into a separate repo and add via submodules.
- Restructuring the repo accordingly.

## [0.10.6] - 2020-11-10
### Changes
- No longer force the position of default init windows and keep it up to the window manager.
- Add WindowExt::free_position().
- Change C api set_style_table_entry to set_highlight_data.

## [0.10.5] - 2020-11-08
### Changes
- Patch fltk to fix behavior of Fl_Win32_At_Exit() with mingw toolchain.
- Fix android build which doesn't support Fl_Printer.
- Fix WidgetExt::with_size() resizability.
- Add WidgetExt::measure_label() and draw::measure().
- Simplified some return types in the printer module.

## [0.10.4] - 2020-11-06
### Changes
- Change return type of Printer::begin_job().
- Use AtomicUsize for refcounting instead of Arc<Mutex>.
- Decrease refcount when unsetting an image or setting another image.
- Remove unwrapping when querying for windows, which could fail.

## [0.10.3] - 2020-11-04
### Changes
- BREAKING (Security update): Methods and functions returning widget and image instances now return a safer boxed trait object since these might not be constructed by user code (like in dialogs).
- Fix support for msvc 2013 and lower.
- Add WidgetBase trait.
- Images and TextBuffer are now refcounted.


## [0.10.2] - 2020-11-02
### Changes
- Add unsafe RgbImage::from_data() which creates an RgbImage from non-owned data.
- <image type>Image::write_to_file has an AsRef<std::path::Path> parameter.
- Add SurfaceDevice trait and ImageSurface type.
- Add Printer type for native print support.
- Prelim support for musl builds.
- Remove back-compat SurfaceDevice::set_current().
- Enable window grabbing.

## [0.10.1] - 2020-10-30
### Changes
- BREAKING: App::wait() returns bool instead of Result since it doesn't error out.
- BREAKING: Require pango headers on linux distros for rtl and cjk unicode support.
- BREAKING: Deletion static methods take ownership of widgets and images.
- Update docs regarding build dependencies.
- Add WidgetExt::draw2(), WidgetExt::handle2() and TableExt::draw_cell2() all of which expose the widget in the closure arguments.
- Remove the requirement to box callbacks.
- Passing CString to C/C++ should not panic on inner null characters.
- Removed several unused internal methods from the public interface.

## [0.9.7] - 2020-10-20
### Changes
- Add doc comment on MenuExt::value() which might return -1 when no choice is made.
- Add WidgetExt::set_callback2 which exposes the widget in the closure arguments.
- Remove unused safe_new.

## [0.9.6] - 2020-10-15
### Changes
- Update FLTK.
- Internal refactoring.
- Fix app::wait erroneous result.

## [0.9.5] - 2020-10-09
### Changes
- Change Tree methods to take immutable borrows of tree items.
- Pull fixes from FLTK.

## [0.9.4] - 2020-10-05
### Changes
- Move threads initialization to occur on App initialization.
- Pull NSOpenPanel fix from FLTK.

## [0.9.3] - 2020-10-04
### Changes
- Add app::set_scrollbar_size(u32) and app::scrollbar_size().
- Add wrapping methods to DisplayExt as well the text::WrapMode enum.
- Fix Window::show_with_args().

## [0.9.2] - 2020-09-26
### Changes
- Require ANDROID_SDK_ROOT, which has cmake and ninja.
- No need to specify use-ninja feature for Android.
- Require ANDROID_NDK_ROOT. Still uses NDK_HOME for back-compat.
- Fix channels on Android.

## [0.9.1] - 2020-09-24
### Changes
- Breaking:
    - Some draw functions now take a Coord struct:
        - draw_polygon2()
        - draw_curve()
- Add prelim alpha support for Android, this requires:
    - Android NDK to be installed.
    - NDK_HOME to be set.
- Add support for passing cmake toolchain files via env var CFLTK_TOOLCHAIN.
- Add struct AndroidWindow.
- Update FLTK.
- Update syn.
- Add a [roadmap](./ROADMAP.md) for the project.


## [0.8.8] - 2020-09-19
### Changes
- Fix typos, thanks @nicolasbauw 
- Add DisplayExt::style_buffer().
- Fix wrapper bug which ignored return values.
- Update FLTK.
- Impl Send and Sync for FltkError.
- Remove opaque type StyleTables.
- BrowserExt::set_column_widths() takes a static slice.

## [0.8.7] - 2020-09-14
### Changes
- Accept AsRef<Path> where &Path was previously required.
- Update FLTK.
- Update deps.

## [0.8.6] - 2020-09-09
### Changes
- Add null check on C++ side when loading fonts.
- Update FLTK.
- Add doc comments on MenuExt::add() labels.

## [0.8.5] - 2020-09-04
### Changes
- Add App::load_font() to load a font from a path.
- Update build.rs to reflect latest FLTK cmake options.

## [0.8.4] - 2020-09-01
### Changes
- Remove potentially failing internal unwraps.

## [0.8.3] - 2020-08-21
### Changes
- Update FLTK.
- Update syn.
- Clean up examples.
- Add fltk-sys example.
- Restructure C wrapper.
- Rename enum InputType field value InputType to Input.

## [0.8.2] - 2020-08-21
### Changes
- Add experimental no-images feature flag.
- cfltk and fltk are compiled with no-rtti and no-exceptions.
- Override placement new in cfltk.
- Add note on exception-safety in FAQ.
- mark app::set_raw_callback as unsafe.
- Add static widget, buffer and image deletion functions.
- Fixed typo app::dispaly -> app::display (Thanks @DBLouis).
- Remove default linkage to C++ std library.

## [0.8.1] - 2020-08-13
### Changes
- Add app::set_raw_callback() which avoids boxing callbacks.
- Update FLTK to after commit cd26829 which adds MacOS 11.0 support.
- Add missing docs for callback arguments.
- Add TextBuffer::save_file().
- Use expect() in the build script to give meaningful messages on missing dependencies.

## [0.8.0] - 2020-08-02
### Changes
- Make fltk-sys no_std.
- Add feature flag "enable-glwindow".
- Remove feature flag "no-opengl".
- Separate raw opengl functions into their own crate "glu-sys".
- Add DoubleWindow::flush().
- Update dependencies (syn to 1.0.36).

## [0.7.26] - 2020-08-01
### Changes
- Add alias Scheme to AppScheme.
- Add WindowExt::fullscreen_active() and WindowExt::iconize().
- Update FLTK.


## [0.7.25] - 2020-07-30
### Changes
- Fix WindowExt::raw_handle() method on systems where `unsigned long int` is u32.
- Make several free functions in the app module public.

## [0.7.24] - 2020-07-29
### Changes
- Add convenience functions dialog::file_chooser() and dialog::dir_chooser().
- Add color_chooser() convenience function.
- Add dialog::FileChooser::window().
- impl BitOr for dialog::FileChooserType.
- Add WidgetExt::set_pos and WidgetExt::set_size.

## [0.7.23] - 2020-07-28
### Changes
- Add Fltk's own FileChooser. In addition to the already available native FileDialog.
- Add app::wait_for(dur: f64).

## [0.7.22] - 2020-07-25
### Changes
- Add impl BitOr<Key> for Shortcut.
- Change impl Add to impl BitOr for Shortcut.
- Add Shortcut::from_key() and Shortcut::from_i32().

## [0.7.21] - 2020-07-24
### Changes
- Add Window::show_with_args() and show_with_env_args() for FLTK specific command-line arguments.
- Add FrameType::by_index().
- Add some missing asserts.

## [0.7.20] - 2020-07-23
### Changes
- Add missing docs for new TextEditor functions.
- Add SingleWindow widget.
- Add more FileBrowser methods.

## [0.7.19] - 2020-07-22
### Changes
- Add Pixmap.
- Use high res GL on MacOS by default when OpenGL is supported.
- Add more Tabs methods.
- Add rest of TextEditor methods.
- Move LineStyle to draw module.
- Move ChartType to misc module.

## [0.7.18] - 2020-07-20
### Changes
- Allow getting the fl_display and fl_gc globabl variables.
- Add null checks for methods acquiring and requiring raw handles.

## [0.7.17] - 2020-07-19
### Changes
- Add several more image types.
- Add SysMenuBar widget.
- Add WindowExt::region and set_region methods.

## [0.7.16] - 2020-07-17
### Changes
- Change interface of raw handles to return and accept RawHandle (HWND on windows, NSWindow on macos, Xid on X11).
- Rename Window::from_raw_handle() to Window::find_by_handle().
- Add event coordinate getters relative to the screen position of the cursor.

## [0.7.15] - 2020-07-16
### Changes
- Add fl_find(Window xid) as Window::from_raw_handle().
- Add RawWindowHandle as a wrapper around an opaque type.

## [0.7.14] - 2020-07-12
### Changes
- FLTK update.
- Add ScrollType enum.
- Update versions of syn, quote and cmake.
- Add Scroll::scrollbar() and Scroll::hscrollbar().
- Add BrowserExt::scrollbar() and BrowserExt::hscrollbar().

## [0.7.13] - 2020-07-10
### Changes
- Add missing stdint.h header at cfl.cpp.
- Expose unsafe TableExt::draw_cell_data() and set_draw_cell_data() methods.

## [0.7.12] - 2020-07-05
### Changes
- Refactoring.
- Add ``` # Safety``` to docs.
- Add to readme and faq.
- Fix "save as" in the editor example.

## [0.7.11] - 2020-07-04
### Changes
- Fixed Chart colors.

## [0.7.10] - 2020-07-03
### Changes
- MenuItem::new now takes a slice instead of a vector.

## [0.7.9] - 2020-06-30
### Changes
- Add exception checks in callbacks.

## [0.7.8] - 2020-06-29
### Changes
- Add app::belowmouse, pushed and focus functions.
- Add functions to query FLTK's api and abi versions.

## [0.7.7] - 2020-06-28
### Changes
- Remove Copy trait from TextBuffer and Images since it was potentially unsound.
- Rename TextBuffer::copy to copy_from to avoid conflict with copy method which does a deep copy.

## [0.7.6] - 2020-06-27
### Changes
- Deprecate App::set_scheme in favor of App::with_scheme.


## [0.7.5] - 2020-06-26
### Changes
- Change char flags to return raw::c_char instead of i8/u8.
- Add is_ascii assert for set_column_char and set_format_char.
- Add missing docs for BrowserExt methods.
- Add None to BrowserScrollbar Enum.
- Rename BrowserScrollBar to BrowserScrollbar.
- Remove redundancies in WidgetType enum values.

## [0.7.4] - 2020-06-25
### Changes
- Add several missing methods to BrowserExt methods.

## [0.7.3] - 2020-06-24
### Changes
- Added PackType which can be used with the Pack widget.

## [0.7.2] - 2020-06-23
### Changes
- Clean up fonts interface. Add example on how to use system fonts.
- Add app::delay() which delays the execution of the application.
- Add missing docs.

## [0.7.1] - 2020-06-22
### Changes
- Add asserts around TextBuffer operations.

## [0.7.0] - 2020-06-21
### Changes
- Change interface for working with images. Now widgets take an Option<ImageExt>. This allows unsetting images.
- Remove unnecessary copy when setting a widget image.
- Add deimage and set_deimage methods to WidgetExt.
- The DisplayExt::set_buffer() takes an optional buffer, this allows unsetting the buffer.
- The DisplayExt widgets have the same constructors as other widgets.
- Fixed TextBuffer load_file method.
- Added assert to length of style entries.

## [0.6.9] - 2020-06-20
### Changes
- Fix image deletion.
- Add asserts to check if an image was deleted.
- Add note on image lifetimes in relation to widgets.

## [0.6.8] - 2020-06-19
### Changes
- Give image type specialized drop impls.

## [0.6.7] - 2020-06-18
### Changes
- Provide unsafe variant for widget deletion, which would delete user_data recursively for when needed.
- Improve docs about TableRow.

## [0.6.6] - 2020-06-17
### Changes
- RgbImage takes ownership of the underlying data.
- Remove gl_start and gl_finish.
- Add App::set_visual(Mode), app::own_colormap, app::set_focus.
- Fix Cursor naming.
- Update FLTK to commit 5005d04 for further SVG fixes.
- Add TableExt::draw_cell and TableContext.
- Rename Align enum members.

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
- Add App::windows() method which returns an optional Vector of the application's windows.
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
