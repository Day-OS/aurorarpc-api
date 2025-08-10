FROM rust:1.89 as builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/aurorarpc-api .
COPY --from=builder /app/assets ./assets

EXPOSE 8000

# Run the application
CMD ["./aurorarpc-api"]