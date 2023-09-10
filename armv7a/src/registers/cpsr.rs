use core::arch::asm;

tock_registers::register_bitfields! {
    u32,
    pub CPSR [
        M OFFSET(0) NUMBITS(5) [
            USR = 0x10,
            FIQ = 0x11,
            IRQ = 0x12,
            SVC = 0x13,
            MON = 0x16,
            ABT = 0x17,
            HYP = 0x1A,
            UND = 0x1B,
            SYS = 0x1F
        ],

        T OFFSET(5) NUMBITS(1) [],

        F OFFSET(6) NUMBITS(1) [
            NotMasked = 0,
            Masked = 1
        ],

        I OFFSET(7) NUMBITS(1) [
            NotMasked = 0,
            Masked = 1
        ],

        A OFFSET(8) NUMBITS(1) [
            NotMasked = 0,
            Masked = 1
        ],

        E OFFSET(9) NUMBITS(1) [
            LittleEndian = 0,
            BigEndian = 1
        ],

        IT_High OFFSET(10) NUMBITS(6) [],
        GE OFFSET(16) NUMBITS(4) [],
        J OFFSET(24) NUMBITS(1) [],
        IT_Low OFFSET(25) NUMBITS(2) [],
        Q OFFSET(27) NUMBITS(1) [],
        V OFFSET(28) NUMBITS(1) [],
        C OFFSET(29) NUMBITS(1) [],
        Z OFFSET(30) NUMBITS(1) [],
        N OFFSET(31) NUMBITS(1) [],
    ]
}

pub struct CPSRAccessor;
pub const CPSR: CPSRAccessor = CPSRAccessor;

impl tock_registers::interfaces::Readable for CPSRAccessor {
    type T = u32;
    type R = CPSR::Register;

    fn get(&self) -> Self::T {
        let value: u32;
        unsafe {
            asm!("mrs {}, cpsr", out(reg) value);
        }
        value
    }
}

impl tock_registers::interfaces::Writeable for CPSRAccessor {
    type T = u32;
    type R = CPSR::Register;

    fn set(&self, value: Self::T) {
        unsafe {
            asm!("msr cpsr, {}", in(reg) value);
        }
    }
}
