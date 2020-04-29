FROM rust as builder

WORKDIR /usr/src/pp
COPY . .

RUN cargo build --release


FROM debian

WORKDIR /bin/pp
COPY --from=builder /usr/src/pp/target/release/ping_pong .

CMD ["./ping_pong"]
