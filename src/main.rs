use rppal::gpio::Gpio;

const GPIO_PWM: u8 = 12;

fn main() {
    println!("Hello, world!");
    let mut pin = Gpio::new().expect("It no worky").get(GPIO_PWM).expect("IT REALLY NO WORKY").into_output();
    pin.set_pwm_frequency(500.0, 0.60).expect("Im so sorry, it not work :(");

    loop {}
}
