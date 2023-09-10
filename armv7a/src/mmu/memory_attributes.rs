use tock_registers::{fields::FieldValue, RegisterLongName};

use super::bitfields::{SectionEntry, SmallPageEntry};

/// Default device memory (Read-only, Never-execute, Privilege-level 1)
pub const DEVICE: MemoryAttributes = MemoryAttributes {
    typ: MemoryType::Device,
    access: MemoryAccess {
        read_write: ReadWritePolicy::ReadOnly,
        execute: ExecutePolicy::ExecuteNever,
        privilege: PrivilegeLevel::PrivilegeLevel1,
    },
};

/// Default strongly-ordered memory (Read-only, Never-execute, Privilege-level 1)
pub const STRONGLY_ORDERED: MemoryAttributes = MemoryAttributes {
    typ: MemoryType::StronglyOrdered,
    access: MemoryAccess {
        read_write: ReadWritePolicy::ReadOnly,
        execute: ExecutePolicy::ExecuteNever,
        privilege: PrivilegeLevel::PrivilegeLevel1,
    },
};

/// Default normal memory (Non-cacheable, Non-shareable, Read-only, Never-execute, Privilege-level 1)
pub const NORMAL: MemoryAttributes = MemoryAttributes {
    typ: MemoryType::Normal {
        inner: CachePolicy::NonCacheable,
        outer: CachePolicy::NonCacheable,
        share: SharePolicy::NonShareable,
    },
    access: MemoryAccess {
        read_write: ReadWritePolicy::ReadOnly,
        execute: ExecutePolicy::ExecuteNever,
        privilege: PrivilegeLevel::PrivilegeLevel1,
    },
};

pub struct MemoryAttributes {
    typ: MemoryType,
    access: MemoryAccess,
}

impl MemoryAttributes {
    /// Set inner cache policy. Only affects Normal memory.
    pub const fn inner(self, inner: CachePolicy) -> Self {
        match self.typ {
            MemoryType::Normal {
                inner: _,
                outer,
                share,
            } => Self {
                typ: MemoryType::Normal {
                    inner,
                    outer,
                    share,
                },
                ..self
            },
            _ => self,
        }
    }

    /// Set outer cache policy. Only affects Normal memory.
    pub const fn outer(self, outer: CachePolicy) -> Self {
        match self.typ {
            MemoryType::Normal {
                inner,
                outer: _,
                share,
            } => Self {
                typ: MemoryType::Normal {
                    inner,
                    outer,
                    share,
                },
                ..self
            },
            _ => self,
        }
    }

    /// Mark memory as non-shareable. Only affects Normal memory.
    pub const fn non_shareable(self) -> Self {
        match self.typ {
            MemoryType::Normal {
                inner,
                outer,
                share: _,
            } => Self {
                typ: MemoryType::Normal {
                    inner,
                    outer,
                    share: SharePolicy::NonShareable,
                },
                ..self
            },
            _ => self,
        }
    }

    /// Mark memory as shareable. Only affects Normal memory.
    pub const fn shareable(self) -> Self {
        match self.typ {
            MemoryType::Normal {
                inner,
                outer,
                share: _,
            } => Self {
                typ: MemoryType::Normal {
                    inner,
                    outer,
                    share: SharePolicy::Shareable,
                },
                ..self
            },
            _ => self,
        }
    }

    /// Mark memory as read-only.
    pub const fn read_only(self) -> Self {
        Self {
            access: self.access.read_only(),
            ..self
        }
    }

    /// Mark memory as read-writeable.
    pub const fn read_writeable(self) -> Self {
        Self {
            access: self.access.read_writeable(),
            ..self
        }
    }

    /// Mark memory as not-executable.
    pub const fn execute_never(self) -> Self {
        Self {
            access: self.access.execute_never(),
            ..self
        }
    }

    /// Mark memory as executeable.
    pub const fn executeable(self) -> Self {
        Self {
            access: self.access.executeable(),
            ..self
        }
    }

    /// Mark memory as accessible by privilege level 0 (and 1).
    pub const fn privilege_level_0(self) -> Self {
        Self {
            access: self.access.privilege_level_0(),
            ..self
        }
    }

    /// Mark memory as accessible only by privilege level 1.
    pub const fn privilege_level_1(self) -> Self {
        Self {
            access: self.access.privilege_level_1(),
            ..self
        }
    }
}

// ----------------------------------------------

pub enum MemoryType {
    Device,
    StronglyOrdered,
    Normal {
        inner: CachePolicy,
        outer: CachePolicy,
        share: SharePolicy,
    },
}

pub enum CachePolicy {
    NonCacheable,
    WriteBackWriteAllocate,
    WriteThroughNoWriteAllocate,
    WriteBackNoWriteAllocate,
}

pub enum SharePolicy {
    NonShareable,
    Shareable,
}

// ----------------------------------------------

pub struct MemoryAccess {
    read_write: ReadWritePolicy,
    execute: ExecutePolicy,
    privilege: PrivilegeLevel,
}

impl MemoryAccess {
    const fn read_only(self) -> Self {
        Self {
            read_write: ReadWritePolicy::ReadOnly,
            ..self
        }
    }

    const fn read_writeable(self) -> Self {
        Self {
            read_write: ReadWritePolicy::ReadWrite,
            ..self
        }
    }

    const fn execute_never(self) -> Self {
        Self {
            execute: ExecutePolicy::ExecuteNever,
            ..self
        }
    }

    const fn executeable(self) -> Self {
        Self {
            execute: ExecutePolicy::Executeable,
            ..self
        }
    }

    const fn privilege_level_0(self) -> Self {
        Self {
            privilege: PrivilegeLevel::PrivilegeLevel0,
            ..self
        }
    }

    const fn privilege_level_1(self) -> Self {
        Self {
            privilege: PrivilegeLevel::PrivilegeLevel1,
            ..self
        }
    }
}

pub enum ReadWritePolicy {
    ReadOnly,
    ReadWrite,
}

pub enum ExecutePolicy {
    ExecuteNever,
    Executeable,
}

pub enum PrivilegeLevel {
    PrivilegeLevel1,
    PrivilegeLevel0,
}

// ------------------------------------------------------------------------

pub trait FieldEncoding<R: RegisterLongName> {
    fn encoding(&self) -> FieldValue<u32, R>;
}

impl FieldEncoding<SectionEntry::Register> for MemoryAttributes {
    fn encoding(&self) -> FieldValue<u32, SectionEntry::Register> {
        self.typ.encoding().combine(self.access.encoding())
    }
}

impl FieldEncoding<SmallPageEntry::Register> for MemoryAttributes {
    fn encoding(&self) -> FieldValue<u32, SmallPageEntry::Register> {
        self.typ.encoding().combine(self.access.encoding())
    }
}

impl FieldEncoding<SectionEntry::Register> for MemoryType {
    fn encoding(&self) -> FieldValue<u32, SectionEntry::Register> {
        match self {
            MemoryType::Device => {
                SectionEntry::TEX::DeviceOrStronglyOrdered.combine(SectionEntry::CB::Device)
            }
            MemoryType::StronglyOrdered => SectionEntry::TEX::DeviceOrStronglyOrdered
                .combine(SectionEntry::CB::StronglyOrdered),
            MemoryType::Normal {
                inner,
                outer,
                share,
            } => SectionEntry::TEX::Normal
                .combine(
                    CachePolicyEncoding::<InnerEncoding, SectionEntry::Register>::encoding(inner),
                )
                .combine(
                    CachePolicyEncoding::<OuterEncoding, SectionEntry::Register>::encoding(outer),
                )
                .combine(share.encoding()),
        }
    }
}

impl FieldEncoding<SmallPageEntry::Register> for MemoryType {
    fn encoding(&self) -> FieldValue<u32, SmallPageEntry::Register> {
        match self {
            MemoryType::Device => {
                SmallPageEntry::TEX::DeviceOrStronglyOrdered.combine(SmallPageEntry::CB::Device)
            }
            MemoryType::StronglyOrdered => SmallPageEntry::TEX::DeviceOrStronglyOrdered
                .combine(SmallPageEntry::CB::StronglyOrdered),
            MemoryType::Normal {
                inner,
                outer,
                share,
            } => SmallPageEntry::TEX::Normal
                .combine(
                    CachePolicyEncoding::<InnerEncoding, SmallPageEntry::Register>::encoding(inner),
                )
                .combine(
                    CachePolicyEncoding::<OuterEncoding, SmallPageEntry::Register>::encoding(outer),
                )
                .combine(share.encoding()),
        }
    }
}

struct InnerEncoding;
struct OuterEncoding;

trait CachePolicyEncoding<InnerOuter, R: RegisterLongName> {
    fn encoding(&self) -> FieldValue<u32, R>;
}

impl CachePolicyEncoding<InnerEncoding, SectionEntry::Register> for CachePolicy {
    fn encoding(&self) -> FieldValue<u32, SectionEntry::Register> {
        match self {
            CachePolicy::NonCacheable => SectionEntry::AA::NonCacheable,
            CachePolicy::WriteBackWriteAllocate => SectionEntry::AA::WriteBackWriteAllocate,
            CachePolicy::WriteThroughNoWriteAllocate => {
                SectionEntry::AA::WriteThroughNoWriteAllocate
            }
            CachePolicy::WriteBackNoWriteAllocate => SectionEntry::AA::WriteBackNoWriteAllocate,
        }
    }
}

impl CachePolicyEncoding<OuterEncoding, SectionEntry::Register> for CachePolicy {
    fn encoding(&self) -> FieldValue<u32, SectionEntry::Register> {
        match self {
            CachePolicy::NonCacheable => SectionEntry::BB::NonCacheable,
            CachePolicy::WriteBackWriteAllocate => SectionEntry::BB::WriteBackWriteAllocate,
            CachePolicy::WriteThroughNoWriteAllocate => {
                SectionEntry::BB::WriteThroughNoWriteAllocate
            }
            CachePolicy::WriteBackNoWriteAllocate => SectionEntry::BB::WriteBackNoWriteAllocate,
        }
    }
}

impl CachePolicyEncoding<InnerEncoding, SmallPageEntry::Register> for CachePolicy {
    fn encoding(&self) -> FieldValue<u32, SmallPageEntry::Register> {
        match self {
            CachePolicy::NonCacheable => SmallPageEntry::AA::NonCacheable,
            CachePolicy::WriteBackWriteAllocate => SmallPageEntry::AA::WriteBackWriteAllocate,
            CachePolicy::WriteThroughNoWriteAllocate => {
                SmallPageEntry::AA::WriteThroughNoWriteAllocate
            }
            CachePolicy::WriteBackNoWriteAllocate => SmallPageEntry::AA::WriteBackNoWriteAllocate,
        }
    }
}

impl CachePolicyEncoding<OuterEncoding, SmallPageEntry::Register> for CachePolicy {
    fn encoding(&self) -> FieldValue<u32, SmallPageEntry::Register> {
        match self {
            CachePolicy::NonCacheable => SmallPageEntry::BB::NonCacheable,
            CachePolicy::WriteBackWriteAllocate => SmallPageEntry::BB::WriteBackWriteAllocate,
            CachePolicy::WriteThroughNoWriteAllocate => {
                SmallPageEntry::BB::WriteThroughNoWriteAllocate
            }
            CachePolicy::WriteBackNoWriteAllocate => SmallPageEntry::BB::WriteBackNoWriteAllocate,
        }
    }
}

impl FieldEncoding<SectionEntry::Register> for SharePolicy {
    fn encoding(&self) -> FieldValue<u32, SectionEntry::Register> {
        match self {
            SharePolicy::NonShareable => SectionEntry::S::NonShareable,
            SharePolicy::Shareable => SectionEntry::S::Shareable,
        }
    }
}

impl FieldEncoding<SmallPageEntry::Register> for SharePolicy {
    fn encoding(&self) -> FieldValue<u32, SmallPageEntry::Register> {
        match self {
            SharePolicy::NonShareable => SmallPageEntry::S::NonShareable,
            SharePolicy::Shareable => SmallPageEntry::S::Shareable,
        }
    }
}

impl FieldEncoding<SectionEntry::Register> for MemoryAccess {
    fn encoding(&self) -> FieldValue<u32, SectionEntry::Register> {
        self.read_write
            .encoding()
            .combine(self.execute.encoding())
            .combine(self.privilege.encoding())
    }
}

impl FieldEncoding<SmallPageEntry::Register> for MemoryAccess {
    fn encoding(&self) -> FieldValue<u32, SmallPageEntry::Register> {
        self.read_write
            .encoding()
            .combine(self.execute.encoding())
            .combine(self.privilege.encoding())
    }
}

impl FieldEncoding<SectionEntry::Register> for ReadWritePolicy {
    fn encoding(&self) -> FieldValue<u32, SectionEntry::Register> {
        match self {
            ReadWritePolicy::ReadOnly => SectionEntry::APX::DisableWrite,
            ReadWritePolicy::ReadWrite => SectionEntry::APX::EnableWrite,
        }
    }
}

impl FieldEncoding<SmallPageEntry::Register> for ReadWritePolicy {
    fn encoding(&self) -> FieldValue<u32, SmallPageEntry::Register> {
        match self {
            ReadWritePolicy::ReadOnly => SmallPageEntry::APX::DisableWrite,
            ReadWritePolicy::ReadWrite => SmallPageEntry::APX::EnableWrite,
        }
    }
}

impl FieldEncoding<SectionEntry::Register> for ExecutePolicy {
    fn encoding(&self) -> FieldValue<u32, SectionEntry::Register> {
        match self {
            ExecutePolicy::ExecuteNever => SectionEntry::XN::ExecuteNever,
            ExecutePolicy::Executeable => SectionEntry::XN::Executeable,
        }
    }
}

impl FieldEncoding<SmallPageEntry::Register> for ExecutePolicy {
    fn encoding(&self) -> FieldValue<u32, SmallPageEntry::Register> {
        match self {
            ExecutePolicy::ExecuteNever => SmallPageEntry::XN::ExecuteNever,
            ExecutePolicy::Executeable => SmallPageEntry::XN::Executeable,
        }
    }
}

impl FieldEncoding<SectionEntry::Register> for PrivilegeLevel {
    fn encoding(&self) -> FieldValue<u32, SectionEntry::Register> {
        match self {
            PrivilegeLevel::PrivilegeLevel0 => SectionEntry::AP::PL0,
            PrivilegeLevel::PrivilegeLevel1 => SectionEntry::AP::PL1,
        }
    }
}

impl FieldEncoding<SmallPageEntry::Register> for PrivilegeLevel {
    fn encoding(&self) -> FieldValue<u32, SmallPageEntry::Register> {
        match self {
            PrivilegeLevel::PrivilegeLevel0 => SmallPageEntry::AP::PL0,
            PrivilegeLevel::PrivilegeLevel1 => SmallPageEntry::AP::PL1,
        }
    }
}
