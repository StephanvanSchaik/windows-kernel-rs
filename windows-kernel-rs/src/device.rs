use alloc::boxed::Box;
use bitflags::bitflags;
use crate::error::Error;
use crate::request::{IoRequest, IoControlRequest, ReadRequest, WriteRequest};
use windows_kernel_sys::base::{DEVICE_OBJECT, IRP, NTSTATUS};
use windows_kernel_sys::base::STATUS_SUCCESS;
use windows_kernel_sys::base::{IRP_MJ_CREATE, IRP_MJ_CLOSE, IRP_MJ_CLEANUP, IRP_MJ_READ, IRP_MJ_WRITE, IRP_MJ_DEVICE_CONTROL};
use windows_kernel_sys::ntoskrnl::{IoDeleteDevice, IoGetCurrentIrpStackLocation};

#[derive(Copy, Clone, Debug)]
pub enum Access {
    NonExclusive,
    Exclusive,
}

impl Access {
    pub fn is_exclusive(&self) -> bool {
        match *self {
            Access::Exclusive => true,
            _ => false,
        }
    }
}

bitflags! {
    pub struct DeviceFlags: u32 {
        const SECURE_OPEN = windows_kernel_sys::base::FILE_DEVICE_SECURE_OPEN;
    }
}

bitflags! {
    pub struct DeviceDoFlags: u32 {
        const DO_BUFFERED_IO = windows_kernel_sys::base::DO_BUFFERED_IO;
        const DO_DIRECT_IO   = windows_kernel_sys::base::DO_DIRECT_IO;
    }
}

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
            DeviceType::Port8042 => windows_kernel_sys::base::FILE_DEVICE_8042_PORT,
            DeviceType::Acpi => windows_kernel_sys::base::FILE_DEVICE_ACPI,
            DeviceType::Battery => windows_kernel_sys::base::FILE_DEVICE_BATTERY,
            DeviceType::Beep => windows_kernel_sys::base::FILE_DEVICE_BEEP,
            DeviceType::BusExtender => windows_kernel_sys::base::FILE_DEVICE_BUS_EXTENDER,
            DeviceType::Cdrom => windows_kernel_sys::base::FILE_DEVICE_CD_ROM,
            DeviceType::CdromFileSystem => windows_kernel_sys::base::FILE_DEVICE_CD_ROM_FILE_SYSTEM,
            DeviceType::Changer => windows_kernel_sys::base::FILE_DEVICE_CHANGER,
            DeviceType::Controller => windows_kernel_sys::base::FILE_DEVICE_CONTROLLER,
            DeviceType::DataLink => windows_kernel_sys::base::FILE_DEVICE_DATALINK,
            DeviceType::Dfs => windows_kernel_sys::base::FILE_DEVICE_DFS,
            DeviceType::DfsFileSystem => windows_kernel_sys::base::FILE_DEVICE_DFS_FILE_SYSTEM,
            DeviceType::DfsVolume => windows_kernel_sys::base::FILE_DEVICE_DFS_VOLUME,
            DeviceType::Disk => windows_kernel_sys::base::FILE_DEVICE_DISK,
            DeviceType::DiskFileSystem => windows_kernel_sys::base::FILE_DEVICE_DISK_FILE_SYSTEM,
            DeviceType::Dvd => windows_kernel_sys::base::FILE_DEVICE_DVD,
            DeviceType::FileSystem => windows_kernel_sys::base::FILE_DEVICE_FILE_SYSTEM,
            DeviceType::Fips => windows_kernel_sys::base::FILE_DEVICE_FIPS,
            DeviceType::FullscreenVideo => windows_kernel_sys::base::FILE_DEVICE_FULLSCREEN_VIDEO,
            DeviceType::InportPort => windows_kernel_sys::base::FILE_DEVICE_INPORT_PORT,
            DeviceType::Keyboard => windows_kernel_sys::base::FILE_DEVICE_KEYBOARD,
            DeviceType::Ks => windows_kernel_sys::base::FILE_DEVICE_KS,
            DeviceType::Ksec => windows_kernel_sys::base::FILE_DEVICE_KSEC,
            DeviceType::Mailslot => windows_kernel_sys::base::FILE_DEVICE_MAILSLOT,
            DeviceType::MassStorage => windows_kernel_sys::base::FILE_DEVICE_MASS_STORAGE,
            DeviceType::MidiIn => windows_kernel_sys::base::FILE_DEVICE_MIDI_IN,
            DeviceType::MidiOut => windows_kernel_sys::base::FILE_DEVICE_MIDI_OUT,
            DeviceType::Modem => windows_kernel_sys::base::FILE_DEVICE_MODEM,
            DeviceType::Mouse => windows_kernel_sys::base::FILE_DEVICE_MOUSE,
            DeviceType::MultiUncProvider => windows_kernel_sys::base::FILE_DEVICE_MULTI_UNC_PROVIDER,
            DeviceType::NamedPipe => windows_kernel_sys::base::FILE_DEVICE_NAMED_PIPE,
            DeviceType::Network => windows_kernel_sys::base::FILE_DEVICE_NETWORK,
            DeviceType::NetworkBrowser => windows_kernel_sys::base::FILE_DEVICE_NETWORK_BROWSER,
            DeviceType::NetworkFileSystem => windows_kernel_sys::base::FILE_DEVICE_NETWORK_FILE_SYSTEM,
            DeviceType::NetworkRedirector => windows_kernel_sys::base::FILE_DEVICE_NETWORK_REDIRECTOR,
            DeviceType::Null => windows_kernel_sys::base::FILE_DEVICE_NULL,
            DeviceType::ParallelPort => windows_kernel_sys::base::FILE_DEVICE_PARALLEL_PORT,
            DeviceType::PhysicalNetcard => windows_kernel_sys::base::FILE_DEVICE_PHYSICAL_NETCARD,
            DeviceType::Printer => windows_kernel_sys::base::FILE_DEVICE_PRINTER,
            DeviceType::Scanner => windows_kernel_sys::base::FILE_DEVICE_SCANNER,
            DeviceType::Screen => windows_kernel_sys::base::FILE_DEVICE_SCREEN,
            DeviceType::Serenum => windows_kernel_sys::base::FILE_DEVICE_SERENUM,
            DeviceType::SerialMousePort => windows_kernel_sys::base::FILE_DEVICE_SERIAL_MOUSE_PORT,
            DeviceType::SerialPort => windows_kernel_sys::base::FILE_DEVICE_SERIAL_PORT,
            DeviceType::Smartcard => windows_kernel_sys::base::FILE_DEVICE_SMARTCARD,
            DeviceType::Smb => windows_kernel_sys::base::FILE_DEVICE_SMB,
            DeviceType::Sound => windows_kernel_sys::base::FILE_DEVICE_SOUND,
            DeviceType::Streams => windows_kernel_sys::base::FILE_DEVICE_STREAMS,
            DeviceType::Tape => windows_kernel_sys::base::FILE_DEVICE_TAPE,
            DeviceType::TapeFileSystem => windows_kernel_sys::base::FILE_DEVICE_TAPE_FILE_SYSTEM,
            DeviceType::Termsrv => windows_kernel_sys::base::FILE_DEVICE_TERMSRV,
            DeviceType::Transport => windows_kernel_sys::base::FILE_DEVICE_TRANSPORT,
            DeviceType::Unknown => windows_kernel_sys::base::FILE_DEVICE_UNKNOWN,
            DeviceType::Vdm => windows_kernel_sys::base::FILE_DEVICE_VDM,
            DeviceType::Video => windows_kernel_sys::base::FILE_DEVICE_VIDEO,
            DeviceType::VirtualDisk => windows_kernel_sys::base::FILE_DEVICE_VIRTUAL_DISK,
            DeviceType::WaveIn => windows_kernel_sys::base::FILE_DEVICE_WAVE_IN,
            DeviceType::WaveOut => windows_kernel_sys::base::FILE_DEVICE_WAVE_OUT,
        }
    }
}

impl From<u32> for DeviceType {
    fn from(value: u32) -> Self {
        match value {
            windows_kernel_sys::base::FILE_DEVICE_8042_PORT => DeviceType::Port8042,
            windows_kernel_sys::base::FILE_DEVICE_ACPI => DeviceType::Acpi,
            windows_kernel_sys::base::FILE_DEVICE_BATTERY => DeviceType::Battery,
            windows_kernel_sys::base::FILE_DEVICE_BEEP => DeviceType::Beep,
            windows_kernel_sys::base::FILE_DEVICE_BUS_EXTENDER => DeviceType::BusExtender,
            windows_kernel_sys::base::FILE_DEVICE_CD_ROM => DeviceType::Cdrom,
            windows_kernel_sys::base::FILE_DEVICE_CD_ROM_FILE_SYSTEM => DeviceType::CdromFileSystem,
            windows_kernel_sys::base::FILE_DEVICE_CHANGER => DeviceType::Changer,
            windows_kernel_sys::base::FILE_DEVICE_CONTROLLER => DeviceType::Controller,
            windows_kernel_sys::base::FILE_DEVICE_DATALINK => DeviceType::DataLink,
            windows_kernel_sys::base::FILE_DEVICE_DFS => DeviceType::Dfs,
            windows_kernel_sys::base::FILE_DEVICE_DFS_FILE_SYSTEM => DeviceType::DfsFileSystem,
            windows_kernel_sys::base::FILE_DEVICE_DFS_VOLUME => DeviceType::DfsVolume,
            windows_kernel_sys::base::FILE_DEVICE_DISK => DeviceType::Disk,
            windows_kernel_sys::base::FILE_DEVICE_DISK_FILE_SYSTEM => DeviceType::DiskFileSystem,
            windows_kernel_sys::base::FILE_DEVICE_DVD => DeviceType::Dvd,
            windows_kernel_sys::base::FILE_DEVICE_FILE_SYSTEM => DeviceType::FileSystem,
            windows_kernel_sys::base::FILE_DEVICE_FIPS => DeviceType::Fips,
            windows_kernel_sys::base::FILE_DEVICE_FULLSCREEN_VIDEO => DeviceType::FullscreenVideo,
            windows_kernel_sys::base::FILE_DEVICE_INPORT_PORT => DeviceType::InportPort,
            windows_kernel_sys::base::FILE_DEVICE_KEYBOARD => DeviceType::Keyboard,
            windows_kernel_sys::base::FILE_DEVICE_KS => DeviceType::Ks,
            windows_kernel_sys::base::FILE_DEVICE_KSEC => DeviceType::Ksec,
            windows_kernel_sys::base::FILE_DEVICE_MAILSLOT => DeviceType::Mailslot,
            windows_kernel_sys::base::FILE_DEVICE_MASS_STORAGE => DeviceType::MassStorage,
            windows_kernel_sys::base::FILE_DEVICE_MIDI_IN => DeviceType::MidiIn,
            windows_kernel_sys::base::FILE_DEVICE_MIDI_OUT => DeviceType::MidiOut,
            windows_kernel_sys::base::FILE_DEVICE_MODEM => DeviceType::Modem,
            windows_kernel_sys::base::FILE_DEVICE_MOUSE => DeviceType::Mouse,
            windows_kernel_sys::base::FILE_DEVICE_MULTI_UNC_PROVIDER => DeviceType::MultiUncProvider,
            windows_kernel_sys::base::FILE_DEVICE_NAMED_PIPE => DeviceType::NamedPipe,
            windows_kernel_sys::base::FILE_DEVICE_NETWORK => DeviceType::Network,
            windows_kernel_sys::base::FILE_DEVICE_NETWORK_BROWSER => DeviceType::NetworkBrowser,
            windows_kernel_sys::base::FILE_DEVICE_NETWORK_FILE_SYSTEM => DeviceType::NetworkFileSystem,
            windows_kernel_sys::base::FILE_DEVICE_NETWORK_REDIRECTOR => DeviceType::NetworkRedirector,
            windows_kernel_sys::base::FILE_DEVICE_NULL => DeviceType::Null,
            windows_kernel_sys::base::FILE_DEVICE_PARALLEL_PORT => DeviceType::ParallelPort,
            windows_kernel_sys::base::FILE_DEVICE_PHYSICAL_NETCARD => DeviceType::PhysicalNetcard,
            windows_kernel_sys::base::FILE_DEVICE_PRINTER => DeviceType::Printer,
            windows_kernel_sys::base::FILE_DEVICE_SCANNER => DeviceType::Scanner,
            windows_kernel_sys::base::FILE_DEVICE_SCREEN => DeviceType::Screen,
            windows_kernel_sys::base::FILE_DEVICE_SERENUM => DeviceType::Serenum,
            windows_kernel_sys::base::FILE_DEVICE_SERIAL_MOUSE_PORT => DeviceType::SerialMousePort,
            windows_kernel_sys::base::FILE_DEVICE_SERIAL_PORT => DeviceType::SerialPort,
            windows_kernel_sys::base::FILE_DEVICE_SMARTCARD => DeviceType::Smartcard,
            windows_kernel_sys::base::FILE_DEVICE_SMB => DeviceType::Smb,
            windows_kernel_sys::base::FILE_DEVICE_SOUND => DeviceType::Sound,
            windows_kernel_sys::base::FILE_DEVICE_STREAMS => DeviceType::Streams,
            windows_kernel_sys::base::FILE_DEVICE_TAPE => DeviceType::Tape,
            windows_kernel_sys::base::FILE_DEVICE_TAPE_FILE_SYSTEM => DeviceType::TapeFileSystem,
            windows_kernel_sys::base::FILE_DEVICE_TERMSRV => DeviceType::Termsrv,
            windows_kernel_sys::base::FILE_DEVICE_TRANSPORT => DeviceType::Transport,
            windows_kernel_sys::base::FILE_DEVICE_UNKNOWN => DeviceType::Unknown,
            windows_kernel_sys::base::FILE_DEVICE_VDM => DeviceType::Vdm,
            windows_kernel_sys::base::FILE_DEVICE_VIDEO => DeviceType::Video,
            windows_kernel_sys::base::FILE_DEVICE_VIRTUAL_DISK => DeviceType::VirtualDisk,
            windows_kernel_sys::base::FILE_DEVICE_WAVE_IN => DeviceType::WaveIn,
            windows_kernel_sys::base::FILE_DEVICE_WAVE_OUT => DeviceType::WaveOut,
            _ => DeviceType::Unknown,
        }
    }
}

#[repr(C)]
pub struct device_operations {
    dispatch: Option<extern "C" fn (*mut DEVICE_OBJECT, *mut IRP, u8) -> NTSTATUS>,
    release: Option<extern "C" fn (*mut DEVICE_OBJECT)>,
}

pub struct Device {
    raw: *mut DEVICE_OBJECT,
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Device {
    pub unsafe fn from_raw(raw: *mut DEVICE_OBJECT) -> Self {
        Self {
            raw,
        }
    }

    pub unsafe fn as_raw(&self) -> *const DEVICE_OBJECT {
        self.raw as *const _
    }

    pub unsafe fn as_raw_mut(&self) -> *mut DEVICE_OBJECT {
        self.raw
    }

    pub fn into_raw(mut self) -> *mut DEVICE_OBJECT {
        core::mem::replace(&mut self.raw, core::ptr::null_mut())
    }

    pub(crate) fn extension(&self) -> &DeviceExtension {
        unsafe {
            &*((*self.raw).DeviceExtension as *const DeviceExtension)
        }
    }

    pub(crate) fn extension_mut(&self) -> &mut DeviceExtension {
        unsafe {
            &mut *((*self.raw).DeviceExtension as *mut DeviceExtension)
        }
    }

    pub(crate) fn device_type(&self) -> DeviceType {
        self.extension().device_type
    }

    pub(crate) fn vtable(&self) -> &device_operations {
        unsafe {
            &*(self.extension().vtable as *const _)
        }
    }

    pub fn data<T: DeviceOperations>(&self) -> &T {
        unsafe {
            &*(self.extension().data as *const T)
        }
    }

    pub fn data_mut<T: DeviceOperations>(&self) -> &mut T {
        unsafe {
            &mut *(self.extension().data as *mut T)
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }

        unsafe {    
            if let Some(release) = self.vtable().release {
                release(self.raw);
            }

            IoDeleteDevice(self.raw);
        }
    }
}

pub struct RequestError(pub Error, pub IoRequest);

pub enum Completion {
    Complete(u32, IoRequest),
}

pub trait DeviceOperations: Sync + Sized {
    fn create(&mut self, _device: &Device, request: IoRequest) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request))
    }

    fn close(&mut self, _device: &Device, request: IoRequest) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request))
    }

    fn cleanup(&mut self, _device: &Device, request: IoRequest) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request))
    }

    fn read(&mut self, _device: &Device, request: ReadRequest) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request.into()))
    }

    fn write(&mut self, _device: &Device, request: WriteRequest) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request.into()))
    }

    fn ioctl(&mut self, _device: &Device, request: IoControlRequest) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request.into()))
    }
}

extern "C" fn dispatch_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
    irp: *mut IRP,
    major: u8,
) -> NTSTATUS {
    let device = unsafe { Device::from_raw(device) };
    let data: &mut T = device.data_mut();
    let request = unsafe { IoRequest::from_raw(irp) };

    let result = match major as _ {
        IRP_MJ_CREATE => data.create(&device, request),
        IRP_MJ_CLOSE => data.close(&device, request),
        IRP_MJ_CLEANUP => data.cleanup(&device, request),
        IRP_MJ_READ => {
            let read_request = ReadRequest {
                inner: request,
            };

            data.read(&device, read_request)
        }
        IRP_MJ_WRITE => {
            let write_request = WriteRequest {
                inner: request,
            };

            data.write(&device, write_request)
        }
        IRP_MJ_DEVICE_CONTROL => {
            let control_request = IoControlRequest {
                inner: request,
            };

            if device.device_type() == control_request.control_code().device_type() {
                data.ioctl(&device, control_request)
            } else {
                Err(RequestError(Error::INVALID_PARAMETER, control_request.into()))
            }
        }
        _ => {
            Err(RequestError(Error::INVALID_PARAMETER, request))
        }
    };

    device.into_raw();

    match result {
        Ok(Completion::Complete(size, request)) => {
            request.complete(Ok(size));
            STATUS_SUCCESS
        }
        Err(RequestError(e, request)) => {
            let status = e.to_ntstatus();
            request.complete(Err(e));
            status
        }
    }
}

extern fn release_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
) {
    unsafe {
        let extension = (*device).DeviceExtension as *mut DeviceExtension;

        let ptr = core::mem::replace(&mut (*extension).data, core::ptr::null_mut());
        Box::from_raw(ptr as *mut T);
    }
}

pub(crate) struct DeviceOperationsVtable<T>(core::marker::PhantomData<T>);

impl<T: DeviceOperations> DeviceOperationsVtable<T> {
    pub(crate) const VTABLE: device_operations = device_operations {
        dispatch: Some(dispatch_callback::<T>),
        release: Some(release_callback::<T>),
    };
}

#[repr(C)]
pub struct DeviceExtension {
    pub(crate) vtable: *const device_operations,
    pub(crate) data: *mut cty::c_void,
    pub(crate) device_type: DeviceType,
}

pub extern "C" fn dispatch_device(
    device: *mut DEVICE_OBJECT,
    irp: *mut IRP,
) -> NTSTATUS {
    let stack_location = unsafe { &*IoGetCurrentIrpStackLocation(irp) };
    let device = unsafe { Device::from_raw(device) };
    let vtable = device.vtable();

    match vtable.dispatch {
        Some(dispatch) => dispatch(device.into_raw(), irp, stack_location.MajorFunction),
        _ => {
            device.into_raw();
            STATUS_SUCCESS
        }
    }
}
