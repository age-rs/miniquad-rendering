extern crate cc;

use cc::{Build, Tool};

use std::env;

fn build_new() -> (Build, Tool) {
    let build = Build::new();
    let tool = build.try_get_compiler().unwrap_or_else(|e| panic!(e));

    (build, tool)
}

fn main() {
    let (mut build, _) = build_new();

    build.include("/usr/include/libdrm");
    build.file("magic.c");

    build.compile("magic-sys");

    let target = env::var("TARGET").unwrap_or_else(|e| panic!(e));

    if target.contains("linux") == false {
        panic!("sapp_linux support only linux target");
    }

    println!("cargo:rustc-link-lib=dylib=GLESv2");
    println!("cargo:rustc-link-lib=dylib=EGL");
    println!("cargo:rustc-link-lib=dylib=drm");
    println!("cargo:rustc-link-lib=dylib=gbm");
}
