use windows_kernel_sys::base::ANSI_STRING;
use windows_kernel_sys::ntoskrnl::DbgPrint;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    // Format the string using the `alloc::format!` as this is guaranteed to return a `String`
    // instead of a `Result` that we would have to `unwrap`. This ensures that this code stays
    // panic-free.
    let s = alloc::format!("{}", args);

    // Print the string. We must make sure to not pass this user-supplied string as the format
    // string, as `DbgPrint` may then format any format specifiers it contains. This could
    // potentially be an attack vector.
    let s = ANSI_STRING {
        Length: s.len() as u16,
        MaximumLength: s.len() as u16,
        Buffer: s.as_ptr() as _,
    };

    unsafe { DbgPrint("%Z\0".as_ptr() as _, &s) };
}
