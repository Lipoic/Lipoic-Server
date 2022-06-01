FROM rust:1.61.0

WORKDIR /usr/lipoic-backend
COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN cargo build --release

RUN cp /usr/lipoic-backend/target/release/lipoic_server /usr/local/bin/lipoic_server
COPY ./Rocket.toml .

ENTRYPOINT ["lipoic_server"]