use crate::filter::*;
use std::ptr::NonNull;

pub unsafe fn AviUtl_GetFilterNumber(fp: &FILTER) -> i32 {
    let mut si: SYS_INFO = SYS_INFO::default();
    ((*fp).exfunc.unwrap().as_ref().get_sys_info.unwrap())(None, NonNull::new(&mut si));
    si.filter_n
}
