FROM rust:1.80.1 AS build
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release
FROM debian:stable-slim
WORKDIR /app
COPY --from=build /app/target/release/x_snippet_worker .
EXPOSE 8080
# TODO: ENV_VAR for Split_Size
CMD ["./x_snippet_worker"]