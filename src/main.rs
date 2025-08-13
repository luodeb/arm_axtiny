#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(get_mut_unchecked)]

use log::info;

extern crate axplat_aarch64_raspi;
extern crate alloc;
extern crate log;

mod utils;

fn init_kernel(cpu_id: usize, arg: usize) {
    // Initialize trap, console, time.
    axplat::init::init_early(cpu_id, arg);

    // Initialize platform peripherals (not used in this example).
    axplat::init::init_later(cpu_id, arg);
}

#[axplat::main]
fn kernel_main(cpu_id: usize, arg: usize) -> ! {
    init_kernel(cpu_id, arg);

    utils::heap_allocator::init_heap();

    axplat::console_println!("Hello, ArceOS!");
    axplat::console_println!("cpu_id = {cpu_id}, arg = {arg:#x}");

    utils::logging::log_init();

    for _ in 0..5 {
        axplat::time::busy_wait(axplat::time::TimeValue::from_secs(1));
        info!("{:?} elapsed.", axplat::time::monotonic_time());
    }

    info!("All done, shutting down!");

    utils::allocator::run_allocator_tests();

    axplat::power::system_off();
}

#[cfg(all(target_os = "none", not(test)))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    axplat::console_println!("{info}");
    axplat::power::system_off()
}
