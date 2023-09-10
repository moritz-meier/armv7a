tock_registers::register_bitfields! {
    u32,
    pub(super) SectionEntry [
        PXN OFFSET(0) NUMBITS(1) [],
        B OFFSET(2) NUMBITS(1) [],
        C OFFSET(3) NUMBITS(1) [],
        CB OFFSET(2) NUMBITS(2) [
            StronglyOrdered = 0,
            Device = 1
        ],
        AA OFFSET(2) NUMBITS(2) [
            NonCacheable = 0b00,
            WriteBackWriteAllocate = 0b01,
            WriteThroughNoWriteAllocate = 0b10,
            WriteBackNoWriteAllocate = 0b11
        ],
        XN OFFSET(4) NUMBITS(1) [
            Executeable = 0,
            ExecuteNever = 1
        ],
        DOMAIN OFFSET(5) NUMBITS(4) [],
        AP OFFSET(10) NUMBITS(2) [
            PL0 = 0b11,
            PL1 = 0b01
        ],
        BB OFFSET(12) NUMBITS(2) [
            NonCacheable = 0b00,
            WriteBackWriteAllocate = 0b01,
            WriteThroughNoWriteAllocate = 0b10,
            WriteBackNoWriteAllocate = 0b11
        ],
        TEX OFFSET(12) NUMBITS(3) [
            DeviceOrStronglyOrdered = 0,
            Normal = 0b100
        ],
        APX OFFSET(15) NUMBITS(1) [
            EnableWrite = 0,
            DisableWrite = 1
        ],
        S OFFSET(16) NUMBITS(1) [
            NonShareable = 0,
            Shareable = 1
        ],
        NG OFFSET(17) NUMBITS(1) [],
        NS OFFSET(19) NUMBITS(1) [],
        BASE_ADDR OFFSET(20) NUMBITS(12) []
    ],

    pub(super) Level2PageTableEntry [
        PXN OFFSET(2) NUMBITS(1) [],
        NS OFFSET(3) NUMBITS(1) [],
        DOMAIN OFFSET(5) NUMBITS(4) [],
        BASE_ADDR OFFSET(10) NUMBITS(22) [],
    ],
}

tock_registers::register_bitfields! {
    u32,
    pub(super) SmallPageEntry [
        XN OFFSET(0) NUMBITS(1) [
            Executeable = 0,
            ExecuteNever = 1
        ],
        B OFFSET(2) NUMBITS(1) [],
        C OFFSET(3) NUMBITS(1) [],
        CB OFFSET(2) NUMBITS(2) [
            StronglyOrdered = 0,
            Device = 1
        ],
        AA OFFSET(2) NUMBITS(2) [
            NonCacheable = 0b00,
            WriteBackWriteAllocate = 0b01,
            WriteThroughNoWriteAllocate = 0b10,
            WriteBackNoWriteAllocate = 0b11
        ],
        AP OFFSET(4) NUMBITS(2) [
            PL0 = 0b11,
            PL1 = 0b01
        ],
        BB OFFSET(6) NUMBITS(2) [
            NonCacheable = 0b00,
            WriteBackWriteAllocate = 0b01,
            WriteThroughNoWriteAllocate = 0b10,
            WriteBackNoWriteAllocate = 0b11
        ],
        TEX OFFSET(6) NUMBITS(3) [
            DeviceOrStronglyOrdered = 0,
            Normal = 0b100
        ],
        APX OFFSET(9) NUMBITS(1) [
            EnableWrite = 0,
            DisableWrite = 1
        ],
        S OFFSET(10) NUMBITS(1) [
            NonShareable = 0,
            Shareable = 1
        ],
        NG OFFSET(11) NUMBITS(1) [],
        BASE_ADDR OFFSET(12) NUMBITS(20) [],
    ]
}
