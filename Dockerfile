FROM rust:slim-bullseye AS builder

RUN apt-get update; apt-get upgrade -y
RUN apt-get install -y cmake build-essential libssl-dev pkg-config

WORKDIR /src

# https://stackoverflow.com/questions/58473606/cache-rust-dependencies-with-docker-build
RUN echo "fn main() {}" > dummy.rs
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim AS runner

RUN apt-get update; apt-get upgrade -y
RUN apt-get install -y cmake build-essential libssl-dev pkg-config

WORKDIR /app

COPY --from=builder /src/target/release/attachment-service /usr/local/bin/attachment-service
COPY --from=builder /src/.env /app/.env

CMD ["attachment-service"]
