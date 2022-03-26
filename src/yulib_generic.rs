#![allow(non_snake_case)]
use std::ffi::c_void;

#[inline]
pub unsafe fn Crc32_3(data: *mut c_void, size: u32, table: *mut u32) -> u32 {
    let p = data as *mut u8;
    let end = p.add(size as usize);
    let mut crc = !0;
    while p < end {
        crc = (crc >> 8) ^ *table.add((crc as u8 ^ *p) as usize);
        *p += 1;
    }
    return !crc;
}

#[inline]
pub unsafe fn Crc32_2(data: *mut c_void, size: u32) -> u32 {
    let mut table: [u32; 256] = [0; 256];
    MakeCrc32Table(table.as_mut_ptr());
    return Crc32_3(data, size, table.as_mut_ptr());
}

#[inline]
pub unsafe fn MakeCrc32Table(table: *mut u32) {
    const CRCPOLY: u32 = 0xEDB88320;
    for i in 0u32..256 {
        let mut v = i;
        for _ in 0..8 {
            if v & 1 != 0 {
                v = (v >> 1) ^ CRCPOLY;
            } else {
                v >>= 1;
            }
        }
        *table.add(i as usize) = v;
    }
}

pub fn Max<T: Ord>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}
