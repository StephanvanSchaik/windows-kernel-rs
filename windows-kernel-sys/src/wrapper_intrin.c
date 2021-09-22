#define _AMD64_

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

void write_msr(
	unsigned long Register,
	unsigned __int64 Value
) {
	__writemsr(Register, Value);
}
