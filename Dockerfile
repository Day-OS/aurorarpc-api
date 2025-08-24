FROM rust:1.89 as builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev

COPY ./x360connect-web ./x360connect-web
COPY ./x360connect-global ./x360connect-global

RUN cd ./x360connect-web && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/x360connect-web/target/release/x360connect-web .
COPY --from=builder /app/x360connect-web/templates ./templates
COPY --from=builder /app/x360connect-web/assets ./assets
COPY --from=builder /app/x360connect-web/Rocket.toml ./Rocket.toml
EXPOSE 8000

# Run the application
CMD ["./x360connect-web"]