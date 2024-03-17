fn main() -> Result<(), Box<dyn std::error::Error>> {
    cc::Build::new().file("src/lib.s").compile("asm");
    println!("cargo:rerun-if-changed=src/lib.s");
    Ok(())
}
