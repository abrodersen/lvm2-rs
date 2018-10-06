extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=lvm2app");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_type("lvm_.*")
        .whitelist_type("vg_t")
        .whitelist_type("lv_t")
        .whitelist_type("pv_t")
        .whitelist_type("lvseg_t")
        .whitelist_type("pvseg_t")
        .whitelist_function("lvm_.*")
        .generate()
        .expect("Unable to generate bindings");
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}