#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(get_mut_unchecked)]

extern crate axplat_aarch64_raspi;
extern crate alloc;

#[macro_use]
extern crate log;

mod utils;
mod user;

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

    user::user_main();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("{info}");
    axplat::power::system_off()
}
