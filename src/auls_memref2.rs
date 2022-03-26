#![allow(non_snake_case)]
use crate::auls_exedit::EXEDIT_OBJECT;
use crate::filter::*;
use crate::yulib_file;
use crate::yulib_generic;
use crate::yulib_memory;
use std::ffi::c_void;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::LibraryLoader::*;
use windows_sys::Win32::System::WindowsProgramming::GetPrivateProfileStringA;
use windows_sys::Win32::UI::Shell::*;

type PathType = [u8; MAX_PATH as usize];

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct CMemref {
    pub m_exedit: u64,
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
    pub unsafe fn Init(&mut self, fp: *mut FILTER) -> BOOL {
        self.m_exedit =
            LoadLibraryExA(b"exedit.auf".as_ptr(), 0, LOAD_WITH_ALTERED_SEARCH_PATH) as u64;
        if self.m_exedit == 0 {
            return false as BOOL;
        }
        return self.loadAddress(fp);
    }

    unsafe fn loadAddress(&mut self, fp: *mut FILTER) -> BOOL {
        let mut iniFilePath: PathType = [0; MAX_PATH as usize];
        let iniFilePathPtr = iniFilePath.as_mut_ptr();
        GetModuleFileNameA((*fp).dll_hinst as isize, iniFilePathPtr, iniFilePath.len() as u32);
        PathRemoveFileSpecA(iniFilePathPtr);
        PathAppendA(iniFilePathPtr, b"auls_memref.ini".as_ptr());
        let mut _appName: PathType = [0; MAX_PATH as usize];
        {
            let mut path: PathType = [0; MAX_PATH as usize];
            GetModuleFileNameA(self.m_exedit as isize, path.as_mut_ptr(), path.len() as u32);
            let mut file: yulib_file::CFile = yulib_file::CFile::default();
            file.OpenExisting(path.as_ptr());
            let filesize = file.Size();
            let mut fileData = yulib_memory::CMemory::<u8>::default();
            fileData.Alloc(filesize,true);
            let result = file.Read(fileData.mem as *mut c_void, filesize);
            if result == 0 {
                return false as BOOL;
            }
            let crc32 = yulib_generic::Crc32_2(fileData.mem as *mut c_void, filesize);
            _appName = format!("{:08X}", crc32).as_bytes().try_into().unwrap();
        }
        let appNamePtr = _appName.as_ptr();
        self.m_Exedit_StaticFilterTable = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_StaticFilterTable".as_ptr(),
        );
        self.m_Exedit_StaticFilterTable = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_StaticFilterTable".as_ptr(),
        );
        self.m_Exedit_SortedObjectTable_LayerIndexEnd = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_SortedObjectTable_LayerIndexEnd".as_ptr(),
        );
        self.m_Exedit_AliasNameBuffer = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_AliasNameBuffer".as_ptr(),
        );
        self.m_Exedit_SortedObjectCount = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_SortedObjectCount".as_ptr(),
        );
        self.m_Exedit_ObjDlg_CommandTarget = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_ObjDlg_CommandTarget".as_ptr(),
        );
        self.m_Exedit_SortedObjectTable_LayerIndexBegin = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_SortedObjectTable_LayerIndexBegin".as_ptr(),
        );
        self.m_Exedit_ObjDlg_FilterStatus = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_ObjDlg_FilterStatus".as_ptr(),
        );
        self.m_Exedit_SortedObjectTable = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_SortedObjectTable".as_ptr(),
        );
        self.m_Exedit_ObjDlg_ObjectIndex = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_ObjDlg_ObjectIndex".as_ptr(),
        );
        self.m_Exedit_SceneSetting =
            Self::getHex(iniFilePathPtr, appNamePtr, b"Exedit_SceneSetting".as_ptr());
        self.m_Exedit_LoadedFilterTable = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_LoadedFilterTable".as_ptr(),
        );
        self.m_Exedit_LayerSetting =
            Self::getHex(iniFilePathPtr, appNamePtr, b"Exedit_LayerSetting".as_ptr());
        self.m_Exedit_SceneDisplaying = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_SceneDisplaying".as_ptr(),
        );
        self.m_Exedit_TextBuffer =
            Self::getHex(iniFilePathPtr, appNamePtr, b"Exedit_TextBuffer".as_ptr());
        self.m_Exedit_TraScript_ProcessingTrackBarIndex = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_TraScript_ProcessingTrackBarIndex".as_ptr(),
        );
        self.m_Exedit_TraScript_ProcessingObjectIndex = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_TraScript_ProcessingObjectIndex".as_ptr(),
        );
        self.m_Exedit_ScriptProcessingFilter = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_ScriptProcessingFilter".as_ptr(),
        );
        self.m_Exedit_LuaState =
            Self::getHex(iniFilePathPtr, appNamePtr, b"Exedit_LuaState".as_ptr());
        self.m_Exedit_ObjectBufferInfo = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_ObjectBufferInfo".as_ptr(),
        );
        self.m_Exedit_CameraZBuffer =
            Self::getHex(iniFilePathPtr, appNamePtr, b"Exedit_CameraZBuffer".as_ptr());
        self.m_Exedit_UndoInfo =
            Self::getHex(iniFilePathPtr, appNamePtr, b"Exedit_UndoInfo".as_ptr());

        self.m_Exedit_ObjectBufferInfo_exdata_size =
            Self::getHex(iniFilePathPtr, appNamePtr, b"Exedit_UndoInfo".as_ptr());
        self.m_Exedit_ObjectBufferInfo_max_data_num = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_ObjectBufferInfo_max_data_num".as_ptr(),
        );
        self.m_Exedit_ObjectBufferInfo_data = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_ObjectBufferInfo_data".as_ptr(),
        );
        self.m_Exedit_ObjectBufferInfo_exdata = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_ObjectBufferInfo_exdata".as_ptr(),
        );

        self.m_Exedit_UndoInfo_current_id = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_UndoInfo_current_id".as_ptr(),
        );
        self.m_Exedit_UndoInfo_write_offset = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_UndoInfo_write_offset".as_ptr(),
        );
        self.m_Exedit_UndoInfo_object_num = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_UndoInfo_object_num".as_ptr(),
        );
        self.m_Exedit_UndoInfo_buffer_ptr = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_UndoInfo_buffer_ptr".as_ptr(),
        );
        self.m_Exedit_UndoInfo_buffer_size = Self::getHex(
            iniFilePathPtr,
            appNamePtr,
            b"Exedit_UndoInfo_buffer_size".as_ptr(),
        );
        true as BOOL
    }

    unsafe fn getHex(path: *const u8, appName: *const u8, keyName: *const u8) -> u32 {
        let mut buffer: PathType = [0; MAX_PATH as usize];
        GetPrivateProfileStringA(
            appName,
            keyName,
            b"".as_ptr(),
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            path,
        );
        let value = u32::from_str_radix(std::str::from_utf8(buffer.as_slice()).unwrap(), 16);
        match value {
            Ok(some) => some,
            Err(_) => panic!("ParseIntError"),
        }
    }

    pub unsafe fn Exedit_SceneDisplaying(&self)->i32{
        *((self.m_exedit + self.m_Exedit_SceneDisplaying as u64) as *mut i32)
    }

    pub unsafe 	fn Exedit_SortedObjectCount(&self)->i32
	{
		 *((self.m_exedit + self.m_Exedit_SortedObjectCount as u64) as *mut i32)
	}

    pub unsafe fn Exedit_SortedObjectTable(&self)->*mut *mut EXEDIT_OBJECT{
        (self.m_exedit+self.m_Exedit_SortedObjectTable as u64) as *mut *mut EXEDIT_OBJECT
    }
}
