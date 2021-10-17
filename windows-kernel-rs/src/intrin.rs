use crate::error::{Error, IntoResult};
use windows_kernel_sys::intrin::{read_msr_safe, write_msr_safe};

pub fn read_msr(register: u32) -> Result<u64, Error> {
    let mut value = 0;

    unsafe {
        read_msr_safe(register, &mut value)
    }.into_result()?;

    Ok(value)
}

pub fn write_msr(register: u32, value: u64) -> Result<(), Error> {
    unsafe {
        write_msr_safe(register, value)
    }.into_result()?;

    Ok(())
}
