FROM rust:1.60.0

WORKDIR /usr/src/myapp
COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./Rocket.toml ./Rocket.toml

RUN cargo build --release
RUN openssl genrsa -out privkey.pem 2048
RUN openssl rsa -in privkey.pem -pubout > publickey.pub
RUN export ROCKET_PRIVATE_KEY=`cat privkey.pem`
RUN export ROCKET_PUBLIC_KEY=`cat publickey.pub`

ENV ROCKET_PRIVATE_KEY=${ROCKET_PRIVATE_KEY}
ENV ROCKET_PUBLIC_KEY=${ROCKET_PUBLIC_KEY}

ENTRYPOINT ["./target/release/lipoic_server"]