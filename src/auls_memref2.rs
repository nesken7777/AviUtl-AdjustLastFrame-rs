#![allow(non_snake_case)]
use crate::auls_exedit::EXEDIT_OBJECT;
use crate::filter::*;
use crate::yulib_file;
use crate::yulib_generic;
use crate::yulib_memory;
use crate::Errors;
use std::slice;
use windows::core::s;
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
    pub fn Init(&mut self, fp: &FILTER) -> Result<(), Errors> {
        unsafe {
            let handle =
                LoadLibraryExA(s!("exedit.auf"), HANDLE(0), LOAD_WITH_ALTERED_SEARCH_PATH)?;
            self.m_exedit = handle;
            self.loadAddress(fp)
        }
    }

    fn loadAddress(&mut self, fp: &FILTER) -> Result<(), Errors> {
        unsafe {
            let mut iniFilePath: PathType = [b'\0'; MAX_PATH as usize];
            GetModuleFileNameA((*fp).dll_hinst, iniFilePath.as_mut_slice());
            PathRemoveFileSpecA(PSTR(iniFilePath.as_mut_ptr()));
            PathAppendA(&mut iniFilePath, s!("auls_memref.ini"));
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
                let crc32 =
                    yulib_generic::Crc32(std::slice::from_raw_parts(fileData.mem, filesize));
                let mut temp: [u8; 9] = [0; 9];
                wvnsprintfA(
                    temp.as_mut_slice(),
                    s!("%08X"),
                    (&crc32 as *const u32).cast::<i8>(),
                );
                appName[0..temp.len()]
                    .iter_mut()
                    .zip(temp.iter())
                    .for_each(|(dest, src)| {
                        *dest = *src;
                    });
            }
            self.m_Exedit_StaticFilterTable = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_StaticFilterTable"),
            )?;
            self.m_Exedit_SortedObjectTable_LayerIndexEnd = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_SortedObjectTable_LayerIndexEnd"),
            )?;
            self.m_Exedit_AliasNameBuffer = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_AliasNameBuffer"),
            )?;
            self.m_Exedit_SortedObjectCount = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_SortedObjectCount"),
            )?;
            self.m_Exedit_ObjDlg_CommandTarget = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_ObjDlg_CommandTarget"),
            )?;
            self.m_Exedit_SortedObjectTable_LayerIndexBegin = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_SortedObjectTable_LayerIndexBegin"),
            )?;
            self.m_Exedit_ObjDlg_FilterStatus = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_ObjDlg_FilterStatus"),
            )?;
            self.m_Exedit_SortedObjectTable = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_SortedObjectTable"),
            )?;
            self.m_Exedit_ObjDlg_ObjectIndex = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_ObjDlg_ObjectIndex"),
            )?;
            self.m_Exedit_SceneSetting = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_SceneSetting"),
            )?;
            self.m_Exedit_LoadedFilterTable = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_LoadedFilterTable"),
            )?;
            self.m_Exedit_LayerSetting = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_LayerSetting"),
            )?;
            self.m_Exedit_SceneDisplaying = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_SceneDisplaying"),
            )?;
            self.m_Exedit_TextBuffer = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_TextBuffer"),
            )?;
            self.m_Exedit_TraScript_ProcessingTrackBarIndex = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_TraScript_ProcessingTrackBarIndex"),
            )?;
            self.m_Exedit_TraScript_ProcessingObjectIndex = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_TraScript_ProcessingObjectIndex"),
            )?;
            self.m_Exedit_ScriptProcessingFilter = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_ScriptProcessingFilter"),
            )?;
            self.m_Exedit_LuaState = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_LuaState"),
            )?;
            self.m_Exedit_CameraZBuffer = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_CameraZBuffer"),
            )?;

            self.m_Exedit_ObjectBufferInfo = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_ObjectBufferInfo"),
            )?;
            self.m_Exedit_ObjectBufferInfo_exdata_size = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_ObjectBufferInfo_exdata_size"),
            )?;
            self.m_Exedit_ObjectBufferInfo_max_data_num = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_ObjectBufferInfo_max_data_num"),
            )?;
            self.m_Exedit_ObjectBufferInfo_data = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_ObjectBufferInfo_data"),
            )?;
            self.m_Exedit_ObjectBufferInfo_exdata = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_ObjectBufferInfo_exdata"),
            )?;

            self.m_Exedit_UndoInfo = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_UndoInfo"),
            )?;
            self.m_Exedit_UndoInfo_current_id = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_UndoInfo_current_id"),
            )?;
            self.m_Exedit_UndoInfo_write_offset = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_UndoInfo_write_offset"),
            )?;
            self.m_Exedit_UndoInfo_object_num = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_UndoInfo_object_num"),
            )?;
            self.m_Exedit_UndoInfo_buffer_ptr = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_UndoInfo_buffer_ptr"),
            )?;
            self.m_Exedit_UndoInfo_buffer_size = Self::getHex(
                PCSTR(iniFilePath.as_ptr()),
                PCSTR(appName.as_ptr()),
                s!("Exedit_UndoInfo_buffer_size"),
            )?;
        }
        Ok(())
    }

    fn getHex(path: PCSTR, appName: PCSTR, keyName: PCSTR) -> Result<u32, Errors> {
        unsafe {
            let mut buffer: PathType = [b'\0'; MAX_PATH as usize];

            let l = GetPrivateProfileStringA(
                appName,
                keyName,
                s!(""),
                Some(buffer.as_mut_slice()),
                path,
            );
            u32::from_str_radix(core::str::from_utf8(&buffer[0..l as usize])?, 16)
                .map_err(|e| e.into())
        }
    }

    pub fn Exedit_SceneDisplaying(&self) -> i32 {
        unsafe { *((self.m_exedit.0 + self.m_Exedit_SceneDisplaying as isize) as *mut i32) }
    }

    pub fn Exedit_SortedObjectCount(&self) -> i32 {
        unsafe { *((self.m_exedit.0 + self.m_Exedit_SortedObjectCount as isize) as *mut i32) }
    }

    pub fn Exedit_SortedObjectTable(&self) -> *mut *mut EXEDIT_OBJECT {
        (self.m_exedit.0 + self.m_Exedit_SortedObjectTable as isize) as *mut *mut EXEDIT_OBJECT
    }
}
