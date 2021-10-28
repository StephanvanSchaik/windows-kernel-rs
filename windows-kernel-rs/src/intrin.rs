use crate::error::{Error, IntoResult};
use windows_kernel_sys::intrin::{read_msr_safe, write_msr_safe};

/// Attempts to read the given model-specific register. Accessing an invalid model-specific
/// register would normally result in a CPU exception. This function uses Structured Exception
/// Handling (SEH) to safely catch CPU exceptions and to turn them into an [`Error`]. This prevents
/// a hang.
pub fn read_msr(register: u32) -> Result<u64, Error> {
    let mut value = 0;

    unsafe {
        read_msr_safe(register, &mut value)
    }.into_result()?;

    Ok(value)
}

/// Attempts to write the given value to the given model-specific register. Accessing an invalid
/// model-specific register would normally result in a CPU exception. This function uses Structured
/// Handling (SEH) to safely catch CPU exceptions and to turn them into an [`Error`]. This prevents
/// a hang.
pub fn write_msr(register: u32, value: u64) -> Result<(), Error> {
    unsafe {
        write_msr_safe(register, value)
    }.into_result()?;

    Ok(())
}
