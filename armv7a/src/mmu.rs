use core::ops::{Range, RangeInclusive};

use tock_registers::fields::FieldValue;

mod bitfields;
mod level1;
mod level2;
mod memory_attributes;

use self::bitfields::*;
pub use memory_attributes::*;

/// Simple Memory Management for armv7a proessors.
/// Uses only the short-descriptor format, simplified (2-bit) access control, and no TEX-remapping.

pub trait MemoryMap {
    /// Memory regions for the inital mapping
    const MAP: &'static [MemoryRegion];
}

pub(crate) trait PageTables {
    fn level1_page_table_ptr() -> *const level1::PageTable {
        &level1::LEVEL1_PAGE_TABLE as *const _ as *const level1::PageTable
    }
}

impl<T: MemoryMap> PageTables for T {}

pub enum MemoryRegion {
    Section {
        virt_addr: usize,
        phys_addr: usize,
        attrs: MemoryAttributes,
    },
    Sections {
        virt_range: Range<u64>,
        phys_addr: usize,
        attrs: MemoryAttributes,
    },
    Page {
        virt_addr: usize,
        phys_addr: usize,
        attrs: MemoryAttributes,
    },
    Pages {
        virt_range: Range<u64>,
        phys_addr: usize,
        attrs: MemoryAttributes,
    },
    Image {
        attrs: MemoryAttributes,
    },
}

impl MemoryRegion {
    /// Maps a 1MiB section into virtual memory. Must be aligned to 1MiB.
    pub const fn section(virt_addr: usize, phys_addr: usize, attrs: MemoryAttributes) -> Self {
        assert!(
            (virt_addr as *const ()).is_aligned_to(0x0010_0000),
            "Virtual address of section must be aligned to 1 MiB!"
        );

        assert!(
            (phys_addr as *const ()).is_aligned_to(0x0010_0000),
            "Physical address of section must be aligned to 1 MiB!"
        );

        Self::Section {
            virt_addr,
            phys_addr,
            attrs,
        }
    }

    /// Maps multiple, contiguous 1MiB sections into virtual memory. Must be aligned to 1MiB.
    pub const fn sections(
        virt_range: RangeInclusive<usize>,
        phys_addr: usize,
        attrs: MemoryAttributes,
    ) -> Self {
        let virt_range = (*virt_range.start() as u64)..(*virt_range.end() as u64 + 1);

        assert!(
            virt_range.start < virt_range.end,
            "Range of sections must not be empty!"
        );

        assert!(
            (virt_range.start as *const ()).is_aligned_to(0x0010_0000),
            "Virtual start address of the sections must be aligned to 1 MiB!"
        );

        assert!(
            (virt_range.end as *const ()).is_aligned_to(0x0010_0000),
            "Length of the sections must be a multiple of 1 MiB!"
        );

        assert!(
            (phys_addr as *const ()).is_aligned_to(0x0010_0000),
            "Physical start address of the sections must be aligned to 1 MiB!"
        );

        let len = virt_range.end - virt_range.start;
        assert!(
            phys_addr as u64 + len <= 0x1_0000_0000,
            "Physical end address of the sections overflows the address space; must be within 4 GiB!"
        );

        Self::Sections {
            virt_range,
            phys_addr,
            attrs,
        }
    }

    /// Maps a 4KiB page into virtual memory. Must be aligned to 4KiB.
    pub const fn page(virt_addr: usize, phys_addr: usize, attrs: MemoryAttributes) -> Self {
        assert!(
            (virt_addr as *const ()).is_aligned_to(0x1000),
            "Virtual address of page must be aligned to 4 KiB!"
        );

        assert!(
            (phys_addr as *const ()).is_aligned_to(0x1000),
            "Physical address of page must be aligned to 4 KiB!"
        );

        Self::Page {
            virt_addr,
            phys_addr,
            attrs,
        }
    }

    /// Maps multiple, contiguous 4KiB pages into virtual memory. Must be aligned to 4KiB.
    pub const fn pages(
        virt_range: RangeInclusive<usize>,
        phys_addr: usize,
        attrs: MemoryAttributes,
    ) -> Self {
        let virt_range = (*virt_range.start() as u64)..(*virt_range.end() as u64 + 1);

        assert!(
            virt_range.start < virt_range.end,
            "Range of pages must not be empty!"
        );

        assert!(
            (virt_range.start as *const ()).is_aligned_to(0x1000),
            "Virtual start address of the sections must be aligned to 4 KiB!"
        );

        assert!(
            (virt_range.end as *const ()).is_aligned_to(0x1000),
            "Length of the pages must be a multiple of 4 KiB!"
        );

        assert!(
            (phys_addr as *const ()).is_aligned_to(0x1000),
            "Physical start address of the pages must be aligned to 4 KiB!"
        );

        let len = virt_range.end - virt_range.start;
        assert!(
            phys_addr as u64 + len <= 0x1_0000_0000,
            "Physical end address of the pages overflows the address space; must be within 4 GiB!"
        );

        Self::Pages {
            virt_range,
            phys_addr,
            attrs,
        }
    }

    /// Maps the binary image itself into virtual memory, with a unit mapping (phys addr == virt addr).
    /// The image region is aligned to 1MiB boundaries.
    pub const fn image(attrs: MemoryAttributes) -> Self {
        MemoryRegion::Image { attrs }
    }

    pub fn map(&self) {
        match self {
            MemoryRegion::Section {
                virt_addr,
                phys_addr,
                attrs,
            } => Self::map_section(*virt_addr, *phys_addr, attrs),
            MemoryRegion::Sections {
                virt_range,
                phys_addr,
                attrs,
            } => Self::map_sections(virt_range.start..virt_range.end, *phys_addr, attrs),
            MemoryRegion::Page {
                virt_addr,
                phys_addr,
                attrs,
            } => Self::map_page(*virt_addr, *phys_addr, attrs),
            MemoryRegion::Pages {
                virt_range,
                phys_addr,
                attrs,
            } => Self::map_pages(virt_range.clone(), *phys_addr, attrs),
            MemoryRegion::Image { attrs } => Self::map_image(attrs),
        }
    }

    fn map_section(virt_addr: usize, phys_addr: usize, attrs: &MemoryAttributes) {
        critical_section::with(|cs| {
            level1::LEVEL1_PAGE_TABLE.borrow(cs).map_section(
                virt_addr,
                phys_addr,
                attrs.encoding(),
            );
        });
    }

    fn map_sections(virt_range: Range<u64>, phys_addr: usize, attrs: &MemoryAttributes) {
        for (i, virt_addr) in virt_range.step_by(0x0010_0000).enumerate() {
            Self::map_section(virt_addr as usize, phys_addr + i * 0x0010_0000, attrs);
        }
    }

    fn map_page(virt_addr: usize, phys_addr: usize, attrs: &MemoryAttributes) {
        critical_section::with(|cs| {
            let mut page_table_alloc = level2::LEVEL2_PAGE_TABLE_ALLOC.borrow_ref_mut(cs);

            let level2_page_table = page_table_alloc.get_or_alloc(virt_addr).unwrap();
            level2_page_table.map_small_page(virt_addr, phys_addr, attrs.encoding());

            level1::LEVEL1_PAGE_TABLE
                .borrow(cs)
                .map_level2_page_table(virt_addr, level2_page_table);
        });
    }

    fn map_pages(virt_range: Range<u64>, phys_addr: usize, attrs: &MemoryAttributes) {
        for (i, virt_addr) in virt_range.step_by(0x1000).enumerate() {
            Self::map_page(virt_addr as usize, phys_addr + i * 0x1000, attrs);
        }
    }

    fn map_image(attrs: &MemoryAttributes) {
        extern "C" {
            static mut __image_start: u8;
            static mut __image_end: u8;
        }

        let image_start_addr = unsafe { &__image_start as *const u8 as usize };
        let image_end_addr = unsafe { &__image_end as *const u8 as usize };

        let image_start_addr = image_start_addr & 0xFFF0_0000;
        let image_end_addr = (image_end_addr & 0xFFF0_0000) + 0x0010_0000;

        Self::map_sections(
            (image_start_addr as u64)..(image_end_addr as u64),
            image_start_addr,
            attrs,
        )
    }
}

type SectionAttributes = FieldValue<u32, SectionEntry::Register>;
type SmallPageAttributes = FieldValue<u32, SmallPageEntry::Register>;
