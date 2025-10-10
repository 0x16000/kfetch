use std::fs;
use std::{env, path::Path};

mod r#config;
use config::load_config;

mod r#mod;
use r#mod::{nixos, arch, arco, artix, debian, endeavour, fedora, gentoo, mint, manjaro, opensuse, slackware, ubuntu, void, linux, raspbian};

fn get_distro_ascii(distro: &str) -> r#mod::Distro {
    match distro.to_lowercase().as_str() {
        "nixos" => nixos(),
        "arch linux" | "arch" => arch(),
        "arco linux" | "arco" => arco(),
        "artix linux" | "artix" => artix(),
        "debian" => debian(),
        "endeavour linux" | "endeavour" => endeavour(),
        "fedora" => fedora(),
        "gentoo" => gentoo(),
        "linux mint" | "mint" => mint(),
        "manjaro" => manjaro(),
        "opensuse" => opensuse(),
        "slackware" => slackware(),
        "ubuntu" => ubuntu(),
        "void linux" | "void" => void(),
        "raspbian" | "raspberry pi os" => raspbian(),
        &_ => linux(),
    }
}

fn username() -> Option<String> {
    env::var("USER").ok()
}

fn hostname() -> Option<String> {
    fs::read_to_string("/etc/hostname").ok().map(|s| s.trim().to_string())
}

fn user_at_host() -> String {
    let user = username().unwrap_or("unknown".into());
    let host = hostname().unwrap_or("unknown".into());
    format!("{}@{}", user, host)
}

fn os() -> Option<String> {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("NAME=") {
                return Some(line.trim_start_matches("NAME=").trim_matches('"').to_string());
            }
        }
    }
    None
}

fn kernel() -> Option<String> {
    fs::read_to_string("/proc/version")
        .ok()
        .and_then(|s| s.split_whitespace().nth(2).map(|v| v.to_string()))
}

fn uptime() -> Option<String> {
    let content = fs::read_to_string("/proc/uptime").ok()?;
    let seconds: f64 = content.split_whitespace().next()?.parse().ok()?;

    let hours = (seconds / 3600.0).floor() as u64;
    let minutes = ((seconds % 3600.0) / 60.0).floor() as u64;
    let seconds = (seconds % 60.0).floor() as u64;

    Some(format!("{}h {}m {}s", hours, minutes, seconds))
}

fn shell() -> Option<String> {
    if let Ok(shell) = env::var("SHELL") {
        if let Some(name) = Path::new(&shell).file_name() {
            return Some(name.to_string_lossy().to_string());
        }
    }
    None
}

fn wm() -> Option<String> {
    env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .or_else(|_| env::var("GDMSESSION"))
        .ok()
}

fn memory() -> Option<String> {
    let content = std::fs::read_to_string("/proc/meminfo").ok()?;

    let mut mem_total = 0.0;
    let mut mem_available = 0.0;

    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                mem_total = value.parse::<f64>().ok()? / 1024.0;
            }
        }
        if line.starts_with("MemAvailable:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                mem_available = value.parse::<f64>().ok()? / 1024.0;
            }
        }
    }

    if mem_total > 0.0 {
        let used = mem_total - mem_available;
        Some(format!("{:.0} / {:.0} MB", used, mem_total))
    } else {
        None
    }
}

fn main() {
    let config = load_config();

    let distro_name = if let Some(cfg_distro) = &config.distro_override {
        cfg_distro.clone()
    } else {
        os().unwrap_or("Unknown".into())
    };

    let distro_ascii = get_distro_ascii(&distro_name);

    let info_lines = vec![
    	format!("{}{}{}\x1b[0m", distro_ascii.color, "User:", "\x1b[97m ".to_string() + &user_at_host()),
    	format!("{}{}{}\x1b[0m", distro_ascii.color, "OS:", "\x1b[97m ".to_string() + &distro_name),
    	format!("{}{}{}\x1b[0m", distro_ascii.color, "Kernel:", "\x1b[97m ".to_string() + &kernel().unwrap_or("Unknown".into())),
    	format!("{}{}{}\x1b[0m", distro_ascii.color, "Uptime:", "\x1b[97m ".to_string() + &uptime().unwrap_or("Unknown".into())),
    	format!("{}{}{}\x1b[0m", distro_ascii.color, "Shell:", "\x1b[97m ".to_string() + &shell().unwrap_or("Unknown".into())),
    	format!("{}{}{}\x1b[0m", distro_ascii.color, "WM:", "\x1b[97m ".to_string() + &wm().unwrap_or("Unknown".into())),
        format!("{}{}{}\x1b[0m", distro_ascii.color, "Memory:", "\x1b[97m ".to_string() + &memory().unwrap_or("Unknown".into())),
    ];

    let ascii_lines = &distro_ascii.ascii;
    let max_lines = ascii_lines.len().max(info_lines.len());
    let ascii_width = ascii_lines.iter().map(|line| line.len()).max().unwrap_or(0);

    for i in 0..max_lines {
        let ascii_part = ascii_lines.get(i).unwrap_or(&"");
        let info_part = info_lines.get(i).map_or("", |v| v);
        println!(
            "{}{:<width$}{}\x1b[0m",
            distro_ascii.color,
            ascii_part,
            info_part,
            width = ascii_width + 2
        );
    }
}
