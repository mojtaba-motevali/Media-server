FROM rust:1.72-slim-buster AS build

RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev pkg-config

WORKDIR /build
RUN cargo new --bin server

WORKDIR /build/server

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm -rf src target/release/deps/server*
COPY src ./src
RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=build /usr/local/cargo/bin/server ./server
COPY app_settings.json ./

CMD ["./server"]
