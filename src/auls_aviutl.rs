use crate::filter::*;
use std::ptr::NonNull;

pub fn AviUtl_GetFilterNumber(fp: &FILTER) -> Option<i32> {
    unsafe {
        let mut si: SYS_INFO = SYS_INFO::default();
        ((*fp).exfunc?.as_ref().get_sys_info?)(None, NonNull::new(&mut si));
        Some(si.filter_n)
    }
}
