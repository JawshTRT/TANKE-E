// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use tungstenite::{Message, connect};
slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    //Probably an environment logger?

    let ui = AppWindow::new()?;

    ui.on_connect({
        let ui_handle = ui.as_weak();
        //Port and ip arguments lie within the pipe operator of move
        move |port, ip| {
            let ui = ui_handle.unwrap();
            // TODO: Connect to robot server

            let (mut socket, response) =
                connect("ws://someip:someport/socket").expect("Can't connect");

            println!("Connected to port {0} at ip {1}", port, ip);
        }
    });
    ui.on_request_move({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            // TODO: Send message to robot server
            println!("moved");
            //ui.set_counter(ui.get_counter() + 1);
        }
    });
    ui.run()?;

    Ok(())
}
