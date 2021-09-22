#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[link(name = "wrapper_intrin")]
extern "C" {
    pub fn read_cr3() -> u64;
    pub fn write_cr3(value: u64);
    pub fn read_msr(register: u32) -> u64;
    pub fn write_msr(register: u32, value: u64);
}
