#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::{arch::asm, panic::PanicInfo};

use armv7a::{startup, EntryPoint, MemoryMap, MemoryRegion, NORMAL};

// Generates the global start symbol
startup!(System);
struct System;

// Initial memory management
impl MemoryMap for System {
    // Map the binary image into virtual memory with a unit mapping (phys addr == virt addr),
    // so that we can keep executing after enabling the MMU.
    const MAP: &'static [armv7a::MemoryRegion] = &[MemoryRegion::image(
        // Image sections need to be read- and write-able as well executable,
        // otherwise a data or prefetch abort will occur.
        NORMAL.read_writeable().executeable(),
    )];
}

impl EntryPoint for System {
    fn main() -> ! {
        // Here we are running from virtual memory

        loop {
            unsafe { asm!("nop") }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
