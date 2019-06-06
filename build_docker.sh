#!/bin/bash

set -e

# build the musl-cross Docker image
docker build -f docker/Dockerfile.musl -t musl-cross .

# build rust musl with protobuf Docker image
docker build -f docker/Dockerfile.rust-protoc -t rust-musl-protoc .

# build a docker image with the cargo project and grpc-rs as dependency
docker build -f docker/Dockerfile -t grpc-rs-musl .
