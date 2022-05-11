FROM rust:1.60.0

WORKDIR /usr/src/myapp
COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN mv ./target/release/lipoic_server ./lipoic_server
RUN rm -rf ./src
RUN rm -rf ./target

EXPOSE 8000
ENTRYPOINT ["./lipoic_server"]