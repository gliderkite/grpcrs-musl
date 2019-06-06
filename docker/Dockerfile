FROM rust-musl-protoc

COPY scripts/musl-linker.sh .
WORKDIR /tmp
ADD crate .
COPY scripts/build.sh .
RUN ./build.sh
