use colored::Colorize;
use std::{error::Error, fs, process::Command, str::from_utf8};
use svtools::{fmt, RUNSVDIR, SVDIR};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Service Overview");

    let mut enabled: Vec<_> = fs::read_dir(RUNSVDIR)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name())
        .collect();
    let mut disabled: Vec<_> = fs::read_dir(SVDIR)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name())
        .filter(|entry| !enabled.contains(entry))
        .collect();

    enabled.sort();
    disabled.sort();

    let mut up = 0;

    for svc in enabled
        .iter()
        .map(|x| x.to_str().expect("Failed to convert string"))
    {
        let status = Command::new("sv").arg("status").arg(svc).output()?;
        let status = fmt(from_utf8(&status.stdout)?.trim_end());

        if status.contains("up") {
            up += 1;
        }

        println!("{} {} {}", "[*]".green(), status, svc.green())
    }

    for svc in disabled
        .iter()
        .map(|x| x.to_str().expect("Failed to convert string"))
    {
        println!("{} {}", "[ ] [ disabled ]".red(), svc.red());
    }

    let (en, dis) = (enabled.len(), disabled.len());
    println!("{en}/{} enabled, {}/{} up", en + dis, up, en);

    Ok(())
}
