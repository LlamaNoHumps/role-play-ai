FROM rust:1.90.0-alpine3.22 AS builder

RUN apk add --update --no-cache build-base libressl-dev

COPY ./back-end/src /app/src
COPY ./back-end/Cargo.lock /app/Cargo.lock
COPY ./back-end/Cargo.toml /app/Cargo.toml

WORKDIR /app

RUN cargo build --release

FROM scratch

COPY --from=builder /app/target/release/back-end /
COPY --from=builder /etc/ssl/cert.pem /etc/ssl/
COPY ./back-end/index.html /index.html

ENV RUST_BACKTRACE=1

ENTRYPOINT [ "/back-end" ]