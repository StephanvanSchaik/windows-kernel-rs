use bitflags::bitflags;
use crate::device::DeviceType;
use windows_kernel_sys::base::{
    FILE_ANY_ACCESS, FILE_READ_DATA, FILE_WRITE_DATA,
    METHOD_NEITHER, METHOD_IN_DIRECT, METHOD_OUT_DIRECT, METHOD_BUFFERED
};

bitflags! {
    pub struct RequiredAccess: u32 {
        const ANY_ACCESS = FILE_ANY_ACCESS;
        const READ_DATA = FILE_READ_DATA;
        const WRITE_DATA = FILE_WRITE_DATA;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TransferMethod {
    Neither = METHOD_NEITHER,
    InputDirect = METHOD_IN_DIRECT,
    OutputDirect = METHOD_OUT_DIRECT,
    Buffered = METHOD_BUFFERED,
}

impl From<u32> for TransferMethod {
    fn from(value: u32) -> Self {
        match value & 0x3 {
            METHOD_NEITHER => Self::Neither,
            METHOD_IN_DIRECT => Self::InputDirect,
            METHOD_OUT_DIRECT => Self::OutputDirect,
            METHOD_BUFFERED => Self::Buffered,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlCode(DeviceType, RequiredAccess, u32, TransferMethod);

impl From<u32> for ControlCode {
    fn from(value: u32) -> Self {
        Self(
            ((value >> 16) & 0xffff).into(),
            RequiredAccess::from_bits((value >> 14) & 0x3)
                .unwrap_or(RequiredAccess::empty()),
            (value >> 2) & 0xfff,
            ((value >> 0) & 0x3).into(),
        )
    }
}
