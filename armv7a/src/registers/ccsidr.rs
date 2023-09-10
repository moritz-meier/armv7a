tock_registers::register_bitfields! {u32,
    pub CCSIDR [
        LineSize OFFSET(0) NUMBITS(3) [],
        Associativity OFFSET(3) NUMBITS(10) [],
        NumSets OFFSET(13) NUMBITS(15) [],
    ]
}

pub struct CCSIDRAccessor;
pub const CCSIDR: CCSIDRAccessor = CCSIDRAccessor;

impl tock_registers::interfaces::Readable for CCSIDRAccessor {
    type T = u32;
    type R = CCSIDR::Register;
    coproc_read_raw!(p15, 1, value, c0, c0, 0);
}
