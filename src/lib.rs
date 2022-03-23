mod auls;

#[no_mangle]
extern "system" fn GetFilterTable()->FILTER_DLL{FILTER_DLL { flag: 0 }}

pub type CHAR = ::std::os::raw::c_char;
type TCHAR = ::std::os::raw::c_char;
pub type BOOL = ::std::os::raw::c_int;
pub type HWND = *mut HWND__;
pub type UINT = ::std::os::raw::c_uint;
pub type WPARAM = UINT_PTR;
pub type UINT_PTR = ::std::os::raw::c_ulonglong;
pub type LPARAM = LONG_PTR;
pub type LONG_PTR = ::std::os::raw::c_longlong;
pub type LPSTR = *mut CHAR;
pub type DWORD = ::std::os::raw::c_ulong;
pub type HFONT = *mut HFONT__;
pub type AVI_FILE_HANDLE = *mut ::std::os::raw::c_void;
pub type BYTE = ::std::os::raw::c_uchar;
pub type HINSTANCE = *mut HINSTANCE__;

pub type MULTI_THREAD_FUNC = ::std::option::Option<
    unsafe extern "C" fn(
        thread_id: ::std::os::raw::c_int,
        thread_num: ::std::os::raw::c_int,
        param1: *mut ::std::os::raw::c_void,
        param2: *mut ::std::os::raw::c_void,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HINSTANCE__ {
    pub unused: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HFONT__ {
    pub unused: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SYS_INFO {
    pub flag: ::std::os::raw::c_int,
    pub info: LPSTR,
    pub filter_n: ::std::os::raw::c_int,
    pub min_w: ::std::os::raw::c_int,
    pub min_h: ::std::os::raw::c_int,
    pub max_w: ::std::os::raw::c_int,
    pub max_h: ::std::os::raw::c_int,
    pub max_frame: ::std::os::raw::c_int,
    pub edit_name: LPSTR,
    pub project_name: LPSTR,
    pub output_name: LPSTR,
    pub vram_w: ::std::os::raw::c_int,
    pub vram_h: ::std::os::raw::c_int,
    pub vram_yc_size: ::std::os::raw::c_int,
    pub vram_line_size: ::std::os::raw::c_int,
    pub hfont: HFONT,
    pub build: ::std::os::raw::c_int,
    pub reserve: [::std::os::raw::c_int; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FILE_INFO {
    pub flag: ::std::os::raw::c_int,
    pub name: LPSTR,
    pub w: ::std::os::raw::c_int,
    pub h: ::std::os::raw::c_int,
    pub video_rate: ::std::os::raw::c_int,
    pub video_scale: ::std::os::raw::c_int,
    pub audio_rate: ::std::os::raw::c_int,
    pub audio_ch: ::std::os::raw::c_int,
    pub frame_n: ::std::os::raw::c_int,
    pub video_decode_format: DWORD,
    pub video_decode_bit: ::std::os::raw::c_int,
    pub audio_n: ::std::os::raw::c_int,
    pub reserve: [::std::os::raw::c_int; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FRAME_STATUS {
    pub video: ::std::os::raw::c_int,
    pub audio: ::std::os::raw::c_int,
    pub inter: ::std::os::raw::c_int,
    pub index24fps: ::std::os::raw::c_int,
    pub config: ::std::os::raw::c_int,
    pub vcm: ::std::os::raw::c_int,
    pub edit_flag: ::std::os::raw::c_int,
    pub reserve: [::std::os::raw::c_int; 9usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EXFUNC {
    pub get_ycp_ofs: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            ofs: ::std::os::raw::c_int,
        ),
    >,
    pub get_ycp: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_pixelp: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_audio: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_editing:
        ::std::option::Option<unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub is_saving:
        ::std::option::Option<unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub get_frame: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub get_frame_n: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub get_frame_size: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            w: *mut ::std::os::raw::c_int,
            h: *mut ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub set_frame: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_frame_n: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub copy_frame: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            d: ::std::os::raw::c_int,
            s: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub copy_video: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            d: ::std::os::raw::c_int,
            s: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub copy_audio: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            d: ::std::os::raw::c_int,
            s: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub copy_clip: ::std::option::Option<
        unsafe extern "C" fn(
            hwnd: HWND,
            pixelp: *mut ::std::os::raw::c_void,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub paste_clip: ::std::option::Option<
        unsafe extern "C" fn(
            hwnd: HWND,
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub get_frame_status: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            fsp: *mut FRAME_STATUS,
        ) -> BOOL,
    >,
    pub set_frame_status: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            fsp: *mut FRAME_STATUS,
        ) -> BOOL,
    >,
    pub is_saveframe: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, n: ::std::os::raw::c_int) -> BOOL,
    >,
    pub is_keyframe: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, n: ::std::os::raw::c_int) -> BOOL,
    >,
    pub is_recompress: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, n: ::std::os::raw::c_int) -> BOOL,
    >,
    pub filter_window_update:
        ::std::option::Option<unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub is_filter_window_disp:
        ::std::option::Option<unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub get_file_info: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, fip: *mut FILE_INFO) -> BOOL,
    >,
    pub get_config_name: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, n: ::std::os::raw::c_int) -> LPSTR,
    >,
    pub is_filter_active:
        ::std::option::Option<unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub get_pixel_filtered: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            pixelp: *mut ::std::os::raw::c_void,
            w: *mut ::std::os::raw::c_int,
            h: *mut ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub get_audio_filtered: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_select_frame: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            s: *mut ::std::os::raw::c_int,
            e: *mut ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub set_select_frame: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            s: ::std::os::raw::c_int,
            e: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub rgb2yc: ::std::option::Option<
        unsafe extern "C" fn(
            ycp: *mut PIXEL_YC,
            pixelp: *mut PIXEL,
            w: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub yc2rgb: ::std::option::Option<
        unsafe extern "C" fn(
            pixelp: *mut PIXEL,
            ycp: *mut PIXEL_YC,
            w: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub dlg_get_load_name:
        ::std::option::Option<unsafe extern "C" fn(name: LPSTR, filter: LPSTR, def: LPSTR) -> BOOL>,
    pub dlg_get_save_name:
        ::std::option::Option<unsafe extern "C" fn(name: LPSTR, filter: LPSTR, def: LPSTR) -> BOOL>,
    pub ini_load_int: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            key: LPSTR,
            n: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub ini_save_int: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            key: LPSTR,
            n: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub ini_load_str: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            key: LPSTR,
            str_: LPSTR,
            def: LPSTR,
        ) -> BOOL,
    >,
    pub ini_save_str: ::std::option::Option<
        unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void, key: LPSTR, str_: LPSTR) -> BOOL,
    >,
    pub get_source_file_info: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            fip: *mut FILE_INFO,
            source_file_id: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub get_source_video_number: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            source_file_id: *mut ::std::os::raw::c_int,
            source_video_number: *mut ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub get_sys_info: ::std::option::Option<
        unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void, sip: *mut SYS_INFO) -> BOOL,
    >,
    pub get_filterp: ::std::option::Option<
        unsafe extern "C" fn(filter_id: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void,
    >,
    pub get_ycp_filtering: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            reserve: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_audio_filtering: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_ycp_filtering_cache_size: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
            d: ::std::os::raw::c_int,
            flag: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub get_ycp_filtering_cache: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_ycp_source_cache: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            ofs: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_disp_pixelp: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            format: DWORD,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_pixel_source: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            pixelp: *mut ::std::os::raw::c_void,
            format: DWORD,
        ) -> BOOL,
    >,
    pub get_pixel_filtered_ex: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            pixelp: *mut ::std::os::raw::c_void,
            w: *mut ::std::os::raw::c_int,
            h: *mut ::std::os::raw::c_int,
            format: DWORD,
        ) -> BOOL,
    >,
    pub get_ycp_filtering_cache_ex: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            w: *mut ::std::os::raw::c_int,
            h: *mut ::std::os::raw::c_int,
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
            file: LPSTR,
            w: *mut ::std::os::raw::c_int,
            h: *mut ::std::os::raw::c_int,
            flag: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub resize_yc: ::std::option::Option<
        unsafe extern "C" fn(
            ycp: *mut PIXEL_YC,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
            ycp_src: *mut PIXEL_YC,
            sx: ::std::os::raw::c_int,
            sy: ::std::os::raw::c_int,
            sw: ::std::os::raw::c_int,
            sh: ::std::os::raw::c_int,
        ),
    >,
    pub copy_yc: ::std::option::Option<
        unsafe extern "C" fn(
            ycp: *mut PIXEL_YC,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            ycp_src: *mut PIXEL_YC,
            sx: ::std::os::raw::c_int,
            sy: ::std::os::raw::c_int,
            sw: ::std::os::raw::c_int,
            sh: ::std::os::raw::c_int,
            tr: ::std::os::raw::c_int,
        ),
    >,
    pub draw_text: ::std::option::Option<
        unsafe extern "C" fn(
            ycp: *mut PIXEL_YC,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            text: LPSTR,
            r: ::std::os::raw::c_int,
            g: ::std::os::raw::c_int,
            b: ::std::os::raw::c_int,
            tr: ::std::os::raw::c_int,
            hfont: HFONT,
            w: *mut ::std::os::raw::c_int,
            h: *mut ::std::os::raw::c_int,
        ),
    >,
    pub avi_file_open: ::std::option::Option<
        unsafe extern "C" fn(
            file: LPSTR,
            fip: *mut FILE_INFO,
            flag: ::std::os::raw::c_int,
        ) -> AVI_FILE_HANDLE,
    >,
    pub avi_file_close: ::std::option::Option<unsafe extern "C" fn(afh: AVI_FILE_HANDLE)>,
    pub avi_file_read_video: ::std::option::Option<
        unsafe extern "C" fn(
            afh: AVI_FILE_HANDLE,
            ycp: *mut PIXEL_YC,
            n: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub avi_file_read_audio: ::std::option::Option<
        unsafe extern "C" fn(
            afh: AVI_FILE_HANDLE,
            buf: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub avi_file_get_video_pixelp: ::std::option::Option<
        unsafe extern "C" fn(
            afh: AVI_FILE_HANDLE,
            n: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_avi_file_filter:
        ::std::option::Option<unsafe extern "C" fn(type_: ::std::os::raw::c_int) -> LPSTR>,
    pub avi_file_read_audio_sample: ::std::option::Option<
        unsafe extern "C" fn(
            afh: AVI_FILE_HANDLE,
            start: ::std::os::raw::c_int,
            length: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub avi_file_set_audio_sample_rate: ::std::option::Option<
        unsafe extern "C" fn(
            afh: AVI_FILE_HANDLE,
            audio_rate: ::std::os::raw::c_int,
            audio_ch: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_frame_status_table: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            type_: ::std::os::raw::c_int,
        ) -> *mut BYTE,
    >,
    pub set_undo:
        ::std::option::Option<unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub add_menu_item: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            name: LPSTR,
            hwnd: HWND,
            id: ::std::os::raw::c_int,
            def_key: ::std::os::raw::c_int,
            flag: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub edit_open: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            file: LPSTR,
            flag: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub edit_close:
        ::std::option::Option<unsafe extern "C" fn(editp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub edit_output: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            file: LPSTR,
            flag: ::std::os::raw::c_int,
            type_: LPSTR,
        ) -> BOOL,
    >,
    pub set_config: ::std::option::Option<
        unsafe extern "C" fn(
            editp: *mut ::std::os::raw::c_void,
            n: ::std::os::raw::c_int,
            name: LPSTR,
        ) -> BOOL,
    >,
    pub reserve: [::std::os::raw::c_int; 7usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HWND__ {
    pub unused: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PIXEL {
    pub b: ::std::os::raw::c_uchar,
    pub g: ::std::os::raw::c_uchar,
    pub r: ::std::os::raw::c_uchar,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PIXEL_YC {
    pub y: ::std::os::raw::c_short,
    pub cb: ::std::os::raw::c_short,
    pub cr: ::std::os::raw::c_short,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FILTER_PROC_INFO {
    pub flag: ::std::os::raw::c_int,
    pub ycp_edit: *mut PIXEL_YC,
    pub ycp_temp: *mut PIXEL_YC,
    pub w: ::std::os::raw::c_int,
    pub h: ::std::os::raw::c_int,
    pub max_w: ::std::os::raw::c_int,
    pub max_h: ::std::os::raw::c_int,
    pub frame: ::std::os::raw::c_int,
    pub frame_n: ::std::os::raw::c_int,
    pub org_w: ::std::os::raw::c_int,
    pub org_h: ::std::os::raw::c_int,
    pub audiop: *mut ::std::os::raw::c_short,
    pub audio_n: ::std::os::raw::c_int,
    pub audio_ch: ::std::os::raw::c_int,
    pub pixelp: *mut PIXEL,
    pub editp: *mut ::std::os::raw::c_void,
    pub yc_size: ::std::os::raw::c_int,
    pub line_size: ::std::os::raw::c_int,
    pub reserve: [::std::os::raw::c_int; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FILTER {
    pub flag: ::std::os::raw::c_int,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub name: *mut TCHAR,
    pub track_n: ::std::os::raw::c_int,
    pub track_name: *mut *mut TCHAR,
    pub track_default: *mut ::std::os::raw::c_int,
    pub track_s: *mut ::std::os::raw::c_int,
    pub track_e: *mut ::std::os::raw::c_int,
    pub check_n: ::std::os::raw::c_int,
    pub check_name: *mut *mut TCHAR,
    pub check_default: *mut ::std::os::raw::c_int,
    pub func_proc: ::std::option::Option<
        unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void, fpip: *mut FILTER_PROC_INFO) -> BOOL,
    >,
    pub func_init:
        ::std::option::Option<unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub func_exit:
        ::std::option::Option<unsafe extern "C" fn(fp: *mut ::std::os::raw::c_void) -> BOOL>,
    pub func_update: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            status: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub func_WndProc: ::std::option::Option<
        unsafe extern "C" fn(
            hwnd: HWND,
            message: UINT,
            wparam: WPARAM,
            lparam: LPARAM,
            editp: *mut ::std::os::raw::c_void,
            fp: *mut ::std::os::raw::c_void,
        ) -> BOOL,
    >,
    pub track: *mut ::std::os::raw::c_int,
    pub check: *mut ::std::os::raw::c_int,
    pub ex_data_ptr: *mut ::std::os::raw::c_void,
    pub ex_data_size: ::std::os::raw::c_int,
    pub information: *mut TCHAR,
    pub func_save_start: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            s: ::std::os::raw::c_int,
            e: ::std::os::raw::c_int,
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
    pub dll_hinst: HINSTANCE,
    pub ex_data_def: *mut ::std::os::raw::c_void,
    pub func_is_saveframe: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            saveno: ::std::os::raw::c_int,
            frame: ::std::os::raw::c_int,
            fps: ::std::os::raw::c_int,
            edit_flag: ::std::os::raw::c_int,
            inter: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub func_project_load: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            data: *mut ::std::os::raw::c_void,
            size: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub func_project_save: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            data: *mut ::std::os::raw::c_void,
            size: *mut ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub func_modify_title: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut ::std::os::raw::c_void,
            editp: *mut ::std::os::raw::c_void,
            frame: ::std::os::raw::c_int,
            title: LPSTR,
            max_title: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub dll_path: *mut TCHAR,
    pub reserve: [::std::os::raw::c_int; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FILTER_DLL {
    pub flag: ::std::os::raw::c_int,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub name: *mut TCHAR,
    pub track_n: ::std::os::raw::c_int,
    pub track_name: *mut *mut TCHAR,
    pub track_default: *mut ::std::os::raw::c_int,
    pub track_s: *mut ::std::os::raw::c_int,
    pub track_e: *mut ::std::os::raw::c_int,
    pub check_n: ::std::os::raw::c_int,
    pub check_name: *mut *mut TCHAR,
    pub check_default: *mut ::std::os::raw::c_int,
    pub func_proc: ::std::option::Option<
        unsafe extern "C" fn(fp: *mut FILTER, fpip: *mut FILTER_PROC_INFO) -> BOOL,
    >,
    pub func_init: ::std::option::Option<unsafe extern "C" fn(fp: *mut FILTER) -> BOOL>,
    pub func_exit: ::std::option::Option<unsafe extern "C" fn(fp: *mut FILTER) -> BOOL>,
    pub func_update: ::std::option::Option<
        unsafe extern "C" fn(fp: *mut FILTER, status: ::std::os::raw::c_int) -> BOOL,
    >,
    pub func_WndProc: ::std::option::Option<
        unsafe extern "C" fn(
            hwnd: HWND,
            message: UINT,
            wparam: WPARAM,
            lparam: LPARAM,
            editp: *mut ::std::os::raw::c_void,
            fp: *mut FILTER,
        ) -> BOOL,
    >,
    pub track: *mut ::std::os::raw::c_int,
    pub check: *mut ::std::os::raw::c_int,
    pub ex_data_ptr: *mut ::std::os::raw::c_void,
    pub ex_data_size: ::std::os::raw::c_int,
    pub information: *mut TCHAR,
    pub func_save_start: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut FILTER,
            s: ::std::os::raw::c_int,
            e: ::std::os::raw::c_int,
            editp: *mut ::std::os::raw::c_void,
        ) -> BOOL,
    >,
    pub func_save_end: ::std::option::Option<
        unsafe extern "C" fn(fp: *mut FILTER, editp: *mut ::std::os::raw::c_void) -> BOOL,
    >,
    pub exfunc: *mut EXFUNC,
    pub hwnd: HWND,
    pub dll_hinst: HINSTANCE,
    pub ex_data_def: *mut ::std::os::raw::c_void,
    pub func_is_saveframe: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut FILTER,
            editp: *mut ::std::os::raw::c_void,
            saveno: ::std::os::raw::c_int,
            frame: ::std::os::raw::c_int,
            fps: ::std::os::raw::c_int,
            edit_flag: ::std::os::raw::c_int,
            inter: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub func_project_load: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut FILTER,
            editp: *mut ::std::os::raw::c_void,
            data: *mut ::std::os::raw::c_void,
            size: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub func_project_save: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut FILTER,
            editp: *mut ::std::os::raw::c_void,
            data: *mut ::std::os::raw::c_void,
            size: *mut ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub func_modify_title: ::std::option::Option<
        unsafe extern "C" fn(
            fp: *mut FILTER,
            editp: *mut ::std::os::raw::c_void,
            frame: ::std::os::raw::c_int,
            title: LPSTR,
            max_title: ::std::os::raw::c_int,
        ) -> BOOL,
    >,
    pub dll_path: *mut TCHAR,
    pub reserve: [::std::os::raw::c_int; 2usize],
}
