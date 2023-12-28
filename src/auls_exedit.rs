#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::auls_aviutl;
use crate::filter::FILTER;
use core::slice;
use windows::Win32::Foundation::HWND;
const EXEDIT_NAME: [u8; 9] = [138, 103, 146, 163, 149, 210, 143, 87, 0]; //"拡張編集\0"って書いてあります！

const MAX_FILTER: usize = 12;
const MAX_TRACK: usize = 64;
const MAX_CHECK: usize = 48;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXEDIT_OBJECT {
    pub exists: u8,
    pub flag: u8,
    pub typea: u8,
    pub padding_: u8,
    pub layer_disp: i32,
    pub frame_begin: i32,
    pub frame_end: i32,
    pub dispname: [u8; 64],
    pub index_midpt_leader: i32,
    pub filter_param: [FILTER_PARAM; MAX_FILTER],
    pub filter_status: [u8; MAX_FILTER],
    pub track_sum: i16,
    pub check_sum: i16,
    pub exdata_sum: u32,
    pub track_value_left: [i32; MAX_TRACK],
    pub track_value_right: [i32; MAX_TRACK],
    pub track_mode: [TRACK_MODE; MAX_TRACK],
    pub check_value: [i32; MAX_CHECK],
    pub exdata_offset: u32,
    pub group_belong: i32,
    pub track_param: [i32; MAX_TRACK],
    pub layer_set: i32,
    pub scene_set: i32,
}

impl EXEDIT_OBJECT {
    pub fn GetFilterNum(&self) -> i32 {
        for i in 0..MAX_FILTER {
            if self.filter_param[i].id == FILTER_PARAM::INVALID_ID {
                return i as i32;
            }
        }
        MAX_FILTER as i32
    }
    pub fn ExdataOffset(&self, idx: i32) -> u32 {
        self.exdata_offset + self.filter_param[idx as usize].exdata_offset
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILTER_PARAM {
    pub id: i32,
    pub track_begin: i16, // このフィルタの先頭のトラックバー番号
    pub check_begin: i16,
    pub exdata_offset: u32,
}

impl FILTER_PARAM {
    pub const INVALID_ID: i32 = -1;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TRACK_MODE {
    num: i16,
    script_num: i16,
}

impl TRACK_MODE {
    fn Accelerate(&self) -> bool {
        (self.num & 0x40) != 0
    }
    fn Decelerate(&self) -> bool {
        (self.num & 0x20) != 0
    }
    fn UseScript(&self) -> bool {
        (self.num & 0x0F) == 0x0F
    }
}

pub unsafe fn Exedit_GetWindow(fp: &FILTER) -> HWND {
    let exedit = Exedit_GetFilter(fp);
    exedit.map_or(HWND(0), |exedit| exedit.hwnd)
}

unsafe fn Exedit_GetFilter(fp: &FILTER) -> Option<&FILTER> {
    let i = auls_aviutl::AviUtl_GetFilterNumber(fp)?;
    fn exedit_getfilter_internal(fp: &FILTER, number: i32) -> Option<&FILTER> {
        if number <= 0 {
            return None;
        }
        unsafe {
            let exedit =
                fp.exfunc?.as_ref().get_filterp?(number).map(|x| x.cast::<FILTER>().as_ref())?;

            if slice::from_raw_parts(exedit.name.0, 9) == &EXEDIT_NAME[..] {
                Some(exedit)
            } else {
                exedit_getfilter_internal(fp, number - 1)
            }
        }
    }
    exedit_getfilter_internal(fp, i - 1)
}
