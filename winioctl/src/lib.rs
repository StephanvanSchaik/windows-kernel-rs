mod error;
mod ioctl;

pub use crate::error::Error;
pub use crate::ioctl::{ControlCode, DeviceType, RequiredAccess, TransferMethod};

pub use winapi::um::ioapiset::DeviceIoControl;
