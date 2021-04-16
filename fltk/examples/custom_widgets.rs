use fltk::{
    app, button, draw, enums::*, frame::Frame, image::SvgImage, prelude::*, valuator::*,
    widget::Widget, window::Window,
};
use std::ops::{Deref, DerefMut};
use std::{cell::RefCell, rc::Rc};

const POWER: &str = r#"<?xml version="1.0" encoding="iso-8859-1"?>
<!-- Generator: Adobe Illustrator 19.1.0, SVG Export Plug-In . SVG Version: 6.00 Build 0)  -->
<svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
	 viewBox="0 0 315.083 315.083" style="enable-background:new 0 0 315.083 315.083;" xml:space="preserve">
<g id="Layer_1">
	<linearGradient id="SVGID_1_" gradientUnits="userSpaceOnUse" x1="157.5417" y1="4.5417" x2="157.5417" y2="310.5417">
		<stop  offset="0" style="stop-color:#939598"/>
		<stop  offset="0.25" style="stop-color:#414042"/>
		<stop  offset="0.5" style="stop-color:#252223"/>
		<stop  offset="1" style="stop-color:#000000"/>
	</linearGradient>
	<circle style="fill:url(#SVGID_1_);" cx="157.542" cy="157.542" r="153"/>
</g>
<g id="Layer_2">
	<linearGradient id="SVGID_2_" gradientUnits="userSpaceOnUse" x1="157.5417" y1="292.5417" x2="157.5417" y2="22.5417">
		<stop  offset="0" style="stop-color:#58595B"/>
		<stop  offset="0.1" style="stop-color:#414042"/>
		<stop  offset="0.2" style="stop-color:#242122"/>
		<stop  offset="1" style="stop-color:#000000"/>
	</linearGradient>
	<circle style="fill:url(#SVGID_2_);stroke:#58595B;stroke-miterlimit:10;" cx="157.542" cy="157.542" r="135"/>
</g>
<g id="Layer_4">
	<radialGradient id="SVGID_3_" cx="157.5417" cy="89.9217" r="62.2727" gradientUnits="userSpaceOnUse">
		<stop  offset="0" style="stop-color:#58595B"/>
		<stop  offset="0.5" style="stop-color:#414042"/>
		<stop  offset="1" style="stop-color:#231F20"/>
	</radialGradient>
	<radialGradient id="SVGID_4_" cx="157.5417" cy="89.9217" r="62.7723" gradientUnits="userSpaceOnUse">
		<stop  offset="0" style="stop-color:#FFFFFF"/>
		<stop  offset="0.6561" style="stop-color:#231F20"/>
		<stop  offset="1" style="stop-color:#000000"/>
	</radialGradient>
	
		<ellipse style="fill:url(#SVGID_3_);stroke:url(#SVGID_4_);stroke-miterlimit:10;" cx="157.542" cy="89.922" rx="59.833" ry="64.62"/>
</g>
<g id="Layer_6">
	<path style="fill:none;stroke:red;stroke-width:10;stroke-linecap:round;stroke-miterlimit:10;" d="M119.358,119.358
		c-9.772,9.772-15.816,23.272-15.816,38.184c0,14.912,6.044,28.412,15.816,38.184s23.272,15.816,38.184,15.816
		c14.912,0,28.412-6.044,38.184-15.816s15.816-23.272,15.816-38.184c0-14.912-6.044-28.412-15.816-38.184"/>
	
		<line style="fill:none;stroke:red;stroke-width:10;stroke-linecap:round;stroke-miterlimit:10;" x1="157.542" y1="154.542" x2="157.542" y2="100.542"/>
</g>
</svg>"#;

pub struct FlatButton {
    wid: Widget,
}

impl FlatButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &'static str) -> FlatButton {
        let mut x = FlatButton {
            wid: Widget::new(x, y, w, h, label),
        };
        x.draw();
        x.handle();
        x
    }

    // Overrides the draw function
    fn draw(&mut self) {
        self.wid.draw(move |b| {
            draw::draw_box(
                FrameType::FlatBox,
                b.x(),
                b.y(),
                b.width(),
                b.height(),
                Color::from_u32(0x304FFE),
            );
            draw::set_draw_color(Color::White);
            draw::set_font(Font::Courier, 24);
            draw::draw_text2(
                &b.label(),
                b.x(),
                b.y(),
                b.width(),
                b.height(),
                Align::Center,
            );
        });
    }

    // Overrides the handle function.
    // Notice the do_callback which allows the set_callback method to work
    fn handle(&mut self) {
        let mut wid = self.wid.clone();
        self.wid.handle(move |_, ev| match ev {
            Event::Push => {
                wid.do_callback();
                true
            }
            _ => false,
        });
    }
}

impl Deref for FlatButton {
    type Target = Widget;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for FlatButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}

pub struct PowerButton {
    frm: Frame,
    on: Rc<RefCell<bool>>,
}

impl PowerButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let mut frm = Frame::new(x, y, w, h, "");
        let on = Rc::from(RefCell::from(false));
        frm.draw({
            let on = on.clone();
            move |f| {
                let image_data = if *on.borrow() {
                    POWER.to_string().replace("red", "green")
                } else {
                    POWER.to_string()
                };
                let mut svg = SvgImage::from_data(&image_data).unwrap();
                svg.scale(f.width(), f.height(), true, true);
                svg.draw(f.x(), f.y(), f.width(), f.height());
            }
        });
        frm.handle({
            let on = on.clone();
            move |f, ev| match ev {
                Event::Push => {
                    let prev = *on.borrow();
                    *on.borrow_mut() = !prev;
                    f.do_callback();
                    f.redraw();
                    true
                }
                _ => false,
            }
        });
        Self { frm, on }
    }
    pub fn is_on(&self) -> bool {
        *self.on.borrow()
    }
}

impl Deref for PowerButton {
    type Target = Frame;

    fn deref(&self) -> &Self::Target {
        &self.frm
    }
}

impl DerefMut for PowerButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frm
    }
}

pub struct FancyHorSlider {
    s: Slider,
}

impl FancyHorSlider {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut s = Slider::new(x, y, width, height, "");
        s.set_type(SliderType::Horizontal);
        s.set_frame(FrameType::RFlatBox);
        s.set_color(Color::from_u32(0x868db1));
        s.draw(|s| {
            draw::set_draw_color(Color::Blue);
            draw::draw_pie(
                s.x() - 10 + (s.w() as f64 * s.value()) as i32,
                s.y() - 10,
                30,
                30,
                0.,
                360.,
            );
        });
        Self { s }
    }
}

impl Deref for FancyHorSlider {
    type Target = Slider;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl DerefMut for FancyHorSlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    // app::set_visible_focus(false);

    let mut wind = Window::default()
        .with_size(800, 600)
        .with_label("Custom Widgets");
    let mut but = FlatButton::new(350, 350, 160, 80, "Increment");
    let mut power = PowerButton::new(600, 100, 100, 100);
    let mut dial = FillDial::new(100, 100, 200, 200, "0");
    let mut frame = Frame::default()
        .with_size(160, 80)
        .with_label("0")
        .above_of(&*but, 20);
    let mut fancy_slider = FancyHorSlider::new(100, 550, 500, 10);
    let mut toggle = button::ToggleButton::new(650, 400, 80, 35, "@+9circle")
        .with_align(Align::Left | Align::Inside);
    wind.end();
    wind.show();

    wind.set_color(Color::Black);
    frame.set_label_size(32);
    frame.set_label_color(Color::from_u32(0xFFC300));
    dial.set_label_color(Color::White);
    dial.set_label_font(Font::CourierBold);
    dial.set_label_size(24);
    dial.set_color(Color::from_u32(0x6D4C41));
    dial.set_color(Color::White);
    dial.set_selection_color(Color::Red);
    toggle.set_frame(FrameType::RFlatBox);
    toggle.set_label_color(Color::White);
    toggle.set_selection_color(Color::from_u32(0x00008B));
    toggle.set_color(Color::from_u32(0x585858));
    toggle.clear_visible_focus();

    toggle.set_callback(|t| {
        if t.is_set() {
            t.set_align(Align::Right | Align::Inside);
        } else {
            t.set_align(Align::Left | Align::Inside);
        }
        t.parent().unwrap().redraw();
    });

    dial.draw(|d| {
        draw::set_draw_color(Color::Black);
        draw::draw_pie(d.x() + 20, d.y() + 20, 160, 160, 0., 360.);
        draw::draw_pie(d.x() - 5, d.y() - 5, 210, 210, -135., -45.);
    });

    dial.set_callback(|d| {
        d.set_label(&format!("{}", (d.value() * 100.) as i32));
        app::redraw();
    });
    but.set_callback(move |_| {
        frame.set_label(&(frame.label().parse::<i32>().unwrap() + 1).to_string())
    });

    power.set_callback(move |_| {
        println!("power button clicked");
    });

    fancy_slider.set_callback(|s| s.parent().unwrap().redraw());

    app.run().unwrap();
}
