#![no_std]
#![no_main]

#[macro_use]
extern crate pg;
use core::ptr;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x48001018;
        const GPIOE_ODR: u32 = 0x4800_1014;

        iprintln!(
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        // Turn on the "North" LED (red)
        // *(GPIOE_BSRR as *mut u32) = 1 << 9;
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        iprintln!(
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        // Turn on the "East" LED (green)
        // *(GPIOE_BSRR as *mut u32) = 1 << 11;
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
        iprintln!(
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        // Turn off the "North" LED
        // *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        iprintln!(
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        // Turn off the "East" LED
        // *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
        iprintln!(
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        // 0xBAAAAAAD address
        // ptr::read_volatile(0x4800_1800 as *const u32);
    }

    loop {}
}
