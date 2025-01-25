https://github.com/cross-rs/wiki_assets

https://github.com/cross-rs/cross/wiki

```shell

cross build --target aarch64-unknown-linux-gnu
cross build --target x86_64-unknown-linux-gnu
cross build --target x86_64-unknown-linux-musl
cross build --target x86_64-pc-windows-gnu



// Error
cargo build --target=x86_64-unknown-linux-musl 

// Error
rustup target add x86_64-unknown-linux-musl

rustup toolchain install stable-x86_64-unknown-linux-gnu



cargo build --target=x86_64-unknown-linux-gnu

rustup target add x86_64-unknown-linux-musl
cargo build --target=x86_64-unknown-linux-musl

rustup toolchain install stable-x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu
cargo build --target=x86_64-unknown-linux-gnu


rustup target add x86_64-pc-windows-gnu
cargo build --target=x86_64-pc-windows-gnu

cross build --target x86_64-pc-windows-gnu

cross build --target x86_64-unknown-linux-gnu


cross build --target x86_64-pc-windows-gnu --release

CROSS_DEBUG=1 QEMU_STRACE=1 CROSS_CONTAINER_ENGINE_NO_BUILDKIT=1 cross build --target x86_64-pc-windows-gnu --release

cross component add rustfmt
```