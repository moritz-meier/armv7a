tock_registers::register_bitfields! {u32,
    pub CSSELR [
        InD OFFSET(0) NUMBITS(1) [
            DataOrUnified = 0,
            Instruction = 1
        ],

        Level OFFSET(1) NUMBITS(3) []
    ]
}

pub struct CSSELRAccessor;
pub const CSSELR: CSSELRAccessor = CSSELRAccessor;

impl tock_registers::interfaces::Readable for CSSELRAccessor {
    type T = u32;
    type R = CSSELR::Register;
    coproc_read_raw!(p15, 2, value, c0, c0, 0);
}

impl tock_registers::interfaces::Writeable for CSSELRAccessor {
    type T = u32;
    type R = CSSELR::Register;
    coproc_write_raw!(p15, 2, value, c0, c0, 0);
}
