FROM rust as builder

WORKDIR /usr/src/pp
COPY . .

RUN cargo build --release


FROM debian

WORKDIR /bin/pp

COPY .env .
COPY --from=builder /usr/src/pp/target/release/ping_pong .

ENTRYPOINT ["./ping_pong"]
