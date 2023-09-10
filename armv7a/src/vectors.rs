use core::arch::asm;

#[naked]
#[no_mangle]
#[link_section = ".vectors"]
pub unsafe extern "C" fn vectors() {
    asm!(
        "b {reset_handler}",
        "b {und_handler}",
        "b {svc_handler}",
        "b {pabt_handler}",
        "b {dabt_handler}",
        "nop",
        "b {irq_handler}",
        "b {fiq_handler}",
        reset_handler = sym reset_handler,
        und_handler = sym und_handler,
        svc_handler = sym svc_handler,
        pabt_handler = sym pabt_handler,
        dabt_handler = sym dabt_handler,
        irq_handler = sym irq_handler,
        fiq_handler = sym fiq_handler,
        options(noreturn)
    );
}

#[naked]
#[no_mangle]
#[link_section = ".vectors"]
pub unsafe extern "C" fn reset_handler() {
    asm!("b .", options(noreturn))
}

#[naked]
#[no_mangle]
#[link_section = ".vectors"]
pub unsafe extern "C" fn und_handler() {
    asm!("b .", options(noreturn))
}

#[naked]
#[no_mangle]
#[link_section = ".vectors"]
pub unsafe extern "C" fn svc_handler() {
    asm!("b .", options(noreturn))
}

#[naked]
#[no_mangle]
#[link_section = ".vectors"]
pub unsafe extern "C" fn pabt_handler() {
    asm!("b .", options(noreturn))
}

#[naked]
#[no_mangle]
#[link_section = ".vectors"]
pub unsafe extern "C" fn dabt_handler() {
    asm!("b .", options(noreturn))
}

#[naked]
#[no_mangle]
#[link_section = ".vectors"]
pub unsafe extern "C" fn __dummy__() {
    asm!("b .", options(noreturn))
}

#[naked]
#[no_mangle]
#[link_section = ".vectors"]
pub unsafe extern "C" fn irq_handler() {
    asm!("b .", options(noreturn))
}

#[naked]
#[no_mangle]
#[link_section = ".vectors"]
pub unsafe extern "C" fn fiq_handler() {
    asm!("b .", options(noreturn))
}
