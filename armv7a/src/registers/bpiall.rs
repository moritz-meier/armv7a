pub struct BPIALLAccessor;
pub const BPIALL: BPIALLAccessor = BPIALLAccessor;

impl tock_registers::interfaces::Writeable for BPIALLAccessor {
    type T = u32;
    type R = ();

    coproc_write_raw!(p15, 0, value, c7, c5, 6);
}
