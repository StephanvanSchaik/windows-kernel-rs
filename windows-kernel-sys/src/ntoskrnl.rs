#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::base::*;

#[link(name = "wrapper_ntoskrnl")]
extern "C" {
    pub fn _ExInitializeFastMutex(mutex: PFAST_MUTEX);
    pub fn _ExTryAcquirePushLockExclusive(push_lock: PEX_PUSH_LOCK) -> bool;
    pub fn _ExAcquirePushLockExclusive(push_lock: PEX_PUSH_LOCK);
    pub fn _ExReleasePushLockExclusive(push_lock: PEX_PUSH_LOCK);
    pub fn _ExTryAcquirePushLockShared(push_lock: PEX_PUSH_LOCK) -> bool;
    pub fn _ExAcquirePushLockShared(push_lock: PEX_PUSH_LOCK);
    pub fn _ExReleasePushLockShared(push_lock: PEX_PUSH_LOCK);
    pub fn _ExReleasePushLock(push_lock: PEX_PUSH_LOCK);
    pub fn _IoGetCurrentIrpStackLocation(irp: PIRP) -> PIO_STACK_LOCATION;
    pub fn _IoGetNextIrpStackLocation(irp: PIRP) -> PIO_STACK_LOCATION;
    pub fn _IoSetCompletionRoutine(
        irp: PIRP,
        completion_routine: PIO_COMPLETION_ROUTINE,
        context: PVOID,
        invoke_on_success: BOOLEAN,
        invoke_on_error: BOOLEAN,
        invoke_on_cancel: BOOLEAN,
    );
    pub fn _IoCompleteRequest(irp: PIRP, priority_boost: CCHAR);
    pub fn _MmGetMdlByteCount(mdl: PMDL) -> ULONG;
    pub fn _MmGetMdlByteOffset(mdl: PMDL) -> ULONG;
    pub fn _MmGetSystemAddressForMdlSafe(mdl: PMDL, priority: ULONG) -> PVOID;
    pub fn _ObDereferenceObject(p: *mut cty::c_void);
    pub fn _ObReferenceObject(p: *mut cty::c_void);
}

pub use self::_ExInitializeFastMutex as ExInitializeFastMutex;
pub use self::_ExTryAcquirePushLockExclusive as ExTryAcquirePushLockExclusive;
pub use self::_ExAcquirePushLockExclusive as ExAcquirePushLockExclusive;
pub use self::_ExReleasePushLockExclusive as ExReleasePushLockExclusive;
pub use self::_ExTryAcquirePushLockShared as ExTryAcquirePushLockShared;
pub use self::_ExAcquirePushLockShared as ExAcquirePushLockShared;
pub use self::_ExReleasePushLockShared as ExReleasePushLockShared;
pub use self::_ExReleasePushLock as ExReleasePushLock;
pub use self::_IoGetCurrentIrpStackLocation as IoGetCurrentIrpStackLocation;
pub use self::_IoGetNextIrpStackLocation as IoGetNextIrpStackLocation;
pub use self::_IoSetCompletionRoutine as IoSetCompletionRoutine;
pub use self::_IoCompleteRequest as IoCompleteRequest;
pub use self::_MmGetMdlByteCount as MmGetMdlByteCount;
pub use self::_MmGetMdlByteOffset as MmGetMdlByteOffset;
pub use self::_MmGetSystemAddressForMdlSafe as MmGetSystemAddressForMdlSafe;
pub use self::_ObDereferenceObject as ObDereferenceObject;
pub use self::_ObReferenceObject as ObReferenceObject;

pub use self::IoGetCurrentProcess as PsGetCurrentProcess;

include!(concat!(env!("OUT_DIR"), "/ntoskrnl.rs"));
