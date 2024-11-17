fn main() {
    tauri_build::build();
    // Set the path to the MPV include and lib directories
    let mpv_include_path = "C:/mpv-dev/include";
    let mpv_lib_path = "C:/mpv-dev/lib";

    // Add the include directory to the build
    println!("cargo:include={}", mpv_include_path);

    // Add the library search path
    println!("cargo:rustc-link-search=native={}", mpv_lib_path);

    // Link against the dynamic library 'mpv'
    println!("cargo:rustc-link-lib=dylib=mpv");

    // Optionally, add additional flags if needed
    // println!("cargo:rustc-flags=-C link-args=-L{}", mpv_lib_path);

    // Ensure that the build script is re-run if these paths change
    println!("cargo:rerun-if-changed=build.rs");
}
