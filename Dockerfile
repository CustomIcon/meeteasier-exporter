FROM rust:1.82.0 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./

COPY . .
RUN cargo build --release

FROM ubuntu:25.04

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /root/

COPY --from=builder /app/target/release/meeting-room-exporter .

EXPOSE 8000

CMD ["./meeting-room-exporter"]
