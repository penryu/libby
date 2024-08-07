#![allow(non_camel_case_types)]

use std::ffi::CStr;
use std::fmt;
use std::ptr::null_mut;

use dlopen2::wrapper::{Container, WrapperApi};
use libc::{c_char, c_int, c_ulong, c_void, size_t};

#[derive(WrapperApi)]
struct GmpApi {
    // HACK: This is just a C string with type `const char * const`, but I wasn't able to
    // convince the compiler to let me use pointers, since they aren't Sync-safe.
    __gmp_version: &'static &'static u8,
    __gmpz_clear: fn(n: &mut MpInt),
    __gmpz_cmp_ui: fn(a: &MpInt, b: c_ulong) -> c_int,
    __gmpz_get_str: fn(s: *mut c_char, base: c_int, src: &MpInt) -> *mut c_char,
    __gmpz_init: fn(dst: &mut MpInt),
    __gmpz_init_set: fn(dst: &mut MpInt, src: &MpInt),
    __gmpz_init_set_ui: fn(dst: &mut MpInt, src: c_ulong),
    __gmpz_mul: fn(dst: &mut MpInt, fact1: &MpInt, fact2: &MpInt),
    __gmpz_set: fn(dst: &mut MpInt, src: &MpInt),
    __gmpz_sizeinbase: fn(src: &MpInt, base: c_int) -> size_t,
    __gmpz_sub_ui: fn(diff: &mut MpInt, min: &MpInt, sub: c_ulong),
    __gmpz_swap: fn(a: &mut MpInt, b: &mut MpInt),
}

lazy_static::lazy_static! {
    static ref GMP: Container<GmpApi> = unsafe {
        Container::load("libgmp.so").expect("failed to load gmp library")
    };
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
        let mut n = MpInt::from(self);
        let mut result = MpInt::from(1);

        if GMP.__gmpz_cmp_ui(&n, 0) == 0 {
            return result;
        }

        // HACK: `dlopen2` lets us use Rust syntax and semantics... but the borrow checker
        // directly interferes with gmp's intended use case here. To work around this,
        // we assign the result to a temporary mpz, then swap them back.
        let mut tmp = MpInt::from(0);
        while GMP.__gmpz_cmp_ui(&n, 1) > 0 {
            GMP.__gmpz_mul(&mut tmp, &result, &n);
            GMP.__gmpz_swap(&mut result, &mut tmp);
            GMP.__gmpz_sub_ui(&mut tmp, &n, 1);
            GMP.__gmpz_swap(&mut n, &mut tmp);
        }

        result
    }

    pub fn gmp_version() -> String {
        let c_s = unsafe { CStr::from_ptr(*GMP.__gmp_version) };
        let s = c_s.to_string_lossy();
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
        let buf_size = GMP.__gmpz_sizeinbase(self, 10);
        let mut buf = vec![0u8; buf_size + 2];
        GMP.__gmpz_get_str(buf.as_mut_ptr(), 10, self);
        write!(f, "{}", String::from_utf8(buf).expect("encoding error"))
    }
}

impl Drop for MpInt {
    fn drop(&mut self) {
        GMP.__gmpz_clear(self);
    }
}

impl From<u64> for MpInt {
    fn from(ui: u64) -> Self {
        let mut n = Self::raw();
        GMP.__gmpz_init_set_ui(&mut n, ui);
        n
    }
}

impl From<&MpInt> for MpInt {
    fn from(src: &Self) -> Self {
        let mut dst = Self::raw();
        GMP.__gmpz_init_set(&mut dst, src);
        dst
    }
}
