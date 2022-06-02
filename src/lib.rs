use std::{
    fs::{self, File},
    io::{self, ErrorKind, Write},
    path::{Path, PathBuf},
};

#[cfg(not(target_os = "linux"))]
compile_error!("This application only works on Linux systems.");

pub const DB_ROOT: &str = "/var/lib/pacreason";

pub fn get_reason(package_name: &str) -> io::Result<Option<String>> {
    let path = package_path(package_name);
    match fs::read_to_string(path) {
        Ok(reason) => Ok(Some(reason)),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn set_reason(package_name: &str, reason: &str) -> io::Result<()> {
    if let Err(e) = fs::create_dir(DB_ROOT) {
        if e.kind() != ErrorKind::AlreadyExists {
            return Err(e);
        }
    }
    let path = package_path(package_name);
    let reason = reason.trim();
    if reason.is_empty() {
        // Clear the reason
        match fs::remove_file(path) {
            Ok(()) => (),
            Err(e) if e.kind() == ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    } else {
        // Save the reason
        let mut file = File::create(path)?;
        write!(file, "{reason}")?;
    }
    Ok(())
}

fn package_path(package_name: &str) -> PathBuf {
    Path::new(DB_ROOT).join(package_name)
}
