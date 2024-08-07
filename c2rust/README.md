# c2rust - Calling Rust functions from C

Cargo projects:
- [roll](./src/lib.rs) - a toy library that provides a dice roll function: `roll`

C source:
- [c2rust.c](./c2rust.c) - C front-end to demo the `roll()` function

This is a C binary project, linking against a Rust lib in three different ways:

- by linking against a static `.a` library
- by linking against a `.so` / `.dylib` shared library
- by using `dlopen()` / `dlsym()` to load the shared library from the running process

## Quick Start

This build is driven entirely from the `GNUmakefile` file. Because of this, you will need require
GNU `make` to be installed.

GNU make is the default `make` on both macOS and Linux. If you're on FreeBSD, you may need to
install GNU `make` and use `gmake` in the examples below.

```sh
c2rust $ make clean all
rm -rf c2rust-static c2rust-shared c2rust-dynamic *.dSYM

cargo clean
    ... ...

cargo build --lib
    ... ...
   Compiling rand v0.8.5
   Compiling roll v0.2.0 (libby/c2rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.49s

ln -sf "libroll.a" "/.../libby/c2rust/target/debug/libroll-static.a"

LIBRARY_PATH="/.../libby/c2rust/target/debug" \
  cc -g -O0 -Wall -Werror -Wconversion -fno-builtin c2rust.c -o c2rust-static  -l roll-static

LIBRARY_PATH="/.../libby/c2rust/target/debug" \
  cc -g -O0 -Wall -Werror -Wconversion -fno-builtin c2rust.c -o c2rust-shared  -Wl,-rpath,.../libby/c2rust/target/debug -l roll

cc -g -O0 -Wall -Werror -Wconversion -fno-builtin c2rust.c -o c2rust-dynamic  -DDYLIB_PATH="/.../libby/c2rust/target/debug/libroll.so"

./c2rust-static 3 8
3d8 => 17
./c2rust-shared 3 8
3d8 => 16
./c2rust-dynamic 3 8
3d8 => 14
```

These are the high level stages in the above session. We'll go into more detail below.

1. Clean up the previous build.
2. Build `libroll`
3. Build the binaries: `c2rust-dynamic`, `c2rust-shared`, `c2rust-static`
4. Run the binaries

## A Closer Look

### Cleanup

If your checkout gets to an inconsistent state, try `make distclean` for a deeper clean.

You can also try `git clean -dxn` to find out what other files might have crept in.

### Build `libroll`

The `libroll` build products include: `libroll.a` and either `libroll.so` (Linux, FreeBSD) or
`libroll.dylib` (macOS).

`libroll.a` is the object file for static linking.

`libroll.so` and `libroll.dylib` are shared object files for linking at launch or runtime,
respectively.

### Build the binaries

To better contrast the different complexities of the linking styles, all three binaries are built
from the same [Makefile](./GNUmakefile) and [source file](./c2rust.c). Take a look at these to see
the different design tradeoffs.

### Run the binaries

Each of the above mentioned binaries are invoked with the same arguments (the contents of the `ARGS`
Makefile macro). The output will be different simply because they are each using an PRNG with a
different seed value.

I went to some effort to hardcode the paths to any shared libraries in the binary, so that you won't
need to mess with `LD_LIBRARY_PATH`s at runtime.

## Going further

Some useful utilities to inspect the differences between these binaries:

```sh
# on Linux / FreeBSD
$ ldd c2rust-*
c2rust-dynamic:
    linux-vdso.so.1 (0x0000ffff05e9c000)
    libc.so.6 => /lib64/libc.so.6 (0x0000ffff05c50000)
    /lib/ld-linux-aarch64.so.1 (0x0000ffff05e54000)
c2rust-shared:
    linux-vdso.so.1 (0x0000ffff30e10000)
    libroll.so => /.../libby/c2rust/target/debug/libroll.so (0x0000ffff30d30000)
    libc.so.6 => /lib64/libc.so.6 (0x0000ffff30b30000)
    libgcc_s.so.1 => /lib64/libgcc_s.so.1 (0x0000ffff30ad0000)
    /lib/ld-linux-aarch64.so.1 (0x0000ffff30dc8000)
c2rust-static:
    linux-vdso.so.1 (0x0000ffff843c0000)
    libgcc_s.so.1 => /lib64/libgcc_s.so.1 (0x0000ffff84300000)
    libc.so.6 => /lib64/libc.so.6 (0x0000ffff84120000)
    /lib/ld-linux-aarch64.so.1 (0x0000ffff84378000)

# on macOS, this is roughly equivalent
$ otool -L c2rust-{static,shared,dynamic}
c2rust-static:
    /usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1345.100.2)
c2rust-shared:
    /.../libby/c2rust/target/debug/deps/libroll.dylib (compatibility version 0.0.0, current version 0.0.0)
    /usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1345.100.2)
c2rust-dynamic:
    /usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1345.100.2)

```

`ldd` tells you which shared libraries a binary uses by examining the binary's import table,
and attempts to find and load these dependencies.


### `c2rust-static`

```sh
$ ldd c2rust-static
    linux-vdso.so.1 (0x0000fffed79c4000)
    libgcc_s.so.1 => /lib64/libgcc_s.so.1 (0x0000fffed7900000)
    libc.so.6 => /lib64/libc.so.6 (0x0000fffed7720000)
    /lib/ld-linux-aarch64.so.1 (0x0000fffed797c000)
```

Here we have several shared libraries listed, but not `libroll`. We made sure to include the static
library _in_ the `c2rust-static` binary, saving the need to find and load it at linktime or runtime.


### `c2rust-shared`

```sh
$ ldd c2rust-shared
    linux-vdso.so.1 (0x0000ffff43458000)
    libroll.so => /.../libby/c2rust/target/debug/libroll.so (0x0000ffff43370000)
    libc.so.6 => /lib64/libc.so.6 (0x0000ffff43170000)
    libgcc_s.so.1 => /lib64/libgcc_s.so.1 (0x0000ffff43110000)
    /lib/ld-linux-aarch64.so.1 (0x0000ffff43410000)
```

Here we see `libroll.so` listed alongside those other libraries. Also note the full, absolute path
to `libroll.so`.

So what happens if you ran `cargo clean` on the roll project and tried to run `c2rust-shared` again?

```sh
$ cargo clean
     Removed 159 files, 106.7MiB total

$ ./c2rust-shared 2 6
./c2rust-shared: error while loading shared libraries: libroll.so: cannot open shared object file: No such file or directory

$ cargo build
    ... ...
   Compiling rand v0.8.5
   Compiling roll v0.2.0 (/.../libby/c2rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.44s

$ ./c2rust-shared 2 6
2d6 => 6
```


### `c2rust-dynamic`

```sh
$ ldd c2rust-dynamic
    linux-vdso.so.1 (0x0000ffff6f0c8000)
    libc.so.6 => /lib64/libc.so.6 (0x0000ffff6ee80000)
    /lib/ld-linux-aarch64.so.1 (0x0000ffff6f080000)
```

Wait, you probably weren't expecting to see `libroll.so`, but we're also missing `libgcc_s.so`. It
was mentioned in `c2rust-static` *and* `c2rust-shared`, so where did it go?

This is because `libgcc_s.so` is a dependency of `libroll.so`, and is only a _transitive_ dependency
of `c2demo`. But in the case of `c2demo-static` and `c2rust-shared`, the linker knows that it *WILL*
need `libgcc_s.so`, so goes ahead and adds it to the ELF data.

In the dynamic method, the linker doesn't know anything about `libroll.so` until we call `dlopen()`.
Only *then* does it find the `libgcc_s.so` dependency, find it, and load it.

> "But wait, you said `ldd` lists the libraries the binary depends on!"

True. However, `ldd` only examines the libraries listed in the header, and invokes the system linker
to try to resolve them. In order to discover `c2demo-dynamic`'s dependency on `libroll.so`, `ldd` it
would need to _run_ it.[*]

[*] In some cases, `ldd` _might_ run the binary. This can be a security concern. Don't run `ldd(1)`
on untrusted binaries, and check the `ldd(1)` manpage for more info.

