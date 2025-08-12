use std::io::Result;

fn kernel_base(_arch: &str) -> usize {
    0xffff_0000_4020_0000
}

fn gen_linker_script(arch: &str) -> Result<()> {
    let ld_content = std::fs::read_to_string("linker.lds.S")?;
    let ld_content = ld_content.replace("%KERNEL_BASE%", &format!("{:#x}", kernel_base(arch)));
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_fname = format!("linker_{arch}.lds");
    std::fs::write(&out_fname, ld_content)?;
    println!("cargo:rustc-link-arg=-T{root}/{out_fname}");
    Ok(())
}

fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    gen_linker_script(&arch).unwrap();
    println!("cargo:rustc-link-arg=-no-pie");
}
