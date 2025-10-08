use std::fs;
use dirs::config_dir;

pub struct Config {
    pub distro_override: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self { distro_override: None }
    }
}

pub fn load_config() -> Config {
    let mut cfg = Config::new();

    if let Some(mut path) = config_dir() {
        path.push("kfetch");
        path.push("kfetch.conf");

        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                for line in content.lines() {
                    if let Some(val) = line.strip_prefix("distro=") {
                        cfg.distro_override = Some(val.trim().to_string());
                    }
                }
            }
        }
    }

    cfg
}
