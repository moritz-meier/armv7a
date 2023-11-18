#[macropol::macropol]
macro_rules! coproc_read_raw {
    ($cp:ident, $opc1:literal, value, $crn:ident, $crm:ident, $opc2:literal) => {
        /// `mrc $&cp, $&opc1, {out_reg}, $&crn, $&crm, $&opc2`
        #[inline]
        fn get(&self) -> u32 {
            let reg;
            unsafe {
                core::arch::asm!(
                    "mrc $&cp, $&opc1, {}, $&crn, $&crm, $&opc2",
                    lateout(reg) reg,
                );
            }
            reg
        }
    };
}
/// Implements `register::cpu::RegisterReadWrite::set`.
#[macropol::macropol]
macro_rules! coproc_write_raw {
    ($cp:ident, $opc1:literal, value, $crn:ident, $crm:ident, $opc2:literal) => {
        /// `mcr $&cp, $&opc1, {in_reg}, $&crn, $&crm, $&opc2`
        #[inline]
        fn set(&self, value: u32) {
            unsafe {
                core::arch::asm!(
                    "mcr $&cp, $&opc1, {}, $&crn, $&crm, $&opc2",
                    in(reg) value,
                );
            }
        }
    };
}

mod cache_ops;
mod ccsidr;
mod clidr;
mod cpsr;
mod csselr;
mod dacr;
mod dfsr;
mod sctlr;
mod tlbiall;
mod ttbcr;
mod ttbr0;

pub use cache_ops::*;
pub use ccsidr::*;
pub use clidr::*;
pub use cpsr::*;
pub use csselr::*;
pub use dacr::*;
pub use dfsr::*;
pub use sctlr::*;
pub use tlbiall::*;
pub use ttbcr::*;
pub use ttbr0::*;

pub use tock_registers::interfaces::*;
