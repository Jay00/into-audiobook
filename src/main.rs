use std::path::PathBuf;

use clap::Parser;

mod docker;

/// CLI tool to run the docker text-to-audio container
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the directory containing documents
    #[arg(value_name = "DIR")]
    dir_path: PathBuf,
}

fn main() {
    let args = Args::parse();

    // Verify the path exists and is actually a directory
    if !args.dir_path.is_dir() {
        eprintln!(
            "Error: '{}' is not a valid directory.",
            args.dir_path.display()
        );
        std::process::exit(1);
    }

    // Due to Windows user-session limitations, mapped network drives (e.g., `R:\`)
    // are not visible to the Docker daemon.
    // Use a Docker CIFS volume to attach directly to the SMB share.

    // Check if the directory is a network drive (e.g., starts with `\\` on Windows) OR a mapped network drive (e.g., `R:\`)
    let is_network_drive = args.dir_path.to_string_lossy().starts_with(r"\\");
    let is_mapped_network_drive = args.dir_path.to_string_lossy().chars().nth(1) == Some(':')
        && args.dir_path.to_string_lossy().chars().nth(2) == Some('\\');

    // If the directory is a network drive or a mapped network drive, we should use the Docker CIFS volume approach
    if is_network_drive || is_mapped_network_drive {
        // Here you would extract the necessary information to create the CIFS volume
        // For example, you might parse the UNC path to get the IP address and share name
        println!(
            "Network drive or mapped network drive detected: {}",
            args.dir_path.display()
        );

        // Print the current Docker volumes for debugging purposes
        if let Err(e) = docker::list_docker_volumes() {
            eprintln!("Failed to list Docker volumes: {}", e);
        }
    } else {
        // Run the local Docker container on the specified directory
        if let Err(e) =
            docker::run_docker_container_on_local_directory(&args.dir_path.to_string_lossy())
        {
            eprintln!("Failed to run Docker container: {}", e);
        }
    }
}
