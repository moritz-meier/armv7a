pub struct DCISWAccessor;
pub const DCISW: DCISWAccessor = DCISWAccessor;

impl tock_registers::interfaces::Writeable for DCISWAccessor {
    type T = u32;
    type R = ();
    coproc_write_raw!(p15, 0, value, c7, c6, 2);
}
