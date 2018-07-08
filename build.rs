extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path;

fn main() {
    let library = pkg_config::Config::new().probe("opencv").unwrap();

    let bindings = bindgen::Builder::default()
        .clang_args(library
                        .include_paths
                        .iter()
                        .map(|p| format!("-I{}", p.to_str().unwrap())))
        .header("wrapper.h")
        .whitelist_function("cvCreateImageHeader")
        .whitelist_function("cvSetData")
        .whitelist_function("cvReleaseImageHeader")
        .whitelist_function("cvCopy")
        .whitelist_function("cvShowImage")
        .whitelist_function("cvWaitKey")
        .whitelist_function("cvRectangle")
        .whitelist_function("cvCreateCameraCapture")
        .whitelist_function("cvReleaseCapture")
        .whitelist_function("cvQueryFrame")
        .prepend_enum_name(false)
        .rustfmt_bindings(false)
        .generate()
        .unwrap();
    bindings
        .write_to_file(path::PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap();
}
