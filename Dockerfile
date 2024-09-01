
# Use the official Rust image as the build stage
FROM rust:1.80.1 AS build

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the container
COPY src ./src

# Build the release version of the application
RUN cargo build --release

# Use a smaller image for the final stage
FROM debian:stable-slim

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the build stage
COPY --from=build /app/target/release/x_snippet_worker .

# Expose the port that the Actix Web app will run on
EXPOSE 8080

# Set the environment variables, if any
# ENV ENV_VAR_NAME=value

# Run the application
CMD ["./x_snippet_worker"]
