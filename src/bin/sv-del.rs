use colored::Colorize;
use std::{
    env::args,
    fs,
    io::{ErrorKind::*, Result},
    path::Path,
    process::exit,
};
use svtools::RUNSVDIR;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        eprintln!("Usage: sv-del [<service>...]");
        exit(1);
    }

    let mut errors = 0;
    for service in args.iter().skip(1) {
        let svc = RUNSVDIR.to_owned() + service;
        let svc = Path::new(&svc);

        if svc.exists() {
            if let Err(e) = fs::remove_file(svc) {
                eprint!(
                    "{} {}",
                    "[Error] While disabling {service}: ".red().bold(),
                    e.to_string().red()
                );

                errors += 1;
            } else {
                println!("{} {}", "Disabled service".green(), service.green().bold());
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
            "Failed to disable".red(),
            errors.to_string().red().bold(),
            "services".red(),
        );
        exit(errors);
    } else {
        println!(
            "{} {} {}",
            "Successfully disable".green(),
            (args.len() - 1).to_string().green().bold(),
            "services".green()
        );
    }

    Ok(())
}
