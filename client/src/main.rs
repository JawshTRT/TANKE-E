// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use tungstenite::{connect, Message};

slint::include_modules!();

#[derive(Serialize, Deserialize, Debug)]
struct Throttle {
    x: i8,
    y: i8,
}

#[derive(Serialize, Deserialize, Debug)]
struct ControllerInput {
    throttle: Throttle,
}

#[derive(Serialize, Deserialize, Debug)]
struct SimpleError {
    error: String,
}
fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let (mut socket, response) = connect("ws://localhost:9001").expect("Can't connect");

    println!("Connected to the socket");
    println!("Response HTTP code: {}", response.status());
    println!("Response ocntains the following headers:");

    for (header, _value) in response.headers() {
        println!("* {header}");
    }

    ui.on_connect({
        let ui_handle = ui.as_weak();
        //Port and ip arguments lie within the pipe operator just after move command
        move |port, ip| {
            let ui = ui_handle.unwrap();
            // TODO: Connect to robot server

            println!("Connected to port {0} at ip {1}", port, ip);
        }
    });
    ui.on_request_move({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            // TODO: Send message to robot server
            println!("moved");
            let data = r#" {
                "throttle": {
                    "x": 10,
                    "y": 0
                }
            }"#;
           // let input: Value = serde_json::from_str(data);
            socket
                .send(Message::Text(data.into()))
                .unwrap();
        }
    });
    ui.run()?;

    Ok(())
}
