#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::{arch::asm, panic::PanicInfo};

use armv7a::{startup, EntryPoint, MemoryMap, MemoryRegion, NORMAL};

startup!(System);
struct System;

impl MemoryMap for System {
    const MAP: &'static [armv7a::MemoryRegion] = &[MemoryRegion::Image {
        attrs: NORMAL.read_writeable().executeable(),
    }];
}

impl EntryPoint for System {
    fn main() -> ! {
        loop {
            unsafe { asm!("nop") }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
