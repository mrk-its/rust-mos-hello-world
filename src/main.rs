#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use ufmt_stdio::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    println!("PANIC!!!");
    loop {}
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    println!("Hello {}!", 6502);
    0
}
