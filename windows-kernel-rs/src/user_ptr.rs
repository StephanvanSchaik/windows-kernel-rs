use crate::error::Error;

pub enum UserPtr {
    Buffered {
        ptr: *mut cty::c_void,
        read_size: usize,
        write_size: usize,
    },
    Direct {
        read_ptr: *const cty::c_void,
        write_ptr: *mut cty::c_void,
        read_size: usize,
        write_size: usize,
    },
    Neither,
}

impl UserPtr {
    pub unsafe fn new_buffered(
        ptr: *mut cty::c_void,
        read_size: usize,
        write_size: usize,
    ) -> Self {
        Self::Buffered{
            ptr,
            read_size,
            write_size,
        }
    }

    pub unsafe fn new_direct(
        read_ptr: *const cty::c_void,
        write_ptr: *mut cty::c_void,
        read_size: usize,
        write_size: usize,
    ) -> Self {
        Self::Direct {
            read_ptr,
            write_ptr,
            read_size,
            write_size,
        }
    }

    pub unsafe fn new_neither() -> Self {
        Self::Neither
    }

    pub fn read_size(&self) -> usize {
        match self {
            Self::Buffered { read_size, .. } => *read_size,
            Self::Direct { read_size, .. } => *read_size,
            Self::Neither => 0,
        }
    }

    pub fn write_size(&self) -> usize {
        match self {
            Self::Buffered { write_size, .. } => *write_size,
            Self::Direct { write_size, .. } => *write_size,
            Self::Neither => 0,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        let (ptr, size) = match self {
            Self::Buffered { ptr, read_size, .. } => (*ptr as _, *read_size),
            Self::Direct { read_ptr, read_size, .. } => (*read_ptr, *read_size),
            Self::Neither => (core::ptr::null(), 0),
        };

        if ptr.is_null() || size == 0 {
            &[]
        } else {
            unsafe {
                core::slice::from_raw_parts(ptr as *const u8, size)
            }
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        let (ptr, size) = match self {
            Self::Buffered { ptr, write_size, .. } => (*ptr, *write_size),
            Self::Direct { write_ptr, write_size, .. } => (*write_ptr, *write_size),
            Self::Neither => (core::ptr::null_mut(), 0),
        };

        if ptr.is_null() || size == 0 {
            &mut []
        } else {
            unsafe {
                core::slice::from_raw_parts_mut(ptr as *mut u8, size)
            }
        }
    }

    pub fn read<T: Copy + Default>(&self) -> Result<T, Error> {
        let (ptr, size) = match self {
            Self::Buffered { ptr, read_size, .. } => (*ptr as _, *read_size),
            Self::Direct { read_ptr, read_size, .. } => (*read_ptr, *read_size),
            Self::Neither => (core::ptr::null(), 0),
        };

        if ptr.is_null() || size == 0 {
            return Err(Error::INVALID_PARAMETER);
        }

        if core::mem::size_of::<T>() > size {
            return Err(Error::INVALID_USER_BUFFER);
        }

        let mut obj = T::default();

        unsafe {
            core::ptr::copy_nonoverlapping(
                ptr as _,
                &mut obj,
                1,
            );
        }

        Ok(obj)
    }

    pub fn write<T: Copy>(&mut self, obj: &T) -> Result<(), Error> {
        let (ptr, size) = match self {
            Self::Buffered { ptr, write_size, .. } => (*ptr, *write_size),
            Self::Direct { write_ptr, write_size, .. } => (*write_ptr, *write_size),
            Self::Neither => (core::ptr::null_mut(), 0),
        };

        if ptr.is_null() || size == 0 {
            return Err(Error::INVALID_PARAMETER);
        }

        if core::mem::size_of::<T>() > size {
            return Err(Error::INVALID_USER_BUFFER);
        }

        unsafe {
            core::ptr::copy_nonoverlapping(
                obj,
                ptr as _,
                1,
            );
        }

        Ok(())
    }
}
