use std::process::{Command, Stdio};

// Function to list current Docker volumes
pub fn list_docker_volumes() -> std::io::Result<()> {
    let status = Command::new("docker")
        .args(["volume", "ls"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Docker exited with status: {}", status),
        ));
    }

    Ok(())
}

// If the directory is local we can run the command:
// MSYS_NO_PATHCONV=1 docker run --gpus all \
// -v C:/Users/jason/Downloads/test:/app/data \ text-to-audio
pub fn run_docker_container_on_local_directory(work_dir: &str) -> std::io::Result<()> {
    // The name of the Docker container to run
    let container_name = "text-to-audio";

    // Clean Windows path prefix and backslashes
    let clean_dir = work_dir.trim_start_matches(r"\\?\");
    let normalized_dir = clean_dir.replace('\\', "/");

    // Prepare the volume mount and input/output paths for the Docker container
    // We need to mount the working directory into the Docker container so it can access the input and output files
    let volume_mount = format!("{}:/app/data", normalized_dir);

    // MSYS_NO_PATHCONV=1 docker run --gpus all \
    // -v C:/Users/jason/Downloads/test:/app/data \ text-to-audio
    let status = Command::new("docker")
        .env("MSYS_NO_PATHCONV", "1") // <-- Prevents Git Bash from rewriting /app/...
        .args([
            "run",
            "--gpus",
            "all",
            "-e",
            "PYTHONUNBUFFERED=1", // Stream container prints immediately
            "-v",
            &volume_mount,
            container_name,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Docker exited with status: {}", status),
        ));
    }

    Ok(())
}

pub fn create_docker_cifs_volume(
    volume_name: &str,
    ip_address: &str,
    smb_share: &str,
    username: &str,
    password: &str,
) -> std::io::Result<()> {
    let status = Command::new("docker")
        .args([
            "volume",
            "create",
            "--driver",
            "local",
            "--opt",
            "type=cifs",
            "--opt",
            &format!("device=//{}/{}", ip_address, smb_share),
            "--opt",
            &format!("o=username={},password={},vers=3.0", username, password),
            volume_name,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Docker volume creation exited with status: {}", status),
        ));
    }

    Ok(())
}

pub fn run_docker_container_on_cifs_volume(
    volume_name: &str,
    subpath: &str,
) -> std::io::Result<()> {
    let container_name = "text-to-audio";

    let mut mount = format!("type=volume,src={},dst=/app/data", volume_name);
    if !subpath.is_empty() {
        mount.push_str(&format!(",volume-subpath={}", subpath));
    }

    let status = Command::new("docker")
        .env("MSYS_NO_PATHCONV", "1") // Prevents Git Bash from rewriting /app/...
        .args([
            "run",
            "--rm",
            "--gpus",
            "all",
            "-e",
            "PYTHONUNBUFFERED=1", // Stream container prints immediately
            "--mount",
            &mount,
            container_name,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Docker exited with status: {}", status),
        ));
    }

    Ok(())
}
