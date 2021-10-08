use bitflags::bitflags;
use winapi::um::winioctl::{
    FILE_ANY_ACCESS,
    METHOD_NEITHER, METHOD_IN_DIRECT, METHOD_OUT_DIRECT, METHOD_BUFFERED
};
use winapi::um::winnt::{
    FILE_READ_DATA, FILE_WRITE_DATA,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceType {
    Port8042,
    Acpi,
    Battery,
    Beep,
    BusExtender,
    Cdrom,
    CdromFileSystem,
    Changer,
    Controller,
    DataLink,
    Dfs,
    DfsFileSystem,
    DfsVolume,
    Disk,
    DiskFileSystem,
    Dvd,
    FileSystem,
    Fips,
    FullscreenVideo,
    InportPort,
    Keyboard,
    Ks,
    Ksec,
    Mailslot,
    MassStorage,
    MidiIn,
    MidiOut,
    Modem,
    Mouse,
    MultiUncProvider,
    NamedPipe,
    Network,
    NetworkBrowser,
    NetworkFileSystem,
    NetworkRedirector,
    Null,
    ParallelPort,
    PhysicalNetcard,
    Printer,
    Scanner,
    Screen,
    Serenum,
    SerialPort,
    SerialMousePort,
    Smartcard,
    Smb,
    Sound,
    Streams,
    Tape,
    TapeFileSystem,
    Termsrv,
    Transport,
    Unknown,
    Vdm,
    Video,
    VirtualDisk,
    WaveIn,
    WaveOut,
}

impl Into<u32> for DeviceType {
    fn into(self) -> u32 {
        match self {
            DeviceType::Port8042 => winapi::um::winioctl::FILE_DEVICE_8042_PORT,
            DeviceType::Acpi => winapi::um::winioctl::FILE_DEVICE_ACPI,
            DeviceType::Battery => winapi::um::winioctl::FILE_DEVICE_BATTERY,
            DeviceType::Beep => winapi::um::winioctl::FILE_DEVICE_BEEP,
            DeviceType::BusExtender => winapi::um::winioctl::FILE_DEVICE_BUS_EXTENDER,
            DeviceType::Cdrom => winapi::um::winioctl::FILE_DEVICE_CD_ROM,
            DeviceType::CdromFileSystem => winapi::um::winioctl::FILE_DEVICE_CD_ROM_FILE_SYSTEM,
            DeviceType::Changer => winapi::um::winioctl::FILE_DEVICE_CHANGER,
            DeviceType::Controller => winapi::um::winioctl::FILE_DEVICE_CONTROLLER,
            DeviceType::DataLink => winapi::um::winioctl::FILE_DEVICE_DATALINK,
            DeviceType::Dfs => winapi::um::winioctl::FILE_DEVICE_DFS,
            DeviceType::DfsFileSystem => winapi::um::winioctl::FILE_DEVICE_DFS_FILE_SYSTEM,
            DeviceType::DfsVolume => winapi::um::winioctl::FILE_DEVICE_DFS_VOLUME,
            DeviceType::Disk => winapi::um::winioctl::FILE_DEVICE_DISK,
            DeviceType::DiskFileSystem => winapi::um::winioctl::FILE_DEVICE_DISK_FILE_SYSTEM,
            DeviceType::Dvd => winapi::um::winioctl::FILE_DEVICE_DVD,
            DeviceType::FileSystem => winapi::um::winioctl::FILE_DEVICE_FILE_SYSTEM,
            DeviceType::Fips => winapi::um::winioctl::FILE_DEVICE_FIPS,
            DeviceType::FullscreenVideo => winapi::um::winioctl::FILE_DEVICE_FULLSCREEN_VIDEO,
            DeviceType::InportPort => winapi::um::winioctl::FILE_DEVICE_INPORT_PORT,
            DeviceType::Keyboard => winapi::um::winioctl::FILE_DEVICE_KEYBOARD,
            DeviceType::Ks => winapi::um::winioctl::FILE_DEVICE_KS,
            DeviceType::Ksec => winapi::um::winioctl::FILE_DEVICE_KSEC,
            DeviceType::Mailslot => winapi::um::winioctl::FILE_DEVICE_MAILSLOT,
            DeviceType::MassStorage => winapi::um::winioctl::FILE_DEVICE_MASS_STORAGE,
            DeviceType::MidiIn => winapi::um::winioctl::FILE_DEVICE_MIDI_IN,
            DeviceType::MidiOut => winapi::um::winioctl::FILE_DEVICE_MIDI_OUT,
            DeviceType::Modem => winapi::um::winioctl::FILE_DEVICE_MODEM,
            DeviceType::Mouse => winapi::um::winioctl::FILE_DEVICE_MOUSE,
            DeviceType::MultiUncProvider => winapi::um::winioctl::FILE_DEVICE_MULTI_UNC_PROVIDER,
            DeviceType::NamedPipe => winapi::um::winioctl::FILE_DEVICE_NAMED_PIPE,
            DeviceType::Network => winapi::um::winioctl::FILE_DEVICE_NETWORK,
            DeviceType::NetworkBrowser => winapi::um::winioctl::FILE_DEVICE_NETWORK_BROWSER,
            DeviceType::NetworkFileSystem => winapi::um::winioctl::FILE_DEVICE_NETWORK_FILE_SYSTEM,
            DeviceType::NetworkRedirector => winapi::um::winioctl::FILE_DEVICE_NETWORK_REDIRECTOR,
            DeviceType::Null => winapi::um::winioctl::FILE_DEVICE_NULL,
            DeviceType::ParallelPort => winapi::um::winioctl::FILE_DEVICE_PARALLEL_PORT,
            DeviceType::PhysicalNetcard => winapi::um::winioctl::FILE_DEVICE_PHYSICAL_NETCARD,
            DeviceType::Printer => winapi::um::winioctl::FILE_DEVICE_PRINTER,
            DeviceType::Scanner => winapi::um::winioctl::FILE_DEVICE_SCANNER,
            DeviceType::Screen => winapi::um::winioctl::FILE_DEVICE_SCREEN,
            DeviceType::Serenum => winapi::um::winioctl::FILE_DEVICE_SERENUM,
            DeviceType::SerialMousePort => winapi::um::winioctl::FILE_DEVICE_SERIAL_MOUSE_PORT,
            DeviceType::SerialPort => winapi::um::winioctl::FILE_DEVICE_SERIAL_PORT,
            DeviceType::Smartcard => winapi::um::winioctl::FILE_DEVICE_SMARTCARD,
            DeviceType::Smb => winapi::um::winioctl::FILE_DEVICE_SMB,
            DeviceType::Sound => winapi::um::winioctl::FILE_DEVICE_SOUND,
            DeviceType::Streams => winapi::um::winioctl::FILE_DEVICE_STREAMS,
            DeviceType::Tape => winapi::um::winioctl::FILE_DEVICE_TAPE,
            DeviceType::TapeFileSystem => winapi::um::winioctl::FILE_DEVICE_TAPE_FILE_SYSTEM,
            DeviceType::Termsrv => winapi::um::winioctl::FILE_DEVICE_TERMSRV,
            DeviceType::Transport => winapi::um::winioctl::FILE_DEVICE_TRANSPORT,
            DeviceType::Unknown => winapi::um::winioctl::FILE_DEVICE_UNKNOWN,
            DeviceType::Vdm => winapi::um::winioctl::FILE_DEVICE_VDM,
            DeviceType::Video => winapi::um::winioctl::FILE_DEVICE_VIDEO,
            DeviceType::VirtualDisk => winapi::um::winioctl::FILE_DEVICE_VIRTUAL_DISK,
            DeviceType::WaveIn => winapi::um::winioctl::FILE_DEVICE_WAVE_IN,
            DeviceType::WaveOut => winapi::um::winioctl::FILE_DEVICE_WAVE_OUT,
        }
    }
}

impl From<u32> for DeviceType {
    fn from(value: u32) -> Self {
        match value {
            winapi::um::winioctl::FILE_DEVICE_8042_PORT => DeviceType::Port8042,
            winapi::um::winioctl::FILE_DEVICE_ACPI => DeviceType::Acpi,
            winapi::um::winioctl::FILE_DEVICE_BATTERY => DeviceType::Battery,
            winapi::um::winioctl::FILE_DEVICE_BEEP => DeviceType::Beep,
            winapi::um::winioctl::FILE_DEVICE_BUS_EXTENDER => DeviceType::BusExtender,
            winapi::um::winioctl::FILE_DEVICE_CD_ROM => DeviceType::Cdrom,
            winapi::um::winioctl::FILE_DEVICE_CD_ROM_FILE_SYSTEM => DeviceType::CdromFileSystem,
            winapi::um::winioctl::FILE_DEVICE_CHANGER => DeviceType::Changer,
            winapi::um::winioctl::FILE_DEVICE_CONTROLLER => DeviceType::Controller,
            winapi::um::winioctl::FILE_DEVICE_DATALINK => DeviceType::DataLink,
            winapi::um::winioctl::FILE_DEVICE_DFS => DeviceType::Dfs,
            winapi::um::winioctl::FILE_DEVICE_DFS_FILE_SYSTEM => DeviceType::DfsFileSystem,
            winapi::um::winioctl::FILE_DEVICE_DFS_VOLUME => DeviceType::DfsVolume,
            winapi::um::winioctl::FILE_DEVICE_DISK => DeviceType::Disk,
            winapi::um::winioctl::FILE_DEVICE_DISK_FILE_SYSTEM => DeviceType::DiskFileSystem,
            winapi::um::winioctl::FILE_DEVICE_DVD => DeviceType::Dvd,
            winapi::um::winioctl::FILE_DEVICE_FILE_SYSTEM => DeviceType::FileSystem,
            winapi::um::winioctl::FILE_DEVICE_FIPS => DeviceType::Fips,
            winapi::um::winioctl::FILE_DEVICE_FULLSCREEN_VIDEO => DeviceType::FullscreenVideo,
            winapi::um::winioctl::FILE_DEVICE_INPORT_PORT => DeviceType::InportPort,
            winapi::um::winioctl::FILE_DEVICE_KEYBOARD => DeviceType::Keyboard,
            winapi::um::winioctl::FILE_DEVICE_KS => DeviceType::Ks,
            winapi::um::winioctl::FILE_DEVICE_KSEC => DeviceType::Ksec,
            winapi::um::winioctl::FILE_DEVICE_MAILSLOT => DeviceType::Mailslot,
            winapi::um::winioctl::FILE_DEVICE_MASS_STORAGE => DeviceType::MassStorage,
            winapi::um::winioctl::FILE_DEVICE_MIDI_IN => DeviceType::MidiIn,
            winapi::um::winioctl::FILE_DEVICE_MIDI_OUT => DeviceType::MidiOut,
            winapi::um::winioctl::FILE_DEVICE_MODEM => DeviceType::Modem,
            winapi::um::winioctl::FILE_DEVICE_MOUSE => DeviceType::Mouse,
            winapi::um::winioctl::FILE_DEVICE_MULTI_UNC_PROVIDER => DeviceType::MultiUncProvider,
            winapi::um::winioctl::FILE_DEVICE_NAMED_PIPE => DeviceType::NamedPipe,
            winapi::um::winioctl::FILE_DEVICE_NETWORK => DeviceType::Network,
            winapi::um::winioctl::FILE_DEVICE_NETWORK_BROWSER => DeviceType::NetworkBrowser,
            winapi::um::winioctl::FILE_DEVICE_NETWORK_FILE_SYSTEM => DeviceType::NetworkFileSystem,
            winapi::um::winioctl::FILE_DEVICE_NETWORK_REDIRECTOR => DeviceType::NetworkRedirector,
            winapi::um::winioctl::FILE_DEVICE_NULL => DeviceType::Null,
            winapi::um::winioctl::FILE_DEVICE_PARALLEL_PORT => DeviceType::ParallelPort,
            winapi::um::winioctl::FILE_DEVICE_PHYSICAL_NETCARD => DeviceType::PhysicalNetcard,
            winapi::um::winioctl::FILE_DEVICE_PRINTER => DeviceType::Printer,
            winapi::um::winioctl::FILE_DEVICE_SCANNER => DeviceType::Scanner,
            winapi::um::winioctl::FILE_DEVICE_SCREEN => DeviceType::Screen,
            winapi::um::winioctl::FILE_DEVICE_SERENUM => DeviceType::Serenum,
            winapi::um::winioctl::FILE_DEVICE_SERIAL_MOUSE_PORT => DeviceType::SerialMousePort,
            winapi::um::winioctl::FILE_DEVICE_SERIAL_PORT => DeviceType::SerialPort,
            winapi::um::winioctl::FILE_DEVICE_SMARTCARD => DeviceType::Smartcard,
            winapi::um::winioctl::FILE_DEVICE_SMB => DeviceType::Smb,
            winapi::um::winioctl::FILE_DEVICE_SOUND => DeviceType::Sound,
            winapi::um::winioctl::FILE_DEVICE_STREAMS => DeviceType::Streams,
            winapi::um::winioctl::FILE_DEVICE_TAPE => DeviceType::Tape,
            winapi::um::winioctl::FILE_DEVICE_TAPE_FILE_SYSTEM => DeviceType::TapeFileSystem,
            winapi::um::winioctl::FILE_DEVICE_TERMSRV => DeviceType::Termsrv,
            winapi::um::winioctl::FILE_DEVICE_TRANSPORT => DeviceType::Transport,
            winapi::um::winioctl::FILE_DEVICE_UNKNOWN => DeviceType::Unknown,
            winapi::um::winioctl::FILE_DEVICE_VDM => DeviceType::Vdm,
            winapi::um::winioctl::FILE_DEVICE_VIDEO => DeviceType::Video,
            winapi::um::winioctl::FILE_DEVICE_VIRTUAL_DISK => DeviceType::VirtualDisk,
            winapi::um::winioctl::FILE_DEVICE_WAVE_IN => DeviceType::WaveIn,
            winapi::um::winioctl::FILE_DEVICE_WAVE_OUT => DeviceType::WaveOut,
            _ => DeviceType::Unknown,
        }
    }
}

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

#[macro_export]
macro_rules! ioctl_none {
    ($(#[$attr:meta])* $name:ident, $dev_ty:expr, $nr:expr) => {
        $(#[$attr])*
        pub unsafe fn $name(handle: *mut std::ffi::c_void) -> Result<u32, $crate::Error> {
            let code = $crate::ControlCode(
                $dev_ty,
                $crate::RequiredAccess::ANY_ACCESS,
                $nr,
                $crate::TransferMethod::Neither,
            ).into();
            let mut return_value = 0;

            let status = $crate::DeviceIoControl(
                handle as _,
                code,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
                &mut return_value,
                std::ptr::null_mut(),
            ) != 0;

            match status {
                true => Ok(return_value),
                _ => Err(std::io::Error::last_os_error())?,
            }
        }
    }
}

#[macro_export]
macro_rules! ioctl_read {
    ($(#[$attr:meta])* $name:ident, $dev_ty:expr, $nr:expr, $ty:ty) => {
        $(#[$attr])*
        pub unsafe fn $name(handle: *mut std::ffi::c_void, data: *mut $ty) -> Result<u32, $crate::Error> {
            let code = $crate::ControlCode(
                $dev_ty,
                $crate::RequiredAccess::READ_DATA,
                $nr,
                $crate::TransferMethod::Buffered,
            ).into();
            let mut return_value = 0;

            let status = $crate::DeviceIoControl(
                handle as _,
                code,
                data as _,
                std::mem::size_of::<$ty>() as _,
                data as _,
                std::mem::size_of::<$ty>() as _,
                &mut return_value,
                std::ptr::null_mut(),
            ) != 0;

            match status {
                true => Ok(return_value),
                _ => Err(std::io::Error::last_os_error())?,
            }
        }
    }
}

#[macro_export]
macro_rules! ioctl_write {
    ($(#[$attr:meta])* $name:ident, $dev_ty:expr, $nr:expr, $ty:ty) => {
        $(#[$attr])*
        pub unsafe fn $name(handle: *mut std::ffi::c_void, data: *const $ty) -> Result<u32, $crate::Error> {
            let code = $crate::ControlCode(
                $dev_ty,
                $crate::RequiredAccess::WRITE_DATA,
                $nr,
                $crate::TransferMethod::Buffered,
            ).into();
            let mut return_value = 0;

            let status = $crate::DeviceIoControl(
                handle as _,
                code,
                data as _,
                std::mem::size_of::<$ty>() as _,
                std::ptr::null_mut(),
                0,
                &mut return_value,
                std::ptr::null_mut(),
            ) != 0;

            match status {
                true => Ok(return_value),
                _ => Err(std::io::Error::last_os_error())?,
            }
        }
    }
}

#[macro_export]
macro_rules! ioctl_readwrite {
    ($(#[$attr:meta])* $name:ident, $dev_ty:expr, $nr:expr, $ty:ty) => {
        $(#[$attr])*
        pub unsafe fn $name(handle: *mut std::ffi::c_void, data: *mut $ty) -> Result<u32, $crate::Error> {
            let code = $crate::ControlCode(
                $dev_ty,
                $crate::RequiredAccess::READ_WRITE_DATA,
                $nr,
                $crate::TransferMethod::Buffered,
            ).into();
            let mut return_value = 0;

            let status = $crate::DeviceIoControl(
                handle as _,
                code,
                data as _,
                std::mem::size_of::<$ty>() as _,
                data as _,
                std::mem::size_of::<$ty>() as _,
                &mut return_value,
                std::ptr::null_mut(),
            ) != 0;

            match status {
                true => Ok(return_value),
                _ => Err(std::io::Error::last_os_error())?,
            }
        }
    }
}
