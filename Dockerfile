FROM rust:1.60.0 AS builder
WORKDIR /complier

# build for musl
# TARGET=x86_64-unknown-linux-musl
ARG TARGET

RUN USER=root cargo new wake-up-on-lan
WORKDIR /complier/wake-up-on-lan
COPY . .

RUN cargo install --target ${TARGET} --path .

FROM alpine
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/wake-up-on-lan .
COPY ./static ./static
CMD ["./wake-up-on-lan"]