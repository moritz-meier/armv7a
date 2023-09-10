use core::arch::asm;

#[inline(always)]
pub fn dsb() {
    unsafe {
        asm!("dsb 0xf");
    }
}

#[inline(always)]
pub fn isb() {
    unsafe {
        asm!("isb 0xf");
    }
}

#[inline(always)]
pub fn dmb() {
    unsafe {
        asm!("dmb 0xf");
    }
}
