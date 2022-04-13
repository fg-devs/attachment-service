FROM rust:1.60.0-alpine AS builder

RUN apk add cmake musl-dev build-base

WORKDIR /src
COPY . .

RUN cargo install --path .

FROM alpine:latest AS runner

COPY --from=builder /usr/local/cargo/bin/attachment-service /usr/local/bin/attachment-service

CMD ["attachment-service"]
