tock_registers::register_bitfields! {u32,
    #[allow(clippy::enum_variant_names)]
    pub TTBR0 [
        IRGN1 OFFSET(1) NUMBITS(1) [],

        S OFFSET(1) NUMBITS(1) [
            NonShareable = 0,
            Shareable = 1
        ],

        RGN OFFSET(3) NUMBITS(2) [
            OuterNonCacheable = 0b00,
            OuterWriteBackWriteAllocate = 0b01,
            OuterWriteThrough = 0b10,
            OuterWriteBackNoWriteAllocate = 0b11
        ],
        NOS OFFSET(5) NUMBITS(1) [
            OuterShareable = 0,
            InnerShareable = 1
        ],
        IRGN0 OFFSET(6) NUMBITS(1) [],
        BASE OFFSET(14) NUMBITS(18) [],
    ]
}

pub struct TTBR0Accessor;

#[allow(unused)]
pub const TTBR0: TTBR0Accessor = TTBR0Accessor;

impl tock_registers::interfaces::Readable for TTBR0Accessor {
    type T = u32;
    type R = TTBR0::Register;
    coproc_read_raw!(p15, 0, value, c2, c0, 0);
}

impl tock_registers::interfaces::Writeable for TTBR0Accessor {
    type T = u32;
    type R = TTBR0::Register;
    coproc_write_raw!(p15, 0, value, c2, c0, 0);
}
