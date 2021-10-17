//! This module provides utilities to query information about the version of Microsoft Windows.

use crate::error::{Error, IntoResult};
use windows_kernel_sys::base::RTL_OSVERSIONINFOW;
use windows_kernel_sys::ntoskrnl::RtlGetVersion;

/// Represents version information for Microsoft Windows.
pub struct VersionInfo {
    version_info: RTL_OSVERSIONINFOW,
}

impl VersionInfo {
    /// Uses [`RtlGetVersion`] to query the version info for Microsoft Windows.
    pub fn query() -> Result<Self, Error> {
        let mut version_info: RTL_OSVERSIONINFOW = unsafe { core::mem::zeroed() };

        version_info.dwOSVersionInfoSize = core::mem::size_of::<RTL_OSVERSIONINFOW>() as u32;

        unsafe {
            RtlGetVersion(&mut version_info)
        }.into_result()?;

        Ok(Self {
            version_info,
        })
    }

    /// Retrieves the major version of Microsoft Windows.
    pub fn major(&self) -> u32 {
        self.version_info.dwMajorVersion
    }

    /// Retrieves the minor version of Microsoft Windows.
    pub fn minor(&self) -> u32 {
        self.version_info.dwMinorVersion
    }

    /// Retrieves the build number of Microsoft Windows.
    pub fn build_number(&self) -> u32 {
        self.version_info.dwBuildNumber
    }
}
