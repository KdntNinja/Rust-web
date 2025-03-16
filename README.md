# Backend as a Service (BaaS)

A simple project demonstrating a Rust backend API with a web frontend.

## Project Structure

- `/backend` - Rust API server using Rocket
- `/frontend` - Web frontend 

## Development Setup

### Backend

```bash
cd backend
cargo run
```

The API server will run on http://localhost:8000

### Frontend

For development, you can use any local server to serve the frontend files. For example:

```bash
# Using Python's built-in server
cd frontend
python -m http.server 3000
```

For production, the backend will serve the static files from the frontend/dist directory.

## Building for Production

### Release Mode (Recommended)

To always run in release mode (optimized performance):

```bash
# Make the script executable
chmod +x scripts/run-release.sh

# Run in release mode
./scripts/run-release.sh
```

### Manual Steps

1. Copy your frontend files to the `frontend/dist` directory:

```bash
chmod +x scripts/build-frontend.sh
./scripts/build-frontend.sh
```

2. Run the backend server in release mode:

```bash
cd backend
cargo run --release
```

Now you can access the application at http://localhost:8000

## Docker Setup

The Docker setup is configured to always run in release mode.

### Running with a Single Docker Compose Command

```bash
# Build and start the application
docker-compose up -d

# Stop the application
docker-compose down
```

The application will be available at http://localhost:8000

### Manual Docker Commands

```bash
# Build the Docker image
docker build -t baas-app .

# Run the container
docker run -p 8000:8000 baas-app
```
