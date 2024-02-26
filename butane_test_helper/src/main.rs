use std::path::PathBuf;
use std::sync::mpsc::channel;

use cargo_metadata::MetadataCommand;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() {
    let app = clap::Command::new("butane_test_helper")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Run a temporary PostgreSQL server")
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("run_tmp_pg_server")
                .about("Initialize the database"),
        );

    let args = app.get_matches();

    match args.subcommand() {
        Some(("run_tmp_pg_server", _sub_args)) => {
            run_pg_server();
        }
        Some((_, _)) | None => panic!("Unreachable as clap handles this automatically"),
    };
}


fn run_pg_server() {
    let (tx, rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    let backend = "pg";
    env_logger::try_init().ok();
    let backend = butane_core::db::get_backend(backend).expect("Could not find backend");
    let instance = butane_test_helper::pg_setup();
    let connection_string = instance.connstr.clone();
    eprintln!("connecting to {}", connection_string);
    let conn = backend
        .connect(&connection_string)
        .expect("Could not connect backend");
    eprintln!("connected. disconnecting ...");
    drop(conn);
    eprintln!("disconnected. waiting for Control-C ..");
    rx.recv().expect("Could not receive from channel.");
    println!("Got it! Exiting...");

    butane_test_helper::pg_teardown(instance);
}

pub fn working_dir_path() -> PathBuf {
    match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => PathBuf::from("."),
    }
}

/// Extract the directory of a cargo workspace member identified by PackageId
pub fn extract_package_directory(
    packages: &[cargo_metadata::Package],
    package_id: cargo_metadata::PackageId,
) -> Result<std::path::PathBuf> {
    let pkg = packages
        .iter()
        .find(|p| p.id == package_id)
        .ok_or(anyhow::anyhow!("No package found"))?;
    // Strip 'Cargo.toml' from the manifest_path
    let parent = pkg.manifest_path.parent().unwrap();
    Ok(parent.to_owned().into())
}

/// Find all cargo workspace members that have a `.butane` subdirectory
pub fn find_butane_workspace_member_paths() -> Result<Vec<PathBuf>> {
    let metadata = MetadataCommand::new().no_deps().exec()?;
    let workspace_members = metadata.workspace_members;

    let mut possible_directories: Vec<PathBuf> = vec![];
    // Find all workspace member with a .butane
    for member in workspace_members {
        let package_dir = extract_package_directory(&metadata.packages, member)?;
        let member_butane_dir = package_dir.join(".butane/");

        if member_butane_dir.exists() {
            possible_directories.push(package_dir);
        }
    }
    Ok(possible_directories)
}

/// Get the project path if only one workspace member contains a `.butane` directory
pub fn get_butane_project_path() -> Result<PathBuf> {
    let possible_directories = find_butane_workspace_member_paths()?;

    match possible_directories.len() {
        0 => Err(anyhow::anyhow!("No .butane exists")),
        1 => Ok(possible_directories[0].to_owned()),
        _ => Err(anyhow::anyhow!("Multiple .butane exists")),
    }
}

/// Find a .butane directory to act as the base for butane.
pub fn base_dir() -> PathBuf {
    let current_directory = working_dir_path();
    let local_butane_dir = current_directory.join(".butane/");

    if !local_butane_dir.exists() {
        if let Ok(member_dir) = get_butane_project_path() {
            println!("Using workspace member {:?}", member_dir);
            return member_dir;
        }
    }

    // Fallback to the current directory
    current_directory
}
