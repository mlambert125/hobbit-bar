fn main() {
    // Tell cargo to link to the gtk-layer-shell shared library
    //println!("cargo:rustc-link-lib=gtk4-layer-shell");

    // Optional: If your system needs help finding the lib
    // (e.g., pkg-config for extra include paths), use:
    //
    // pkg_config::probe_library("gtk-layer-shell").unwrap();
}
