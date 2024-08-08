use std::ffi::CStr;
use std::fmt;
use std::ptr::null_mut;

use dlopen2::wrapper::{Container, WrapperApi};
use libc::{c_char, c_int, c_ulong, c_void, size_t};

#[derive(WrapperApi)]
struct GmpApi {
    // HACK: #1 __gmp_version is just a `const char * const`, but I wasn't able to convince the
    // compiler to let me use pointers as entry types, only arguments or return values.
    __gmp_version: &'static &'static c_char,
    __gmpz_clear: fn(n: &mut MpInt),
    __gmpz_cmp: fn(a: *const MpInt, b: *const MpInt) -> c_int,
    __gmpz_cmp_ui: fn(a: *const MpInt, b: c_ulong) -> c_int,
    __gmpz_get_str: fn(s: *mut c_char, base: c_int, src: *const MpInt) -> *mut c_char,
    __gmpz_init: fn(dst: *mut MpInt),
    __gmpz_mul: fn(dst: *mut MpInt, fact1: *const MpInt, fact2: &MpInt),
    __gmpz_set: fn(dst: *mut MpInt, src: *const MpInt),
    __gmpz_set_ui: fn(dst: *mut MpInt, src: c_ulong),
    __gmpz_sizeinbase: fn(src: *const MpInt, base: c_int) -> size_t,
    __gmpz_sub_ui: fn(diff: *mut MpInt, min: *const MpInt, sub: c_ulong),
}

lazy_static::lazy_static! {
    static ref GMP: Container<GmpApi> = unsafe {
        cfg_if::cfg_if! {
            if #[cfg(all(target_os = "macos", target_arch = "aarch64"))] {
                // HACK: #2 - Homebrew moved to /opt/homebrew on M1 macs, and dyld on macOS hates
                // to search for dylibs - see `dlopen(3)` under "Searching".
                // So here's the full path to homebrew's libgmp. If anyone knows the innards of
                // dyld enough to let me do this with the build script, please let me know!
                //
                // Note that we hardcode both the path and the file extension here.
                Container::load("/opt/homebrew/lib/libgmp.dylib").expect("failed to load gmp library")
            } else if #[cfg(target_os = "macos")] {
                // HACK: #3a - On intel Macs, homebrew ist at /usr/local, which should be on the
                // dyld search path, so here we just need to hardcode the library file extension.
                Container::load("libgmp.dylib").expect("failed to load gmp library")
            } else if #[cfg(any(target_os = "linux", target_os = "freebsd"))] {
                // HACK: #3b - rustc will automatically choose `.dylib` or `.so` based on the
                // OS, but when we use `dlopen()`, we need to do all of this manually.
                Container::load("libgmp.so").expect("failed to load gmp library")
            }
        }
    };
}

#[repr(C)]
#[derive(Debug)]
pub struct MpInt {
    alloc: i32,
    size: i32,
    d: *mut c_void,
}

impl MpInt {
    pub fn new() -> Self {
        let mut n = Self {
            alloc: 0,
            size: 0,
            d: null_mut::<c_void>(),
        };
        GMP.__gmpz_init(&mut n);
        n
    }

    pub fn gmp_version() -> String {
        let cstr_ver = unsafe { CStr::from_ptr(*GMP.__gmp_version) };
        let gmp_ver = cstr_ver.to_string_lossy();
        format!("using gmp {gmp_ver} (dlopen)")
    }

    pub fn factorial(&self) -> MpInt {
        let mut n = MpInt::from(self);
        let mut result = MpInt::from(1);

        if GMP.__gmpz_cmp_ui(&n, 0) == 0 {
            return result;
        }

        while GMP.__gmpz_cmp_ui(&n, 1) > 0 {
            GMP.__gmpz_mul(&mut result, &result, &n);
            GMP.__gmpz_sub_ui(&mut n, &n, 1);
        }

        result
    }
}

impl fmt::Display for MpInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf_size = GMP.__gmpz_sizeinbase(self, 10);
        let mut buf = vec![0u8; buf_size + 2];
        GMP.__gmpz_get_str(buf.as_mut_ptr().cast::<c_char>(), 10, self);
        let cstr = unsafe { CStr::from_ptr(buf.as_ptr()) };
        write!(f, "{}", cstr.to_str().expect("encoding error"))
    }
}

impl From<u64> for MpInt {
    fn from(ui: u64) -> Self {
        let mut n = Self::new();
        GMP.__gmpz_set_ui(&mut n, ui);
        n
    }
}

impl From<&MpInt> for MpInt {
    fn from(src: &Self) -> Self {
        let mut dst = Self::new();
        GMP.__gmpz_set(&mut dst, src);
        dst
    }
}

impl PartialEq for MpInt {
    fn eq(&self, other: &Self) -> bool {
        if self.alloc == other.alloc && self.size == other.size && self.d == other.d {
            return true;
        }
        GMP.__gmpz_cmp(self, other) == 0
    }
}

impl Eq for MpInt {}

impl Drop for MpInt {
    fn drop(&mut self) {
        GMP.__gmpz_clear(self);
    }
}
