# rust2c

Cargo projects:
- [rust2c](./src/lib.rs) - a Rust cargo project that demonstrates two methods to link against an
    `extern "C"` library

This demo uses the [`dlopen2`](https://github.com/OpenByteDev/dlopen2) crate to dynamically load the
`libroll.so` library at runtime.

## `src/main.rs`

main is a very short, straightforward driver for the demo. All it does is import a type, `MpInt`
(for "[GNU] Multiple Precision Integer") and exercise it.

`MpInt` is a Rust struct and associated methods and traits that bind and wrap the `gmp` library that
is most likely already on your system. If not, you may need to install the `gmp` package and
possibly the `gmp-dev` or `gmp-devel` packages.

The only magic is in the file `src/mp_int/mod.rs`. It uses the `cfg_if` macro to import the correct
`MpInt` type based on the feature flags.

To try the shared (launch-time link), use:

```sh
$ cargo run --features shared 50 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rust2c 50`
rust2c 0.2.0 using gmp 6.2.1

50! -> 30414093201713378043612608166064768844377641568960512000000000000
```

For the dynamic (`dlopen`) implementation, use:

```sh
$ cargo run --features dynamic 50
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rust2c 50`
rust2c 0.2.0 using gmp 6.2.1

50! -> 30414093201713378043612608166064768844377641568960512000000000000
```


## Implementations

Now that you've seen both implementations work and (hopefully) yield the same results, check out the
code:

- [shared](./src/mp_int/gmp_shared.md)
- [dynamic](./src/mp_int/gmp_dynamic.md)

The shared implementation involves more use of `unsafe` blocks, as well as the use of raw pointer
types, where the dynamic implementation uses far less `unsafe`, and allows the use of Rust-style
syntax for managing calls to the underlying library functions.

However, look at lines 51-60 in the [dynamic](./src/mp_int/gmp_dynamic.rs) source. The use of Rust
syntax requires compliance with the Rust borrow checker, preventing the passing of the same `mpz*`
variable as 2 arguments to the same function, a call that works perfectly well by design in lines
45-46 of [shared](./src/mp_int/gmp_shared.md).

This is half of my preference of the shared method: you are still permitted (within the `unsafe`
blocks we knew we would be dealing with) to use C/C++ logic to interface with the underlying C/C++
library functions.

The other half of my preference for standard shared library linking is that you get the validation
of library linkage at both compile time and launch time.

Compared to the dlopen method, which leaves all library validation to late in the program execution.
