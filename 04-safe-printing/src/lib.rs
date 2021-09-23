#![no_std]

use core::panic::PanicInfo;
use windows_kernel_rs::println;
use windows_kernel_rs::alloc::KernelAlloc;
use windows_kernel_sys::base::{DRIVER_OBJECT, NTSTATUS, STATUS_SUCCESS, UNICODE_STRING};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

#[no_mangle]
pub extern "system" fn __CxxFrameHandler3(_: *mut u8, _: *mut u8, _: *mut u8, _: *mut u8) -> i32 {
    unimplemented!()
}

#[global_allocator]
static GLOBAL: KernelAlloc = KernelAlloc;

#[no_mangle]
pub extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, _: &UNICODE_STRING) -> NTSTATUS {
    println!("Hello, world!");

    driver.DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

pub unsafe extern "C" fn driver_exit(_driver: *mut DRIVER_OBJECT) {
    println!("Bye bye!");
}
