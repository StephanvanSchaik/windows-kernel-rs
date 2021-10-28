#include "wrapper.h"

void _ExInitializeFastMutex(
	PFAST_MUTEX fast_mutex
) {
	ExInitializeFastMutex(fast_mutex);
}

BOOLEAN _ExTryAcquirePushLockExclusive(
	PEX_PUSH_LOCK push_lock
) {
	return ExTryAcquirePushLockExclusive(push_lock);
}

void _ExAcquirePushLockExclusive(
	PEX_PUSH_LOCK push_lock
) {
	ExAcquirePushLockExclusive(push_lock);
}

void _ExReleasePushLockExclusive(
	PEX_PUSH_LOCK push_lock
) {
	ExReleasePushLockExclusive(push_lock);
}

BOOLEAN _ExTryAcquirePushLockShared(
	PEX_PUSH_LOCK push_lock
) {
	return ExTryAcquirePushLockShared(push_lock);
}

void _ExAcquirePushLockShared(
	PEX_PUSH_LOCK push_lock
) {
	ExAcquirePushLockShared(push_lock);
}

void _ExReleasePushLockShared(
	PEX_PUSH_LOCK push_lock
) {
	ExReleasePushLockShared(push_lock);
}

void _ExReleasePushLock(
	PEX_PUSH_LOCK push_lock
) {
	ExReleasePushLock(push_lock);
}

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

void _IoCompleteRequest(
	PIRP irp,
	CCHAR priority_boost
) {
	IoCompleteRequest(irp, priority_boost);
}

ULONG _MmGetMdlByteCount(PMDL mdl) {
	return MmGetMdlByteCount(mdl);
}

ULONG _MmGetMdlByteOffset(PMDL mdl) {
	return MmGetMdlByteOffset(mdl);
}

PVOID _MmGetSystemAddressForMdlSafe(PMDL mdl, ULONG priority) {
	return MmGetSystemAddressForMdlSafe(mdl, priority);
}

void _ObDereferenceObject(PVOID p) {
	ObDereferenceObject(p);
}

void _ObReferenceObject(PVOID p) {
	ObReferenceObject(p);
}
