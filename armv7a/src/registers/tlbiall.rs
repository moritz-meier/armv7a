pub struct TLBIALLAccessor;
pub const TLBIALL: TLBIALLAccessor = TLBIALLAccessor;

impl tock_registers::interfaces::Writeable for TLBIALLAccessor {
    type T = u32;
    type R = ();

    coproc_write_raw!(p15, 0, value, c8, c7, 0);
}
