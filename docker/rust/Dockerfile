FROM rust:alpine
RUN apk add --no-cache musl-dev pkgconfig openssl-dev
RUN cargo install cargo-watch
WORKDIR /workspace
