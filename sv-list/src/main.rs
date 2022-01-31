use color_eyre::eyre::Result;
use std::{fs, process::Command, str::from_utf8};
use termion::color;

<<<<<<< HEAD
const RUNSVDIR: &str = "/var/service/";
const SV_DIR: &str = "/etc/sv/";
||||||| 7b33e0c
const RUNSVDIR: &'static str = "/var/service/";
const SV_DIR: &'static str = "/etc/sv/";
=======
const SVDIR: &'static str = include_str!("SVDIR");
const RUNSVDIR: &'static str = include_str!("RUNSVDIR");
>>>>>>> faafbe17756e1a43485a5883be4c3364ec329aaf

fn main() -> Result<()> {
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

        println!("{}[*] {} {svc}", color::Fg(color::Green), status);
    }

    print!("{}", color::Fg(color::Red));
    for svc in disabled
        .iter()
        .map(|x| x.to_str().expect("Failed to convert string"))
    {
        println!("[ ] [ disabled ] {svc}");
    }
    println!(
        "{}{}/{} enabled, {}/{} up",
        color::Fg(color::Reset),
        enabled.len(),
        enabled.len() + disabled.len(),
        up,
        enabled.len(),
    );

    Ok(())
}

fn fmt(input: &str) -> String {
    if input.contains("access denied") {
        format!(
            "{}[access denied]{}",
            color::Fg(color::Red),
            color::Fg(color::Reset)
        )
    } else {
        let input: String = input.split(';').take(1).collect();

        let status = if input.contains("run:") {
            format!("{}up", color::Fg(color::Green))
        } else {
            format!("{}down", color::Fg(color::Red))
        };

        let time = secs_fmt(if status.contains("up") {
            input
                .split_whitespace() // The last element will be the time
                .last()
                .expect("Will never fail")
                .replace('s', "")
                .parse() // turn it into a u64
                .expect("Will never fail")
        } else {
            input
                .split_whitespace() // The last element will be the time
                .nth(2)
                .expect("Will never fail")
                .replace("s,", "")
                .parse() // turn it into a u64
                .expect("Will never fail")
        });

        let pid = if status.contains("up") {
            format!(
                "PID {}",
                input
                    .split("(pid ")
                    .last()
                    .expect("Will never fail")
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .replace(')', "")
            )
        } else {
            "".to_owned()
        };

        format!(
            "{}[ {status} {}for {}{time}{} ]\t{}{pid}{}\t",
            color::Fg(color::Yellow),
            color::Fg(color::Reset),
            color::Fg(color::Blue),
            color::Fg(color::Yellow),
            color::Fg(color::Red),
            color::Fg(color::Reset)
        )
    }
}

fn secs_fmt(s: u64) -> String {
    let hours = s / 3600;
    let mins = s % 3600 / 60;
    let secs = s % 3600 % 60;

    let mut s = String::new();
    if hours > 0 {
        s.push_str(&format!("{}h", hours));
    }
    if mins > 0 {
        s.push_str(&format!("{}m", mins));
    }
    if secs > 0 {
        s.push_str(&format!("{}s", secs));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::secs_fmt;

    #[test]
    fn fmt_1() {
        let time = secs_fmt(3880);
        assert_eq!(time, String::from("1h4m40s"));
    }
    #[test]
    fn fmt_2() {
        let time = secs_fmt(628);
        assert_eq!(time, String::from("10m28s"));
    }
}
