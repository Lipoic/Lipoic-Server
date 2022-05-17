FROM rust:1.60.0

WORKDIR /usr/src/myapp
COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./Rocket.toml ./Rocket.toml

RUN cargo build --release

ENTRYPOINT ["./target/release/lipoic_server"]