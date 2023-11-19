#![no_std]

// Alternatively there is `PhysicalAllocator`
// Usage: https://github.com/not-matthias/kernel-alloc-rs/blob/master/README.md
use kernel_alloc::KernelAlloc;
#[global_allocator]
static GLOBAL: KernelAlloc = KernelAlloc;

mod dbg;
mod driver;
mod types;

use crate::driver::entry;
#[allow(unused_imports)]
use dbg::*;
use types::*;

// Needed for fix linker error
#[no_mangle]
pub extern "system" fn __CxxFrameHandler3(_: *mut u8, _: *mut u8, _: *mut u8, _: *mut u8) -> i32 {
    unimplemented!()
}

// Needed for fix linker error. Floating point calculations aren`t allowed when running in the Windows Kernel
#[export_name = "_fltused"]
static _FLTUSED: i32 = 0;

// Panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // When a panic occurs, the loop initiates an infinite loop to prevent program termination
    // Allowing for diagnostics or debugging at the panic point
    loop {}
}

// Entry point
#[no_mangle]
pub extern "system" fn driver_entry(_driver: PVOID, _path: PVOID) -> u32 {
    entry();

    0
}
