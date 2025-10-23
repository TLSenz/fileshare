FROM rust:latest AS builder
WORKDIR /app
RUN apt update && apt install -y lld clang
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release


FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove \
  && apt-get clean \
   && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/fileshare fileshare

COPY configuration.yaml .
ENTRYPOINT ["./fileshare"]
