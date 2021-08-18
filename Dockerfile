# --- Builder -----------------------------------------------------------------
FROM rust:alpine as builder

# - Build cache ---
RUN cargo new --bin /usr/src/music-compressor
WORKDIR /usr/src/music-compressor

COPY Cargo.* ./
RUN cargo build --release \
    && rm target/release/* -rf

# - Compile ---
COPY ./src/ ./src/
RUN cargo build --release

# --- Executioner -------------------------------------------------------------
FROM alpine:latest

# Runtime Dependencies
RUN apk upgrade \
    && apk add \
        opus-tools

COPY --from=builder /usr/src/music-compressor/target/release/music-compressor /usr/local/bin/

CMD /usr/local/bin/music-compressor
