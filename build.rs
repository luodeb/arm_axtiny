fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-arg=-T{root}/linker_aarch64.lds");
    println!("cargo:rustc-link-arg=-no-pie");
}
