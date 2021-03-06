#![no_std]

extern crate hifive;

fn main() {}

/*

use hifive::{Peripherals, Channel, Align, Pwm};

const RED: Channel = Channel::_3;
const GREEN: Channel = Channel::_1;
const BLUE: Channel = Channel::_2;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn from(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
}

fn set_color(pwm: Pwm<PWM1>, color: Color) {
    pwm.set_cmp(RED, u16::max_value() / 255 * color.red as u16);
    pwm.set_cmp(GREEN, u16::max_value() / 255 * color.green as u16);
    pwm.set_cmp(BLUE, u16::max_value() / 255 * color.blue as u16);
}

fn main() {
    let p = Peripherals::take().unwrap();

    //let clint = Clint(peripherals.CLINT);
    let pwm = Pwm(&p.PWM1);

    pwm.set_cmp(Channel::_0, u16::max_value());
    //pwm.set_period(63);

    pwm.enable(RED, Align::Left, &p.GPIO0);
    pwm.invert(RED, &p.GPIO0);
    pwm.set_cmp(RED, u16::max_value() / 3);

    pwm.enable(GREEN, Align::Center, &p.GPIO0);
    pwm.invert(GREEN, &p.GPIO0);
    pwm.set_cmp(GREEN, u16::max_value() / 2);

    pwm.enable(BLUE, Align::Right, &p.GPIO0);
    pwm.invert(BLUE, &p.GPIO0);
    pwm.set_cmp(BLUE, u16::max_value() / 3 * 2);

    pwm.init();

    //let delay = 1.s();

    loop {
        // Gray
        set_color(pwm, Color::from(0x80, 0x80, 0x80));
        clint.set_timeout(delay);
        block!(clint.wait());
        // Purple
        set_color(pwm, Color::from(0x80, 0x00, 0x80));
        clint.set_timeout(delay);
        block!(clint.wait());
        // Maroon
        set_color(pwm, Color::from(0x80, 0x00, 0x00));
        clint.set_timeout(delay);
        block!(clint.wait());
    }

    //pwm.invert(GREEN, gpio, true);
    //pwm.align_center(GREEN);
    //pwm.set_cmp(GREEN, u16::max_value() / 2);

    //pwm.align_left(RED);
    //pwm.align_right(BLUE);
    //pwm.set_cmp(BLUE, u16::max_value() / 3  * 2);
    //pwm.set_cmp(BLUE, 0);
    //pwm.enable(BLUE, gpio);
}
*/
