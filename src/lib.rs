//! Basic service management wrappers for `runit`
//! Designed for use on Void Linux

use colored::Colorize;

pub const SVDIR: &str = "/etc/sv/";
pub const RUNSVDIR: &str = "/var/service/";

pub fn fmt(input: &str) -> String {
    if input.contains("access denied") {
        format!("{}", "[access denied]".red(),)
    } else {
        let input: String = input.split(';').take(1).collect();

        let status = if input.contains("run:") {
            format!("{}", "up".green())
        } else {
            format!("{}", "down".red())
        };

        let time = secs_fmt(if status.contains("up") {
            input
                .split_whitespace()
                .last()
                .expect("Time is present")
                .replace('s', "")
                .parse()
                .expect("Time is a valid `u64` value")
        } else {
            input
                .split_whitespace()
                .nth(2)
                .expect("Time is present")
                .replace("s,", "")
                .parse()
                .expect("Time is a valid `u64` value")
        });

        let pid = if status.contains("up") {
            format!(
                "PID {}",
                input
                    .split("(pid ")
                    .last()
                    .unwrap()
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .replace(')', "")
            )
        } else {
            "".to_owned()
        };

        format!(
            "{} {} {} {} {} {}",
            "[".yellow(),
            status.yellow(),
            "for".blue(),
            time.blue(),
            "]\t".yellow(),
            pid.red()
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
