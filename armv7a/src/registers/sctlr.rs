tock_registers::register_bitfields! {
    u32,
    pub SCTLR [
        /// MMU enable/disable
        M OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Alignment fault checking enable/disable
        A OFFSET(1) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Data and unified caches enable/disable
        C OFFSET(2) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Branch prediction enable/disable
        Z OFFSET(11) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Instruction cache enable/disable
        I OFFSET(12) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Vector bits
        V OFFSET(13) NUMBITS(1) [
            LowVectors = 0,
            HighVectors = 1
        ],

        /// TEX remap enable/disable
        TRE OFFSET(28) NUMBITS(1) [
            DisableTEXRemap = 0,
            EnableTEXRemap = 1
        ],

        /// Access flag enable/disable
        AFE OFFSET(29) NUMBITS(1) [
            ThreeBitAccessPermissions = 0,
            TwoBitAccessPermissions = 1
        ],

        /// Thumb exception enable/disable
        TE OFFSET(30) NUMBITS(1) [
            Arm = 0,
            Thumb = 1
        ]
    ]
}

pub struct SCTLRAccessor;

#[allow(unused)]
pub const SCTLR: SCTLRAccessor = SCTLRAccessor;

impl tock_registers::interfaces::Readable for SCTLRAccessor {
    type T = u32;
    type R = SCTLR::Register;
    coproc_read_raw!(p15, 0, value, c1, c0, 0);
}

impl tock_registers::interfaces::Writeable for SCTLRAccessor {
    type T = u32;
    type R = SCTLR::Register;
    coproc_write_raw!(p15, 0, value, c1, c0, 0);
}
