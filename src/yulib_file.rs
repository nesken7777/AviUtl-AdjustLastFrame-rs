use std::ffi::c_void;
use std::ptr::null;
use std::ptr::null_mut;
use windows_sys::core::*;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Storage::FileSystem::*;
use windows_sys::Win32::System::SystemServices::*;

#[repr(C)]
pub struct CFile {
    pub file: HANDLE,
}

impl Default for CFile {
    fn default() -> Self {
        CFile {
            file: INVALID_HANDLE_VALUE,
        }
    }
}

impl Drop for CFile {
    fn drop(&mut self) {
        unsafe {
            if self.file != -1 && self.file != 0 {
                CloseHandle(self.file);
            }
        }
    }
}

impl CFile {
    pub unsafe fn OpenExisting(&mut self, filename: PCSTR) -> bool {
        if !self.Close() {
            return false;
        }
        self.file = CreateFileA(
            filename,
            GENERIC_READ,
            0,
            null(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            0,
        );
        self.file != INVALID_HANDLE_VALUE
    }

    pub unsafe fn Close(&mut self) -> bool {
        if self.file == INVALID_HANDLE_VALUE {
            return true;
        }
        if CloseHandle(self.file) == 0 {
            return false;
        }
        self.file = INVALID_HANDLE_VALUE;
        true
    }

    pub unsafe fn Size(&self) -> u32 {
        GetFileSize(self.file, null_mut())
    }

    pub unsafe fn Read(&self, buf: *mut c_void, mut size: u32) -> BOOL {
        ReadFile(self.file, buf, size, &mut size, null_mut())
    }
}
