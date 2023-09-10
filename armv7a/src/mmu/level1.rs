use core::mem::ManuallyDrop;

use critical_section::Mutex;
use tock_registers::{fields::FieldValue, interfaces::Writeable, registers::InMemoryRegister};

use super::{bitfields::*, level2, SectionAttributes};

pub(crate) static LEVEL1_PAGE_TABLE: Mutex<PageTable> = Mutex::new(PageTable::DEFAULT);

#[repr(align(0x4000))]
pub(crate) struct PageTable {
    entries: [PageTableEntry; 4096],
}

impl PageTable {
    const DEFAULT: Self = Self {
        entries: [PageTableEntry::INVALID; 4096],
    };

    pub(super) fn map_section(&self, virt_addr: usize, phys_addr: usize, attrs: SectionAttributes) {
        self.entries[virt_addr >> 20].write_section_entry(phys_addr, attrs);
    }

    pub(super) fn map_level2_page_table(
        &self,
        virt_addr: usize,
        level2_page_table: *const level2::PageTable,
    ) {
        self.entries[virt_addr >> 20].write_level2_page_table_entry(level2_page_table);
    }
}

union PageTableEntry {
    invalid: ManuallyDrop<InMemoryRegister<u32, ()>>,
    section: ManuallyDrop<InMemoryRegister<u32, SectionEntry::Register>>,
    page_table: ManuallyDrop<InMemoryRegister<u32, Level2PageTableEntry::Register>>,
}

impl PageTableEntry {
    const INVALID: Self = Self {
        invalid: ManuallyDrop::new(InMemoryRegister::new(0)),
    };

    const SECTION_ENTRY: FieldValue<u32, SectionEntry::Register> =
        FieldValue::<u32, SectionEntry::Register>::new(1, 1, 1);

    const LEVEL2_PAGE_TABLE_ENTRY: FieldValue<u32, Level2PageTableEntry::Register> =
        FieldValue::<u32, Level2PageTableEntry::Register>::new(1, 0, 1);

    fn write_section_entry(&self, phys_addr: usize, attrs: SectionAttributes) {
        let register =
            Self::SECTION_ENTRY + SectionEntry::BASE_ADDR.from_value(phys_addr as u32) + attrs;

        unsafe { self.section.write(register) }
    }

    fn write_level2_page_table_entry(&self, level2_page_table: *const level2::PageTable) {
        let register = Self::LEVEL2_PAGE_TABLE_ENTRY
            + Level2PageTableEntry::BASE_ADDR.from_value(level2_page_table as u32);

        unsafe { self.page_table.write(register) }
    }
}
