INCLUDE memory.x

ENTRY(start);

PROVIDE(__SYS_STACK_SIZE = 0x1000);
PROVIDE(__FIQ_STACK_SIZE = 0x0100);
PROVIDE(__IRQ_STACK_SIZE = 0x0100);
PROVIDE(__SVC_STACK_SIZE = 0x0100);
PROVIDE(__ABT_STACK_SIZE = 0x0100);
PROVIDE(__UND_STACK_SIZE = 0x0100);

SECTIONS {
	. = ORIGIN(RAM);
	__image_start = .;
	
    .text : ALIGN(4) {
		__text_start = .;

		*(.vectors)
		*(.text .text.*)

		. = ALIGN(4);
		__text_end = .;
    } >RAM

	.rodata __text_end : ALIGN(4) {
		__rodata_start = .;

		*(.rodata .rodata.*)

		. = ALIGN(4);
		__rodata_end = .;
	} > RAM

	.data : ALIGN(4) {
		__data_start = .;

		*(.data .data.*)

		. = ALIGN(4);
		__data_end = .;
	} > RAM

	.bss (NOLOAD) : ALIGN(4) {
		__bss_start = .;

		*(.bss .bss.*)
		*(COMMON)

		. = ALIGN(4);
		__bss_end = .;
	} > RAM

	.stack (NOLOAD) : {
		. = ALIGN(8);

		__sys_stack_start = .;
		. += __SYS_STACK_SIZE;
		. = ALIGN(8);
		__sys_stack_end = .;

		__fiq_stack_start = .;
		. += __FIQ_STACK_SIZE;
		. = ALIGN(8);
		__fiq_stack_end = .;

		__irq_stack_start = .;
		. += __IRQ_STACK_SIZE;
		. = ALIGN(8);
		__irq_stack_end = .;

		__svc_stack_start = .;
		. += __SVC_STACK_SIZE;
		. = ALIGN(8);
		__svc_stack_end = .;

		__abt_stack_start = .;
		. += __ABT_STACK_SIZE;
		. = ALIGN(8);
		__abt_stack_end = .;

		__und_stack_start = .;
		. += __UND_STACK_SIZE;
		. = ALIGN(8);
		__und_stack_end = .;
	} > RAM

	__image_end = .;

	/DISCARD/ :
	{
    	*(.ARM.exidx);
    	*(.ARM.exidx.*);
    	*(.ARM.extab.*);
  	}
}