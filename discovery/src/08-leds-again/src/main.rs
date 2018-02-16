#![no_std]
#![no_main]
#[macro_use]
extern crate pg;

use pg::peripheral;
use core::ptr;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let (gpioe, rcc) = unsafe { (peripheral::gpioe_mut(), peripheral::rcc_mut()) };

    // TODO initialize GPIOE
    // rcc.apb1enr.write(|w| w.iopeen(true));
    // rcc.apb1enr.write(|w| w.enr(true));
    // rcc.apb2enr.modify(|w| w);
    // unsafe {
    //     // rcc.enr();
    //     rcc.ahbenr.enr();
    // }
    // rcc.ahbenr;
    unsafe {
        const RCC: u32 = 0x4002_1000;
        const RCC_AHBENR: u32 = 0x14;

        const RCC_AHBENR_IOPEEN: u32 = 1 << 21;
        // ptr::write_volatile(rcc.ahbenr, 1 << 20);
        // const APB1ENR: u32 = 0x4002101C;
        // const APBENR: u32 = 0x40021014;
        ptr::write_volatile((RCC + RCC_AHBENR) as *mut u32, RCC_AHBENR_IOPEEN);
        // ptr::write_volatile(APB1ENR as *mut u32, 28 << 1);
        // ptr::write_volatile(APBENR as *mut u32, 21 << 1);
        // ptr::write_volatile(APB1ENR as *mut u32, 1 << 28);
        // ptr::write_volatile(APBENR as *mut u32, 1 << 20);
        // iprintln!(
        //     "APB1ENR ODR = 0x{:032x}",
        //     ptr::read_volatile(APB1ENR as *const u32)
        // );
        // iprintln!(
        //     "APBENR ODR = 0x{:032x}",
        //     ptr::read_volatile(APBENR as *const u32)
        // );
    }

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8(true)
            .odr9(true)
            .odr10(true)
            .odr11(true)
            .odr12(true)
            .odr13(true)
            .odr14(true)
            .odr15(true)
    });

    loop {}
}
