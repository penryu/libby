# rust2c

Cargo projects:
- [rust2c](./src/main.rs) - a Rust cargo project that demonstrates two methods to link against an
    `extern "C"` library

This demo uses the [`dlopen2`](https://github.com/OpenByteDev/dlopen2) crate to dynamically load the
`libroll.so` library at runtime.

## `src/main.rs`

main is a very short, straightforward driver for the demo. All it does is import a type, `MpInt`
(for "[GNU] Multiple Precision Integer") and exercise it.

`MpInt` is a Rust struct and associated methods and traits that bind and wrap the `gmp` library that
is quite possibly already on your system. If not, you may need to install the `gmp` package and
possibly the `gmp-dev` or `gmp-devel` packages.

The only magic is in the file [`src/main.rs`](src/main.rs). It uses the `cfg_if` macro to import the
correct `MpInt` type based on feature flags.

To try both the `shared` (launch-time link) and the `dlopen` (runtime-load) implementations:

```sh
$ make run
```


## Implementations

Now that you've seen both implementations work and (hopefully) yield the same results, check out the
code:

- [shared](./src/mp_shared.rs)
- [dlopen](./src/mp_dlopen.rs)

The `shared` implementation involves more use of `unsafe` blocks, 
where the `dlopen` implementation uses far less `unsafe`, and allows the use of Rust-style
syntax for managing calls to the underlying library functions.

My preference for standard `shared` library linking is that you get the validation of knowing the
library contains the necessary symbols at compile time, which seems very much in the spirit of Rust.
By contrast, `dlopen` method leaves all library validation to the coder, and all mistakes until
runtime, after the Rust program has already started.

In addition to the runtime punting, you can also find several hacks I used to keep the `dlopen`
approach well-behaved in the [dlopen implementation](src/mp_dlopen.rs).
