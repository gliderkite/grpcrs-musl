# grpc-rs on musl

This project contains a set of Docker files used to build an environment where it
should be possible to build (and link) a Rust project which depends on
[grpc-rs](https://github.com/pingcap/grpc-rs) for musl.


## Project structure

There are 3 mains Dockerfiles in the `docker` directory:

- `Dockerfile.musl`: docker image based on `debian:stretch-slim` that builds a
    musl libc cross-compiler into `/opt/cross/<arch>-linux-musl` and install
    basic required dependencies.

- `Dockerfile.rust-protoc`: docker image based on the above one that installs the
    latest version of Rust from the stable channel, as well as `protoc` (compiled
    with the musl compiler) and `golang` (required by `grpc-rs`).

- `Dockerfile`: docker image based on the above one that copies the cargo project
    from the `crate` directory in the image, as well as the linker and build
    scripts adapted for musl (according to https://github.com/rust-lang/rust/issues/36710),
    and then build the crate.


## Hot to test

You can run `./build_docker.sh` to build the 3 images.


## Issues

The proposed solution does not allow to build the cargo project due to linking
issues:

```
/opt/cross/x86_64-linux-musl/lib/gcc/x86_64-linux-musl/5.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/target/x86_64-unknown-linux-musl/debug/deps/libgrpcio_sys-491b4cf86ab89274.rlib(log_linux.cc.o): unrecognized relocation (0x2a) in section `.text._Z15gpr_default_logP17gpr_log_func_args'
/opt/cross/x86_64-linux-musl/lib/gcc/x86_64-linux-musl/5.3.0/../../../../x86_64-linux-musl/bin/ld: final link failed: Bad value
collect2: error: ld returned 1 exit status
```