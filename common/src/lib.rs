#![no_std]
#![feature(alloc_error_handler)]

#![feature(lang_items)]
extern crate alloc;
extern crate alloc_cortex_m;
extern crate cortex_m_rt as rt; // v0.5.x
use rt::entry;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;

#[global_allocator]
pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_semihosting::{debug, hprintln};

pub mod error;
pub mod macros;
pub mod pdcn_core;
pub mod pdcn_systems;
pub mod serial_system;
pub mod timer_system;
pub mod define;

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}
