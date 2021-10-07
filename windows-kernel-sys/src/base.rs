#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use cty::*;

include!(concat!(env!("OUT_DIR"), "/base.rs"));

pub const STATUS_SUCCESS:             NTSTATUS = 0x00000000;
pub const STATUS_UNSUCCESSFUL:        NTSTATUS = 0xC0000001 as u32 as i32;
pub const STATUS_NOT_IMPLEMENTED:     NTSTATUS = 0xC0000002 as u32 as i32;
pub const STATUS_ACCESS_VIOLATION:    NTSTATUS = 0xC0000005 as u32 as i32;
pub const STATUS_INVALID_PARAMETER:   NTSTATUS = 0xC000000D as u32 as i32;
pub const STATUS_END_OF_FILE:         NTSTATUS = 0xC0000011 as u32 as i32;
pub const STATUS_INVALID_USER_BUFFER: NTSTATUS = 0xC00000E8 as u32 as i32;
