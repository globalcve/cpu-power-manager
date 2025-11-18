// Power management module
use anyhow::Result;

pub struct PowerManager;

impl PowerManager {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
    
    pub fn is_on_ac_power(&self) -> Result<bool> {
        // Check if system is on AC power
        std::path::Path::new("/sys/class/power_supply/AC/online")
            .exists()
            .then(|| std::fs::read_to_string("/sys/class/power_supply/AC/online")
                .ok()
                .and_then(|s| s.trim().parse::<u8>().ok())
                .map(|v| v == 1))
            .flatten()
            .ok_or_else(|| anyhow::anyhow!("Failed to determine power state"))
    }
}
