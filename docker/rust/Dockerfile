FROM rust:alpine
RUN apk add --no-cache musl-dev pkgconfig openssl-dev
ENV RUSTFLAGS='-C target-feature=-crt-static'
RUN cargo install cargo-watch
RUN cargo install cargo-llvm-cov
WORKDIR /workspace
