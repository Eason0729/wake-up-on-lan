FROM rust:1.62.1 AS builder

WORKDIR /complier/wake-up-on-lan

RUN apt update -y
RUN apt install musl-tools -y

# TARGET=x86_64-unknown-linux-musl
# TARGET=aarch64-unknown-linux-musl
ARG TARGET

RUN rustup target add ${TARGET}

COPY . .

RUN cargo install --target ${TARGET} --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/wake-up-on-lan .
CMD ["./wake-up-on-lan"]