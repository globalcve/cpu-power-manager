# Quick Start Guide

## Installation

### Option 1: Install from .deb (Recommended for Debian/Ubuntu)

```bash
# Install cargo-deb if not already installed
cargo install cargo-deb

# Build the .deb package
cd cpu-power-manager
cargo deb

# Install the package
sudo dpkg -i target/debian/cpu-power-manager_*.deb
```

### Option 2: Build and Install Manually

```bash
cd cpu-power-manager

# Build
./build.sh

# Install
sudo ./install.sh
```

## First Run

1. Launch from application menu or run: `cpu-power-manager`
2. The app will request permission to access CPU controls (via PolicyKit)
3. You'll see the main dashboard with:
   - Current CPU information
   - Real-time frequency
   - Temperature
   - Active governor

## Quick Actions

### Apply a Profile
Click one of the profile buttons:
- **Performance**: Maximum CPU performance
- **Balanced**: Good balance of power and performance
- **Power Saver**: Extended battery life
- **Silent**: Quiet operation

### Command Line Usage

```bash
# Check current status
cpu-power-manager status

# Set governor
cpu-power-manager set-governor performance

# Enable turbo boost
cpu-power-manager set-turbo true

# Apply a profile
cpu-power-manager apply-profile balanced
```

## Configuration

Edit `~/.config/cpu-power-manager/config.toml` to customize:
- Auto-start behavior
- Temperature thresholds
- Auto-tuning settings
- Monitoring options

## Troubleshooting

### "Permission denied" errors
- Ensure PolicyKit is installed: `sudo apt install policykit-1`
- Your user should be in the `sudo` or `wheel` group

### CPU frequency not changing
- Check cpufreq driver: `cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_driver`
- Verify governor is set: `cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor`
- Try setting manually as root to test: `echo performance | sudo tee /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor`

### Application won't start
- Check GTK4 is installed: `pkg-config --modversion gtk4`
- Run with debug: `cpu-power-manager --debug`
- Check logs: `~/.local/share/cpu-power-manager/app.log`

## Support

- GitHub Issues: https://github.com/globalcve/cpu-power-manager/issues

