#![allow(non_snake_case)]

use std::{ffi::c_void, mem::size_of, ptr::null_mut};

use windows::{
    core::Error,
    Win32::System::Memory::{
        GetProcessHeap, HeapAlloc, HeapFree, HeapReAlloc, HEAP_NONE, HEAP_ZERO_MEMORY,
    },
};

pub struct CMemory<T> {
    pub mem: *mut T,
    pub size: usize,
}
impl<T> Default for CMemory<T> {
    fn default() -> Self {
        CMemory {
            mem: null_mut(),
            size: 0,
        }
    }
}

impl<T> Drop for CMemory<T> {
    fn drop(&mut self) {
        if !self.mem.is_null() {
            let _ = Memfree(self.mem as *mut c_void);
        }
    }
}

impl<T> CMemory<T> {
    pub unsafe fn Alloc(&mut self, size: usize, zeroinit: bool) -> Result<(), Error> {
        if self.mem.is_null() {
            self.mem = MemAlloc(size * size_of::<T>(), zeroinit)? as *mut T;
            self.size = size;
            Ok(())
        } else {
            let temp =
                MemReAlloc(self.mem as *mut c_void, size * size_of::<T>(), zeroinit)? as *mut T;
            self.mem = temp;
            self.size = size;
            Ok(())
        }
    }
}

unsafe fn MemAlloc(size: usize, zeroinit: bool) -> Result<*mut c_void, Error> {
    Ok(HeapAlloc(
        GetProcessHeap()?,
        zeroinit.then_some(HEAP_ZERO_MEMORY).unwrap_or(HEAP_NONE),
        size,
    ))
}

unsafe fn MemReAlloc(mem: *mut c_void, size: usize, zeroinit: bool) -> Result<*mut c_void, Error> {
    Ok(HeapReAlloc(
        GetProcessHeap()?,
        zeroinit.then_some(HEAP_ZERO_MEMORY).unwrap_or(HEAP_NONE),
        Some(mem),
        size,
    ))
}

fn Memfree(mem: *mut c_void) -> Result<(), Error> {
    unsafe {
        let process_heap = GetProcessHeap()?;
        HeapFree(process_heap, HEAP_NONE, Some(mem))
    }
}
