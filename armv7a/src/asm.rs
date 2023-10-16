use core::arch::asm;
use core::sync::atomic::{compiler_fence, Ordering};

#[inline(always)]
pub fn dmb() {
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("dmb", options(nomem, nostack, preserves_flags)) };
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub fn dsb() {
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("dsb", options(nomem, nostack, preserves_flags)) };
    compiler_fence(Ordering::SeqCst);
}

#[inline(always)]
pub fn isb() {
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("isb", options(nomem, nostack, preserves_flags)) };
    compiler_fence(Ordering::SeqCst);
}
