FROM rust:1.88 AS builder
WORKDIR /code

RUN USER=root cargo init

COPY migrations migrations
COPY Cargo.toml Cargo.toml
COPY .env.example .env

RUN cargo fetch

COPY src src

RUN cargo build --release

FROM bitnami/minideb:bookworm
WORKDIR /app

RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /code/target/release/lightning_stats lightning_stats
COPY --from=builder /code/.env /app/.env
COPY --from=builder /code/migrations /app/migrations

RUN chown 1001:1001 /app/.env
RUN chown 1001:1001 /app/migrations
USER 1001

EXPOSE 3333

CMD [ "/app/lightning_stats" ]