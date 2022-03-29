use crate::filter::*;
use std::ptr::null_mut;

pub unsafe fn AviUtl_GetFilterNumber(fp: *mut FILTER) -> i32 {
    let mut si: SYS_INFO = SYS_INFO::default();
    ((*(*fp).exfunc).get_sys_info.unwrap())(null_mut(), &mut si);
    si.filter_n
}
