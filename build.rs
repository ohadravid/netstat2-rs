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
        "linux" | "android" => {
            let bindings = bindgen::builder()
                .header_contents(
                    "linux_bindings.h",
                    r#"
                    #include <linux/sock_diag.h>
                    #include <linux/inet_diag.h>
                    #include <linux/rtnetlink.h>
                    #include <linux/netlink.h>
                    #include <linux/tcp.h>
                "#,
                )
                .layout_tests(false)
                .generate()
                .expect("Failed to build linux bindings");

            let output_path =
                Path::new(&env::var("OUT_DIR").expect("OUT_DIR env var was not defined"))
                    .join("linux_bindings.rs");

            bindings
                .write_to_file(output_path)
                .expect("Failed to write linux bindings");
        }
        _ => {}
    }
}

#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "linux", target_os = "android")))]
fn main() {}