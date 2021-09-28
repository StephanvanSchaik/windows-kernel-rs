use crate::error::Error;

pub struct UserPtr {
    ptr: *mut cty::c_void,
    read_size: usize,
    write_size: usize,
    return_size: usize,
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
            return_size: 0,
        }
    }

    pub fn return_size(&self) -> usize {
        self.return_size
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

        self.return_size = core::mem::size_of::<T>();

        Ok(())
    }
}


