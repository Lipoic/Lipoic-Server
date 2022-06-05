FROM rust:1.61.0

WORKDIR /usr/lipoic-backend
COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./Rocket.toml .

RUN cargo build --release

RUN cp /usr/lipoic-backend/target/release/lipoic_server /usr/local/bin/lipoic_server

ENTRYPOINT ["lipoic_server"]