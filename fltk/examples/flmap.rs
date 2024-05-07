#![forbid(unsafe_code)]

use {
    fltk::{
        app, button::Button, enums::FrameType, frame::Frame, input::Input, prelude::*,
        window::Window,
    },
    std::{
        collections::HashMap,
        net::{SocketAddr, TcpStream},
        thread,
        time::Duration,
    },
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut scan: bool = false;
    let mut timeout: u64 = 8;

    let netport = app::App::default().with_scheme(app::AppScheme::Plastic);
    let mut window = Window::default()
        .with_size(300, 180)
        .with_label("FlMap")
        .center_screen();

    /* IP PANEL */
    let mut ip_panel = Frame::new(16, 10, 270, 70, "");
    ip_panel.set_frame(FrameType::DownFrame);

    Frame::new(125, 20, 50, 8, "IP address:");

    // IP dots
    Frame::new(105, 54, 5, 5, ".");
    Frame::new(148, 54, 5, 5, ".");
    Frame::new(191, 54, 5, 5, ".");

    // Byte fields
    let mut bytes_txt: [Input; 4] = [
        Input::new(69, 40, 34, 24, ""),
        Input::new(112, 40, 34, 24, ""),
        Input::new(155, 40, 34, 24, ""),
        Input::new(198, 40, 34, 24, ""),
    ];

    for input in &mut bytes_txt {
        input.set_maximum_size(3)
    }

    /* PORT PANEL */
    let mut port_panel = Frame::new(15, 90, 100, 70, "");
    port_panel.set_frame(FrameType::DownFrame);

    Frame::new(24, 107, 30, 8, "Port:");

    let mut port_txt = Input::new(57, 98, 48, 24, "");
    port_txt.set_maximum_size(5);

    let mut check_btn = Button::new(24, 128, 81, 21, "Check");

    /* STATUS PANEL */
    let mut status_panel = Frame::new(125, 90, 160, 70, "");
    status_panel.set_frame(FrameType::DownFrame);

    let mut status_label = Frame::new(155, 100, 100, 14, "Status");
    let mut address_label = Frame::new(145, 105, 130, 50, "");

    /* ARGS handling (Address and Timeout) */
    if args.len() >= 2 && args[1].parse::<SocketAddr>().is_ok() {
        let address: Vec<&str> = args[1].split(':').collect();
        let bytes: Vec<&str> = address[0].split('.').collect();

        for i in 0..4 {
            bytes_txt[i].set_value(bytes[i])
        }
        port_txt.set_value(address[1]);

        scan = true;
    }

    if args.len() == 4 && args[2] == "-t" && args[3].parse::<u64>().is_ok() {
        timeout = args[3].parse::<u64>().unwrap()
    }

    window.show();

    /* SCAN */
    check_btn.set_callback(move |check_btn| {
        check_btn.deactivate();

        let bytes: String = bytes_txt
            .iter()
            .map(|byte_txt| String::from(byte_txt.value().trim()))
            .collect::<Vec<String>>()
            .join(".");

        let address: String = format!("{}:{}", bytes, port_txt.value().trim());
        let abort: bool = address.parse::<SocketAddr>().is_err();

        if !abort {
            address_label.set_pos(140, 105);
            address_label.set_label(&address);
            status_label.set_label("Scanning...");

            // Pointers copy
            let mut status_label = status_label.clone();
            let mut check_btn = check_btn.clone();

            thread::spawn(move || {
                if TcpStream::connect_timeout(
                    &address.parse::<SocketAddr>().unwrap(),
                    Duration::from_secs(timeout),
                )
                .is_ok()
                {
                    status_label.set_label("Status: Open")
                } else {
                    status_label.set_label("Status: Closed")
                }
                check_btn.activate();
            });

            let port: u32 = port_txt.value().parse::<u32>().unwrap();
            let services = HashMap::from(SERVICES);
            if services.contains_key(&port) {
                window.set_label(&format!("{} - FlMap", services[&port]))
            }
        } else {
            status_label.set_label("");
            address_label.set_pos(139, 99);
            address_label.set_label("Invalid IP/Port");
            check_btn.activate();
        }
    });

    if scan {
        check_btn.do_callback()
    }
    netport.run().unwrap();
}

const SERVICES: [(u32, &str); 31] = [
    (21, "FTP"),
    (990, "FTPS"),
    (22, "SSH"),
    (23, "Telnet"),
    (53, "DNS"),
    (25, "SMTP"),
    (587, "SMTP (SSL)"),
    (110, "POP"),
    (995, "POP (SSL)"),
    (143, "IMAP"),
    (993, "IMAP (SSL)"),
    (67, "DHCP"),
    (123, "NTP"),
    (80, "HTTP"),
    (8080, "HTTP"),
    (443, "HTTPS"),
    (194, "IRC"),
    (445, "SMB"),
    (5060, "SIP"),
    (3306, "MySQL"),
    (5432, "PostgreSQL"),
    (27017, "MongoDB"),
    (6379, "Redis"),
    (2082, "cPanel"),
    (6000, "X11"),
    (5672, "AMQP"),
    (389, "LDAP"),
    (636, "LDAPS"),
    (9987, "TeamSpeak 3"),
    (666, "Doom"),
    (25565, "Minecraft"),
];
