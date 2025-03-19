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

        // Wait for PostgreSQL to be ready (max 30 seconds)
        for _ in 0..30 {
            if is_postgres_ready() {
                println!("✅ PostgreSQL is ready!");
                return Ok(());
            }
            thread::sleep(Duration::from_secs(1));
            print!(".");
        }

        return Err("Timed out waiting for PostgreSQL to be ready.".to_string());
    }

    println!("✅ PostgreSQL Docker container is already running.");
    Ok(())
}

fn is_docker_available() -> bool {
    let output = Command::new("docker").arg("--version").output();

    output.is_ok() && output.unwrap().status.success()
}

fn is_container_running(container_name: &str) -> bool {
    let output = Command::new("docker")
        .args(["ps", "-q", "-f", &format!("name={}", container_name)])
        .output();

    if let Ok(output) = output {
        !output.stdout.is_empty()
    } else {
        false
    }
}

fn start_docker_compose() -> Result<(), String> {
    // Run docker-compose up -d from the project root
    let output = Command::new("docker-compose")
        .args(["-f", "docker-compose.yml", "up", "-d"])
        .output()
        .map_err(|e| format!("Failed to start Docker containers: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to start Docker containers: {}", error))
    }
}

fn is_postgres_ready() -> bool {
    let output = Command::new("docker")
        .args(["exec", "rust_web_db", "pg_isready"])
        .output();

    if let Ok(output) = output {
        output.status.success()
    } else {
        false
    }
}

pub fn run_migrations() -> Result<(), String> {
    println!("🚀 Running database migrations...");

    // Use the diesel CLI command internally from the Docker container
    let output = Command::new("docker")
        .args([
            "exec", "rust_web_db", 
            "psql", "-U", "postgres", "-d", "rust_web_db",
            "-c", "CREATE TABLE IF NOT EXISTS __diesel_schema_migrations (version VARCHAR(50) PRIMARY KEY, run_on TIMESTAMP NOT NULL DEFAULT NOW())"
        ])
        .output()
        .map_err(|e| format!("Failed to create migrations table: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to create migrations table: {}", error));
    }

    // Copy migration files into container and run them
    let migration_sql = include_str!("../migrations/202310_create_orders_table.sql");

    let mut output = Command::new("docker")
        .args([
            "exec",
            "-i",
            "rust_web_db",
            "psql",
            "-U",
            "postgres",
            "-d",
            "rust_web_db",
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to run migrations: {}", e))?;

    if let Some(ref mut stdin) = output.stdin {
        use std::io::Write;
        stdin
            .write_all(migration_sql.as_bytes())
            .map_err(|e| format!("Failed to write migration SQL: {}", e))?;
    }

    let output = output
        .wait_with_output()
        .map_err(|e| format!("Failed to run migrations: {}", e))?;

    if output.status.success() {
        println!("✅ Database migrations completed successfully!");
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to run migrations: {}", error))
    }
}
