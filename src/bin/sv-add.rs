use colored::Colorize;
use std::{
    env::args,
    io::{ErrorKind::*, Result},
    os::unix::fs::symlink,
    path::Path,
    process::exit,
};
use svtools::{RUNSVDIR, SVDIR};

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        eprintln!("Usage: sv-add [<service>...]");
        exit(1);
    }

    let mut errors = 0;
    for service in args.iter().skip(1) {
        let s1 = SVDIR.to_owned() + service;
        let svc = Path::new(&s1);
        let s2 = RUNSVDIR.to_owned() + service;
        let dest = Path::new(&s2);

        if svc.exists() {
            if let Err(e) = symlink(svc, dest) {
                eprint!("{}", "[Error] While enabling {service}: ".red().bold());
                match e.kind() {
                    AlreadyExists => eprintln!("{}", "Service is already enabled".red()),
                    PermissionDenied => eprintln!("{}", "Access denied, try sudo?".red()),
                    _ => eprintln!("Unknown error: {:?}", e),
                }
                errors += 1;
            } else {
                println!("{} {}", "Enabled service".green(), service.green().bold());
            }
        } else {
            eprintln!(
                "{} {}",
                "Error: could not find service".red(),
                service.red().bold()
            );
            errors += 1;
        }
    }

    if errors != 0 {
        eprintln!(
            "{} {} {}",
            "Failed to enable".red(),
            errors.to_string().red().bold(),
            "services".red(),
        );
        exit(errors);
    } else {
        println!(
            "{} {} {}",
            "Successfully enabled".green(),
            (args.len() - 1).to_string().green().bold(),
            "services".green()
        );
    }

    Ok(())
}
