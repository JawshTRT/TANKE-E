use rppal::gpio::{Gpio, OutputPin};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum PWMMotorError {
    #[snafu(display(
        "Your duty cycle: {duty} on the motor with pin {motor_pin} should be between -1 and 1"
    ))]
    DutyCycleOutOfBounds { motor_pin: u8, duty: f64 },

    #[snafu(display("GPIO Initialization failed"))]
    GPIOInitializationFailed { source: rppal::gpio::Error },

    #[snafu(display("Pin Initialization failed"))]
    PINInitializationFailed { source: rppal::gpio::Error },

    #[snafu(display("Unable to set duty cycle/frequency"))]
    DutyCycleFrequencyUnable { source: rppal::gpio::Error },
}

pub struct PWMMotor {
    output: OutputPin,
    frequency: f64,
    idle_duty: f64,
    forward_duty: f64,
    reverse_duty: f64,
}

impl PWMMotor {
    pub fn talon_srx(pin: u8) -> Result<PWMMotor, PWMMotorError> {
        let gpio = Gpio::new().context(GPIOInitializationFailedSnafu)?;
        let output = gpio
            .get(pin)
            .context(PINInitializationFailedSnafu)?
            .into_output();

        Ok(PWMMotor {
            output,
            frequency: 500.0,
            idle_duty: 0.0,
            forward_duty: 0.72,
            reverse_duty: 0.30,
        })
    }

    pub fn set_speed(&mut self, speed: f64) -> Result<(), PWMMotorError> {
        self.output
            .set_pwm_frequency(self.frequency, self.constrain(speed)?)
            .context(DutyCycleFrequencyUnableSnafu)?;
        Ok(())
    }

    pub fn constrain(&self, duty: f64) -> Result<f64, PWMMotorError> {
        ensure!(
            (-1.0..=1.0).contains(&duty),
            DutyCycleOutOfBoundsSnafu {
                motor_pin: self.output.pin(),
                duty
            }
        );
        Ok(((self.forward_duty - self.reverse_duty) / 2.) * duty
            + ((self.reverse_duty + self.forward_duty - 2. * self.idle_duty) / 2.) * duty.abs()
            + self.idle_duty)
    }
}
