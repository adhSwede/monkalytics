# Build stage
FROM rust:alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

# Create a new empty project
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:3.20

# Install runtime dependencies
RUN apk add --no-cache libgcc

# Copy the binary from builder
COPY --from=builder /app/target/release/monkalytics /usr/local/bin/monkalytics

# Expose the port the app runs on
EXPOSE 8080

# Run the binary
CMD ["monkalytics"]
