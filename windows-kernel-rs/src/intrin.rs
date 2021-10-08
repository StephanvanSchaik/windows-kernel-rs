use crate::error::Error;
use windows_kernel_sys::base::STATUS_SUCCESS;
use windows_kernel_sys::intrin::{read_msr_safe, write_msr_safe};

pub fn read_msr(register: u32) -> Result<u64, Error> {
    let mut value = 0;

    let status = unsafe {
        read_msr_safe(register, &mut value)
    };

    match status {
        STATUS_SUCCESS => Ok(value),
        status => Err(Error::from_kernel_errno(status)),
    }
}

pub fn write_msr(register: u32, value: u64) -> Result<(), Error> {
    let status = unsafe {
        write_msr_safe(register, value)
    };

    match status {
        STATUS_SUCCESS => Ok(()),
        status => Err(Error::from_kernel_errno(status)),
    }
}
