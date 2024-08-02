# libby

A set Rust and C programs demonstrating the various ways to call across the "C" binary interface.

# Rust calling Rust

Cargo projects:
- [roll](./roll/src/lib.rs) - a toy library that provides a dice roll function: `roll`
- [rustroll](./rustroll/src/main.rs) - a Rust front-end for `libroll`

This is a native Rust process, linking against a neighboring Rust lib project by adding it to the
`Cargo.toml` dependencies.

```sh
% cargo run --release --bin rustroll -- 3 8
    ... ...
   Compiling roll v0.1.0 (.../libby/roll)
   Compiling rustroll v0.1.0 (.../libby/rustroll)
    Finished `release` profile [optimized] target(s) in 4.09s
     Running `.../libby/target/release/rustroll 3 8`
3d8 => 10
```

# C calling Rust

The following examples all come from the same source:

Cargo projects:
- [roll](./roll/src/lib.rs) - a toy library that provides a dice roll function: `roll`

C source:
- [roll.h](./roll.h) - C header declaring the `roll()` function
- [demo.c](./demo.c) - C front-end to demo the `roll()` function

*NB* `#ifdef` is used in `demo.c` to build the `dlopen()` code only for the dynamic demo.

*NB* `LD_LIBRARY_PATH` is set so `dlopen` can find the local build of the library at runtime. If the
library is installed on the system, this won't be necessary.


## *static* - C statically linked against Rust library

Builds the demo with `demo` binary statically linked against `libroll.a`. Note that we direct the
compiler directly to the library.

```sh
% make static
# Builds Rust lib
cargo build --lib
    ... ...
   Compiling rand v0.8.5
   Compiling roll v0.1.0 (.../libby/roll)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.04s
# Build C front-end, linking against static library `libroll.a`
cc -g -Wall -Wconversion -fno-builtin demo.c -o demo-static ./target/debug/libroll.a
# Launch demo
./demo-static 3 8
3d8 => 23
```

## *shared* - C linked against shared Rust library

Builds the demo with `demo` binary dynamically linked against `libroll.so`, and automatically
loading the library at launch time.

Note that we use `LD_LIBRARY_PATH` to tell the linker where to find our local `libroll.so`.

```sh
% make shared
# Build Rust lib
cargo build --lib
    ... ...
   Compiling rand v0.8.5
   Compiling roll v0.1.0 (.../libby/roll)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.04s
# Build C front-end, linking against shared library `libroll.so`
cc -g -Wall -Wconversion -fno-builtin demo.c -o demo-shared -L ./target/debug -l roll
# Launch demo, setting `LD_LIBRARY_PATH` so the library is found at launch time
LD_LIBRARY_PATH="./target/debug" ./demo-shared 3 8
3d8 => 16
```

## *dylib* - C dynamically linked against shared Rust library at runtime

Uses `dlopen(3)` to load the shared library `libroll.so` at runtime. The linker knows nothing about
the library at compile time _or_ at launch time. Look at `demo.c` to see how we lookup `libroll.so`
_at runtime_.

```sh
% make shared
# Build Rust lib
cargo build --lib
    ... ...
   Compiling rand v0.8.5
   Compiling roll v0.1.0 (.../libby/roll)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.04s
# Build C front-end, linking against shared library `libroll.so`
cc -g -Wall -Werror -Wconversion -fno-builtin demo.c -o demo-dynamic -DDYNAMIC_ROLL
# Launch demo, setting `LD_LIBRARY_PATH` so the library is found at runtime
LD_LIBRARY_PATH="./target/debug" ./demo-dynamic 3 8
3d8 => 15
```
