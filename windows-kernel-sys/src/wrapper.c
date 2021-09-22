#include "wrapper.h"

PIO_STACK_LOCATION _IoGetCurrentIrpStackLocation(PIRP irp) {
	return IoGetCurrentIrpStackLocation(irp);
}

PIO_STACK_LOCATION _IoGetNextIrpStackLocation(PIRP irp) {
	return IoGetNextIrpStackLocation(irp);
}

void _IoSetCompletionRoutine(
	PIRP irp,
	PIO_COMPLETION_ROUTINE completion_routine, 
	PVOID context,
	BOOLEAN invoke_on_success,
	BOOLEAN invoke_on_error,
	BOOLEAN invoke_on_cancel
) {
	IoSetCompletionRoutine(irp, completion_routine, context, invoke_on_success, invoke_on_error, invoke_on_cancel);
}
