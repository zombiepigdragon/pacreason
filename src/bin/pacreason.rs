use clap::{Arg, Command};

fn main() {
    let cli = clap::command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("get")
                .about("Read the recorded reason for a package")
                .arg(
                    Arg::new("package")
                        .help("Package(s) to get reason for")
                        .required(true)
                        .multiple_occurrences(true),
                ),
        )
        .subcommand(
            Command::new("set")
                .about("Set a reason for a package")
                .arg(
                    Arg::new("package")
                        .help("Package(s) to set reason for")
                        .required(true),
                )
                .arg(Arg::new("reason").help("The reason (or omit to delete)")),
        );
    let matches = cli.get_matches();
    let mut ret = 0;
    match matches.subcommand() {
        Some(("get", matches)) => {
            for package in matches
                .values_of("package")
                .expect("missing required value")
            {
                match pacreason::get_reason(package) {
                    Ok(Some(reason)) => println!("{package}: {reason}"),
                    Ok(None) => {
                        ret = 1;
                        eprintln!("{package}: no reason set");
                    }
                    Err(e) => {
                        ret = 1;
                        eprintln!("{package}: {e}");
                    }
                }
            }
        }
        Some(("set", matches)) => {
            let package = matches.value_of("package").expect("missing required value");
            let reason = matches.value_of("reason").unwrap_or("");
            if let Err(e) = pacreason::set_reason(package, reason) {
                ret = 1;
                eprintln!("error: {e}");
            }
        }
        Some((other, _)) => unreachable!("unknown subcommand {other}"),
        None => unreachable!("a subcommand is required"),
    }
    std::process::exit(ret);
}
