# armv7a bare-metal support

This crate provides basic support for building bare-metal Rust
applications for ARMv7-A processors.

It provides basic startup code and initialization of virtual memory.

Currently tested on

* STM32MP1 (STM32MP157F-DK2)
* Zynq 7000 (qemu, xilinx-zynq-a9)

## Usage

See examples/zynq7000-hello-world

```rust
// Generates the global start symbol
startup!(System);
struct System;
```

```rust
// Initial memory management
impl MemoryMap for System {
    // Map the binary image into virtual memory with a unit mapping (phys addr == virt addr),
    // so that we can keep executing after enabling the MMU.
    const MAP: &'static [MemoryRegion] = &[MemoryRegion::image(
        // Image section needs to be read- and write-able as well as executable,
        // otherwise a data or prefetch abort will occur.
        NORMAL.read_writeable().executeable(),
    )];
}
```

```rust
// Add main function
impl EntryPoint for System {
    fn main() -> ! {
```

## Qemu

```
cd examples/zynq7000-hello-world
cargo run
```

Then connect with gdb or lldb (see .vscode/launch.json for CodeLLDB)

```
gdb target/armv7a-none-eabi/debug/zynq7000-hello-world
target remote localhost:1234
```

Quit qemu with ctrl+a ... x
