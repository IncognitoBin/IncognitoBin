FROM rust:1.80.1 AS build
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release
FROM debian:stable-slim
WORKDIR /app
COPY resources ./resources
COPY --from=build /app/target/release/IncognitoBin_server .
EXPOSE 8181
CMD ["./IncognitoBin_server"]