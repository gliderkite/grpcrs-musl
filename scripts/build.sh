export RUST_BACKTRACE=full
export TOOLCHAIN_ROOT="${TOOLCHAIN_ROOT:-/opt/cross/x86_64-linux-musl}"
echo "TOOLCHAIN_ROOT: $TOOLCHAIN_ROOT"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=/musl-linker.sh
export TARGET_CC=$TOOLCHAIN_ROOT/bin/x86_64-musl-linux-gcc
export TARGET_AR=$TOOLCHAIN_ROOT/bin/x86_64-linux-musl-ar
export HOST_CC=cc
export PKG_CONFIG_ALLOW_CROSS=1
export RUSTFLAGS="-C link-arg=-v -C linker=$CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER -C ar=$TARGET_AR"
export PATH=/opt/cross/x86_64-linux-musl/bin:$PATH
cargo build --target=x86_64-unknown-linux-musl
