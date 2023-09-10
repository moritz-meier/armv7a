tock_registers::register_bitfields! {u32,
    pub CLIDR [
        Ctype1 OFFSET(0) NUMBITS(3) [
            NoCache = 0b000,
            Instruction = 0b001,
            Data = 0b010,
            InstructionAndData = 0b011,
            Unified = 0b100
        ],

        Ctype2 OFFSET(3) NUMBITS(3) [
            NoCache = 0b000,
            Instruction = 0b001,
            Data = 0b010,
            InstructionAndData = 0b011,
            Unified = 0b100
        ],

        Ctype3 OFFSET(6) NUMBITS(3) [
            NoCache = 0b000,
            Instruction = 0b001,
            Data = 0b010,
            InstructionAndData = 0b011,
            Unified = 0b100
        ],

        Ctype4 OFFSET(9) NUMBITS(3) [
            NoCache = 0b000,
            Instruction = 0b001,
            Data = 0b010,
            InstructionAndData = 0b011,
            Unified = 0b100
        ],

        Ctype5 OFFSET(12) NUMBITS(3) [
            NoCache = 0b000,
            Instruction = 0b001,
            Data = 0b010,
            InstructionAndData = 0b011,
            Unified = 0b100
        ],

        Ctype6 OFFSET(15) NUMBITS(3) [
            NoCache = 0b000,
            Instruction = 0b001,
            Data = 0b010,
            InstructionAndData = 0b011,
            Unified = 0b100
        ],

        Ctype7 OFFSET(18) NUMBITS(3) [
            NoCache = 0b000,
            Instruction = 0b001,
            Data = 0b010,
            InstructionAndData = 0b011,
            Unified = 0b100
        ],

        LoUIS OFFSET(21) NUMBITS(3) [],
        LoC OFFSET(24) NUMBITS(3) [],
        LoUU OFFSET(27) NUMBITS(3) []
    ]
}

pub struct CLIDRAccessor;
pub const CLIDR: CLIDRAccessor = CLIDRAccessor;

impl tock_registers::interfaces::Readable for CLIDRAccessor {
    type T = u32;
    type R = CLIDR::Register;
    coproc_read_raw!(p15, 1, value, c0, c0, 1);
}
