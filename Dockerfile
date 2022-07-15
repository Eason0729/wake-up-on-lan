FROM rust:1.60.0 AS builder
WORKDIR /complier

ARG TARGET

RUN USER=root cargo new wake-up-on-lan
WORKDIR /complier/wake-up-on-lan
COPY . .

RUN cargo install --target ${TARGET} --path .

FROM scratch
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/wake-up-on-lan .
COPY ./static ./static
CMD ["./wake-up-on-lan"]