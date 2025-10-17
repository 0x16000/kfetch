use std::fs;
use std::{env, path::Path};
use std::process::Command;

mod config;
use config::load_config;

mod r#mod;
use r#mod::{
    arch, arco, artix, cachyos, debian, endeavour, fedora, gentoo, linux, 
    manjaro, mint, nixos, opensuse, raspbian, slackware, ubuntu, void, Distro
};

const RESET: &str = "\x1b[0m";
const WHITE: &str = "\x1b[97m";

struct SystemInfo {
    user_at_host: String,
    os: String,
    kernel: String,
    uptime: String,
    shell: String,
    wm: String,
    memory: String,
}

impl SystemInfo {
    fn gather() -> Self {
        Self {
            user_at_host: get_user_at_host(),
            os: get_os().unwrap_or_else(|| "Unknown".to_string()),
            kernel: get_kernel().unwrap_or_else(|| "Unknown".to_string()),
            uptime: get_uptime().unwrap_or_else(|| "Unknown".to_string()),
            shell: get_shell().unwrap_or_else(|| "Unknown".to_string()),
            wm: get_wm().unwrap_or_else(|| "Unknown".to_string()),
            memory: get_memory().unwrap_or_else(|| "Unknown".to_string()),
        }
    }

    fn format_lines(&self, color: &str) -> Vec<String> {
        let fields = [
            ("User:", &self.user_at_host),
            ("OS:", &self.os),
            ("Kernel:", &self.kernel),
            ("Uptime:", &self.uptime),
            ("Shell:", &self.shell),
            ("WM:", &self.wm),
            ("Memory:", &self.memory),
        ];

        fields
            .iter()
            .map(|(label, value)| format!("{}{}{} {}{}", color, label, WHITE, value, RESET))
            .collect()
    }
}

fn get_distro_ascii(distro: &str) -> Distro {
    match distro.to_lowercase().as_str() {
        "cachyos" | "cachyos linux" => cachyos(),
        "nixos" => nixos(),
        "arch linux" | "arch" => arch(),
        "arco linux" | "arco" => arco(),
        "artix linux" | "artix" => artix(),
        "debian" => debian(),
        "endeavour linux" | "endeavour" | "endeavouros" => endeavour(),
        "fedora" | "fedora linux" => fedora(),
        "gentoo" | "gentoo linux" => gentoo(),
        "linux mint" | "mint" => mint(),
        "manjaro" | "manjaro linux" => manjaro(),
        "opensuse" | "opensuse leap" | "opensuse tumbleweed" => opensuse(),
        "slackware" => slackware(),
        "ubuntu" => ubuntu(),
        "void linux" | "void" => void(),
        "raspbian" | "raspberry pi os" => raspbian(),
        _ => linux(),
    }
}

fn get_user_at_host() -> String {
    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let host = fs::read_to_string("/etc/hostname")
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    format!("{}@{}", user, host)
}

fn get_os() -> Option<String> {
    let content = fs::read_to_string("/etc/os-release").ok()?;
    content
        .lines()
        .find(|line| line.starts_with("NAME="))
        .map(|line| line.trim_start_matches("NAME=").trim_matches('"').to_string())
}

fn get_kernel() -> Option<String> {
    let content = fs::read_to_string("/proc/version").ok()?;
    content.split_whitespace().nth(2).map(String::from)
}

fn get_uptime() -> Option<String> {
    let content = fs::read_to_string("/proc/uptime").ok()?;
    let seconds: f64 = content.split_whitespace().next()?.parse().ok()?;

    let hours = (seconds / 3600.0) as u64;
    let minutes = ((seconds % 3600.0) / 60.0) as u64;
    let secs = (seconds % 60.0) as u64;

    Some(format!("{}h {}m {}s", hours, minutes, secs))
}

fn get_shell() -> Option<String> {
    let shell = env::var("SHELL").ok()?;
    Path::new(&shell)
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
}

fn get_wm() -> Option<String> {
    if let Ok(wm) = env::var("XDG_CURRENT_DESKTOP") {
        return Some(wm);
    }

    if let Ok(wm) = env::var("DESKTOP_SESSION") {
        return Some(wm);
    }

    if let Ok(wm) = env::var("GDMSESSION") {
        return Some(wm);
    }

    if env::var("DISPLAY").is_ok() {
        if let Some(wm) = detect_x11_wm() {
            return Some(wm);
        }
    }

    if env::var("WAYLAND_DISPLAY").is_ok() {
        if let Some(compositor) = detect_wayland_compositor() {
            return Some(compositor);
        }
    }

    None
}

fn detect_x11_wm() -> Option<String> {
    let output = Command::new("xprop")
        .args(["-root", "-notype", "_NET_SUPPORTING_WM_CHECK"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let wm_id = String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .last()?
        .to_string();

    let name_output = Command::new("xprop")
        .args(["-id", &wm_id, "-notype", "_NET_WM_NAME"])
        .output()
        .ok()?;

    if !name_output.status.success() {
        return None;
    }

    String::from_utf8_lossy(&name_output.stdout)
        .split('"')
        .nth(1)
        .map(String::from)
}

fn detect_wayland_compositor() -> Option<String> {
    let output = Command::new("ps").args(["-e"]).output().ok()?;
    let processes = String::from_utf8_lossy(&output.stdout);

    const COMPOSITORS: &[&str] = &[
        "sway",
        "hyprland",
        "wayfire",
        "river",
        "dwl",
        "labwc",
        "cage",
        "kwin_wayland",
        "gnome-shell",
        "weston",
        "mutter",
        "niri",
    ];

    COMPOSITORS
        .iter()
        .find(|&&compositor| {
            processes
                .lines()
                .any(|line| line.contains(compositor))
        })
        .map(|&s| s.to_string())
}

fn get_memory() -> Option<String> {
    let content = fs::read_to_string("/proc/meminfo").ok()?;

    let mut mem_total = 0.0;
    let mut mem_available = 0.0;

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("MemTotal:") {
            mem_total = value
                .split_whitespace()
                .next()?
                .parse::<f64>()
                .ok()? / 1024.0;
        } else if let Some(value) = line.strip_prefix("MemAvailable:") {
            mem_available = value
                .split_whitespace()
                .next()?
                .parse::<f64>()
                .ok()? / 1024.0;
        }
    }

    if mem_total > 0.0 {
        let used = mem_total - mem_available;
        Some(format!("{:.0} / {:.0} MB", used, mem_total))
    } else {
        None
    }
}

fn render_output(distro: &Distro, info_lines: &[String]) {
    let ascii_lines = distro.ascii;
    let max_lines = ascii_lines.len().max(info_lines.len());
    let ascii_width = ascii_lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0);

    for i in 0..max_lines {
        let ascii_part = ascii_lines.get(i).copied().unwrap_or("");
        let info_part = info_lines.get(i).map_or("", |v| v.as_str());
        
        println!(
            "{}{:<width$}  {}{}",
            distro.color,
            ascii_part,
            info_part,
            RESET,
            width = ascii_width
        );
    }
}

fn main() {
    let config = load_config();

    let distro_name = config
        .distro_override
        .as_ref()
        .map(String::clone)
        .or_else(get_os)
        .unwrap_or_else(|| "Unknown".to_string());

    let distro = get_distro_ascii(&distro_name);
    let info = SystemInfo::gather();
    let info_lines = info.format_lines(distro.color);

    render_output(&distro, &info_lines);
}
