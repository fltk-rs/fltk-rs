#![forbid(unsafe_code)]

use fltk::{
    app,
    button::Button,
    group::Flex,
    frame::Frame,
    enums::{Font,Event,FrameType},
    browser::{Browser,BrowserType},
    prelude::*,
    window::Window,
};

const HEARTBEAT: i32 = 404;
const SHIFT: i32 = 405;
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 4;

#[derive(Debug, Clone)]
struct Model {
    list: Vec<char>,
    curr: usize,
}

impl Model {
    fn init() -> Self {
        Self {
            list: (0x2700..=0x27BF).map(|x| char::from_u32(x).unwrap()).collect(),
            curr: 0,
        }
    }
    fn choice(&mut self, curr: usize) {
        self.curr = curr;
    }
    pub fn inc(&mut self) {
        if !self.list.is_empty() {
            match self.curr < self.list.len() - 1 {
                true => self.curr += 1,
                false => self.curr = 0,
            };
        }
    }
    pub fn dec(&mut self) {
        if !self.list.is_empty() {
            match self.curr > 0 {
                true => self.curr -= 1,
                false => self.curr = self.list.len() - 1,
            };
        }
    }
}

fn main() -> Result<(), FltkError> {
    app::GlobalState::<Model>::new(Model::init());
    let app = app::App::default().with_scheme(app::AppScheme::Plastic);
    let mut window = crate::window();
    crate::view();
    window.end();
    window.show();
    app::handle_main(Event::from_i32(HEARTBEAT)).unwrap();
    app.run()
}

fn view() {
    let mut page = Flex::default_fill();
    {
        let mut left = Flex::default_fill().column();
        crate::browser("List").handle(crate::choice);
        let mut buttons = Flex::default();
        Button::default().with_label("@#<").handle(crate::shift);
        Button::default().with_label("@#>").handle(crate::shift);
        buttons.end();
        buttons.set_pad(0);
        left.end();
        left.set_pad(PAD);
        left.fixed(&buttons, HEIGHT);
        page.fixed(&left, WIDTH);
        crate::frame("Canvas").handle(crate::show);
    }
    page.end();
    page.set_pad(10);
    page.set_margin(10);
    page.set_frame(FrameType::FlatBox);
}

fn browser(tooltip: &str) -> Browser {
    let mut element = Browser::default().with_type(BrowserType::Hold);
    element.set_tooltip(tooltip);
    element.set_label_font(Font::Zapfdingbats);
    element.set_text_size(16);
    element
}

fn choice(browser: &mut Browser, event: Event) -> bool {
    if event == Event::from_i32(HEARTBEAT) {
        let model = app::GlobalState::<Model>::get().with(move |model| model.clone());
        if !model.list.is_empty() {
            browser.clear();
            for item in model.list {
                browser.add(&item.to_string());
            }
            browser.select(model.curr as i32 + 1);
        }
        true
    } else if event == Event::Push {
        if browser.value() > 0 {
            let curr: usize = browser.value() as usize - 1;
            app::GlobalState::<Model>::get().with(move |model| model.choice(curr));
            app::handle_main(Event::from_i32(HEARTBEAT)).unwrap();
        };
        true
    } else {
        false
    }
}

fn frame(tooltip: &str) -> Frame {
    let mut element = Frame::default();
    element.set_frame(FrameType::DownBox);
    element.set_label_font(Font::Zapfdingbats);
    element.set_label_size(250);
    element.set_tooltip(tooltip);
    element
}

fn show(frame: &mut Frame, event: Event) -> bool {
    if [Event::from_i32(HEARTBEAT), Event::Released].contains(&event) {
        let model = app::GlobalState::<Model>::get().with(move |model| model.clone());
        if !model.list.is_empty() {
            frame.set_label(&model.list[model.curr].to_string());
        };
        true
    } else {
        false
    }
}

fn shift(button: &mut Button, event: Event) -> bool {
    if [Event::from_i32(SHIFT), Event::Push].contains(&event) {
        button.deactivate();
        let label = button.label();
        app::GlobalState::<Model>::get().with(move |model| match label == "@#<" {
            true => model.dec(),
            false => model.inc(),
        });
        app::handle_main(Event::from_i32(HEARTBEAT)).unwrap();
        button.activate();
        true
    } else {
        false
    }
}

fn window() -> Window {
    const NAME: &str = "Glyph";
    let mut element = Window::default()
        .with_size(640, 360)
        .center_screen();
    element.make_resizable(true);
    element.set_xclass(NAME);
    element.handle(move |window, event| {
        if event == Event::from_i32(HEARTBEAT) {
            let value =
                app::GlobalState::<Model>::get().with(move |model| model.list[model.curr]);
            window.set_label(&format!("{value} - {NAME}"));
            true
        } else if app::event() == Event::Close {
            app::quit();
            true
        } else {
            false
        }
    });
    element
}
