use crate::pwm_motors::PWMMotor;
use std::error::Error;

pub fn robot_main() -> Result<(), Box<dyn Error>> {
    let mut motor = PWMMotor::talon_srx(12)?;
    motor.set_speed(1.0)?;
    loop {}
}
