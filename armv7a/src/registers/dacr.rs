tock_registers::register_bitfields! {u32,
    pub DACR [
        D0 OFFSET(0) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D1 OFFSET(2) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D2 OFFSET(4) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D3 OFFSET(6) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D4 OFFSET(8) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D5 OFFSET(10) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D6 OFFSET(12) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D7 OFFSET(14) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D8 OFFSET(16) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D9 OFFSET(18) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D10 OFFSET(20) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D11 OFFSET(22) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D12 OFFSET(24) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D13 OFFSET(26) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D14 OFFSET(28) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],

        D15 OFFSET(30) NUMBITS(2) [
            NoAccess = 0b00,
            Client = 0b01,
            Manager = 0b11
        ],
    ]
}

pub struct DACRAccessor;

#[allow(unused)]
pub const DACR: DACRAccessor = DACRAccessor;

impl tock_registers::interfaces::Readable for DACRAccessor {
    type T = u32;
    type R = DACR::Register;
    coproc_read_raw!(p15, 0, value, c3, c0, 0);
}

impl tock_registers::interfaces::Writeable for DACRAccessor {
    type T = u32;
    type R = DACR::Register;
    coproc_write_raw!(p15, 0, value, c3, c0, 0);
}
