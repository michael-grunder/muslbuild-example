FROM ekidd/rust-musl-builder:nightly AS builder
COPY . ./
RUN sudo apt-get update && sudo apt-get install -y libclang-3.9 clang-3.9 llvm-3.9 \
    curl musl-dev musl-tools make python g++ && sudo chown -R rust:rust /home/rust
RUN LIB_LDFLAGS=-L/usr/lib/x86_64-linux-gnu CFLAGS=-I/usr/local/musl/include CC=musl-gcc cargo build --release
