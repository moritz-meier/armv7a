#![no_std]
#![feature(naked_functions)]
#![feature(inline_const)]
#![feature(const_trait_impl)]
#![feature(pointer_is_aligned)]
#![feature(const_pointer_is_aligned)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::empty_loop)]
#![allow(dead_code)]

mod asm;
mod cache;
mod critical_section_impl;
mod mmu;
mod registers;
mod vectors;

use core::arch::asm;

use asm::*;
use registers::*;

pub use mmu::*;

#[macro_export]
macro_rules! startup {
    ($SystemImpl:ty) => {
        // Global start entrypoint
        #[naked]
        #[no_mangle]
        pub unsafe extern "C" fn _start() {
               ::core::arch::asm!("b {}", sym $crate::start::<$SystemImpl>, options(noreturn));
        }
    };
}

pub trait System: EntryPoint + MemoryMap {}

pub trait EntryPoint {
    fn main() -> !;
}

#[naked]
pub unsafe extern "C" fn start<SystemImpl: EntryPoint + MemoryMap>() -> ! {
    asm!(
        "cpsid if",

        "sev",

        // Put any cores other than 0 to sleep
        "mrc p15, 0, r0, c0, c0, 5",
        "ands r0, r0, #3",
        "1: wfine",
        "bne 1b",

        // Reset SCTLR settings
        "mrc p15, 0, r0, c1, c0, 0",    // Read CP15 System Control register
        "bic r0, r0, #(0x1 << 12)",     // Clear I bit 12 to disable I cache
        "bic r0, r0, #(0x1 << 2)",      // Clear C bit  2 to disable D cache
        "bic r0, r0, #0x1",             // Clear M bit  0 to disable MMU
        "bic r0, r0, #(0x1 << 11)",     // Clear Z bit 11 to disable branch prediction
        "bic r0, r0, #(0x1 << 13)",     // Clear V bit 13 to disable high vectors
        "mcr p15, 0, r0, c1, c0, 0",    // Write value back to CP15 System Control register
        "isb",

        "mrc p15, 0, r0, c1, c0, 1",    // Read CP15 Auxiliary Control Register
        "orr r0, r0, #(1 <<  1)",       // Enable L2 prefetch hint (UNK/WI since r4p1)
        "mcr p15, 0, r0, c1, c0, 1",    // Write CP15 Auxiliary Control Register"

        "ldr r0, ={vectors}",           // Set Vector Base Address Register (VBAR)
        "mcr p15, 0, r0, c12, c0, 0",

        "cps #0x11",
        "ldr sp, =__fiq_stack_end",

        "cps #0x12",
        "ldr sp, =__irq_stack_end",

        "cps #0x13",
        "ldr sp, =__svc_stack_end",

        "cps #0x17",
        "ldr sp, =__abt_stack_end",

        "cps #0x1B",
        "ldr sp, =__und_stack_end",

        "cps #0x1F",
        "ldr sp, =__sys_stack_end",

        // Zero .bss section
        "mov r0, #0",
        "ldr r1, =__bss_start",
        "ldr r2, =__bss_end",
        "2: cmp r1, r2",
        "strlt r0, [r1], #4",
        "blt 2b",

        // Zero .stack section
        "movw r0, #0xFEFE",
        "movt r0, #0xFEFE",
        "ldr r1, =__sys_stack_start",
        "ldr r2, =__und_stack_end",
        "3: cmp r1, r2",
        "strlt r0, [r1], #4",
        "blt 3b",

        "bl {init}",
        "cpsie if",
        "bl {main}",
        "b .",
        vectors = sym vectors::vectors,
        init = sym init::<SystemImpl>,
        main = sym main::<SystemImpl>,
        options(noreturn)
    );
}

extern "C" fn init<SystemImpl: MemoryMap>() {
    // Invalidate entire Unified TLB
    TLBIALL.set(0);

    cache::invalidate_branch_prediction();
    cache::invalidate_icache_all();
    cache::invalidate_dcache_all();

    // Map initial regions
    for region in SystemImpl::MAP {
        region.map();
    }

    let level1_page_table_ptr = SystemImpl::level1_page_table_ptr();

    // Set translation table base
    TTBR0.write(
        TTBR0::IRGN1::CLEAR // Inner Write Back Write Allocate
            + TTBR0::S::NonShareable
            + TTBR0::RGN::OuterWriteBackWriteAllocate
            + TTBR0::NOS::CLEAR // Ignored by S == NonShareable
            + TTBR0::IRGN0::SET // Inner Write Back Write Allocate
            + TTBR0::BASE.val((level1_page_table_ptr as u32) >> 14),
    );
    isb();

    // Only use D0 client domain
    DACR.write(DACR::D0::Client);
    isb();

    // Enable caches + mmu
    // No TEX remapping
    // Simplified (2-bit) access control
    SCTLR.modify(
        SCTLR::M::Enable
            + SCTLR::A::Disable
            + SCTLR::C::Enable
            + SCTLR::Z::Enable
            + SCTLR::I::Enable
            + SCTLR::TRE::DisableTEXRemap
            + SCTLR::AFE::TwoBitAccessPermissions,
    );
    isb();
}

extern "C" fn main<SystemImpl: EntryPoint>() -> ! {
    SystemImpl::main();
}
