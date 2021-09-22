#![no_std]

use core::panic::PanicInfo;
use winapi::km::wdm::{DbgPrint, DRIVER_OBJECT};
use winapi::shared::ntdef::{NTSTATUS, UNICODE_STRING};
use winapi::shared::ntstatus::STATUS_SUCCESS;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, _: &UNICODE_STRING) -> NTSTATUS {
    unsafe {
        DbgPrint("Hello, world!\0".as_ptr());
    }

    driver.DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

pub extern "system" fn driver_exit(_driver: &mut DRIVER_OBJECT) {
    unsafe {
        DbgPrint("Bye bye!\0".as_ptr());
    }
}
