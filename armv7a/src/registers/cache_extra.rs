struct CacheHierarchy {
    level_of_unification_uniprocessor: CacheLevel,
    level_of_coherency: CacheLevel,
    level_of_unification_inner_shareable: CacheLevel,

    levels: [Option<CacheType>; 7],
}

impl CacheHierarchy {
    fn get() -> Self {
        let clidr = CLIDR.extract();

        let mut levels: [Option<CacheType>; 7] = [None; 7];

        CacheLevel::LEVELS
            .into_iter()
            .map(|level| (level, Self::read_cache_type(level)))
            .take_while(|(_, typ)| typ.is_some())
            .for_each(|(level, typ)| levels[level as usize] = typ);

        Self {
            level_of_unification_uniprocessor: clidr
                .read_as_enum::<CLIDR::LoUU::Value>(CLIDR::LoUU)
                .unwrap()
                .into(),

            level_of_coherency: clidr
                .read_as_enum::<CLIDR::LoUU::Value>(CLIDR::LoC)
                .unwrap()
                .into(),

            level_of_unification_inner_shareable: clidr
                .read_as_enum::<CLIDR::LoUU::Value>(CLIDR::LoUIS)
                .unwrap()
                .into(),

            levels,
        }
    }

    fn read_cache_type(level: CacheLevel) -> Option<CacheType> {
        CLIDR
            .read_as_enum::<CLIDR::Ctype1::Value>(match level {
                CacheLevel::Level1 => CLIDR::Ctype1,
                CacheLevel::Level2 => CLIDR::Ctype2,
                CacheLevel::Level3 => CLIDR::Ctype3,
                CacheLevel::Level4 => CLIDR::Ctype4,
                CacheLevel::Level5 => CLIDR::Ctype5,
                CacheLevel::Level6 => CLIDR::Ctype6,
                CacheLevel::Level7 => CLIDR::Ctype7,
            })
            .and_then(CLIDR::Ctype1::Value::into)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CacheLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
}

impl CacheLevel {
    const LEVELS: [Self; CacheLevel::Level7 as usize + 1] = [
        CacheLevel::Level1,
        CacheLevel::Level2,
        CacheLevel::Level3,
        CacheLevel::Level4,
        CacheLevel::Level5,
        CacheLevel::Level6,
        CacheLevel::Level7,
    ];
}

impl From<CLIDR::LoUU::Value> for CacheLevel {
    fn from(level: CLIDR::LoUU::Value) -> Self {
        match level {
            CLIDR::LoUU::Value::Level1 => CacheLevel::Level1,
            CLIDR::LoUU::Value::Level2 => CacheLevel::Level2,
            CLIDR::LoUU::Value::Level3 => CacheLevel::Level3,
            CLIDR::LoUU::Value::Level4 => CacheLevel::Level4,
            CLIDR::LoUU::Value::Level5 => CacheLevel::Level5,
            CLIDR::LoUU::Value::Level6 => CacheLevel::Level6,
            CLIDR::LoUU::Value::Level7 => CacheLevel::Level7,
        }
    }
}

impl From<CacheLevel> for CSSELR::Level::Value {
    fn from(level: CacheLevel) -> Self {
        match level {
            CacheLevel::Level1 => CSSELR::Level::Value::Level1,
            CacheLevel::Level2 => CSSELR::Level::Value::Level2,
            CacheLevel::Level3 => CSSELR::Level::Value::Level3,
            CacheLevel::Level4 => CSSELR::Level::Value::Level4,
            CacheLevel::Level5 => CSSELR::Level::Value::Level5,
            CacheLevel::Level6 => CSSELR::Level::Value::Level6,
            CacheLevel::Level7 => CSSELR::Level::Value::Level7,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CacheType {
    Instruction,
    Data,
    SeperateInstructionAndData,
    Unified,
}

impl From<CLIDR::Ctype1::Value> for Option<CacheType> {
    fn from(ctype: CLIDR::Ctype1::Value) -> Self {
        match ctype {
            CLIDR::Ctype1::Value::NoCache => None,
            CLIDR::Ctype1::Value::Instruction => Some(CacheType::Instruction),
            CLIDR::Ctype1::Value::Data => Some(CacheType::Data),
            CLIDR::Ctype1::Value::InstructionAndData => Some(CacheType::SeperateInstructionAndData),
            CLIDR::Ctype1::Value::Unified => Some(CacheType::Unified),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InstructionOrData {
    DataOrUnified,
    Instruction,
}

impl From<InstructionOrData> for CSSELR::InD::Value {
    fn from(typ: InstructionOrData) -> Self {
        match typ {
            InstructionOrData::DataOrUnified => CSSELR::InD::Value::DataOrUnified,
            InstructionOrData::Instruction => CSSELR::InD::Value::Instruction,
        }
    }
}

struct CacheInfo {
    num_sets: u32,
    num_ways: u32,
    linewidth_bytes: u32,
}

impl CacheInfo {
    fn get(level: CacheLevel, typ: InstructionOrData) -> Self {
        let csselr = CSSELR.extract();
        CSSELR.modify_no_read(
            csselr,
            FieldValue::from(CSSELR::Level::Value::from(level))
                + FieldValue::from(CSSELR::InD::Value::from(typ)),
        );
        let ccsidr = CCSIDR.extract();
        CSSELR.set(csselr.get());

        Self {
            num_sets: ccsidr.read(CCSIDR::NumSets) + 1,
            num_ways: ccsidr.read(CCSIDR::Associativity) + 1,
            linewidth_bytes: (1 << (ccsidr.read(CCSIDR::LineSize) + 2)) * size_of::<u32>() as u32,
        }
    }
}
