use windows_kernel_sys::base::NTSTATUS;
use windows_kernel_sys::base::{
    STATUS_UNSUCCESSFUL,
    STATUS_NOT_IMPLEMENTED,
    STATUS_ACCESS_VIOLATION,
    STATUS_INVALID_PARAMETER,
};

pub struct Error(NTSTATUS);

impl Error {
    pub const UNSUCCESSFULL: Error = Error(STATUS_UNSUCCESSFUL);
    pub const NOT_IMPLEMENTED: Error = Error(STATUS_NOT_IMPLEMENTED);
    pub const ACCESS_VIOLATION: Error = Error(STATUS_ACCESS_VIOLATION);
    pub const INVALID_PARAMETER: Error = Error(STATUS_INVALID_PARAMETER);

    pub fn from_kernel_errno(status: NTSTATUS) -> Error {
        Error(status)
    }

    pub fn to_kernel_errno(&self) -> NTSTATUS {
        self.0
    }
}
