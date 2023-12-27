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
use core::ptr::null_mut;
use windows::core::{Error, PSTR};
use windows::Win32::Foundation::{BOOL, HINSTANCE, HMODULE, HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{PostMessageA, WM_COMMAND};

static mut g_memref: CMemref = CMemref {
    m_exedit: HMODULE(0),
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

fn adjustLastFrame(fp: *mut FILTER, fpip: *mut FILTER_PROC_INFO) -> Result<(), Error> {
    unsafe {
        // 現在編集中のシーンのインデックスを取得する。
        let scene = g_memref.Exedit_SceneDisplaying();

        // 現在編集中のシーンの中で最も後ろにあるオブジェクト位置を取得する。
        let mut frameEndNumber = -1000;
        {
            // オブジェクトの個数を取得する。
            let c = g_memref.Exedit_SortedObjectCount();
            // オブジェクトテーブルを取得する。
            let objects = g_memref.Exedit_SortedObjectTable();
            for i in 0..c {
                if scene != (*(*objects.add(i as usize))).scene_set {
                    continue;
                }
                frameEndNumber =
                    yulib_generic::Max(frameEndNumber, (*(*objects.add(i as usize))).frame_end);
            }
        }

        let frameMaxNumber = {
            let a = (*(*fp).exfunc).get_frame_n;
            match a {
                Some(f) => f,
                None => panic!(),
            }
        }((*fpip).editp);
        if frameEndNumber <= 0 {
            return Err(Error::OK);
        }
        if frameEndNumber + 1 >= frameMaxNumber {
            return Err(Error::OK);
        }
        let exeditWindow = auls_exedit::Exedit_GetWindow(fp);
        if exeditWindow == HWND(0) {
            return Err(Error::OK);
        }
        PostMessageA(exeditWindow, WM_COMMAND, WPARAM(1097), LPARAM(0))?;
        Ok(())
    }
}

#[no_mangle]
pub unsafe extern "system" fn GetFilterTable() -> *mut FILTER_DLL {
    static mut g_Filter: FILTER_DLL = FILTER_DLL {
        flag: FILTER_FLAG_NO_CONFIG
            | FILTER_FLAG_ALWAYS_ACTIVE
            | FILTER_FLAG_DISP_FILTER
            | FILTER_FLAG_EX_INFORMATION,
        x: 0,
        y: 0,
        name: b"AdjustLastFrame-rs\0".as_ptr(),
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
        information: b"AdjustLastFrame-rs v1.0.0(rewrited by nesken7777)\0".as_ptr(),
        func_save_start: None,
        exfunc: null_mut(),
        hwnd: HWND(0),
        dll_hinst: HINSTANCE(0),
        ex_data_def: null_mut(),
        func_is_saveframe: None,
        func_modify_title: None,
        func_project_load: None,
        func_project_save: None,
        func_save_end: None,
        dll_path: PSTR(null_mut()),
        reserve: [0; 2],
    };
    &mut g_Filter
}

//BOOL
fn func_init(fp: *mut FILTER) -> BOOL {
    unsafe {
        g_memref
            .Init(fp)
            .map_or_else(|_| false.into(), |_| true.into()) // auls::CMemref の初期化。
    }
}

fn func_exit(_fp: *mut FILTER) -> BOOL {
    false.into()
}

fn func_proc(fp: *mut FILTER, fpip: *mut FILTER_PROC_INFO) -> BOOL {
    adjustLastFrame(fp, fpip).map_or_else(|_| false.into(), |_| true.into())
}
