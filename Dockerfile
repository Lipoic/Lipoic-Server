FROM rust:1.60.0 AS builder

WORKDIR /usr/lipoic-backend
COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN cargo build --release

FROM debian:sid-slim

WORKDIR /root/

COPY --from=builder /usr/lipoic-backend/target/release/lipoic_server .
COPY ./Rocket.toml .

ENTRYPOINT ["./lipoic_server"]