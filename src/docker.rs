use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn ensure_postgres_running() -> Result<(), String> {
    if !is_docker_available() {
        return Err("Docker is not available on the system.".to_string());
    }

    if !is_container_running("rust_web_db") {
        println!("🚀 Starting PostgreSQL Docker container...");
        start_docker_compose()?;

        for _ in 0..30 {
            if is_postgres_ready() {
                println!("✅ PostgreSQL is ready!");
                return Ok(());
            }
            thread::sleep(Duration::from_secs(1));
        }

        return Err("Timed out waiting for PostgreSQL to be ready.".to_string());
    }

    println!("✅ PostgreSQL Docker container is already running.");
    Ok(())
}

fn is_docker_available() -> bool {
    Command::new("docker")
        .arg("--version")
        .output()
        .map_or(false, |o| o.status.success())
}

fn is_container_running(container_name: &str) -> bool {
    Command::new("docker")
        .args(["ps", "-q", "-f", &format!("name={}", container_name)])
        .output()
        .map_or(false, |o| !o.stdout.is_empty())
}

fn start_docker_compose() -> Result<(), String> {
    Command::new("docker-compose")
        .args(["-f", "docker-compose.yml", "up", "-d"])
        .output()
        .map_err(|e| format!("Failed to start Docker containers: {}", e))
        .and_then(|o| {
            if o.status.success() {
                Ok(())
            } else {
                Err(format!(
                    "Failed to start Docker containers: {}",
                    String::from_utf8_lossy(&o.stderr)
                ))
            }
        })
}

fn is_postgres_ready() -> bool {
    Command::new("docker")
        .args(["exec", "rust_web_db", "pg_isready"])
        .output()
        .map_or(false, |o| o.status.success())
}

pub fn run_migrations() -> Result<(), String> {
    println!("🚀 Running database migrations...");
    Command::new("diesel")
        .args(["migration", "run"])
        .output()
        .map_err(|e| format!("Failed to run migrations: {}", e))
        .and_then(|o| {
            if o.status.success() {
                Ok(())
            } else {
                Err(format!(
                    "Failed to run migrations: {}",
                    String::from_utf8_lossy(&o.stderr)
                ))
            }
        })
}
