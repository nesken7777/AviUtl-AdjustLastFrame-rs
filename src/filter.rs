#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use core::ffi::c_void;
use windows::Win32::Foundation::{BOOL, HWND, WPARAM, LPARAM, HINSTANCE};
use windows::Win32::Graphics::Gdi::HFONT;
use windows::core::PSTR;

pub const FILTER_FLAG_NO_CONFIG: i32 = 0x100000;
pub const FILTER_FLAG_ALWAYS_ACTIVE: i32 = 0x4;
pub const FILTER_FLAG_DISP_FILTER: i32 = 0x8000;
pub const FILTER_FLAG_EX_INFORMATION: i32 = 0x40000;

pub type MULTI_THREAD_FUNC =
    Option<fn(thread_id: i32, thread_num: i32, param1: *mut c_void, param2: *mut c_void)>;
pub type AVI_FILE_HANDLE = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SYS_INFO {
    pub flag: i32,
    pub info: PSTR,
    pub filter_n: i32,
    pub min_w: i32,
    pub min_h: i32,
    pub max_w: i32,
    pub max_h: i32,
    pub max_frame: i32,
    pub edit_name: PSTR,
    pub project_name: PSTR,
    pub output_name: PSTR,
    pub vram_w: i32,
    pub vram_h: i32,
    pub vram_yc_size: i32,
    pub vram_line_size: i32,
    pub hfont: HFONT,
    pub build: i32,
    pub reserve: [i32; 2usize],
}
impl Default for SYS_INFO {
    fn default() -> Self {
        SYS_INFO {
            flag: 0,
            info: PSTR::null(),
            filter_n: 0,
            min_w: 0,
            min_h: 0,
            max_w: 0,
            max_h: 0,
            max_frame: 0,
            edit_name: PSTR::null(),
            project_name: PSTR::null(),
            output_name: PSTR::null(),
            vram_w: 0,
            vram_h: 0,
            vram_yc_size: 0,
            vram_line_size: 0,
            hfont: HFONT::default(),
            build: 0,
            reserve: [0; 2],
        }
    }
}

#[repr(C)]
pub struct FILE_INFO {
    pub flag: i32,
    pub name: PSTR,
    pub w: i32,
    pub h: i32,
    pub video_rate: i32,
    pub video_scale: i32,
    pub audio_rate: i32,
    pub audio_ch: i32,
    pub frame_n: i32,
    pub video_decode_format: u32,
    pub video_decode_bit: i32,
    pub audio_n: i32,
    pub reserve: [i32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PIXEL_YC {
    pub y: i16,
    pub cb: i16,
    pub cr: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PIXEL {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

#[repr(C)]
pub struct FILTER_PROC_INFO {
    pub flag: i32,
    pub ycp_edit: *mut PIXEL_YC,
    pub ycp_temp: *mut PIXEL_YC,
    pub w: i32,
    pub h: i32,
    pub max_w: i32,
    pub max_h: i32,
    pub frame: i32,
    pub frame_n: i32,
    pub org_w: i32,
    pub org_h: i32,
    pub audiop: *mut i16,
    pub audio_n: i32,
    pub audio_ch: i32,
    pub pixelp: *mut PIXEL,
    pub editp: *mut c_void,
    pub yc_size: i32,
    pub line_size: i32,
    pub reserve: [i32; 8usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FRAME_STATUS {
    pub video: i32,
    pub audio: i32,
    pub inter: i32,
    pub index24fps: i32,
    pub config: i32,
    pub vcm: i32,
    pub edit_flag: i32,
    pub reserve: [i32; 9usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EXFUNC {
    pub get_ycp_ofs: Option<fn(editp: *mut c_void, n: i32, ofs: i32)>,
    pub get_ycp: Option<fn(editp: *mut c_void, n: i32) -> *mut c_void>,
    pub get_pixelp: Option<fn(editp: *mut c_void, n: i32) -> *mut c_void>,
    pub get_audio: Option<fn(editp: *mut c_void, n: i32, buf: *mut c_void) -> i32>,
    pub is_editing: Option<fn(editp: *mut c_void) -> BOOL>,
    pub is_saving: Option<fn(editp: *mut c_void) -> BOOL>,
    pub get_frame: Option<fn(editp: *mut c_void) -> i32>,
    pub get_frame_n: Option<fn(editp: *mut c_void) -> i32>,
    pub get_frame_size: Option<fn(editp: *mut c_void, w: *mut i32, h: *mut i32) -> BOOL>,
    pub set_frame: Option<fn(editp: *mut c_void, n: i32) -> i32>,
    pub set_frame_n: Option<fn(editp: *mut c_void, n: i32) -> i32>,
    pub copy_frame: Option<fn(editp: *mut c_void, d: i32, s: i32) -> BOOL>,
    pub copy_video: Option<fn(editp: *mut c_void, d: i32, s: i32) -> BOOL>,
    pub copy_audio: Option<fn(editp: *mut c_void, d: i32, s: i32) -> BOOL>,
    pub copy_clip: Option<fn(hwnd: HWND, pixelp: *mut c_void, w: i32, h: i32) -> BOOL>,
    pub paste_clip: Option<fn(hwnd: HWND, editp: *mut c_void, n: i32) -> BOOL>,
    pub get_frame_status: Option<fn(editp: *mut c_void, n: i32, fsp: *mut FRAME_STATUS) -> BOOL>,
    pub set_frame_status: Option<fn(editp: *mut c_void, n: i32, fsp: *mut FRAME_STATUS) -> BOOL>,
    pub is_saveframe: Option<fn(editp: *mut c_void, n: i32) -> BOOL>,
    pub is_keyframe: Option<fn(editp: *mut c_void, n: i32) -> BOOL>,
    pub is_recompress: Option<fn(editp: *mut c_void, n: i32) -> BOOL>,
    pub filter_window_update: Option<fn(fp: *mut c_void) -> BOOL>,
    pub is_filter_window_disp: Option<fn(fp: *mut c_void) -> BOOL>,
    pub get_file_info: Option<fn(editp: *mut c_void, fip: *mut FILE_INFO) -> BOOL>,
    pub get_config_name: Option<fn(editp: *mut c_void, n: i32) -> PSTR>,
    pub is_filter_active: Option<fn(fp: *mut c_void) -> BOOL>,
    pub get_pixel_filtered: Option<
        fn(editp: *mut c_void, n: i32, pixelp: *mut c_void, w: *mut i32, h: *mut i32) -> BOOL,
    >,
    pub get_audio_filtered: Option<fn(editp: *mut c_void, n: i32, buf: *mut c_void) -> i32>,
    pub get_select_frame: Option<fn(editp: *mut c_void, s: *mut i32, e: *mut i32) -> BOOL>,
    pub set_select_frame: Option<fn(editp: *mut c_void, s: i32, e: i32) -> BOOL>,
    pub rgb2yc: Option<fn(ycp: *mut PIXEL_YC, pixelp: *mut PIXEL, w: i32) -> BOOL>,
    pub yc2rgb: Option<fn(pixelp: *mut PIXEL, ycp: *mut PIXEL_YC, w: i32) -> BOOL>,
    pub dlg_get_load_name: Option<fn(name: PSTR, filter: PSTR, def: PSTR) -> BOOL>,
    pub dlg_get_save_name: Option<fn(name: PSTR, filter: PSTR, def: PSTR) -> BOOL>,
    pub ini_load_int: Option<fn(fp: *mut c_void, key: PSTR, n: i32) -> i32>,
    pub ini_save_int: Option<fn(fp: *mut c_void, key: PSTR, n: i32) -> i32>,
    pub ini_load_str:
        Option<fn(fp: *mut c_void, key: PSTR, str_: PSTR, def: PSTR) -> BOOL>,
    pub ini_save_str: Option<fn(fp: *mut c_void, key: PSTR, str_: PSTR) -> BOOL>,
    pub get_source_file_info:
        Option<fn(editp: *mut c_void, fip: *mut FILE_INFO, source_file_id: i32) -> BOOL>,
    pub get_source_video_number: Option<
        fn(
            editp: *mut c_void,
            n: i32,
            source_file_id: *mut i32,
            source_video_number: *mut i32,
        ) -> BOOL,
    >,
    pub get_sys_info: Option<fn(editp: *mut c_void, sip: *mut SYS_INFO) -> BOOL>,
    pub get_filterp: Option<fn(filter_id: i32) -> *mut c_void>,
    pub get_ycp_filtering: Option<
        fn(fp: *mut c_void, editp: *mut c_void, n: i32, reserve: *mut c_void) -> *mut c_void,
    >,
    pub get_audio_filtering:
        Option<fn(fp: *mut c_void, editp: *mut c_void, n: i32, buf: *mut c_void) -> i32>,
    pub set_ycp_filtering_cache_size:
        Option<fn(fp: *mut c_void, w: i32, h: i32, d: i32, flag: i32) -> BOOL>,
    pub get_ycp_filtering_cache:
        Option<fn(fp: *mut c_void, editp: *mut c_void, n: i32) -> *mut c_void>,
    pub get_ycp_source_cache: Option<fn(editp: *mut c_void, n: i32, ofs: i32) -> *mut c_void>,
    pub get_disp_pixelp: Option<fn(editp: *mut c_void, format: u32) -> *mut c_void>,
    pub get_pixel_source:
        Option<fn(editp: *mut c_void, n: i32, pixelp: *mut c_void, format: u32) -> BOOL>,
    pub get_pixel_filtered_ex: Option<
        fn(
            editp: *mut c_void,
            n: i32,
            pixelp: *mut c_void,
            w: *mut i32,
            h: *mut i32,
            format: u32,
        ) -> BOOL,
    >,
    pub get_ycp_filtering_cache_ex: Option<
        fn(fp: *mut c_void, editp: *mut c_void, n: i32, w: *mut i32, h: *mut i32) -> *mut PIXEL_YC,
    >,
    pub exec_multi_thread_func:
        Option<fn(func: MULTI_THREAD_FUNC, param1: *mut c_void, param2: *mut c_void) -> BOOL>,
    pub create_yc: Option<fn() -> *mut PIXEL_YC>,
    pub delete_yc: Option<fn(ycp: *mut PIXEL_YC)>,
    pub load_image:
        Option<fn(ycp: *mut PIXEL_YC, file: PSTR, w: *mut i32, h: *mut i32, flag: i32) -> BOOL>,
    pub resize_yc: Option<
        fn(
            ycp: *mut PIXEL_YC,
            w: i32,
            h: i32,
            ycp_src: *mut PIXEL_YC,
            sx: i32,
            sy: i32,
            sw: i32,
            sh: i32,
        ),
    >,
    pub copy_yc: Option<
        fn(
            ycp: *mut PIXEL_YC,
            x: i32,
            y: i32,
            ycp_src: *mut PIXEL_YC,
            sx: i32,
            sy: i32,
            sw: i32,
            sh: i32,
            tr: i32,
        ),
    >,
    pub draw_text: Option<
        fn(
            ycp: *mut PIXEL_YC,
            x: i32,
            y: i32,
            text: PSTR,
            r: i32,
            g: i32,
            b: i32,
            tr: i32,
            hfont: HFONT,
            w: *mut i32,
            h: *mut i32,
        ),
    >,
    pub avi_file_open: Option<fn(file: PSTR, fip: *mut FILE_INFO, flag: i32) -> AVI_FILE_HANDLE>,
    pub avi_file_close: Option<fn(afh: AVI_FILE_HANDLE)>,
    pub avi_file_read_video: Option<fn(afh: AVI_FILE_HANDLE, ycp: *mut PIXEL_YC, n: i32) -> BOOL>,
    pub avi_file_read_audio: Option<fn(afh: AVI_FILE_HANDLE, buf: *mut c_void, n: i32) -> i32>,
    pub avi_file_get_video_pixelp: Option<fn(afh: AVI_FILE_HANDLE, n: i32) -> *mut c_void>,
    pub get_avi_file_filter: Option<fn(type_: i32) -> PSTR>,
    pub avi_file_read_audio_sample:
        Option<fn(afh: AVI_FILE_HANDLE, start: i32, length: i32, buf: *mut c_void) -> i32>,
    pub avi_file_set_audio_sample_rate:
        Option<fn(afh: AVI_FILE_HANDLE, audio_rate: i32, audio_ch: i32) -> i32>,
    pub get_frame_status_table: Option<fn(editp: *mut c_void, type_: i32) -> *mut u8>,
    pub set_undo: Option<fn(editp: *mut c_void) -> BOOL>,
    pub add_menu_item: Option<
        fn(fp: *mut c_void, name: PSTR, hwnd: HWND, id: i32, def_key: i32, flag: i32) -> BOOL,
    >,
    pub edit_open: Option<fn(editp: *mut c_void, file: PSTR, flag: i32) -> BOOL>,
    pub edit_close: Option<fn(editp: *mut c_void) -> BOOL>,
    pub edit_output:
        Option<fn(editp: *mut c_void, file: PSTR, flag: i32, type_: PSTR) -> BOOL>,
    pub set_config: Option<fn(editp: *mut c_void, n: i32, name: PSTR) -> BOOL>,
    pub reserve: [i32; 7usize],
}

#[repr(C)]
pub struct FILTER {
    pub flag: i32,
    pub x: i32,
    pub y: i32,
    pub name: PSTR,
    pub track_n: i32,
    pub track_name: *mut PSTR,
    pub track_default: *mut i32,
    pub track_s: *mut i32,
    pub track_e: *mut i32,
    pub check_n: i32,
    pub check_name: *mut PSTR,
    pub check_default: *mut i32,
    pub func_proc: Option<fn(fp: *mut c_void, fpip: *mut FILTER_PROC_INFO) -> BOOL>,
    pub func_init: Option<fn(fp: *mut c_void) -> BOOL>,
    pub func_exit: Option<fn(fp: *mut c_void) -> BOOL>,
    pub func_update: Option<fn(fp: *mut c_void, status: i32) -> BOOL>,
    pub func_WndProc: Option<
        fn(
            hwnd: HWND,
            message: u32,
            wparam: WPARAM,
            lparam: LPARAM,
            editp: *mut c_void,
            fp: *mut c_void,
        ) -> BOOL,
    >,
    pub track: *mut i32,
    pub check: *mut i32,
    pub ex_data_ptr: *mut c_void,
    pub ex_data_size: i32,
    pub information: PSTR,
    pub func_save_start: Option<fn(fp: *mut c_void, s: i32, e: i32, editp: *mut c_void) -> BOOL>,
    pub func_save_end: Option<fn(fp: *mut c_void, editp: *mut c_void) -> BOOL>,
    pub exfunc: *mut EXFUNC,
    pub hwnd: HWND,
    pub dll_hinst: HINSTANCE,
    pub ex_data_def: *mut c_void,
    pub func_is_saveframe: Option<
        fn(
            fp: *mut c_void,
            editp: *mut c_void,
            saveno: i32,
            frame: i32,
            fps: i32,
            edit_flag: i32,
            inter: i32,
        ) -> BOOL,
    >,
    pub func_project_load:
        Option<fn(fp: *mut c_void, editp: *mut c_void, data: *mut c_void, size: i32) -> BOOL>,
    pub func_project_save:
        Option<fn(fp: *mut c_void, editp: *mut c_void, data: *mut c_void, size: *mut i32) -> BOOL>,
    pub func_modify_title: Option<
        fn(fp: *mut c_void, editp: *mut c_void, frame: i32, title: PSTR, max_title: i32) -> BOOL,
    >,
    pub dll_path: PSTR,
    pub reserve: [i32; 2usize],
}

#[repr(C)]
pub struct FILTER_DLL {
    pub flag: i32,
    pub x: i32,
    pub y: i32,
    pub name: *const u8,
    pub track_n: i32,
    pub track_name: *mut PSTR,
    pub track_default: *mut i32,
    pub track_s: *mut i32,
    pub track_e: *mut i32,
    pub check_n: i32,
    pub check_name: *mut PSTR,
    pub check_default: *mut i32,
    pub func_proc: Option<fn(fp: *mut FILTER, fpip: *mut FILTER_PROC_INFO) -> BOOL>,
    pub func_init: Option<fn(fp: *mut FILTER) -> BOOL>,
    pub func_exit: Option<fn(fp: *mut FILTER) -> BOOL>,
    pub func_update: Option<fn(fp: *mut FILTER, status: i32) -> BOOL>,
    pub func_WndProc: Option<
        fn(
            hwnd: HWND,
            message: u32,
            wparam: WPARAM,
            lparam: LPARAM,
            editp: *mut c_void,
            fp: *mut FILTER,
        ) -> BOOL,
    >,
    pub track: *mut i32,
    pub check: *mut i32,
    pub ex_data_ptr: *mut c_void,
    pub ex_data_size: i32,
    pub information: *const u8,
    pub func_save_start: Option<fn(fp: *mut FILTER, s: i32, e: i32, editp: *mut c_void) -> BOOL>,
    pub func_save_end: Option<fn(fp: *mut FILTER, editp: *mut c_void) -> BOOL>,
    pub exfunc: *mut EXFUNC,
    pub hwnd: HWND,
    pub dll_hinst: HINSTANCE,
    pub ex_data_def: *mut c_void,
    pub func_is_saveframe: Option<
        fn(
            fp: *mut FILTER,
            editp: *mut c_void,
            saveno: i32,
            frame: i32,
            fps: i32,
            edit_flag: i32,
            inter: i32,
        ) -> BOOL,
    >,
    pub func_project_load:
        Option<fn(fp: *mut FILTER, editp: *mut c_void, data: *mut c_void, size: i32) -> BOOL>,
    pub func_project_save:
        Option<fn(fp: *mut FILTER, editp: *mut c_void, data: *mut c_void, size: *mut i32) -> BOOL>,
    pub func_modify_title: Option<
        fn(fp: *mut FILTER, editp: *mut c_void, frame: i32, title: PSTR, max_title: i32) -> BOOL,
    >,
    pub dll_path: PSTR,
    pub reserve: [i32; 2usize],
}
