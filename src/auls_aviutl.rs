use crate::filter::*;
use std::ptr::NonNull;

pub unsafe fn AviUtl_GetFilterNumber(fp: &FILTER) -> Option<i32> {
    let mut si: SYS_INFO = SYS_INFO::default();
    ((*fp).exfunc?.as_ref().get_sys_info?)(None, NonNull::new(&mut si));
    Some(si.filter_n)
}
