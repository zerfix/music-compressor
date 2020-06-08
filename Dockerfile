# --- builder -----------------------------------------------------------------
From rust:latest as builder

WORKDIR /usr/src/music-compressor

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo install --path .

# --- runner ------------------------------------------------------------------
FROM debian:stable-slim

RUN apt-get update && apt-get install -y libc-bin opus-tools

COPY --from=builder /usr/local/cargo/bin/music-compressor /usr/local/bin/music-compressor

CMD ["music-compressor"]
