#![allow(non_snake_case)]

use std::{ffi::c_void, mem::size_of};

use windows_sys::Win32::System::Memory::*;

#[repr(C)]
pub struct CMemory<T> {
    pub mem: *mut T,
    pub size: u32,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for CMemory<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

impl<T> CMemory<T> {
    pub unsafe fn Alloc(&mut self, size: u32, zeroinit: bool) -> bool {
        if self.mem.is_null() {
            self.mem = MemAlloc(size * size_of::<T>() as u32, zeroinit) as *mut T;
            if self.mem.is_null() {
                return false;
            }
            self.size = size;
            return true;
        } else {
            let temp = MemReAlloc(
                self.mem as *mut c_void,
                size * size_of::<T>() as u32,
                zeroinit,
            ) as *mut T;
            if temp.is_null() {
                return false;
            }
            self.mem = temp;
            self.size = size;
            return true;
        }
    }
}

unsafe fn MemAlloc(size: u32, zeroinit: bool) -> *mut c_void {
    HeapAlloc(
        GetProcessHeap(),
        if zeroinit { HEAP_ZERO_MEMORY } else { 0 },
        size as usize,
    )
}

unsafe fn MemReAlloc(mem: *mut c_void, size: u32, zeroinit: bool) -> *mut c_void {
    HeapReAlloc(
        GetProcessHeap(),
        if zeroinit { HEAP_ZERO_MEMORY } else { 0 },
        mem,
        size as usize,
    )
}
