use alloc::format;
use mw_std::debug;

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic_handler(_panic: &core::panic::PanicInfo) -> ! {
    debug::println(&format!("{}", _panic));
    loop {}
}
