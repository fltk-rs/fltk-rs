#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(clippy::needless_doctest_main)]
#![warn(missing_docs)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::arc_with_non_send_sync)]
#![allow(clippy::missing_transmute_annotations)]
#![allow(unsafe_op_in_unsafe_fn)]

/// Application related methods and functions
pub mod app;

/// Browser widgets
pub mod browser;

/// Button widgets
pub mod button;

/// Dialog widgets
pub mod dialog;

/// Drawing primitives
pub mod draw;

/// Fltk defined enums: Color, Font, `CallbackTrigger` etc
pub mod enums;

/// Basic fltk box/frame widget
pub mod frame;

/// Group widgets
pub mod group;

/// Image types supported by fltk
pub mod image;

/// Input widgets
pub mod input;

/// mod macros;
pub mod macros;

/// Menu widgets
pub mod menu;

/// Miscellaneous widgets not fitting a certain group
pub mod misc;

/// Output widgets
pub mod output;

/// All fltk widget traits and flt error types
pub mod prelude;

/// Widget surface to image functions
pub mod surface;

/// Table widgets
pub mod table;

/// Terminal widget
pub mod terminal;

/// Text display widgets
pub mod text;

/// Tree widgets
pub mod tree;

/// General utility functions
pub mod utils;

/// Valuator widgets
pub mod valuator;

/// Base Widget
pub mod widget;

/// Window widgets
pub mod window;

/// Printing related functions
#[cfg(not(target_os = "android"))]
pub mod printer;
