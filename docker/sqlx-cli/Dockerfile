FROM rust:alpine
RUN apk add --no-cache musl-dev pkgconfig openssl-dev
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres
WORKDIR /workspace
