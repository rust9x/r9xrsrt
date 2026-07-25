# r9xrsrt: rust9x rust runtime

Absolutely minimal runtime for rust9x. Explicitly _no_ C runtime, just the bare minimum to get a
pure Rust program running.

Actually, it's less than the bare minimum: Floating point support is not included (yet?), so none of
the expected intrincics are available.

## Building

```sh
cargo +rust9x build --release -Zbuild-std=core
```

## Using

`.cargo/config.toml`:

```toml
[target.'cfg(all(target_family = "rust9x", target_env = "msvc"))']
rustflags = [
  '-Ctarget-feature=+crt-static', # link our libcmt.lib
  '-Clink-arg=/LIBPATH:/home/seri/p/rust/rust9x/r9xrsrt/target/i586-rust9x-windows-msvc/release',
  '-Clink-arg=kernel32.lib',
  # any platform sdk with kernel32.lib and ws2_32.lib will do
  '-Clink-arg=/LIBPATH:/home/seri/mnt/ext4/msvc-toolchains/vc8/VC/PlatformSDK/Lib',
]
```

- Unwinding is not supported, use `panic = "abort"` in your `Cargo.toml`.
- Even if unwinding is disabled, you also need to disable it for the standard library (otherwise
  it'll still ask for symbols like `__CxxFrameHandler3`)
- `memcpy`, `memmove`, `memset` are the simplest possible implementations. Use
  `-Zbuild-std-features=compiler-builtins-mem` to use optimized compiler-builtins instead.
- If you don't need backtraces, don't specify the backtrace feature, saves 5-50KiB.

Example build command:

```sh
cargo +rust9x build --target i586-rust9x-windows-msvc -Zbuild-std=std,panic_abort -Zbuild-std-features=backtrace,compiler-builtins-mem --release
```
