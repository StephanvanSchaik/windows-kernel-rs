use crate::error::Error;

pub struct UserPtr {
    ptr: *mut cty::c_void,
    read_size: usize,
    write_size: usize,
}

impl UserPtr {
    pub unsafe fn new(
        ptr: *mut cty::c_void,
        read_size: usize,
        write_size: usize,
    ) -> Self {
        Self {
            ptr,
            read_size,
            write_size,
        }
    }

    pub fn read_size(&self) -> usize {
        self.read_size
    }

    pub fn write_size(&self) -> usize {
        self.write_size
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.ptr as *const u8, self.read_size)
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr as *mut u8, self.write_size)
        }
    }

    pub fn read<T>(&self, obj: &mut T) -> Result<(), Error> {
        if core::mem::size_of::<T>() > self.read_size {
            return Err(Error::ACCESS_VIOLATION);
        }

        unsafe {
            core::ptr::copy_nonoverlapping(
                self.ptr as _,
                obj,
                core::mem::size_of::<T>(),
            );
        }

        Ok(())
    }

    pub fn write<T>(&mut self, obj: &T) -> Result<(), Error> {
        if core::mem::size_of::<T>() > self.write_size {
            return Err(Error::ACCESS_VIOLATION);
        }

        unsafe {
            core::ptr::copy_nonoverlapping(
                obj,
                self.ptr as _,
                core::mem::size_of::<T>(),
            );
        }

        Ok(())
    }
}
