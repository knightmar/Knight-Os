// build.rs
use std::process::Command;
use std::fs;

fn main() {
    fs::create_dir_all("build/iso/boot/grub").expect("Failed to create ISO directory");

    println!("Starting compile");
    // Step 1: Compile boot.s
    if !Command::new("as")
        .args(&["boot.s", "-o", "build/boot.o"])
        .status()
        .expect("Failed to compile boot.s")
        .success()
    {
        panic!("Error assembling boot.s");
    }

    println!("Starting cargo compile");
    // Step 2: Build Rust kernel library (this is done by Cargo)
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=boot.s");

    println!("Starting linking");
    // Step 3: Link to create kernel.bin
    if !Command::new("ld.lld")
        .args(&["-n", "-o", "build/kernel.bin", "-T", "kernel.ld", "build/boot.o", "target/x86_64-unknown-none/release/libknight_os.a"])
        .status()
        .expect("Failed to link kernel")
        .success()
    {
        panic!("Error linking kernel");
    }

    // Step 4: Move kernel.bin to the ISO directory
    fs::rename("build/kernel.bin", "build/iso/boot/kernel.bin").expect("Failed to move kernel.bin");
    fs::copy("grub.cfg", "build/iso/boot/grub/grub.cfg").expect("Failed to move grub config");

    println!("Starting iso making");
    // Step 5: Create the ISO with grub-mkrescue
    if !Command::new("grub-mkrescue")
        .args(&["-o", "build/knight_os.iso", "build/iso"])
        .status()
        .expect("Failed to create ISO")
        .success()
    {
        panic!("Error creating ISO");
    }
}
