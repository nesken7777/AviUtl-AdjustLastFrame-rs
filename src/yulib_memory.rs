#![allow(non_snake_case)]

use core::{ffi::c_void, mem::size_of, ptr::null_mut};

use windows_sys::Win32::Foundation::BOOL;
use windows_sys::Win32::System::Memory::*;

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
            Memfree(self.mem as *mut c_void);
        }
    }
}

impl<T> CMemory<T> {
    pub unsafe fn Alloc(&mut self, size: usize, zeroinit: bool) -> bool {
        if self.mem.is_null() {
            self.mem = MemAlloc(size * size_of::<T>(), zeroinit) as *mut T;
            if self.mem.is_null() {
                return false;
            }
            self.size = size;
            true
        } else {
            let temp =
                MemReAlloc(self.mem as *mut c_void, size * size_of::<T>(), zeroinit) as *mut T;
            if temp.is_null() {
                return false;
            }
            self.mem = temp;
            self.size = size;
            true
        }
    }
}

unsafe fn MemAlloc(size: usize, zeroinit: bool) -> *mut c_void {
    HeapAlloc(
        GetProcessHeap(),
        if zeroinit { HEAP_ZERO_MEMORY } else { 0 },
        size,
    )
}

unsafe fn MemReAlloc(mem: *mut c_void, size: usize, zeroinit: bool) -> *mut c_void {
    HeapReAlloc(
        GetProcessHeap(),
        if zeroinit { HEAP_ZERO_MEMORY } else { 0 },
        mem,
        size,
    )
}

fn Memfree(mem: *mut c_void) -> BOOL {
    unsafe {
        let a = HeapFree(GetProcessHeap(), 0, mem);
        if a != 0 {
            true.into()
        } else {
            false.into()
        }
    }
}
