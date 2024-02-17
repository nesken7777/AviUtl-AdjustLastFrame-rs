use std::{env, fs, path::Path};

use const_gen::{const_array_declaration, CompileConstArray};
use encoding_rs::SHIFT_JIS;

fn main() {
    let kakutyou = {
        let k = SHIFT_JIS.encode("拡張編集\0").0.to_vec();
        const_array_declaration!(EXEDIT_NAME = k)
    };
    let filter_infos = {
        let k = SHIFT_JIS.encode("最終フレーム自動調整 - Rust\0").0.to_vec();
        let name = const_array_declaration!(FILTER_NAME = k);
        let k = SHIFT_JIS
            .encode("最終フレーム自動調整 - Rust (v1.0.0)\0")
            .0
            .to_vec();
        let information = const_array_declaration!(FILTER_INFORMATION = k);
        [name, information].join("\n")
    };
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("exedit_name.rs");
    fs::write(dest_path, kakutyou).unwrap();
    let dest_path = Path::new(&out_dir).join("filter_info.rs");
    fs::write(dest_path, filter_infos).unwrap();
    println!("cargo:rustc-cdylib-link-arg=/DEF:adjust_last_flame.def");
}
