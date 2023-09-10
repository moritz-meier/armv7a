fn main() {
    // Add current directory to linker search path,
    // so that the linker can find the memory.x
    println!(
        "cargo:rustc-link-search={}",
        std::env::current_dir().unwrap().display()
    );

    // Link with armv7a.ld script from armv7a crate
    println!("cargo:rustc-link-arg=-Tarmv7a.ld");
}
