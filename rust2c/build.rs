fn main() {
    // HACK: #1 - This path may already be on the link search path, but this ensures it.
    // This is primarily for FreeBSD which uses /usr/local extensively.
    println!("cargo:rustc-link-search=/usr/local/lib");

    // HACK: #2 - This is only needed on M1 Macs, and only works for the `shared` target.
    // See [the dlopen hack](src/mp_int/gmp_dlopen.rs#28) for to workaround.
    println!("cargo:rustc-link-search=/opt/homebrew/lib");
}
