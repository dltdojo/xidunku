FROM rust:1.40 AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock build.rs ./
#
# https://github.com/rust-lang/cargo/issues/2644
#
RUN mkdir -p src htdocs && \
    echo "hello" > htdocs/dump.txt && \
    echo "fn main() { }" > src/main.rs && \
    cargo build --release

COPY build.rs build.sh ./
COPY src src
COPY htdocs htdocs
RUN cargo build --release && cp target/release/xctixweb /xctixweb

## Install the proxy binary into the base runtime image.
FROM debian:stable-slim
ONBUILD ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y dumb-init bash sudo openssl curl jq tree w3m

COPY --from=build /xctixweb /xctixweb
#
# https://github.com/cdr/code-server/blob/master/Dockerfile
#
RUN adduser --gecos '' --disabled-password alice && \
    echo "alice ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers.d/nopasswd
USER alice
RUN mkdir -p /home/alice/host
WORKDIR /home/alice/host
VOLUME [ "/home/alice/host" ]
ENV RUST_LOG=info
#
# docker build -t foo .
# docker run -p 8443:8443 -v $PWD:/home/alice/host foo
#
ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["bash","-c","/xctixweb"]