//! The main.rs file will use whichever implementation of the `MpInt` type you choose.
//!
//! Neither is enabled by default, but can be enabled at build/run time by passing the `--feature`
//! / `-F` flag to `cargo build` or `cargo run`.
//!
//! ```sh
//! $ cargo run -F shared 10
//!     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
//!      Running `target/debug/rust2c 10`
//! [src/mp_int/gmp_shared.rs:55:18] __gmp_version = 0x0000fffec54e0428
//! rust2c 0.2.0 using gmp 6.2.1
//!
//! 10! -> 3628800
//!
//! $ cargo run -F dynamic 10
//!     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
//!      Running `target/debug/rust2c 10`
//! rust2c 0.2.0 using gmp 6.2.1
//!
//! 10! -> 3628800
//! ```

cfg_if::cfg_if! {
    if #[cfg(all(feature = "dynamic", feature = "shared"))] {
        compile_error!("features `dynamic` and `shared` are mutually exclusive");
    } else if #[cfg(feature = "dynamic")] {
        #[path = "gmp_dynamic.rs"]
        mod gmp;
    } else if #[cfg(feature = "shared")] {
        #[path = "gmp_shared.rs"]
        mod gmp;
    } else {
        compile_error!("Please select the `dynamic` or `shared` feature\neg: cargo run -F shared");
    }
}

pub use gmp::MpInt;