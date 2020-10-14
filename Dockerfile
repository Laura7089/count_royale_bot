FROM rust:slim AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim AS runner

WORKDIR /opt/countnite
COPY --from=builder /app/target/release/countnite .
ENTRYPOINT ./countnite
