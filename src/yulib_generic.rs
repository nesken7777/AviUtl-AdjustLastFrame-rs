#![allow(non_snake_case)]

pub fn Crc32_2(data: &[u8], table: &mut [u32]) -> u32 {
    let mut crc = !0;
    for i in data {
        crc = (crc >> 8) ^ table[(crc as u8 ^ i) as usize];
    }
    !crc
}

pub fn Crc32(data: &[u8]) -> u32 {
    let mut table: [u32; 256] = [0; 256];
    MakeCrc32Table(table.as_mut_slice());
    Crc32_2(data, &mut table)
}

pub fn MakeCrc32Table(table: &mut [u32]) {
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
        table[i as usize] = v;
    }
}

pub fn Max<T: Ord>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}
