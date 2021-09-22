#![no_std]

use core::panic::PanicInfo;
use windows_kernel_sys::base::{DRIVER_OBJECT, NTSTATUS, STATUS_SUCCESS, UNICODE_STRING};
use windows_kernel_sys::ntoskrnl::DbgPrint;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, _: &UNICODE_STRING) -> NTSTATUS {
    unsafe {
        DbgPrint("Hello, world!\0".as_ptr() as _);
    }

    driver.DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

pub unsafe extern "C" fn driver_exit(_driver: *mut DRIVER_OBJECT) {
    DbgPrint("Bye bye!\0".as_ptr() as _);
}
