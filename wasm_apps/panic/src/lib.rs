#![no_std]

#[panic_handler]
pub fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    unreachable!()
}
