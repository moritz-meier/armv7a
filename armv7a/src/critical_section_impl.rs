use core::arch::asm;

use critical_section::{set_impl, RawRestoreState};
use tock_registers::interfaces::Readable;

use crate::registers::CPSR;

struct SingleCoreCriticalSection;

#[cfg(feature = "critical-section-single-core")]
set_impl!(SingleCoreCriticalSection);

unsafe impl critical_section::Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let restore_cpsr = CPSR.get();
        unsafe { asm!("cpsid if") };
        restore_cpsr
    }

    unsafe fn release(restore_cpsr: RawRestoreState) {
        if !CPSR::I.is_set(restore_cpsr) {
            unsafe { asm!("cpsie i") }
        }

        if !CPSR::F.is_set(restore_cpsr) {
            unsafe { asm!("cpsie f") }
        }
    }
}

/*struct MultiCoreCriticalSection;

#[cfg(feature = "critical-section-multi-core")]
set_impl!(MultiCoreCriticalSection);

static mut LOCK: u32 = 0;

unsafe fn spin_lock(lock: *mut u32) {
    asm!(
        "mov r2, #1",
        "1: ldrex r1, [{lock}]",
        "cmp r1, #0",
        "wfene",
        "strexeq r1, r2, [{lock}]",
        "cmpeq r1, #0",
        "bne 1b",
        "dmb",
        lock = in(reg) lock
    )
}

unsafe fn spin_unlock(lock: *mut u32) {
    asm!(
        "mov r1, #0",
        "dmb",
        "str r1, [{lock}]",
        "dsb",
        "sev",
        lock = in(reg) lock
    )
}

unsafe impl critical_section::Impl for MultiCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let restore_cpsr = CPSR.get();
        spin_lock(&mut LOCK);
        unsafe { asm!("cpsid if") };
        restore_cpsr
    }

    unsafe fn release(restore_cpsr: RawRestoreState) {
        if !CPSR::I.is_set(restore_cpsr) {
            unsafe { asm!("cpsie i") }
        }

        if !CPSR::F.is_set(restore_cpsr) {
            unsafe { asm!("cpsie f") }
        }

        spin_unlock(&mut LOCK);
    }
}*/
