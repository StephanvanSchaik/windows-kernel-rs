use crate::error::Error;
use windows_kernel_sys::base::{ALL_PROCESSOR_GROUPS, GROUP_AFFINITY, PROCESSOR_NUMBER, ULONG_PTR};
use windows_kernel_sys::base::STATUS_SUCCESS;
use windows_kernel_sys::ntoskrnl::{
    KeIpiGenericCall, KeGetCurrentProcessorNumberEx, KeGetProcessorNumberFromIndex,
    KeQueryActiveProcessorCountEx, KeRevertToUserGroupAffinityThread,
    KeSetSystemGroupAffinityThread,
};

pub fn get_current_cpu_num() -> u32 {
    unsafe {
        KeGetCurrentProcessorNumberEx(core::ptr::null_mut())
    }
}

pub fn get_cpu_count() -> u32 {
    unsafe {
        KeQueryActiveProcessorCountEx(ALL_PROCESSOR_GROUPS as _)
    }
}

unsafe extern "C" fn broadcast_callback<F>(
    context: ULONG_PTR,
) -> ULONG_PTR
where
    F: FnMut(),
{
    let f = &mut *(context as *mut F);
    f();

    0
}

pub fn run_on_each_cpu<F>(
    f: F,
)
where
    F: Fn(),
{
    unsafe {
        KeIpiGenericCall(Some(broadcast_callback::<F>), &f as *const _ as ULONG_PTR);
    }
}

pub fn run_on_cpu<F>(
    cpu_num: u32,
    mut f: F,
) -> Result<(), Error>
where
    F: FnMut(),
{
    let mut processor_num = PROCESSOR_NUMBER {
        Group: 0,
        Number: 0,
        Reserved: 0,
    };

    let status = unsafe {
        KeGetProcessorNumberFromIndex(cpu_num, &mut processor_num)
    };

    if status != STATUS_SUCCESS {
        return Err(Error::from_kernel_errno(status));
    }

    let mut previous = GROUP_AFFINITY {
        Mask: 0,
        Group: 0,
        Reserved: [0; 3],
    };

    let mut affinity = GROUP_AFFINITY {
        Mask: 1 << processor_num.Number,
        Group: processor_num.Group,
        Reserved: [0; 3],
    };

    unsafe {
        KeSetSystemGroupAffinityThread(&mut affinity, &mut previous);
    }

    f();

    unsafe {
        KeRevertToUserGroupAffinityThread(&mut previous);
    }

    Ok(())
}
