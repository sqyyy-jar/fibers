#[cfg(all(target_arch = "x86_64", unix))]
const ASM_FILE: &str = "./src/arch/x64/system_v/lib.s";

#[cfg(all(target_arch = "x86_64", windows))]
const ASM_FILE: &str = "./src/arch/x64/microsoft/lib.s";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cc::Build::new().file(ASM_FILE).compile("asm");
    println!("cargo:rerun-if-changed={ASM_FILE}");
    Ok(())
}
