FROM rust:1.88-slim AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates ./templates
COPY migrations ./migrations
RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN useradd --create-home --uid 10001 appuser
WORKDIR /app
COPY --from=builder /app/target/release/carteira-inteligente /usr/local/bin/carteira-inteligente
COPY templates ./templates
COPY static ./static

ENV HOST=0.0.0.0
ENV PORT=10000
EXPOSE 10000

USER appuser
CMD ["carteira-inteligente"]
