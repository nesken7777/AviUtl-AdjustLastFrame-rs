use windows::core::Error;
use windows::core::PCSTR;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Foundation::GENERIC_READ;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Foundation::INVALID_HANDLE_VALUE;
use windows::Win32::Storage::FileSystem::GetFileSize;
use windows::Win32::Storage::FileSystem::FILE_ATTRIBUTE_NORMAL;
use windows::Win32::Storage::FileSystem::FILE_SHARE_NONE;
use windows::Win32::Storage::FileSystem::OPEN_EXISTING;
use windows::Win32::Storage::FileSystem::{CreateFileA, ReadFile};
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
            if !self.file.is_invalid() {
                let _ = CloseHandle(self.file);
            }
        }
    }
}

impl CFile {
    pub unsafe fn OpenExisting(&mut self, filename: PCSTR) -> Result<(), Error> {
        self.Close()?;
        self.file = CreateFileA(
            filename,
            GENERIC_READ.0,
            FILE_SHARE_NONE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE(0),
        )?;
        Ok(())
    }

    pub unsafe fn Close(&mut self) -> Result<(), Error> {
        if self.file == INVALID_HANDLE_VALUE {
            return Ok(());
        }
        CloseHandle(self.file)?;
        self.file = INVALID_HANDLE_VALUE;
        Ok(())
    }

    pub unsafe fn Size(&self) -> u32 {
        GetFileSize(self.file, None)
    }

    pub unsafe fn Read(&self, buf: &mut [u8], mut size: u32) -> Result<(), Error> {
        ReadFile(self.file, Some(buf), Some(&mut size), None)
    }
}
