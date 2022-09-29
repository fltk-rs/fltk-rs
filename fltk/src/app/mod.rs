use crate::enums::Mode;
use crate::prelude::*;
use std::path;

mod channel;
pub use channel::*;
mod event;
pub use event::*;
mod font;
pub use font::*;
mod init;
pub use init::*;
mod rt;
pub use rt::*;
mod screen;
pub use screen::*;
mod state;
pub use state::*;
mod version;
pub use version::*;
mod visual;
pub use visual::*;
mod widget;
pub use widget::*;

/// Basic Application struct, used to instantiate, set the scheme and run the event loop
#[derive(Debug, Copy, Clone)]
pub struct App {}

impl App {
    /// Instantiates an App type
    pub fn default() -> App {
        init_all();
        App {}
    }

    /// Sets the scheme of the application
    pub fn set_scheme(&mut self, scheme: Scheme) {
        set_scheme(scheme);
    }

    /// Sets the scheme of the application
    pub fn with_scheme(self, scheme: Scheme) -> App {
        set_scheme(scheme);
        self
    }

    /// Gets the scheme of the application

    pub fn scheme(self) -> Scheme {
        scheme()
    }

    /// Runs the event loop
    /// # Errors
    /// Can error on failure to run the application
    pub fn run(self) -> Result<(), FltkError> {
        run()
    }

    /// Wait for incoming messages.
    /// Calls to redraw within wait require an explicit sleep
    pub fn wait(self) -> bool {
        wait()
    }

    /// Loads system fonts
    pub fn load_system_fonts(self) -> Self {
        *FONTS.lock().unwrap() = get_font_names();
        self
    }

    /**
        Loads a font from a path.
        Requires enabling the ttf-parser feature to get a font name back from the method, otherwise you can pass the name directly to `enums::Font::by_name()`.
        On success, returns a String with the ttf Font Family name. The font's index is always 16.
        As such only one font can be loaded at a time.
        The font name can be used with [`Font::by_name`](`crate::enums::Font::by_name`), and index with [`Font::by_index`](`crate::enums::Font::by_index`).
        # Examples
        ```rust,no_run
        use fltk::{prelude::*, *};
        let app = app::App::default();
        let font = app.load_font("font.ttf").unwrap();
        let mut frame = frame::Frame::new(0, 0, 400, 100, "Hello");
        frame.set_label_font(enums::Font::by_name(&font));
        ```
        # Errors
        Returns `ResourceNotFound` if the Font file was not found
    */
    pub fn load_font<P: AsRef<path::Path>>(self, path: P) -> Result<String, FltkError> {
        Self::load_font_(path.as_ref())
    }

    fn load_font_(path: &path::Path) -> Result<String, FltkError> {
        if !path.exists() {
            return Err::<String, FltkError>(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        if let Some(p) = path.to_str() {
            let name = load_font(p)?;
            Ok(name)
        } else {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        }
    }

    /// Set the visual of the application
    /// # Errors
    /// Returns `FailedOperation` if FLTK failed to set the visual mode
    pub fn set_visual(self, mode: Mode) -> Result<(), FltkError> {
        set_visual(mode)
    }

    /// Redraws the app
    pub fn redraw(self) {
        redraw()
    }

    /// Quit the application
    pub fn quit(self) {
        quit()
    }
}
