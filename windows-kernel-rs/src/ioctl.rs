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
        const READ_WRITE_DATA = FILE_READ_DATA | FILE_WRITE_DATA;
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

impl Into<u32> for TransferMethod {
    fn into(self) -> u32 {
        match self {
            Self::Neither => METHOD_NEITHER,
            Self::InputDirect => METHOD_IN_DIRECT,
            Self::OutputDirect => METHOD_OUT_DIRECT,
            Self::Buffered => METHOD_BUFFERED,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlCode(pub DeviceType, pub RequiredAccess, pub u32, pub TransferMethod);

impl ControlCode {
    const METHOD_BITS: usize = 2;
    const NUM_BITS:    usize = 12;
    const ACCESS_BITS: usize = 2;
    const TYPE_BITS:   usize = 16;

    const METHOD_SHIFT: usize = 0;
    const NUM_SHIFT:    usize = Self::METHOD_SHIFT + Self::METHOD_BITS;
    const ACCESS_SHIFT: usize = Self::NUM_SHIFT + Self::NUM_BITS;
    const TYPE_SHIFT:   usize = Self::ACCESS_SHIFT + Self::ACCESS_BITS;

    const METHOD_MASK: u32 = (1 << Self::METHOD_BITS) - 1;
    const NUM_MASK:    u32 = (1 << Self::NUM_BITS) - 1;
    const ACCESS_MASK: u32 = (1 << Self::ACCESS_BITS) - 1;
    const TYPE_MASK:   u32 = (1 << Self::TYPE_BITS) - 1;

    pub fn device_type(&self) -> DeviceType {
        self.0
    }

    pub fn required_access(&self) -> RequiredAccess {
        self.1
    }

    pub fn number(&self) -> u32 {
        self.2
    }

    pub fn transfer_method(&self) -> TransferMethod {
        self.3
    }
}

impl From<u32> for ControlCode {
    fn from(value: u32) -> Self {
        let method = (value >> Self::METHOD_SHIFT) & Self::METHOD_MASK;
        let num    = (value >> Self::NUM_SHIFT)    & Self::NUM_MASK;
        let access = (value >> Self::ACCESS_SHIFT) & Self::ACCESS_MASK;
        let ty     = (value >> Self::TYPE_SHIFT)   & Self::TYPE_MASK;

        Self(
            ty.into(),
            RequiredAccess::from_bits(access).unwrap_or(RequiredAccess::READ_DATA),
            num,
            method.into()
        )
    }
}

impl Into<u32> for ControlCode {
    fn into(self) -> u32 {
        let method = Into::<u32>::into(self.3) << Self::METHOD_SHIFT;
        let num    = self.2 << Self::NUM_SHIFT;
        let access = self.1.bits() << Self::ACCESS_SHIFT;
        let ty     = Into::<u32>::into(self.0) << Self::TYPE_SHIFT;

        ty | access | num | method
    }
}
