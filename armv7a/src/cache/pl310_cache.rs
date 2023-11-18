use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};

tock_registers::register_structs! {
    Registers {
        (0x000 => cache_id: ReadOnly<u32, ()>),
        (0x004 => cache_type: ReadOnly<u32, ()>),
        (0x100 => control: ReadWrite<u32, ()>),
        (0x104 => auxiliary_control: ReadWrite<u32, ()>),
        (0x200 => event_counter_control: ReadWrite<u32, ()>),
        (0x204 => event_counter1_configuration: ReadWrite<u32, ()>),
        (0x208 => event_counter0_configuration: ReadWrite<u32, ()>),
        (0x20C => event_counter1_value: ReadWrite<u32, ()>),
        (0x210 => event_counter0_value: ReadWrite<u32, ()>),
        (0x214 => interrupt_mask: ReadWrite<u32, ()>),
        (0x218 => masked_interrupt_status: ReadOnly<u32, ()>),
        (0x21C => raw_interrupt_status: ReadOnly<u32, ()>),
        (0x220 => interrupt_clear: WriteOnly<u32, ()>),
        (0x730 => cache_sync: ReadWrite<u32, ()>),
        (0x770 => invalidate_line_by_pa: ReadWrite<u32, ()>),
        (0x770 => invalidate_line_by_pa: ReadWrite<u32, ()>),
        //(0xFFF => @END),
    }
}
