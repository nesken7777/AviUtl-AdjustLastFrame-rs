#![allow(non_snake_case, non_upper_case_globals)]

pub mod auls_aviutl;
pub mod auls_exedit;
pub mod auls_memref2;
pub mod filter;
pub mod yulib_file;
pub mod yulib_generic;
pub mod yulib_memory;
use crate::filter::*;
use auls_memref2::CMemref;
use std::cell::RefCell;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::BOOL;
use windows_sys::Win32::UI::WindowsAndMessaging::{PostMessageA, WM_COMMAND};

pub static mut g_memref: CMemref = CMemref {
    m_exedit: 0,
    m_Exedit_StaticFilterTable: 0,
    m_Exedit_SortedObjectTable_LayerIndexEnd: 0,
    m_Exedit_AliasNameBuffer: 0,
    m_Exedit_SortedObjectCount: 0,
    m_Exedit_ObjDlg_CommandTarget: 0,
    m_Exedit_SortedObjectTable_LayerIndexBegin: 0,
    m_Exedit_ObjDlg_FilterStatus: 0,
    m_Exedit_SortedObjectTable: 0,
    m_Exedit_ObjDlg_ObjectIndex: 0,
    m_Exedit_SceneSetting: 0,
    m_Exedit_LoadedFilterTable: 0,
    m_Exedit_LayerSetting: 0,
    m_Exedit_SceneDisplaying: 0,
    m_Exedit_TextBuffer: 0,
    m_Exedit_TraScript_ProcessingTrackBarIndex: 0,
    m_Exedit_TraScript_ProcessingObjectIndex: 0,
    m_Exedit_ScriptProcessingFilter: 0,
    m_Exedit_LuaState: 0,
    m_Exedit_ObjectBufferInfo: 0,
    m_Exedit_CameraZBuffer: 0,
    m_Exedit_UndoInfo: 0,
    m_Exedit_ObjectBufferInfo_exdata_size: 0,
    m_Exedit_ObjectBufferInfo_max_data_num: 0,
    m_Exedit_ObjectBufferInfo_data: 0,
    m_Exedit_ObjectBufferInfo_exdata: 0,
    m_Exedit_UndoInfo_current_id: 0,
    m_Exedit_UndoInfo_write_offset: 0,
    m_Exedit_UndoInfo_object_num: 0,
    m_Exedit_UndoInfo_buffer_ptr: 0,
    m_Exedit_UndoInfo_buffer_size: 0,
};

unsafe fn adjustLastFrame(fp: *mut FILTER, fpip: *mut FILTER_PROC_INFO) -> BOOL {
    let scene = g_memref.Exedit_SceneDisplaying();
    let mut frameEndNumber = -1000;
    {
        let c = g_memref.Exedit_SortedObjectCount();
        let objects: *mut *mut auls_exedit::EXEDIT_OBJECT = g_memref.Exedit_SortedObjectTable();
        for i in 0..c {
            if scene != (*(*objects.add(i as usize))).scene_set {
                continue;
            }
            frameEndNumber =
                yulib_generic::Max(frameEndNumber, (*(*objects.add(i as usize))).frame_end);
        }
    }

    let frameMaxNumber = (*(*fp).exfunc).get_frame_n.unwrap()((*fpip).editp);
    if frameEndNumber <= 0 {
        return false as BOOL;
    }
    if frameEndNumber + 1 >= frameMaxNumber {
        return false as BOOL;
    }
    let exeditWindow = auls_exedit::Exedit_GetWindow(fp);
    if exeditWindow == 0 {
        return false as BOOL;
    }
    PostMessageA(exeditWindow, WM_COMMAND, 1097, 0);
    true as BOOL
}

#[no_mangle]
pub unsafe extern "system" fn GetFilterTable() -> *mut FILTER_DLL {
    static mut g_Filter: RefCell<FILTER_DLL> = RefCell::new(FILTER_DLL {
        flag: FILTER_FLAG_NO_CONFIG
            | FILTER_FLAG_ALWAYS_ACTIVE
            | FILTER_FLAG_DISP_FILTER
            | FILTER_FLAG_EX_INFORMATION,
        x: 0,
        y: 0,
        name: "最終フレーム自動調整".as_ptr(),
        track_n: 0,
        track_name: null_mut(),
        track_default: null_mut(),
        track_s: null_mut(),
        track_e: null_mut(),
        check_n: 0,
        check_name: null_mut(),
        check_default: null_mut(),
        func_proc: Some(func_proc),
        func_init: Some(func_init),
        func_exit: Some(func_exit),
        func_update: None,
        func_WndProc: None,
        track: null_mut(),
        check: null_mut(),
        ex_data_ptr: null_mut(),
        ex_data_size: 0,
        information: "最終フレーム自動調整 version 1.0.3 by 蛇色\r\nmodified nesken7777".as_ptr(),
        func_save_start: None,
        exfunc: null_mut(),
        hwnd: 0,
        dll_hinst: 0,
        ex_data_def: null_mut(),
        func_is_saveframe: None,
        func_modify_title: None,
        func_project_load: None,
        func_project_save: None,
        func_save_end: None,
        dll_path: null_mut(),
        reserve: [0; 2],
    });
    g_Filter.get_mut()
}

//BOOL
#[no_mangle]
pub unsafe extern "system" fn func_init(fp: *mut FILTER) -> BOOL {
    return g_memref.Init(fp); // auls::CMemref の初期化。
}

#[no_mangle]
pub unsafe extern "system" fn func_exit(_fp: *mut FILTER) -> BOOL {
    return false as BOOL;
}

#[no_mangle]
pub unsafe extern "system" fn func_proc(fp: *mut FILTER, fpip: *mut FILTER_PROC_INFO) -> BOOL {
    return adjustLastFrame(fp, fpip);
}
