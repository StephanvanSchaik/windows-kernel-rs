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
}

pub use self::_IoGetCurrentIrpStackLocation as IoGetCurrentIrpStackLocation;
pub use self::_IoGetNextIrpStackLocation as IoGetNextIrpStackLocation;
pub use self::_IoSetCompletionRoutine as IoSetCompletionRoutine;

include!(concat!(env!("OUT_DIR"), "/ntoskrnl.rs"));
