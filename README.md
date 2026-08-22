# r9xrsrt: rust9x rust runtime

Absolutely minimal runtime for rust9x. Explicitly _no_ C runtime, just the bare minimum to get a
pure Rust program running.

Actually, it's less than the bare minimum: Floating point support is not included (yet?), so none of
the expected intrincics are available.

## Building

```sh
cargo +rust9x build --release
```

## Using

`.cargo/config.toml`:

```toml
[target.'cfg(all(target_family = "rust9x", target_env = "msvc"))']
rustflags = [
  '-Ctarget-feature=+crt-static', # link our libcmt.lib
  '-Clink-arg=/LIBPATH:/home/seri/p/rust/rust9x/r9xrsrt/target/i586-rust9x-windows-msvc/release',
  # any platform sdk with kernel32.lib and ws2_32.lib will do
  '-Clink-arg=/LIBPATH:/home/seri/mnt/ext4/msvc-toolchains/vc8/VC/PlatformSDK/Lib',
]
```

- Unwinding is not supported, use `panic = "abort"` in your `Cargo.toml`.
  - Even if unwinding is disabled, you also need to disable it for the standard library (otherwise
    it'll still ask for symbols like `__CxxFrameHandler3`) by using `-Zbuild-std=panic_abort` in the
    build command.
- `memcpy`, `memmove`, `memset` are the simplest possible implementations. Use
  `-Zbuild-std-features=compiler-builtins-mem` to use optimized compiler-builtins instead.
- If you don't need backtraces, don't specify the backtrace feature, saves 5-50KiB.
- You'll need to always build at `opt-level = 1` or higher so that dead code elimination can remove
  unused/unimplemented functions. Otherwise you'll get link errors for missing symbols like `_fma`,
  even if you don't use them.

Example build command:

```sh
cargo +rust9x build --target i586-rust9x-windows-msvc -Zbuild-std=std,panic_abort -Zbuild-std-features=backtrace,compiler-builtins-mem --release
```

Alternatively you can add this to your `.cargo/config.toml`:

```toml
[unstable]
build-std = ["std", "panic_abort"]
build-std-features = ["compiler-builtins-mem", "optimize_for_size"]
```
