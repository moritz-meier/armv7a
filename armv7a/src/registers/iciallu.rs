pub struct ICIALLUAccessor;
pub const ICIALLU: ICIALLUAccessor = ICIALLUAccessor;

impl tock_registers::interfaces::Writeable for ICIALLUAccessor {
    type T = u32;
    type R = ();

    coproc_write_raw!(p15, 0, value, c7, c5, 0);
}
