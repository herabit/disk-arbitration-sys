use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=framework=DiskArbitration");
    // _bindgen();
}

fn _bindgen() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .parse_callbacks(Box::new(
            bindgen::CargoCallbacks::new().rerun_on_header_files(true),
        ))
        .allowlist_type(r"_*DA.*")
        .allowlist_function(r"DA.*")
        .allowlist_var(r"k?DA.*")
        .blocklist_file(r"(CoreFoundation|CarbonCore|IOKit)/.*")
        .blocklist_item(r"_*CF.*")
        .sort_semantically(true)
        .merge_extern_blocks(true)
        .clang_arg("-fretain-comments-from-system-headers")
        .generate()
        .expect("Failed to build bindings");

    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Failed to write bindings");
}
