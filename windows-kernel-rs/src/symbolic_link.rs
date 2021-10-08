use crate::error::{Error, IntoResult};
use crate::string::create_unicode_string;
use widestring::U16CString;
use windows_kernel_sys::ntoskrnl::{IoCreateSymbolicLink, IoDeleteSymbolicLink};

pub struct SymbolicLink {
    name: U16CString,
}

impl SymbolicLink {
    pub fn new(name: &str, target: &str) -> Result<Self, Error> {
        // Convert the name to UTF-16 and then create a UNICODE_STRING.
        let name = U16CString::from_str(name).unwrap();
        let mut name_ptr = create_unicode_string(name.as_slice());

        // Convert the target to UTF-16 and then create a UNICODE_STRING.
        let target = U16CString::from_str(target).unwrap();
        let mut target_ptr = create_unicode_string(target.as_slice());

        unsafe {
            IoCreateSymbolicLink(&mut name_ptr, &mut target_ptr)
        }.into_result()?;

        Ok(Self {
            name,
        })
    }
}

impl Drop for SymbolicLink {
    fn drop(&mut self) {
        let mut name_ptr = create_unicode_string(self.name.as_slice());

        unsafe {
            IoDeleteSymbolicLink(&mut name_ptr);
        }
    }
}
