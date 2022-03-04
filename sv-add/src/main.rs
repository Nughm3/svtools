use std::{
    env::args,
    io::{ErrorKind::*, Result},
    os::unix::fs::symlink,
    path::Path,
    process::exit,
};
use termion::color;

const SVDIR: &str = "/etc/runit/sv/";
const RUNSVDIR: &str = "/etc/runit/runsvdir/default/";

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
                eprint!(
                    "{}[Error] While enabling {service}: ",
                    color::Fg(color::Red)
                );
                match e.kind() {
                    AlreadyExists => eprintln!("Service is already enabled"),
                    PermissionDenied => eprintln!("Access denied, try sudo?"),
                    _ => eprintln!("Unknown error: {:?}", e),
                }
                errors += 1;
            } else {
                println!("{}Enabled service {service}", color::Fg(color::Green));
            }
        } else {
            eprintln!(
                "{}Error: could not find service {service}",
                color::Fg(color::Red)
            );
            errors += 1;
        }
        print!("{}", color::Fg(color::Reset));
    }

    if errors != 0 {
        eprintln!(
            "{}Failed to enable {errors} services",
            color::Fg(color::Red)
        );
        exit(errors);
    } else {
        println!(
            "{}Successfully enabled {} services",
            color::Fg(color::Green),
            args.len() - 1
        );
    }
    print!("{}", color::Fg(color::Reset));

    Ok(())
}
