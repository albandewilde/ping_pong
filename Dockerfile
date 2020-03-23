FROM rust

WORKDIR /usr/src/pp
COPY . .

RUN cargo build --release

WORKDIR /usr/src/pp/target/release
CMD ["./ping_pong"]
