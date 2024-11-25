use std::env;

fn main() {
    tauri_build::build();

    // Get the target operating system
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "windows" => {
            let mpv_include_path = "C:/mpv-dev/include";
            let mpv_lib_path = "C:/mpv-dev/lib";
            println!("cargo:include={}", mpv_include_path);
            println!("cargo:rustc-link-search=native={}", mpv_lib_path);
        }
        "linux" => {
            let mpv_include_path = "/usr/include/mpv";
            let mpv_lib_path = "/usr/lib";
            println!("cargo:include={}", mpv_include_path);
            println!("cargo:rustc-link-search=native={}", mpv_lib_path);
        }
        "macos" => {
            let mpv_include_path = "/usr/local/include/mpv";
            let mpv_lib_path = "/usr/local/lib";
            println!("cargo:include={}", mpv_include_path);
            println!("cargo:rustc-link-search=native={}", mpv_lib_path);
        }
        _ => {
            panic!("Unsupported operating system: {}", target_os);
        }
    }

    // Link against the dynamic library 'mpv'
    println!("cargo:rustc-link-lib=dylib=mpv");

    // Ensure that the build script is re-run if these paths change
    println!("cargo:rerun-if-changed=build.rs");
}
