FROM rust:1.49.0-buster as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path

FROM debian:buster-slim
RUN apt-get update -y && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
CMD ["server"]
