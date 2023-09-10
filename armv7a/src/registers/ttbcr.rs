tock_registers::register_bitfields! {u32,
    pub TTBCR [
        N OFFSET(0) NUMBITS(3) [],
        PD0 OFFSET(4) NUMBITS(1) [
            Default = 0,
            Fault = 1
        ],
        PD1 OFFSET(5) NUMBITS(1) [
            Default = 0,
            Fault = 1
        ],
        EAE OFFSET(31) NUMBITS(1) []
    ]
}

pub struct TTBCRAccessor;

#[allow(unused)]
pub const TTBCR: TTBCRAccessor = TTBCRAccessor;

impl tock_registers::interfaces::Readable for TTBCRAccessor {
    type T = u32;
    type R = TTBCR::Register;
    coproc_read_raw!(p15, 0, value, c2, c0, 2);
}

impl tock_registers::interfaces::Writeable for TTBCRAccessor {
    type T = u32;
    type R = TTBCR::Register;
    coproc_write_raw!(p15, 0, value, c2, c0, 2);
}
