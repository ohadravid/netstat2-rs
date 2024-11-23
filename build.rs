use std::env;
use std::path::Path;

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "linux", target_os = "android"))]
fn main() {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match os.as_str() {
        "macos" | "ios" => {
            let bindings = bindgen::builder()
                .header_contents("libproc_rs.h", "#include <libproc.h>")
                .layout_tests(false)
                .clang_args(&[
                    "-x",
                    "c++",
                    "-I",
                    "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/",
                ])
                .generate()
                .expect("Failed to build libproc bindings");

            let output_path =
                Path::new(&env::var("OUT_DIR").expect("OUT_DIR env var was not defined"))
                    .join("libproc_bindings.rs");

            bindings
                .write_to_file(output_path)
                .expect("Failed to write libproc bindings");
        }
        _ => {}
    }
}

#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "linux", target_os = "android")))]
fn main() {}