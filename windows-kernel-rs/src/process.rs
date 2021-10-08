use crate::error::{Error, IntoResult};
use windows_kernel_sys::base::{KAPC_STATE, PEPROCESS};
use windows_kernel_sys::ntoskrnl::{KeStackAttachProcess, KeUnstackDetachProcess};
use windows_kernel_sys::ntoskrnl::{ObDereferenceObject, ObReferenceObject};
use windows_kernel_sys::ntoskrnl::{PsGetCurrentProcess, PsLookupProcessByProcessId};

#[derive(Clone, Debug)]
pub struct Process {
    pub process: PEPROCESS,
}

impl Process {
    pub fn current() -> Result<Self, Error> {
        let process = unsafe {
            PsGetCurrentProcess()
        };

        Ok(Self {
            process,
        })
    }

    pub fn by_id(process_id: u64) -> Result<Self, Error> {
        let mut process = core::ptr::null_mut();

        unsafe {
            PsLookupProcessByProcessId(process_id as _, &mut process)
        }.into_result()?;

        Ok(Self {
            process,
        })
    }

    pub fn attach(&self) -> ProcessAttachment {
        unsafe {
            ProcessAttachment::attach(self.process)
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            ObDereferenceObject(self.process as _);
        }
    }
}

pub struct ProcessAttachment {
    process: PEPROCESS,
    state: KAPC_STATE,
}

impl ProcessAttachment {
    pub unsafe fn attach(process: PEPROCESS) -> Self {
        let mut state: KAPC_STATE = core::mem::zeroed();
    
        ObReferenceObject(process as _);
        KeStackAttachProcess(process, &mut state);

        Self {
            process,
            state,
        }
    }
}

impl Drop for ProcessAttachment {
    fn drop(&mut self) {
        unsafe {
            KeUnstackDetachProcess(&mut self.state);
            ObDereferenceObject(self.process as _);
        }
    }
}
