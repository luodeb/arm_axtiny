use crate::utils;

pub fn user_main() -> ! {
    utils::logging::log_init();
    utils::heap_allocator::init_heap();

    // for _ in 0..5 {
    //     axplat::time::busy_wait(axplat::time::TimeValue::from_secs(1));
    //     info!("{:?} elapsed.", axplat::time::monotonic_time());
    // }

    // info!("All done, shutting down!");

    utils::allocator::run_allocator_tests();

    axplat::power::system_off();
}