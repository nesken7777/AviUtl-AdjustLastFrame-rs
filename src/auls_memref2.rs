#![allow(non_snake_case)]
use crate::auls_exedit::EXEDIT_OBJECT;
use crate::filter::*;
use crate::yulib_file;
use crate::yulib_generic;
use crate::yulib_memory;
use core::ffi::c_void;
use core::slice;
use windows::core::Error;
use windows::core::{PCSTR, PSTR};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::System::LibraryLoader::GetModuleFileNameA;
use windows::Win32::System::LibraryLoader::LoadLibraryExA;
use windows::Win32::System::LibraryLoader::LOAD_WITH_ALTERED_SEARCH_PATH;
use windows::Win32::System::WindowsProgramming::GetPrivateProfileStringA;
use windows::Win32::UI::Shell::wvnsprintfA;
use windows::Win32::UI::Shell::PathAppendA;
use windows::Win32::UI::Shell::PathRemoveFileSpecA;

type PathType = [u8; 260usize];

#[derive(Default, Copy, Clone)]
pub struct CMemref {
    pub m_exedit: HMODULE,
    pub m_Exedit_StaticFilterTable: u32,
    pub m_Exedit_SortedObjectTable_LayerIndexEnd: u32,
    pub m_Exedit_AliasNameBuffer: u32,
    pub m_Exedit_SortedObjectCount: u32,
    pub m_Exedit_ObjDlg_CommandTarget: u32,
    pub m_Exedit_SortedObjectTable_LayerIndexBegin: u32,
    pub m_Exedit_ObjDlg_FilterStatus: u32,
    pub m_Exedit_SortedObjectTable: u32,
    pub m_Exedit_ObjDlg_ObjectIndex: u32,
    pub m_Exedit_SceneSetting: u32,
    pub m_Exedit_LoadedFilterTable: u32,
    pub m_Exedit_LayerSetting: u32,
    pub m_Exedit_SceneDisplaying: u32,
    pub m_Exedit_TextBuffer: u32,
    pub m_Exedit_TraScript_ProcessingTrackBarIndex: u32,
    pub m_Exedit_TraScript_ProcessingObjectIndex: u32,
    pub m_Exedit_ScriptProcessingFilter: u32,
    pub m_Exedit_LuaState: u32,
    pub m_Exedit_ObjectBufferInfo: u32,
    pub m_Exedit_CameraZBuffer: u32,
    pub m_Exedit_UndoInfo: u32,
    pub m_Exedit_ObjectBufferInfo_exdata_size: u32,
    pub m_Exedit_ObjectBufferInfo_max_data_num: u32,
    pub m_Exedit_ObjectBufferInfo_data: u32,
    pub m_Exedit_ObjectBufferInfo_exdata: u32,
    pub m_Exedit_UndoInfo_current_id: u32,
    pub m_Exedit_UndoInfo_write_offset: u32,
    pub m_Exedit_UndoInfo_object_num: u32,
    pub m_Exedit_UndoInfo_buffer_ptr: u32,
    pub m_Exedit_UndoInfo_buffer_size: u32,
}

impl CMemref {
    pub unsafe fn Init(&mut self, fp: *mut FILTER) -> Result<(), Error> {
        let handle = LoadLibraryExA(
            PCSTR(b"exedit.auf\0".as_ptr()),
            HANDLE(0),
            LOAD_WITH_ALTERED_SEARCH_PATH,
        )?;
        self.m_exedit = handle;
        self.loadAddress(fp)
    }

    unsafe fn loadAddress(&mut self, fp: *mut FILTER) -> Result<(), Error> {
        let mut iniFilePath: PathType = [b'\0'; MAX_PATH as usize];
        GetModuleFileNameA((*fp).dll_hinst, iniFilePath.as_mut_slice());
        PathRemoveFileSpecA(PSTR(iniFilePath.as_mut_ptr()));
        PathAppendA(&mut iniFilePath, PCSTR(b"auls_memref.ini\0".as_ptr()));
        let mut appName: PathType = [b'\0'; MAX_PATH as usize];
        {
            let mut path: PathType = [b'\0'; MAX_PATH as usize];
            GetModuleFileNameA(self.m_exedit, path.as_mut_slice());
            let mut file = yulib_file::CFile::default();
            file.OpenExisting(PCSTR(path.as_ptr()))?;
            let filesize = file.Size() as usize;
            let mut fileData = yulib_memory::CMemory::<u8>::default();
            fileData.Alloc(filesize, true)?;
            file.Read(
                slice::from_raw_parts_mut(fileData.mem, fileData.size),
                filesize as u32,
            )?;
            let crc32 = yulib_generic::Crc32_2(fileData.mem as *mut c_void, filesize as u32);
            let crc32ptr = &crc32 as *const u32 as *const i8;
            let mut temp: [u8; 9] = [0; 9];
            wvnsprintfA(temp.as_mut_slice(), PCSTR("%08X".as_ptr()), crc32ptr);
            for i in 0..temp.len() {
                appName[i] = *temp.as_ptr().add(i);
            }
        }
        self.m_Exedit_StaticFilterTable = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_StaticFilterTable\0".as_ptr(),
        );
        self.m_Exedit_SortedObjectTable_LayerIndexEnd = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_SortedObjectTable_LayerIndexEnd\0".as_ptr(),
        );
        self.m_Exedit_AliasNameBuffer = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_AliasNameBuffer\0".as_ptr(),
        );
        self.m_Exedit_SortedObjectCount = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_SortedObjectCount\0".as_ptr(),
        );
        self.m_Exedit_ObjDlg_CommandTarget = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_ObjDlg_CommandTarget\0".as_ptr(),
        );
        self.m_Exedit_SortedObjectTable_LayerIndexBegin = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_SortedObjectTable_LayerIndexBegin\0".as_ptr(),
        );
        self.m_Exedit_ObjDlg_FilterStatus = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_ObjDlg_FilterStatus\0".as_ptr(),
        );
        self.m_Exedit_SortedObjectTable = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_SortedObjectTable\0".as_ptr(),
        );
        self.m_Exedit_ObjDlg_ObjectIndex = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_ObjDlg_ObjectIndex\0".as_ptr(),
        );
        self.m_Exedit_SceneSetting = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_SceneSetting\0".as_ptr(),
        );
        self.m_Exedit_LoadedFilterTable = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_LoadedFilterTable\0".as_ptr(),
        );
        self.m_Exedit_LayerSetting = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_LayerSetting\0".as_ptr(),
        );
        self.m_Exedit_SceneDisplaying = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_SceneDisplaying\0".as_ptr(),
        );
        self.m_Exedit_TextBuffer = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_TextBuffer\0".as_ptr(),
        );
        self.m_Exedit_TraScript_ProcessingTrackBarIndex = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_TraScript_ProcessingTrackBarIndex\0".as_ptr(),
        );
        self.m_Exedit_TraScript_ProcessingObjectIndex = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_TraScript_ProcessingObjectIndex\0".as_ptr(),
        );
        self.m_Exedit_ScriptProcessingFilter = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_ScriptProcessingFilter\0".as_ptr(),
        );
        self.m_Exedit_LuaState = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_LuaState\0".as_ptr(),
        );
        self.m_Exedit_CameraZBuffer = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_CameraZBuffer\0".as_ptr(),
        );

        self.m_Exedit_ObjectBufferInfo = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_ObjectBufferInfo\0".as_ptr(),
        );
        self.m_Exedit_ObjectBufferInfo_exdata_size = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_ObjectBufferInfo_exdata_size\0".as_ptr(),
        );
        self.m_Exedit_ObjectBufferInfo_max_data_num = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_ObjectBufferInfo_max_data_num\0".as_ptr(),
        );
        self.m_Exedit_ObjectBufferInfo_data = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_ObjectBufferInfo_data\0".as_ptr(),
        );
        self.m_Exedit_ObjectBufferInfo_exdata = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_ObjectBufferInfo_exdata\0".as_ptr(),
        );

        self.m_Exedit_UndoInfo = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_UndoInfo\0".as_ptr(),
        );
        self.m_Exedit_UndoInfo_current_id = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_UndoInfo_current_id\0".as_ptr(),
        );
        self.m_Exedit_UndoInfo_write_offset = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_UndoInfo_write_offset\0".as_ptr(),
        );
        self.m_Exedit_UndoInfo_object_num = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_UndoInfo_object_num\0".as_ptr(),
        );
        self.m_Exedit_UndoInfo_buffer_ptr = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_UndoInfo_buffer_ptr\0".as_ptr(),
        );
        self.m_Exedit_UndoInfo_buffer_size = Self::getHex(
            iniFilePath.as_ptr(),
            appName.as_ptr(),
            b"Exedit_UndoInfo_buffer_size\0".as_ptr(),
        );
        Ok(())
    }

    unsafe fn getHex(path: *const u8, appName: *const u8, keyName: *const u8) -> u32 {
        let mut buffer: PathType = [b'\0'; MAX_PATH as usize];

        let l = GetPrivateProfileStringA(
            PCSTR(appName),
            PCSTR(keyName),
            PCSTR(b"\0".as_ptr()),
            Some(buffer.as_mut_slice()),
            PCSTR(path),
        );
        let value = u32::from_str_radix(core::str::from_utf8(&buffer[0..l as usize]).unwrap(), 16);
        match value {
            Ok(some) => some,
            Err(_) => panic!(),
        }
    }

    pub unsafe fn Exedit_SceneDisplaying(&self) -> i32 {
        *((self.m_exedit.0 + self.m_Exedit_SceneDisplaying as isize) as *mut i32)
    }

    pub unsafe fn Exedit_SortedObjectCount(&self) -> i32 {
        *((self.m_exedit.0 + self.m_Exedit_SortedObjectCount as isize) as *mut i32)
    }

    pub unsafe fn Exedit_SortedObjectTable(&self) -> *mut *mut EXEDIT_OBJECT {
        (self.m_exedit.0 + self.m_Exedit_SortedObjectTable as isize) as *mut *mut EXEDIT_OBJECT
    }
}
