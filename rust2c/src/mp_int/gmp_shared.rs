#![allow(non_camel_case_types)]

use std::ffi::CStr;
use std::fmt;
use std::ptr::null_mut;

use libc::{c_char, c_int, c_ulong, c_void, size_t};

type mpiptr = *mut MpInt;
type const_mpiptr = *const MpInt;

#[link(name = "gmp")]
extern "C" {
    pub(super) static __gmp_version: *const c_char;
    pub(super) fn __gmpz_clear(n: mpiptr);
    pub(super) fn __gmpz_cmp_ui(a: const_mpiptr, b: c_ulong) -> c_int;
    pub(super) fn __gmpz_get_str(s: *mut c_char, base: c_int, src: const_mpiptr) -> *mut c_char;
    pub(super) fn __gmpz_init(dst: mpiptr);
    pub(super) fn __gmpz_init_set(dst: mpiptr, src: const_mpiptr);
    pub(super) fn __gmpz_init_set_ui(dst: mpiptr, src: c_ulong);
    pub(super) fn __gmpz_mul(dst: mpiptr, fact1: const_mpiptr, fact2: const_mpiptr);
    pub(super) fn __gmpz_sizeinbase(src: const_mpiptr, base: c_int) -> size_t;
    pub(super) fn __gmpz_sub_ui(diff: mpiptr, min: const_mpiptr, sub: c_ulong);
}

#[repr(C)]
#[derive(Debug)]
pub struct MpInt {
    _alloc: i32,
    _size: i32,
    _d: *mut c_void,
}

impl MpInt {
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

    pub fn gmp_version() -> String {
        let c_s = unsafe { CStr::from_ptr(__gmp_version) };
        let s = c_s.to_str().expect("failed to get gmp version");
        s.to_string()
    }

    fn raw() -> Self {
        Self {
            _alloc: 0,
            _size: 0,
            _d: null_mut::<c_void>(),
        }
    }
}

impl fmt::Display for MpInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let buf_size = __gmpz_sizeinbase(self, 10);
            let mut buf = vec![0u8; buf_size + 2];
            __gmpz_get_str(buf.as_mut_ptr(), 10, self);
            write!(f, "{}", String::from_utf8(buf).expect("encoding error"))
        }
    }
}

impl Drop for MpInt {
    fn drop(&mut self) {
        unsafe { __gmpz_clear(self) };
    }
}

impl From<u64> for MpInt {
    fn from(ui: u64) -> Self {
        let mut n = Self::raw();
        unsafe { __gmpz_init_set_ui(&mut n, ui) };
        n
    }
}

impl From<&MpInt> for MpInt {
    fn from(src: &Self) -> Self {
        let mut dst = Self::raw();
        unsafe { __gmpz_init_set(&mut dst, src) };
        dst
    }
}
