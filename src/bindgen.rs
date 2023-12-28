use windows::core::PCSTR;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM,HMODULE};
pub mod yulib {
    use windows::Win32::Foundation::HANDLE;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct auto_cast_t<T> {
        pub v: *mut T,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct format {
        pub buf: [i8; 256usize],
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct CFile {
        pub file: HANDLE,
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct CMemory<T> {
        pub mem: *mut T,
        pub size: u32,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct CVector<T> {
        pub mem: *mut T,
        pub size: usize,
        pub capacity: usize,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct CTextBuffer {
        pub mem: *mut i8,
        pub ptr: *mut i8,
        pub end: *mut i8,
        pub size: u32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ColorYCbCr {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ColorRGB {
        pub __bindgen_anon_1: ColorRGB__bindgen_ty_1,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ColorRGB__bindgen_ty_1 {
        pub b: u8,
        pub g: u8,
        pub r: u8,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ColorRGBA {
        pub __bindgen_anon_1: ColorRGBA__bindgen_ty_1,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union ColorRGBA__bindgen_ty_1 {
        pub val: u32,
        pub __bindgen_anon_1: ColorRGBA__bindgen_ty_1__bindgen_ty_1,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ColorRGBA__bindgen_ty_1__bindgen_ty_1 {
        pub b: u8,
        pub g: u8,
        pub r: u8,
        pub a: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ColorHSV {
        pub h: u16,
        pub s: u8,
        pub v: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ColorYCbCrA {
        pub y: i16,
        pub cb: i16,
        pub cr: i16,
        pub a: i16,
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PIXEL_YC {
    pub y: i16,
    pub cb: i16,
    pub cr: i16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PIXEL {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
    pub editp: *mut ::std::os::raw::c_void,
    pub yc_size: i32,
    pub line_size: i32,
    pub reserve: [i32; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub struct FILE_INFO {
    pub flag: i32,
    pub name: *mut i8,
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
#[derive(Debug, Copy, Clone)]
pub struct SYS_INFO {
    pub flag: i32,
    pub info: *mut i8,
    pub filter_n: i32,
    pub min_w: i32,
    pub min_h: i32,
    pub max_w: i32,
    pub max_h: i32,
    pub max_frame: i32,
    pub edit_name: *mut i8,
    pub project_name: *mut i8,
    pub output_name: *mut i8,
    pub vram_w: i32,
    pub vram_h: i32,
    pub vram_yc_size: i32,
    pub vram_line_size: i32,
    pub hfont: isize,
    pub build: i32,
    pub reserve: [i32; 2usize],
}
pub type MULTI_THREAD_FUNC = ::std::option::Option<
    unsafe extern "C" fn(
        thread_id: i32,
        thread_num: i32,
        param1: *mut ::std::os::raw::c_void,
        param2: *mut ::std::os::raw::c_void,
    ),
>;
pub type AVI_FILE_HANDLE = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EXFUNC {
    pub get_ycp_ofs: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, n: i32, ofs: i32),
    >,
    pub get_ycp: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: i32,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_pixelp: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: i32,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_audio: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: i32,
            buf: *mut ::std::os::raw::c_void,
        ) -> i32,
    >,
    pub is_editing:
        ::std::option::Option<unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub is_saving:
        ::std::option::Option<unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub get_frame:
        ::std::option::Option<unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void) -> i32>,
    pub get_frame_n:
        ::std::option::Option<unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void) -> i32>,
    pub get_frame_size: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, w: *mut i32, h: *mut i32) -> BOOL,
    >,
    pub set_frame: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, n: i32) -> i32,
    >,
    pub set_frame_n: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, n: i32) -> i32,
    >,
    pub copy_frame: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, d: i32, s: i32) -> BOOL,
    >,
    pub copy_video: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, d: i32, s: i32) -> BOOL,
    >,
    pub copy_audio: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, d: i32, s: i32) -> BOOL,
    >,
    pub copy_clip: ::std::option::Option<
        unsafe extern "C" fn(
            hwnd: HWND,
            pixelp: *mut ::std::os::raw::c_void,
            w: i32,
            h: i32,
        ) -> BOOL,
    >,
    pub paste_clip: ::std::option::Option<
        unsafe extern "C" fn(hwnd: HWND, editp: *mut ::std::os::raw::c_void, n: i32) -> BOOL,
    >,
    pub get_frame_status: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: i32,
            fsp: *mut FRAME_STATUS,
        ) -> BOOL,
    >,
    pub set_frame_status: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: i32,
            fsp: *mut FRAME_STATUS,
        ) -> BOOL,
    >,
    pub is_saveframe: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, n: i32) -> BOOL,
    >,
    pub is_keyframe: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, n: i32) -> BOOL,
    >,
    pub is_recompress: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, n: i32) -> BOOL,
    >,
    pub filter_window_update:
        ::std::option::Option<unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub is_filter_window_disp:
        ::std::option::Option<unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub get_file_info: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, fip: *mut FILE_INFO) -> BOOL,
    >,
    pub get_config_name: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, n: i32) -> *mut i8,
    >,
    pub is_filter_active:
        ::std::option::Option<unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub get_pixel_filtered: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: i32,
            pixelp: *mut ::std::os::raw::c_void,
            w: *mut i32,
            h: *mut i32,
        ) -> BOOL,
    >,
    pub get_audio_filtered: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: i32,
            buf: *mut ::std::os::raw::c_void,
        ) -> i32,
    >,
    pub get_select_frame: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, s: *mut i32, e: *mut i32) -> BOOL,
    >,
    pub set_select_frame: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, s: i32, e: i32) -> BOOL,
    >,
    pub rgb2yc: ::std::option::Option<
        unsafe extern "C" fn(ycp: *mut PIXEL_YC, pixelp: *mut PIXEL, w: i32) -> BOOL,
    >,
    pub yc2rgb: ::std::option::Option<
        unsafe extern "C" fn(pixelp: *mut PIXEL, ycp: *mut PIXEL_YC, w: i32) -> BOOL,
    >,
    pub dlg_get_load_name: ::std::option::Option<
        unsafe extern "C" fn(name: *mut i8, filter: *mut i8, def: *mut i8) -> BOOL,
    >,
    pub dlg_get_save_name: ::std::option::Option<
        unsafe extern "C" fn(name: *mut i8, filter: *mut i8, def: *mut i8) -> BOOL,
    >,
    pub ini_load_int: ::std::option::Option<
        unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void, key: *mut i8, n: i32) -> i32,
    >,
    pub ini_save_int: ::std::option::Option<
        unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void, key: *mut i8, n: i32) -> i32,
    >,
    pub ini_load_str: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            key: *mut i8,
            str_: *mut i8,
            def: *mut i8,
        ) -> BOOL,
    >,
    pub ini_save_str: ::std::option::Option<
        unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void, key: *mut i8, str_: *mut i8) -> BOOL,
    >,
    pub get_source_file_info: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            fip: *mut FILE_INFO,
            source_file_id: i32,
        ) -> BOOL,
    >,
    pub get_source_video_number: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: i32,
            source_file_id: *mut i32,
            source_video_number: *mut i32,
        ) -> BOOL,
    >,
    pub get_sys_info: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, sip: *mut SYS_INFO) -> BOOL,
    >,
    pub get_filterp:
        ::std::option::Option<unsafe extern "C" fn(filter_id: i32) -> *mut ::std::os::raw::c_void>,
    pub get_ycp_filtering: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            n: i32,
            reserve: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_audio_filtering: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            n: i32,
            buf: *mut ::std::os::raw::c_void,
        ) -> i32,
    >,
    pub set_ycp_filtering_cache_size: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            w: i32,
            h: i32,
            d: i32,
            flag: i32,
        ) -> BOOL,
    >,
    pub get_ycp_filtering_cache: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            n: i32,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_ycp_source_cache: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: i32,
            ofs: i32,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_disp_pixelp: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            format: u32,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_pixel_source: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: i32,
            pixelp: *mut ::std::os::raw::c_void,
            format: u32,
        ) -> BOOL,
    >,
    pub get_pixel_filtered_ex: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: i32,
            pixelp: *mut ::std::os::raw::c_void,
            w: *mut i32,
            h: *mut i32,
            format: u32,
        ) -> BOOL,
    >,
    pub get_ycp_filtering_cache_ex: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            n: i32,
            w: *mut i32,
            h: *mut i32,
        ) -> *mut PIXEL_YC,
    >,
    pub exec_multi_thread_func: ::std::option::Option<
        unsafe extern "C" fn(
            func: MULTI_THREAD_FUNC,
            param1: *mut ::std::os::raw::c_void,
            param2: *mut ::std::os::raw::c_void,
        ) -> BOOL,
    >,
    pub create_yc: ::std::option::Option<unsafe extern "C" fn() -> *mut PIXEL_YC>,
    pub delete_yc: ::std::option::Option<unsafe extern "C" fn(ycp: *mut PIXEL_YC)>,
    pub load_image: ::std::option::Option<
        unsafe extern "C" fn(
            ycp: *mut PIXEL_YC,
            file: *mut i8,
            w: *mut i32,
            h: *mut i32,
            flag: i32,
        ) -> BOOL,
    >,
    pub resize_yc: ::std::option::Option<
        unsafe extern "C" fn(
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
    pub copy_yc: ::std::option::Option<
        unsafe extern "C" fn(
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
    pub draw_text: ::std::option::Option<
        unsafe extern "C" fn(
            ycp: *mut PIXEL_YC,
            x: i32,
            y: i32,
            text: *mut i8,
            r: i32,
            g: i32,
            b: i32,
            tr: i32,
            hfont: isize,
            w: *mut i32,
            h: *mut i32,
        ),
    >,
    pub avi_file_open: ::std::option::Option<
        unsafe extern "C" fn(file: *mut i8, fip: *mut FILE_INFO, flag: i32) -> AVI_FILE_HANDLE,
    >,
    pub avi_file_close: ::std::option::Option<unsafe extern "C" fn(afh: AVI_FILE_HANDLE)>,
    pub avi_file_read_video: ::std::option::Option<
        unsafe extern "C" fn(afh: AVI_FILE_HANDLE, ycp: *mut PIXEL_YC, n: i32) -> BOOL,
    >,
    pub avi_file_read_audio: ::std::option::Option<
        unsafe extern "C" fn(afh: AVI_FILE_HANDLE, buf: *mut ::std::os::raw::c_void, n: i32) -> i32,
    >,
    pub avi_file_get_video_pixelp: ::std::option::Option<
        unsafe extern "C" fn(afh: AVI_FILE_HANDLE, n: i32) -> *mut ::std::os::raw::c_void,
    >,
    pub get_avi_file_filter: ::std::option::Option<unsafe extern "C" fn(type_: i32) -> *mut i8>,
    pub avi_file_read_audio_sample: ::std::option::Option<
        unsafe extern "C" fn(
            afh: AVI_FILE_HANDLE,
            start: i32,
            length: i32,
            buf: *mut ::std::os::raw::c_void,
        ) -> i32,
    >,
    pub avi_file_set_audio_sample_rate: ::std::option::Option<
        unsafe extern "C" fn(afh: AVI_FILE_HANDLE, audio_rate: i32, audio_ch: i32) -> i32,
    >,
    pub get_frame_status_table: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, type_: i32) -> *mut u8,
    >,
    pub set_undo:
        ::std::option::Option<unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub add_menu_item: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            name: *mut i8,
            hwnd: HWND,
            id: i32,
            def_key: i32,
            flag: i32,
        ) -> BOOL,
    >,
    pub edit_open: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, file: *mut i8, flag: i32) -> BOOL,
    >,
    pub edit_close:
        ::std::option::Option<unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub edit_output: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            file: *mut i8,
            flag: i32,
            type_: *mut i8,
        ) -> BOOL,
    >,
    pub set_config: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, n: i32, name: *mut i8) -> BOOL,
    >,
    pub reserve: [i32; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FILTER {
    pub flag: i32,
    pub x: i32,
    pub y: i32,
    pub name: PCSTR,
    pub track_n: i32,
    pub track_name: *mut PCSTR,
    pub track_default: *mut i32,
    pub track_s: *mut i32,
    pub track_e: *mut i32,
    pub check_n: i32,
    pub check_name: *mut PCSTR,
    pub check_default: *mut i32,
    pub func_proc: ::std::option::Option<
        unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void, fpip: *mut FILTER_PROC_INFO) -> BOOL,
    >,
    pub func_init:
        ::std::option::Option<unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub func_exit:
        ::std::option::Option<unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub func_update: ::std::option::Option<
        unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void, status: i32) -> BOOL,
    >,
    pub func_WndProc: ::std::option::Option<
        unsafe extern "C" fn(
            hwnd: HWND,
            message: u32,
            wparam: WPARAM,
            lparam: LPARAM,
            editp: *mut ::std::os::raw::c_void,
            fp: *mut ::std::os::raw::c_void,
        ) -> BOOL,
    >,
    pub track: *mut i32,
    pub check: *mut i32,
    pub ex_data_ptr: *mut ::std::os::raw::c_void,
    pub ex_data_size: i32,
    pub information: PCSTR,
    pub func_save_start: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            s: i32,
            e: i32,
            editp: *mut ::std::os::raw::c_void,
        ) -> BOOL,
    >,
    pub func_save_end: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
        ) -> BOOL,
    >,
    pub exfunc: *mut EXFUNC,
    pub hwnd: HWND,
    pub dll_hinst: HMODULE,
    pub ex_data_def: *mut ::std::os::raw::c_void,
    pub func_is_saveframe: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            saveno: i32,
            frame: i32,
            fps: i32,
            edit_flag: i32,
            inter: i32,
        ) -> BOOL,
    >,
    pub func_project_load: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            data: *mut ::std::os::raw::c_void,
            size: i32,
        ) -> BOOL,
    >,
    pub func_project_save: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            data: *mut ::std::os::raw::c_void,
            size: *mut i32,
        ) -> BOOL,
    >,
    pub func_modify_title: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            frame: i32,
            title: *mut i8,
            max_title: i32,
        ) -> BOOL,
    >,
    pub dll_path: PCSTR,
    pub reserve: [i32; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FILTER_DLL {
    pub flag: i32,
    pub x: i32,
    pub y: i32,
    pub name: PCSTR,
    pub track_n: i32,
    pub track_name: *mut PCSTR,
    pub track_default: *mut i32,
    pub track_s: *mut i32,
    pub track_e: *mut i32,
    pub check_n: i32,
    pub check_name: *mut PCSTR,
    pub check_default: *mut i32,
    pub func_proc: ::std::option::Option<
        unsafe extern "C" fn(fp: *mut FILTER, fpip: *mut FILTER_PROC_INFO) -> BOOL,
    >,
    pub func_init: ::std::option::Option<unsafe extern "C" fn(fp: *mut FILTER) -> BOOL>,
    pub func_exit: ::std::option::Option<unsafe extern "C" fn(fp: *mut FILTER) -> BOOL>,
    pub func_update:
        ::std::option::Option<unsafe extern "C" fn(fp: *mut FILTER, status: i32) -> BOOL>,
    pub func_WndProc: ::std::option::Option<
        unsafe extern "C" fn(
            hwnd: HWND,
            message: u32,
            wparam: WPARAM,
            lparam: LPARAM,
            editp: *mut ::std::os::raw::c_void,
            fp: *mut FILTER,
        ) -> BOOL,
    >,
    pub track: *mut i32,
    pub check: *mut i32,
    pub ex_data_ptr: *mut ::std::os::raw::c_void,
    pub ex_data_size: i32,
    pub information: PCSTR,
    pub func_save_start: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut FILTER,
            s: i32,
            e: i32,
            editp: *mut ::std::os::raw::c_void,
        ) -> BOOL,
    >,
    pub func_save_end: ::std::option::Option<
        unsafe extern "C" fn(fp: *mut FILTER, editp: *mut ::std::os::raw::c_void) -> BOOL,
    >,
    pub exfunc: *mut EXFUNC,
    pub hwnd: HWND,
    pub dll_hinst: HMODULE,
    pub ex_data_def: *mut ::std::os::raw::c_void,
    pub func_is_saveframe: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut FILTER,
            editp: *mut ::std::os::raw::c_void,
            saveno: i32,
            frame: i32,
            fps: i32,
            edit_flag: i32,
            inter: i32,
        ) -> BOOL,
    >,
    pub func_project_load: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut FILTER,
            editp: *mut ::std::os::raw::c_void,
            data: *mut ::std::os::raw::c_void,
            size: i32,
        ) -> BOOL,
    >,
    pub func_project_save: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut FILTER,
            editp: *mut ::std::os::raw::c_void,
            data: *mut ::std::os::raw::c_void,
            size: *mut i32,
        ) -> BOOL,
    >,
    pub func_modify_title: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut FILTER,
            editp: *mut ::std::os::raw::c_void,
            frame: i32,
            title: *mut i8,
            max_title: i32,
        ) -> BOOL,
    >,
    pub dll_path: PCSTR,
    pub reserve: [i32; 2usize],
}
extern "C" {
    #[link_name = "\u{1}?func_proc@@YAHPEAUFILTER@@PEAUFILTER_PROC_INFO@@@Z"]
    pub fn func_proc(fp: *mut FILTER, fpip: *mut FILTER_PROC_INFO) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?func_init@@YAHPEAUFILTER@@@Z"]
    pub fn func_init(fp: *mut FILTER) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?func_exit@@YAHPEAUFILTER@@@Z"]
    pub fn func_exit(fp: *mut FILTER) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?func_update@@YAHPEAUFILTER@@H@Z"]
    pub fn func_update(fp: *mut FILTER, status: i32) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?func_WndProc@@YAHPEAUHWND__@@I_K_JPEAXPEAUFILTER@@@Z"]
    pub fn func_WndProc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        editp: *mut ::std::os::raw::c_void,
        fp: *mut FILTER,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?func_save_start@@YAHPEAUFILTER@@HHPEAX@Z"]
    pub fn func_save_start(
        fp: *mut FILTER,
        s: i32,
        e: i32,
        editp: *mut ::std::os::raw::c_void,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?func_save_end@@YAHPEAUFILTER@@PEAX@Z"]
    pub fn func_save_end(fp: *mut FILTER, editp: *mut ::std::os::raw::c_void) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?func_is_saveframe@@YAHPEAUFILTER@@PEAXHHHHH@Z"]
    pub fn func_is_saveframe(
        fp: *mut FILTER,
        editp: *mut ::std::os::raw::c_void,
        saveno: i32,
        frame: i32,
        fps: i32,
        edit_flag: i32,
        inter: i32,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?func_project_load@@YAHPEAUFILTER@@PEAX1H@Z"]
    pub fn func_project_load(
        fp: *mut FILTER,
        editp: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void,
        size: i32,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?func_project_save@@YAHPEAUFILTER@@PEAX1PEAH@Z"]
    pub fn func_project_save(
        fp: *mut FILTER,
        editp: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void,
        size: *mut i32,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?func_modify_title@@YAHPEAUFILTER@@PEAXHPEADH@Z"]
    pub fn func_modify_title(
        fp: *mut FILTER,
        editp: *mut ::std::os::raw::c_void,
        frame: i32,
        title: *mut i8,
        max_title: i32,
    ) -> BOOL;
}
pub mod auls {
    use super::{FILTER, EXFUNC, yulib};
    use windows::Win32::Foundation::BOOL;
    extern "C" {
        #[link_name = "\u{1}?AVIUTL_EXFUNC_NAME@auls@@3QBQEBDB"]
        pub static AVIUTL_EXFUNC_NAME: [*const i8; 80usize];
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct AVIUTL_SYSTEM_SETTING {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct EDIT_HANDLE {
        pub flag: u32,
        pub edit_filename: [i8; 260usize],
        pub output_filename: [i8; 260usize],
        pub project_filename: [i8; 260usize],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct CFilterWrapper {
        pub fp: *mut CFilterWrapper_wrap,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct CFilterWrapper_wrap {
        pub _base: FILTER,
    }
    pub const EXEDIT_NAME: &[u8; 17usize] =
        b"\xEF\xBF\xBDg\xEF\xBF\xBD\xEF\xBF\xBD\xEF\xBF\xBD\xD2\x8FW\0";
    pub const OBJDLG_CLASSNAME: &[u8; 20usize] = b"ExtendedFilterClass\0";
    pub const EXEDIT_FILTER_ID_BEGIN: i32 = 2000;
    pub const EXEDIT_ALIAS_ID_BEGIN: i32 = 3000;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct OBJDLG_COMMAND {
        pub _address: u8,
    }
    pub const OBJDLG_COMMAND_FILTER_SET_DEFVALUE: u32 = 1108;
    pub const OBJDLG_COMMAND_FILTER_MAKE_ALIAS: u32 = 1109;
    pub const OBJDLG_COMMAND_FILTER_MOVEUP: u32 = 1116;
    pub const OBJDLG_COMMAND_FILTER_MOVEDOWN: u32 = 1117;
    pub const OBJDLG_COMMAND_FILTER_DELETE: u32 = 4300;
    pub const OBJDLG_COMMAND_FILTER_INIT: u32 = 1105;
    pub const OBJDLG_COMMAND_FILTER_VALIDATE: u32 = 4400;
    pub const OBJDLG_COMMAND_FILTER_FOLD: u32 = 4500;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct EXEDIT_OBJECT {
        pub exists: u8,
        pub flag: u8,
        pub type_: u8,
        pub padding_: u8,
        pub layer_disp: i32,
        pub frame_begin: i32,
        pub frame_end: i32,
        pub dispname: [i8; 64usize],
        pub index_midpt_leader: i32,
        pub filter_param: [EXEDIT_OBJECT_FILTER_PARAM; 12usize],
        pub filter_status: [u8; 12usize],
        pub track_sum: i16,
        pub check_sum: i16,
        pub exdata_sum: u32,
        pub track_value_left: [i32; 64usize],
        pub track_value_right: [i32; 64usize],
        pub track_mode: [EXEDIT_OBJECT_TRACK_MODE; 64usize],
        pub check_value: [i32; 48usize],
        pub exdata_offset: u32,
        pub group_belong: i32,
        pub track_param: [i32; 64usize],
        pub layer_set: i32,
        pub scene_set: i32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct EXEDIT_OBJECT_FILTER_PARAM {
        pub id: i32,
        pub track_begin: i16,
        pub check_begin: i16,
        pub exdata_offset: u32,
    }
    pub const EXEDIT_OBJECT_FILTER_PARAM_INVALID_ID: i32 = -1;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct EXEDIT_OBJECT_TRACK_MODE {
        pub num: i16,
        pub script_num: i16,
    }
    pub const EXEDIT_OBJECT_TRACK_MODE_NUM_USE_SCRIPT: i16 = 15;
    pub const EXEDIT_OBJECT_TRACK_MODE_NUM_DECELERATE: i16 = 32;
    pub const EXEDIT_OBJECT_TRACK_MODE_NUM_ACCELERATE: i16 = 64;
    pub const EXEDIT_OBJECT_FLAG_CLIPPING: u8 = 1;
    pub const EXEDIT_OBJECT_FLAG_CAMERA: u8 = 2;
    pub const EXEDIT_OBJECT_TYPE_OVERLAY: u8 = 1;
    pub const EXEDIT_OBJECT_TYPE_SOUND: u8 = 2;
    pub const EXEDIT_OBJECT_TYPE_EFFECT: u8 = 4;
    pub const EXEDIT_OBJECT_TYPE_LIGHTBLUE: u8 = 8;
    pub const EXEDIT_OBJECT_TYPE_CONTROL: u8 = 16;
    pub const EXEDIT_OBJECT_MAX_DISPNAME: usize = 64;
    pub const EXEDIT_OBJECT_MAX_FILTER: i32 = 12;
    pub const EXEDIT_OBJECT_MAX_TRACK: i32 = 64;
    pub const EXEDIT_OBJECT_MAX_CHECK: i32 = 48;
    pub const EXEDIT_OBJECT_FILTER_STATUS_VALID: u8 = 1;
    pub const EXEDIT_OBJECT_FILTER_STATUS_FOLD: u8 = 2;
    pub const EXEDIT_OBJECT_FILTER_STATUS_GUI_VALID: u8 = 4;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct EXEDIT_FILTER {
        pub flag: u32,
        pub x: i32,
        pub y: i32,
        pub name: *const i8,
        pub track_num: i32,
        pub track_name: *mut *const i8,
        pub track_def: *mut i32,
        pub track_min: *mut i32,
        pub track_max: *mut i32,
        pub check_num: i32,
        pub check_name: *mut *const i8,
        pub check_def: *mut i32,
        pub func_proc: *mut ::std::os::raw::c_void,
        pub func_init: *mut ::std::os::raw::c_void,
        pub func_exit: *mut ::std::os::raw::c_void,
        pub func_update: *mut ::std::os::raw::c_void,
        pub func_WndProc: *mut ::std::os::raw::c_void,
        pub track_value: *mut ::std::os::raw::c_void,
        pub check_value: *mut i32,
        pub exdata_ptr: *mut ::std::os::raw::c_void,
        pub exdata_size: u32,
        pub information: *mut ::std::os::raw::c_void,
        pub func_save_start: *mut ::std::os::raw::c_void,
        pub func_save_end: *mut ::std::os::raw::c_void,
        pub aviutl_exfunc: *mut EXFUNC,
        pub exedit_exfunc: *mut ::std::os::raw::c_void,
        pub dll_inst: *mut ::std::os::raw::c_void,
        pub exdata_def: *mut ::std::os::raw::c_void,
        pub exdata_use: *mut EXEDIT_FILTER_EXDATA_USE,
        pub track_extra: *mut EXEDIT_FILTER_TRACK_EXTRA,
        pub track_gui: *mut EXEDIT_FILTER_TRACK_GUI,
        pub unknown: [i32; 20usize],
        pub track_scale: *mut i32,
        pub track_link: *mut ::std::os::raw::c_void,
        pub track_drag_min: *mut i32,
        pub track_drag_max: *mut i32,
        pub exedit_filter: *mut FILTER,
        pub object_data: *mut EXEDIT_OBJECT,
        pub object_index_processing: i16,
        pub filter_pos_processing: i16,
        pub object_index_objdlg: i16,
        pub filter_pos_objdlg: i16,
        pub frame_start: i32,
        pub frame_end: i32,
        pub track_value_left: *mut i32,
        pub track_value_right: *mut i32,
        pub track_mode: *mut i32,
        pub check_value_: *mut i32,
        pub exdata_: *mut ::std::os::raw::c_void,
        pub track_param: *mut i32,
        pub offset_10C: *mut ::std::os::raw::c_void,
        pub offset_110: *mut ::std::os::raw::c_void,
        pub offset_114: *mut ::std::os::raw::c_void,
        pub frame_start_chain: i32,
        pub frame_end_chain: i32,
        pub layer_set: i32,
        pub scene_set: i32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct EXEDIT_FILTER_EXDATA_USE {
        pub type_: u16,
        pub size: u16,
        pub name: *const i8,
    }
    pub const EXEDIT_FILTER_EXDATA_USE_TYPE_UNKNOWN: u16 = 0;
    pub const EXEDIT_FILTER_EXDATA_USE_TYPE_NUMBER: u16 = 1;
    pub const EXEDIT_FILTER_EXDATA_USE_TYPE_STRING: u16 = 2;
    pub const EXEDIT_FILTER_EXDATA_USE_TYPE_BYTEARRAY: u16 = 3;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct EXEDIT_FILTER_TRACK_EXTRA {
        pub track_scale: *mut i32,
        pub track_link: *mut i32,
        pub track_drag_min: *mut i32,
        pub track_drag_max: *mut i32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct EXEDIT_FILTER_TRACK_GUI {
        pub bx: i32,
        pub by: i32,
        pub bz: i32,
        pub rx: i32,
        pub ry: i32,
        pub rz: i32,
        pub cx: i32,
        pub cy: i32,
        pub cz: i32,
        pub zoom: i32,
        pub aspect: i32,
        pub alpha: i32,
    }
    pub const EXEDIT_FILTER_TRACK_GUI_INVALID: i32 = -1;
    pub const EXEDIT_FILTER_FLAG_INPUT_FILTER: u32 = 8;
    pub const EXEDIT_FILTER_FLAG_OUTPUT_FILTER: u32 = 16;
    pub const EXEDIT_FILTER_FLAG_EFFECT: u32 = 32;
    pub const EXEDIT_FILTER_FLAG_DISABLE_PUT: u32 = 128;
    pub const EXEDIT_FILTER_FLAG_DISABLE_ADDING: u32 = 256;
    pub const EXEDIT_FILTER_FLAG_UNKNOWN1: u32 = 512;
    pub const EXEDIT_FILTER_FLAG_BASIC_EFFECT: u32 = 32768;
    pub const EXEDIT_FILTER_FLAG_SOUND_FILTER: u32 = 2097152;
    pub const EXEDIT_FILTER_FLAG_CONTROL_FILTER: u32 = 16777216;
    pub const EXEDIT_FILTER_FLAG_UNKNOWN_RUNTIME: u32 = 67108864;
    pub const EXEDIT_FILTER_FLAG_UNKNOWN: u32 = 1073741824;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct OBJECT_BUFFER_INFO {
        pub exdata_size: u32,
        pub max_data_num: u32,
        pub data: *mut EXEDIT_OBJECT,
        pub exdata: *mut ::std::os::raw::c_void,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct LAYER_SETTING {
        pub flag: u32,
        pub name: *const i8,
    }
    pub const LAYER_SETTING_FLAG_UNDISP: u32 = 1;
    pub const LAYER_SETTING_FLAG_LOCKED: u32 = 2;
    pub const LAYER_SETTING_FLAG_COORDLINK: u32 = 16;
    pub const LAYER_SETTING_FLAG_CLIP: u32 = 32;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct SCENE_SETTING {
        pub flag: u32,
        pub name: *const i8,
        pub width: i32,
        pub height: i32,
        pub max_frame: i32,
        pub current_frame: i32,
        pub timeline_scale: i32,
        pub timeline_disp_begin_pos: i32,
        pub selected_object: i32,
        pub selected_frame_begin: i32,
        pub selected_frame_end: i32,
        pub disp_bpm_grid: BOOL,
        pub bpm_grid_tempo: i32,
        pub bpm_grid_base: i32,
        pub disp_xy_grid: BOOL,
        pub xy_grid_width: i32,
        pub xy_grid_height: i32,
        pub disp_camera_grid: BOOL,
        pub camera_grid_size: i32,
        pub camera_grid_num: i32,
        pub disp_out_of_frame: BOOL,
        pub out_of_frame_scale: i32,
        pub bpm_grid_beat: i32,
        pub disp_begin_layer: i32,
    }
    pub const SCENE_SETTING_FLAG_DISPED: u32 = 1;
    pub const SCENE_SETTING_FLAG_ALPHA: u32 = 2;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct CAMERA_ZBUFFER {
        pub distance: u32,
        pub rotate: u32,
        pub distance_overlapped: u32,
        pub color: yulib::ColorYCbCrA,
    }
    pub const CAMERA_ZBUFFER_DISTANCE_BASE: u32 = 2000000000;
    pub const CAMERA_ZBUFFER_ROTATE_MAX: u32 = 16384;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct UNDO_INFO {
        pub object_num: i32,
        pub buffer_ptr: *mut ::std::os::raw::c_void,
        pub write_offset: u32,
        pub current_id: i32,
        pub buffer_size: u32,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct UNDO_DATA_HEADER {
        pub data_id: i32,
        pub object_id: i32,
        pub data_size: u32,
        pub data: *mut ::std::os::raw::c_void,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct CACHE_BUFFER {
        pub width: i32,
        pub height: i32,
        pub flag: u32,
        pub data: *mut yulib::ColorYCbCrA,
        pub path: [i8; 260usize],
    }
    pub const CACHE_BUFFER_FLAG_IMAGE_FILE: u32 = 32;
    pub const CACHE_BUFFER_FLAG_IMAGE_COPY: u32 = 64;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
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
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct CObjectBufferInfo {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct CUndoInfo {
        pub _address: u8,
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lua_State {
    _unused: [u8; 0],
}
pub type __builtin_va_list = *mut i8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_multibyte_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ACTIVATION_CONTEXT {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NET_ADDRESS_INFO_ {
    pub _address: u8,
}
