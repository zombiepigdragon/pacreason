use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, ErrorKind, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn main() -> io::Result<()> {
    let command = std::env::args().nth(1);
    match command.as_deref() {
        Some("pre-install") => pre_install(),
        Some("post-install") => post_install(),
        Some(other) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Unexpected command {other}"),
        )),
        None => Err(io::Error::new(io::ErrorKind::Other, "Expected command")),
    }
}

fn pre_install() -> io::Result<()> {
    let status = File::create(status_file())?;
    match pacman_qeq_builder().stdout(status).status()?.code() {
        Some(code) if code == 0 => Ok(()),
        Some(code) => {
            eprintln!("pacman call exited with code {code}");
            std::process::exit(code);
        }
        None => {
            eprintln!("pacman killed by signal");
            std::process::exit(0xff);
        }
    }
}

fn post_install() -> io::Result<()> {
    // Get the packages that changed
    let pre_packages = {
        let file = io::BufReader::new(File::open(status_file())?);
        let pre_packages: HashSet<_> = file.lines().collect::<Result<_, _>>()?;
        pre_packages
    };
    let packages = {
        let command = pacman_qeq_builder().stdout(Stdio::piped()).spawn()?;
        let output = command.wait_with_output()?;
        let stdout =
            String::from_utf8(output.stdout).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
        stdout
            .lines()
            .map(std::borrow::ToOwned::to_owned)
            .collect::<HashSet<_>>()
    };
    let mut diff = packages.difference(&pre_packages).peekable();
    if diff.peek().is_some() {
        let mut input = if let Ok(input) = File::open("/dev/tty") {
            io::BufReader::new(input)
        } else {
            eprintln!("Not running in a TTY, not prompting a reason.");
            return Ok(());
        };
        let mut output = if let Ok(input) = File::options().append(true).open("/dev/tty") {
            io::BufWriter::new(input)
        } else {
            eprintln!("Not running in a TTY, not prompting a reason.");
            return Ok(());
        };
        for package in diff {
            write!(output, "Enter a reason ({package})[]: ")?;
            output.flush()?;

            let mut reason = String::new();
            input.read_line(&mut reason)?;

            pacreason::set_reason(package, &reason)?;
        }
    }
    Ok(())
}

fn pacman_qeq_builder() -> Command {
    let mut command = Command::new("pacman");
    command.arg("--query");
    command.arg("--explicit");
    command.arg("--quiet");
    command
}

fn status_file() -> PathBuf {
    Path::new(pacreason::DB_ROOT).with_extension("status")
}
