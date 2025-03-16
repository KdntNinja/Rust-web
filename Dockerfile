# Build stage for backend
FROM rust:1.72 as builder

WORKDIR /usr/src/app

# Copy the manifests
COPY backend/Cargo.toml backend/Cargo.lock* ./

# Create a dummy source file to cache dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release

# Remove the dummy source file
RUN rm -rf src

# Copy the actual source code
COPY backend/src ./src

# Force rebuild with the actual source code
RUN touch src/main.rs
RUN cargo build --release

# Prepare frontend files
FROM node:16-alpine as frontend-builder

WORKDIR /frontend

# Copy frontend files with the proper directory structure
COPY frontend/templates ./templates
COPY frontend/styles ./styles
COPY frontend/scripts ./scripts

# Create dist directory with proper structure
RUN mkdir -p dist/templates dist/styles dist/scripts
# Copy files to dist maintaining structure
RUN cp templates/* dist/templates/ && \
    cp styles/* dist/styles/ && \
    cp scripts/* dist/scripts/ || true

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the build stage
COPY --from=builder /usr/src/app/target/release/Baas-backend /app/Baas-backend

# Copy the frontend files from the frontend builder maintaining structure
COPY --from=frontend-builder /frontend/dist /app/frontend

# Expose the port
EXPOSE 8000

# Set environment variables to specify release mode
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PROFILE=release
ENV ROCKET_ENV=production

# Run the binary
CMD ["/app/Baas-backend"]
