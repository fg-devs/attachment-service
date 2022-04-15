# Attachment Service
An simple microservice for managing attachments

## Configuration
Before running anything we need to make an `.env` file, 
this can be done by copying `.env.example` and filling in the values

#### Environment variables
`GRPC_PORT`: The port that the gRPC server will use

`HTTP_PORT`: The port that the HTTP server will use

`CDN_URI`: The base URI that will be used for returning url's

`RUST_LOG`: Your log level, more info can be found [here](https://docs.rs/env_logger/0.9.0/env_logger/#enabling-logging)

## Running

You first need to init submodules with 
```bash
git submodule update --init --recursive
```

### Docker
The `docker-compose.yml` is as follows, note that this differs from the `docker-compose.yml` in the repository.
```yaml
version: "3.9"
services:
  attachment-service:
    image: ghcr.io/fg-devs/attachment-service:latest
    restart: always
    env_file:
      - .env
    ports:
      - ${GRPC_PORT}:${GRPC_PORT}
      - ${HTTP_PORT}:${HTTP_PORT}
    volumes:
      - attachment-files:/app/files
    networks:
      - attachments

networks:
  attachments:
```

### Native
Running natively is super simple, just use `cargo run --release`

