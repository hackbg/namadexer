# syntax=docker/dockerfile:1.2

# Stage 1: Build dependencies. --------------------------------------------------------------------
FROM rust:slim-bookworm as dependencies

# Add protoc and git
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
 rm -f /etc/apt/apt.conf.d/docker-clean \
 && apt update \
 && apt install -y -q \
      protobuf-compiler \
      git

# Fix local builds.
RUN printf "[net]\ngit-fetch-with-cli = true" > /usr/local/cargo/config.toml

# Add Cargo.toml and download depdendencies.
WORKDIR /usr/src/namadexer
COPY ./Cargo.toml ./Cargo.lock ./

# Add stub entrypoints. These allow dependencies to be fetched and built,
# without having to rebuild them all any tim the source changes.
RUN mkdir -p benches/ src/bin/ && printf "fn main () {}" \
  | tee benches/get_block_bench.rs  \
  | tee benches/save_blocks_bench.rs \
  | tee src/bin/indexer.rs \
  | tee src/bin/server.rs

# Download depdendencies.
RUN echo "Fetching:" && pwd && ls -al && cargo fetch \
    -vv \
    --locked

# Compile dependencies.
RUN echo "Building dependencies:" && pwd && ls -al && cargo build \
    --locked \
    --release \
    -F prometheus \
  && pwd && ls -al /usr/src/namadexer/target

# Stage 2: Build app. -----------------------------------------------------------------------------
FROM rust:slim-bookworm as builder

# Add protoc and git
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
 rm -f /etc/apt/apt.conf.d/docker-clean \
 && apt update \
 && apt install -y -q \
      protobuf-compiler \
      git

# Add full source.
WORKDIR /usr/src/namadexer
COPY . ./

# Compile output binaries.
COPY --from=dependencies /usr/local/cargo/git /usr/local/cargo/git
COPY --from=dependencies /usr/local/cargo/registry /usr/local/cargo/registry
COPY --from=dependencies /usr/src/namadexer/target /usr/src/namadexer/target
RUN echo "Building:" && pwd && ls -al && ls -al target && cargo install \
    --locked \
    --offline \
    -F prometheus \
    --path /usr/src/namadexer

# Stage 3: Runtime context. -----------------------------------------------------------------------
FROM debian:12-slim AS runtime
RUN mkdir -p /app
WORKDIR /app

# Add wget
RUN --mount=type=cache,target=/var/cache/apt \
 rm -f /etc/apt/apt.conf.d/docker-clean \
 && apt update \
 && apt install -y -q --no-install-recommends \
      wget

# Add binaries.
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY --from=builder /usr/local/cargo/bin/indexer /usr/local/bin/indexer

# Add checksums.
ADD https://raw.githubusercontent.com/anoma/namada/v0.31.0/wasm/checksums.json /app/checksums.json

# Settings.
ENV INDEXER_CONFIG_PATH "/app/config/Settings.toml"
ENV RUST_LOG "namadexer=debug"
CMD indexer
