# Build Rust Application
FROM rust:1.94 AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
COPY migrations ./migrations

RUN touch src/main.rs && cargo build --release

# Export into Scratch image
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/todo .
EXPOSE 8080
CMD ["./todo"]
