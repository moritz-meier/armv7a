use core::{cell::RefCell, mem::ManuallyDrop};

use critical_section::Mutex;
use tock_registers::{fields::FieldValue, interfaces::Writeable, registers::InMemoryRegister};

use super::{bitfields::SmallPageEntry, SmallPageAttributes};

pub(crate) static LEVEL2_PAGE_TABLE_ALLOC: Mutex<RefCell<PageTableAlloc>> =
    Mutex::new(RefCell::new(PageTableAlloc::new()));

pub(crate) struct PageTableAlloc<const N: usize = 64> {
    base_addrs: [Option<usize>; N],
    tables: [Option<PageTable>; N],
}

impl<const N: usize> PageTableAlloc<N> {
    const fn new() -> Self {
        Self {
            base_addrs: [None; N],
            tables: [const { None }; N],
        }
    }

    pub(super) fn get_or_alloc(&mut self, virt_addr: usize) -> Option<&PageTable> {
        let idx = self
            .get_idx(virt_addr)
            .or_else(|| self.alloc_idx(virt_addr))?;

        self.tables[idx].as_ref()
    }

    fn get_idx(&self, virt_addr: usize) -> Option<usize> {
        let (idx, _) = self
            .base_addrs
            .iter()
            .enumerate()
            .filter_map(|(i, base_addr)| Some((i, (*base_addr)?)))
            .find(|(_, base_addr)| *base_addr == virt_addr & 0xFFF0_0000)?;

        Some(idx)
    }

    fn alloc_idx(&mut self, virt_addr: usize) -> Option<usize> {
        let (idx, _) = self
            .base_addrs
            .iter()
            .enumerate()
            .find(|(_, base_addr)| base_addr.is_none())?;

        self.base_addrs[idx] = Some(virt_addr & 0xFFF0_0000);
        self.tables[idx] = Some(PageTable::DEFAULT);

        Some(idx)
    }
}

#[repr(align(0x400))]
pub(crate) struct PageTable {
    entries: [PageTableEntry; 256],
}

impl PageTable {
    const DEFAULT: Self = Self {
        entries: [PageTableEntry::INVALID; 256],
    };

    pub(super) fn map_small_page(
        &self,
        virt_addr: usize,
        phys_addr: usize,
        attrs: SmallPageAttributes,
    ) {
        self.entries[(virt_addr >> 12) & 0xFF].write_small_page_entry(phys_addr, attrs);
    }
}

union PageTableEntry {
    invalid: ManuallyDrop<InMemoryRegister<u32, ()>>,
    small_page: ManuallyDrop<InMemoryRegister<u32, SmallPageEntry::Register>>,
}

impl PageTableEntry {
    const INVALID: Self = Self {
        invalid: ManuallyDrop::new(InMemoryRegister::new(0)),
    };

    const SMALL_PAGE_ENTRY: FieldValue<u32, SmallPageEntry::Register> =
        FieldValue::<u32, SmallPageEntry::Register>::new(1, 1, 1);

    fn write_small_page_entry(&self, phys_addr: usize, attrs: SmallPageAttributes) {
        let register =
            Self::SMALL_PAGE_ENTRY + SmallPageEntry::BASE_ADDR.from_value(phys_addr as u32) + attrs;

        unsafe { self.small_page.write(register) }
    }
}
