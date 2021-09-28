#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::base::*;

#[link(name = "wrapper_ntoskrnl")]
extern "C" {
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
}

pub use self::_IoGetCurrentIrpStackLocation as IoGetCurrentIrpStackLocation;
pub use self::_IoGetNextIrpStackLocation as IoGetNextIrpStackLocation;
pub use self::_IoSetCompletionRoutine as IoSetCompletionRoutine;
pub use self::_IoCompleteRequest as IoCompleteRequest;
pub use self::_MmGetMdlByteCount as MmGetMdlByteCount;
pub use self::_MmGetMdlByteOffset as MmGetMdlByteOffset;
pub use self::_MmGetSystemAddressForMdlSafe as MmGetSystemAddressForMdlSafe;

include!(concat!(env!("OUT_DIR"), "/ntoskrnl.rs"));
