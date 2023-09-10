# armv7a bare-metal support

This crate provides basic support for building bare-metal Rust
applications for ARMv7-A processors.

It provides basic startup code and initialization of virtual memory.

Currently tested on 

* STM32MP1 (STM32MP157F-DK2, Cortex A7)
* Zynq 7000 (qemu, xilinx-zynq-a9, Cortex A9)

## Usage

See examples/zynq7000-hello-world

* Create binary crate
* Make application #![no_std] and #![no_main]
* Add armv7a dependency
* Create memory.x
* Create build.rs that set linker search path and linker script
* Create entry-point
```rust
// Generates the global start symbol
startup!(System);
struct System;
```
* Create initial memory map
```rust
// Initial memory management
impl MemoryMap for System {
    // Map the binary image into virtual memory with a unit mapping (phys addr == virt addr),
    // so that we can keep executing after enabling the MMU.
    const MAP: &'static [armv7a::MemoryRegion] = &[MemoryRegion::image(
        // Image sections need to be read- and write-able as well as executable,
        // otherwise a data or prefetch abort will occur.
        NORMAL.read_writeable().executeable(),
    )];
}
```
* Create main entry point
```rust
impl EntryPoint for System {
    fn main() -> ! {
```

## Qemu

```
qemu-system-arm -cpu cortex-a9 -machine xilinx-zynq-a9 -m 1024M -nographic -serial null -serial mon:stdio -kernel target/armv7a-none-eabi/debug/zynq7000-hello-world -S -gdb tcp::1234
```
Quit with ctrl+a ... x

## STM32MP1

Boot into uboot, then halt the target, load binary and jump to start symbol

## ToDo

* Provide exceptions (irq, fiq, ...) to application
* Improve memory manangement (super-section, large-page, unmap, ...)
* Multi-core support (spin-lock, ...)
* Hypervisor mode support (PL2 mode, second stage memory translation, ...)