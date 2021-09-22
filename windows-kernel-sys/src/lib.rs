#![no_std]

#![feature(untagged_unions)]

pub mod base;

#[cfg(feature = "intrin")]
pub mod intrin;
#[cfg(feature = "ntoskrnl")]
pub mod ntoskrnl;
#[cfg(feature = "netio")]
pub mod netio;

pub use cty::*;
