FROM rust:1.85-bookworm AS builder
WORKDIR /app
COPY nautilus ./nautilus
WORKDIR /app/nautilus
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/nautilus/target/release/nautilus /app/nautilus
CMD ["/app/nautilus"]
