# Inspired by
# https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
FROM rust:latest as builder

# Build dependencies only to save some time by reusing cached layers
WORKDIR /usr/src/myapp
COPY Cargo.toml Cargo.lock ./
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/actix_web_demo*

# Now copy the sources and build again. Up to here the layers are reused uless some deps changed
COPY . .
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update \
    && apt-get install -y libpq-dev \
    && rm -rf /var/lib/{apt,dpkg,cache,log}/ \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

COPY --from=builder /usr/src/myapp/target/release/actix-web-demo /usr/local/bin/actix-web-demo

CMD ["actix-web-demo"]