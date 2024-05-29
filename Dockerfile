FROM rust:1.78.0-alpine AS builder

WORKDIR /app

COPY . .

RUN cargo update

RUN cargo install --path .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/baridge .

EXPOSE 9

CMD ["/app/baridge"]