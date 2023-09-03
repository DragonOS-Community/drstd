extern crate cc;

fn main() {
    #[cfg(target_os = "linux")]
    cc::Build::new()
        .flag("-nostdlib")
        .file("src/platform/c/dragonos_malloc.c")
        .compile("dragonos_malloc");
    #[cfg(not(target_os = "linux"))]
    cc::Build::new()
        .flag("-nostdinc")
        .flag("-nostdlib")
        .file("src/platform/c/dragonos_malloc.c")
        .compile("dragonos_malloc");
}
