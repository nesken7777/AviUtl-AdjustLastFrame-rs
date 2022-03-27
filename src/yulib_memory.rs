#![allow(non_snake_case)]

use std::{ffi::c_void, mem::size_of, ptr::null_mut};

use windows_sys::Win32::Foundation::BOOL;
use windows_sys::Win32::System::Memory::*;

#[repr(C)]
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
            eprintln!("Alloc is {:X}", size * size_of::<T>());
            self.size = size;
            return true;
        } else {
            let temp =
                MemReAlloc(self.mem as *mut c_void, size * size_of::<T>(), zeroinit) as *mut T;
            if temp.is_null() {
                return false;
            }
            self.mem = temp;
            self.size = size;
            return true;
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
    unsafe { HeapFree(GetProcessHeap(), 0, mem) }
}
