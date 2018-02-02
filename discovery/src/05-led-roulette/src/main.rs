#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate pg;

use pg::delay;
use pg::led::LEDS;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let half_period = 50; // ms

    LEDS[0].on();
    let mut idx = 1;
    loop {
        LEDS[idx % 8].on();
        delay::ms(half_period);
        LEDS[(idx - 1) % 8].off();
        delay::ms(half_period);
        idx += 1;
    }

    // loop {
    //     let idxm1 = (idx - 1) % 8;
    //     idx = idx % 8;
    //     let idxp1 = (idx + 1) % 8;
    //     delay::ms(half_period);
    //     LEDS[idxm1].off();
    //     LEDS[idx].on();
    //     LEDS[idxp1].on();
    //     idx += 1;
    // }

    // let half_period = 500; // ms
    //
    // loop {
    //     LEDS[0].on();
    //     delay::ms(half_period);
    //
    //     LEDS[0].off();
    //     delay::ms(half_period);
    // }
    // let y;
    // let x = 42;
    // y = x;
    //
    // loop {}
}
