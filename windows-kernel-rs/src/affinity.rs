//! This module provides functions to get information about the logical CPUs in the system, and to
//! run closures on specific or all CPUs.

use crate::error::{Error, IntoResult};
use windows_kernel_sys::base::{ALL_PROCESSOR_GROUPS, GROUP_AFFINITY, PROCESSOR_NUMBER, ULONG_PTR};
use windows_kernel_sys::ntoskrnl::{
    KeIpiGenericCall, KeGetCurrentProcessorNumberEx, KeGetProcessorNumberFromIndex,
    KeQueryActiveProcessorCountEx, KeRevertToUserGroupAffinityThread,
    KeSetSystemGroupAffinityThread,
};

/// Uses [`KeGetCurrentProcessorNumberEx`] to get the logical number associated with the CPU that
/// is currently running our code.
pub fn get_current_cpu_num() -> u32 {
    unsafe {
        KeGetCurrentProcessorNumberEx(core::ptr::null_mut())
    }
}

/// Uses [`KeQueryActiveProcessorCountEx`] to get the number of CPUs in the system, that is all the
/// CPUs from all the different CPU groups are counted, such that each of them has a logical
/// number.
pub fn get_cpu_count() -> u32 {
    unsafe {
        KeQueryActiveProcessorCountEx(ALL_PROCESSOR_GROUPS as _)
    }
}

/// This is the callback used by [`run_on_each_cpu_parallel`] to run the closure on all CPUs.
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

/// Runs the given closure on all CPUs in the system without interrupting all CPUs to force them to
/// switch to kernel mode. Instead, this is a more graceful version that simply relies on
/// [`run_on_cpu`] to switch to all the possible CPUs by configuring the affinity, and execute the
/// closure on the selected CPU. Upon executing the closure on all CPUs, the affinity is restored.
/// Also see [`run_on_each_cpu_parallel`] which is a more aggressive version that relies on an IPI
/// to run a given closure on all CPUs in parallel.
pub fn run_on_each_cpu<F>(
    f: &mut F,
) -> Result<(), Error>
where
    F: FnMut() -> Result<(), Error>,
{
    for cpu_num in 0..get_cpu_count() {
        run_on_cpu(cpu_num, f)?;
    }

    Ok(())
}

/// Runs the given closure on all CPUs in the system by broadcasting an Inter-Processor Interrupt
/// (IPI) to interrupt all CPUs to force them to switch to kernel mode to run the given closure.
/// Upon execution of the closure, these CPUs resume their work. Also see [`run_on_each_cpu`] which
/// is a friendlier version that does not rely on an IPI but instead configures the affinity to run
/// a given a closure on all CPUs.
pub fn run_on_each_cpu_parallel<F>(
    f: &F,
)
where
    F: Fn(),
{
    unsafe {
        KeIpiGenericCall(Some(broadcast_callback::<F>), f as *const _ as ULONG_PTR);
    }
}

/// Runs the given closure on the CPU with the given CPU number by temporarily configuring the CPU
/// affinity to only contain the given CPU number. Upon switching to the selected CPU, the CPU
/// executes the closure. Then the original affinity is restored.
pub fn run_on_cpu<F>(
    cpu_num: u32,
    f: &mut F,
) -> Result<(), Error>
where
    F: FnMut() -> Result<(), Error>,
{
    let mut processor_num = PROCESSOR_NUMBER {
        Group: 0,
        Number: 0,
        Reserved: 0,
    };

    unsafe {
        KeGetProcessorNumberFromIndex(cpu_num, &mut processor_num)
    }.into_result()?;

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

    let result = f();

    unsafe {
        KeRevertToUserGroupAffinityThread(&mut previous);
    }

    result
}
