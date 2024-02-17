use std::{env, fs, path::Path};

use const_gen::{const_array_declaration,CompileConstArray};
use encoding_rs::SHIFT_JIS;

fn main() {
    let k = SHIFT_JIS.encode("拡張編集\0").0.to_vec();
    let kakutyou = const_array_declaration!(EXEDIT_NAME = k);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("const_gen.rs");
    fs::write(dest_path, kakutyou).unwrap();
    println!("cargo:rustc-cdylib-link-arg=/DEF:adjust_last_flame.def");
}