# Standby 🎵

A terminal-based audio monitoring application that displays real-time audio levels and exits when sound exceeds a specified threshold.

![Demo](https://via.placeholder.com/800x400/333/fff?text=Standby+Audio+Monitor)

## Features ✨

- **Real-time Audio Monitoring**: Visual bar graph showing current dB levels
- **Threshold Detection**: Automatically exits when audio exceeds specified dB threshold
- **Multiple Exit Methods**: Ctrl+C, Escape key, or threshold trigger
- **Smooth Visual Updates**: 10ms refresh rate with exponential smoothing
- **Cross-platform**: Works on macOS, Linux, and Windows
- **Terminal UI**: Beautiful text-based interface using Ratatui

## Installation 🚀

### From Crates.io

```bash
cargo install standby
```

### From Homebrew

```bash
# Option 1: Direct install (when formula is in homebrew-core)
brew install standby

# Option 2: From personal tap
brew tap username/standby-homebrew-tap
brew install standby
```

### From Scoop (Windows)

```bash
# Add the bucket
scoop bucket add username-scoop-bucket https://github.com/username/standby-scoop-bucket

# Install standby
scoop install standby
```

### From Chocolatey (Windows)

```bash
# Install standby
choco install standby
```

### From AUR (Arch Linux)

```bash
# Using yay
yay -S standby

# Or manually from releases
# Download PKGBUILD from releases
# makepkg -si
```

### From Debian/Ubuntu Packages

```bash
# Download .deb from releases
sudo dpkg -i standby_*.deb
sudo apt install -f  # Install dependencies if needed
```

### AppImage (Universal Linux)

```bash
# Download AppImage from releases
chmod +x standby-*.AppImage
./standby-*.AppImage --help
```

### From Source

```bash
git clone <repository-url>
cd standby
cargo build --release
# Binary will be at target/release/standby
```

## Contributing & Development 🤝

### Release Process

For detailed information about creating and publishing releases, see [RELEASE.md](RELEASE.md).

## Usage 📖

### Basic Usage

```bash
# Monitor with default settings (-20 dB threshold)
standby

# Set custom threshold
standby --threshold -30

# Specify audio device
standby --device "MacBook Pro Microphone"

# Combine options
standby --threshold -25 --device "External USB Audio"
```

### Command Line Options

| Option        | Description                      | Default        | Example                     |
| ------------- | -------------------------------- | -------------- | --------------------------- |
| `--threshold` | Audio threshold in dB (-60 to 0) | -20            | `--threshold -30`           |
| `--device`    | Audio input device name          | Default device | `--device "USB Microphone"` |

### Command Chaining Examples

```bash
# Continue to next command only if threshold reached
standby && echo "Audio detected!"

# Run fallback command if user exits
standby || echo "Monitoring cancelled by user"

# Error handling
standby || echo "Failed to start monitoring"
```

## Requirements 📋

### System Requirements

- **macOS**: 10.15 or later
- **Linux**: Kernel 3.16+ with ALSA
- **Windows**: Windows 10+ with WASAPI

### Audio Hardware

- Audio input device (microphone, line-in, etc.)
- Proper audio permissions (especially on macOS)

### Dependencies

- **Rust**: 1.70+ (for edition 2021)
- **Audio Libraries**: System audio frameworks
  - macOS: CoreAudio
  - Linux: ALSA
  - Windows: WASAPI

## Configuration ⚙️

### Audio Device Selection

```bash
# List available devices (if supported by your audio library)
# Then specify the device name
standby --device "External Microphone (USB)"
```

### Threshold Tuning

- **Quiet environments**: -30 to -40 dB
- **Normal conversation**: -20 to -25 dB
- **Loud music/events**: -10 to -15 dB

## Development 🛠️

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt
```

## Troubleshooting 🔍

### Common Issues

**"No default input device"**

- Ensure your microphone/audio input is connected and enabled
- Check system audio settings

**"Device not found"**

- Use `standby --help` to see available options
- Verify the device name spelling

**Blank screen on startup**

- Ensure terminal supports Unicode characters
- Try a different terminal emulator

**Audio levels not updating**

- Check that the correct audio device is selected
- Verify audio input permissions
- Test with different threshold values

### Debug Mode

```bash
# Run with verbose output
RUST_LOG=debug cargo run -- --threshold -20
```

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments 🙏

- [CPAL](https://github.com/RustAudio/cpal) - Cross-platform audio library
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [Tokio](https://tokio.rs/) - Async runtime
- [Clap](https://github.com/clap-rs/clap) - Command line parsing

---

**Made with ❤️ using Rust**</content>
<parameter name="filePath">README.md
