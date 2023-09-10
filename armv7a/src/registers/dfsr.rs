pub struct DFSRAccessor;

#[allow(unused)]
pub const DFSR: DFSRAccessor = DFSRAccessor;

impl tock_registers::interfaces::Readable for DFSRAccessor {
    type T = u32;
    type R = ();
    coproc_read_raw!(p15, 0, value, c5, c0, 0);
}
