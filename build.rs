extern crate cc;
extern crate pkg_config;

fn main() {
    // include
    let library = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("opencv")
        .unwrap();

    let mut build = cc::Build::new();
    build.file("opencv_bindings.cpp").cpp(true);
    for path in library.include_paths.iter() {
        build.include(path);
    }
    build.compile("opencv_bindings");

    // link
    pkg_config::Config::new().probe("opencv").unwrap();
}
