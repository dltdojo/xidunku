FROM docker.io/rust:1.36.0 as build
ARG RUNTIME_IMAGE=debian:stable-slim
ARG APP_NAME=xctixweb

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked

# Build libraries, leaving the proxy mocked out.
ARG PROXY_UNOPTIMIZED

# Build the binary using the already-built dependencies.
COPY src src
RUN cargo build --frozen --release && \
    mv target/release/xcixweb target/xcitxweb

## Install the proxy binary into the base runtime image.
FROM debian:10
ONBUILD ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y openssl curl jq tree \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /build/target/xctixweb /xctixweb
CMD ["/xctixweb"]