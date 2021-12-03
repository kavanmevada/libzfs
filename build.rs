use std::env;

extern crate pkg_config;
extern crate bindgen;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");



    println!("cargo:rustc-link-lib=avl");
    println!("cargo:rustc-link-lib=efi");
    println!("cargo:rustc-link-lib=bicp");
    println!("cargo:rustc-link-lib=nvpair");
    println!("cargo:rustc-link-lib=share");
    println!("cargo:rustc-link-lib=spl");
    println!("cargo:rustc-link-lib=tpool");
    println!("cargo:rustc-link-lib=unicode");
    println!("cargo:rustc-link-lib=uutil");
    println!("cargo:rustc-link-lib=zfs");
    println!("cargo:rustc-link-lib=zfs_core");
    println!("cargo:rustc-link-lib=zfsbootenv");
    println!("cargo:rustc-link-lib=zpool");
    println!("cargo:rustc-link-lib=zstd");
    println!("cargo:rustc-link-lib=zutil");



    let out_file = env::current_dir().unwrap().join("src").join("bindings.rs");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate_inline_functions(true)
        .derive_default(true)
        .rustfmt_bindings(true)

        .allowlist_function("make_root_vdev")
        .allowlist_function("zpool_create")
        .allowlist_function("libzfs_init")



        .rustified_enum("boolean_t")



        .blocklist_type("__uint[0-9]+_t")
        .blocklist_type("__int[0-9]+_t")

        .clang_arg("-I/usr/include")
        .clang_arg("-I/usr/include/libavl")
        .clang_arg("-I/usr/include/libefi")
        .clang_arg("-I/usr/include/libbicp")
        .clang_arg("-I/usr/include/libnvpair")
        .clang_arg("-I/usr/include/libshare")
        .clang_arg("-I/usr/include/libspl")
        .clang_arg("-I/usr/include/libtpool")
        .clang_arg("-I/usr/include/libunicode")
        .clang_arg("-I/usr/include/libuutil")
        .clang_arg("-I/usr/include/libzfs")
        .clang_arg("-I/usr/include/libzfs_core")
        .clang_arg("-I/usr/include/libzfsbootenv")
        .clang_arg("-I/usr/include/libzpool")
        .clang_arg("-I/usr/include/libzstd")
        .clang_arg("-I/usr/include/libzutil")

        .clang_arg("-I/usr/lib/gcc/x86_64-pc-linux-gnu/11.1.0/include/")
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to src.
    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}