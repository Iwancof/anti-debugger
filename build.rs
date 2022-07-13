extern crate cc;

fn main() {
    // println!("cargo:rerun-if-changed=src/aslr.c");
    // println!("cargo:rustc-link-search=src");
    cc::Build::new()
        .warnings(true)
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-v")
        .file("src/aslr.c")
        .compile("libaslr.a");

    cc::Build::new()
        .warnings(true)
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-v")
        .file("src/vdso.c")
        .compile("libvdso.a");
}
