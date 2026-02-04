//use crate::pwm_motors::{PWMMotor, PWMMotorError};
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use std::{error::Error, net::TcpListener, thread::spawn};
use tungstenite::{Message, accept};

#[derive(Debug, Snafu)]
pub enum RobotError {
    #[snafu(display("Cannot parse JSON"))]
    CannotParseJSON { source: serde_json::Error },
}

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

pub fn robot_main() -> Result<(), Box<dyn Error>> {
    //Creating a server
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();

    //For every incoming stream
    for stream in server.incoming() {
        //Create a thread and pass ownership of the variables
        //What is a thread? A thread is a lightweight process that can run concurrently with other threads.
        spawn(move || {
            //Create a mutable variable to store the incoming stream
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    // Receive JSON message from client driver station (msg)
                    // Parse the JSON message (msg -> json)

                    match serde_json::from_str::<ControllerInput>(
                        &msg.into_text().unwrap().as_str(),
                    ) {
                        Ok(controller_input) => {
                            println!("{:?}", controller_input);
                        }
                        Err(error) => {
                            let err = SimpleError {
                                error: error.to_string(),
                            };
                            // yo you messed up...
                            let msg = Message::Text(serde_json::to_string(&err).unwrap().into()); //Placeholder
                            websocket.send(msg.clone()).unwrap();

                            println!("{}", msg);
                        }
                    };
                }
            }
        });
    }
    Ok(())
}
