use std::ffi::CStr;
use std::fmt;
use std::ptr::null_mut;

use libc::{c_char, c_int, c_ulong, c_void, size_t};

#[link(name = "gmp")]
extern "C" {
    pub(super) static __gmp_version: *const c_char;
    pub(super) fn __gmpz_clear(n: *mut MpInt);
    pub(super) fn __gmpz_cmp(a: *const MpInt, b: *const MpInt) -> c_int;
    pub(super) fn __gmpz_cmp_ui(a: *const MpInt, b: c_ulong) -> c_int;
    pub(super) fn __gmpz_get_str(s: *mut c_char, base: c_int, src: *const MpInt) -> *mut c_char;
    pub(super) fn __gmpz_init(dst: *mut MpInt);
    pub(super) fn __gmpz_mul(dst: *mut MpInt, fact1: *const MpInt, fact2: *const MpInt);
    pub(super) fn __gmpz_set(dst: *mut MpInt, src: *const MpInt);
    pub(super) fn __gmpz_set_ui(dst: *mut MpInt, src: c_ulong);
    pub(super) fn __gmpz_sizeinbase(src: *const MpInt, base: c_int) -> size_t;
    pub(super) fn __gmpz_sub_ui(diff: *mut MpInt, min: *const MpInt, sub: c_ulong);
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
        unsafe { __gmpz_init(&mut n) };
        n
    }

    pub fn gmp_version() -> String {
        let cstr_ver = unsafe { CStr::from_ptr(__gmp_version) };
        let gmp_ver = cstr_ver.to_str().expect("failed to get gmp version");
        format!("using gmp {gmp_ver} (shared)")
    }

    pub fn factorial(&self) -> MpInt {
        unsafe {
            let mut n = MpInt::from(self);
            let mut result = MpInt::from(1);

            if __gmpz_cmp_ui(&n, 0) == 0 {
                return result;
            }

            while __gmpz_cmp_ui(&n, 1) > 0 {
                __gmpz_mul(&mut result, &result, &n);
                __gmpz_sub_ui(&mut n, &n, 1);
            }

            result
        }
    }
}

impl fmt::Display for MpInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let buf_size = __gmpz_sizeinbase(self, 10);
            let mut buf = vec![0u8; buf_size + 2];
            __gmpz_get_str(buf.as_mut_ptr().cast::<c_char>(), 10, self);
            let cstr = CStr::from_ptr(buf.as_ptr());
            write!(f, "{}", cstr.to_str().expect("encoding error"))
        }
    }
}

impl From<u64> for MpInt {
    fn from(ui: u64) -> Self {
        let mut n = Self::new();
        unsafe { __gmpz_set_ui(&mut n, ui) };
        n
    }
}

impl From<&MpInt> for MpInt {
    fn from(src: &Self) -> Self {
        let mut dst = Self::new();
        unsafe { __gmpz_set(&mut dst, src) };
        dst
    }
}

impl PartialEq for MpInt {
    fn eq(&self, other: &Self) -> bool {
        if self.alloc == other.alloc && self.size == other.size && self.d == other.d {
            return true;
        }
        unsafe { __gmpz_cmp(self, other) == 0 }
    }
}

impl Eq for MpInt {}

impl Drop for MpInt {
    fn drop(&mut self) {
        unsafe { __gmpz_clear(self) };
    }
}
