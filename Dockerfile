FROM rust:slim AS builder

ENV BIN_NAME="discord_counting_bot"

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim AS runner

WORKDIR /opt/counting_bot
COPY --from=builder /app/target/release/$BIN_NAME .
ENTRYPOINT .
