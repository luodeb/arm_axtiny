#![no_std]
#![no_main]

#[cfg(feature = "plat_qemu")]
extern crate axplat_aarch64_qemu_virt;
#[cfg(feature = "plat_raspi")]
extern crate axplat_aarch64_raspi;

fn init_kernel(cpu_id: usize, arg: usize) {
    // Initialize trap, console, time.
    axplat::init::init_early(cpu_id, arg);

    // Initialize platform peripherals (not used in this example).
    axplat::init::init_later(cpu_id, arg);
}

#[axplat::main]
fn kernel_main(cpu_id: usize, arg: usize) -> ! {
    init_kernel(cpu_id, arg);

    axplat::console_println!("Hello, ArceOS!");
    axplat::console_println!("cpu_id = {cpu_id}, arg = {arg:#x}");

    for _ in 0..5 {
        axplat::time::busy_wait(axplat::time::TimeValue::from_secs(1));
        axplat::console_println!("{:?} elapsed.", axplat::time::monotonic_time());
    }

    axplat::console_println!("All done, shutting down!");
    axplat::power::system_off();
}

#[cfg(all(target_os = "none", not(test)))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    axplat::console_println!("{info}");
    axplat::power::system_off()
}
