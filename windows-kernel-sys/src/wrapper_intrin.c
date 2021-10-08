#define _AMD64_

#include "wdm.h"
#include "intrin.h"

unsigned __int64 read_cr3(void) {
	return __readcr3();
}

void write_cr3(unsigned __int64 Value) {
	__writecr3(Value);
}

unsigned __int64 read_msr(
	unsigned long Register
) {
	return __readmsr(Register);
}

NTSTATUS read_msr_safe(
	unsigned long Register,
	unsigned __int64 *Value
) {
	if (!Value) {
		return STATUS_ACCESS_VIOLATION;
	}

	__try {
		*Value = __readmsr(Register);
	} __except(EXCEPTION_EXECUTE_HANDLER) {
		return STATUS_ACCESS_VIOLATION;
	}

	return STATUS_SUCCESS;
}

void write_msr(
	unsigned long Register,
	unsigned __int64 Value
) {
	__writemsr(Register, Value);
}

NTSTATUS write_msr_safe(
	unsigned long Register,
	unsigned __int64 Value
) {
	__try {
		__writemsr(Register, Value);
	} __except(EXCEPTION_EXECUTE_HANDLER) {
		return STATUS_ACCESS_VIOLATION;
	}

	return STATUS_SUCCESS;
}
