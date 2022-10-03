# rust_plugin

An example of how to write a Rust plugin for Flutter.

## Getting Started

**Note:** This example only works for Android currently, not yet iOS.  It's a
work-in-progress.

This example requires [`cargo-ndk`](https://crates.io/crates/cargo-ndk), which
itself requires `rustup` and the toolchains for Android to be installed.

```
cargo install cargo-ndk
rustup target add \
    aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android
```
