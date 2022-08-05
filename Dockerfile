FROM rust:1.62.1 AS builder

WORKDIR /complier/wake-up-on-lan

# build for musl
# TARGET=x86_64-unknown-linux-musl
ARG TARGET

RUN apt update -y
RUN apt install musl-tools -y

RUN rustup target add ${TARGET}

COPY . .

RUN cargo install --target ${TARGET} --path .

FROM alpine
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/wake-up-on-lan .
COPY . .
CMD ["./wake-up-on-lan"]