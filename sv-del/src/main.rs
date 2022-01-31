use std::{
    env::args,
    fs,
    io::{ErrorKind::*, Result},
    path::Path,
    process::exit,
};
use termion::color;

const RUNSVDIR: &str = "/var/service/";

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
                    "{}[Error] While disabling {service}: ",
                    color::Fg(color::Red),
                );
                match e.kind() {
                    PermissionDenied => eprintln!("Access denied, try sudo?"),
                    _ => eprintln!("Unknown error: {:?}", e),
                }
                errors += 1;
            } else {
                println!("{}Disabled service {service}", color::Fg(color::Green));
            }
        } else {
            eprintln!(
                "{}[Error] Service {service} is already disabled",
                color::Fg(color::Red),
            );
            errors += 1;
        }
        print!("{}", color::Fg(color::Reset));
    }

    if errors != 0 {
        eprintln!(
            "{}Failed to disable {errors} services",
            color::Fg(color::Red)
        );
        exit(errors);
    } else {
        println!(
            "{}Successfully disabled {} services",
            color::Fg(color::Green),
            args.len() - 1
        );
    }

    print!("{}", color::Fg(color::Reset));
    Ok(())
}
