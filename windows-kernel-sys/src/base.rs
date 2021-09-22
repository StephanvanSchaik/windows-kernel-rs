#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use cty::*;

include!(concat!(env!("OUT_DIR"), "/base.rs"));

pub const STATUS_SUCCESS: NTSTATUS = 0x00000000;
