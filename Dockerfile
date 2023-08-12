FROM rust:latest as builder
RUN \
  apt-get update && \
  apt-get install ca-certificates && \
  apt-get clean


WORKDIR /usr/src/app

COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/kitty-bot ./kitty-bot

# Runtime image
FROM debian:bullseye-slim

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/kitty-bot /app/kitty-bot


EXPOSE 3030

# Run the app
CMD ./kitty-bot