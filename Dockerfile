FROM rust:alpine AS builder

RUN apk add cmake musl-dev build-base

WORKDIR /src

COPY . .
RUN cargo build --release

FROM alpine:latest AS runner

WORKDIR /app

COPY --from=builder /src/target/release/attachment-service /usr/local/bin/attachment-service

CMD ["attachment-service"]
