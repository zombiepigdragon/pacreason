use std::env::{self};

fn main() {
    let mut args = env::args().skip(1);
    let subcommand = args.next().unwrap_or_else(|| help());
    let mut ret = 0;
    match subcommand.as_str() {
        "get" => {
            for package in args {
                match pacreason::get_reason(&package) {
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
        "set" => {
            let package = match args.next() {
                Some(package) => package,
                None => {
                    eprintln!("Expected a package name");
                    help();
                }
            };
            let reason: Vec<_> = args.collect();
            let reason = reason.join(" ");
            if let Err(e) = pacreason::set_reason(&package, &reason) {
                ret = 1;
                eprintln!("error: {e}");
            }
        }
        _ => {
            eprintln!("Unknown subcommand {}", subcommand);
            help();
        }
    }
    std::process::exit(ret);
}

fn help() -> ! {
    let exe = env::args_os().next();
    let exe = exe
        .as_ref()
        .and_then(|exe| exe.to_str())
        .unwrap_or("pacreason");
    // XXX: Double check this is the standard notation
    eprintln!(
        "USAGE:
{} get package...
{} set package reason...",
        exe, exe
    );
    std::process::exit(1);
}
