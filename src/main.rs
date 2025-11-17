pub mod pwm_motors;
mod robot;

use snafu::prelude::*;
use std::error::Error;

#[derive(Debug, Snafu)]
#[snafu(transparent)]
struct GenericError {
    source: Box<dyn Error>,
}

#[snafu::report]
fn main() -> Result<(), GenericError> {
    robot::robot_main().map_err(|error| GenericError { source: error })
}
