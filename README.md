# Standby 🎵

A terminal-based audio monitoring application that displays real-time audio levels and detects when sound exceeds a specified threshold.

![Demo](https://via.placeholder.com/800x400/333/fff?text=Standby+Audio+Monitor)

## Features ✨

- **Real-time Audio Monitoring**: Visual bar graph showing current dB levels
- **Threshold Detection**: Automatically exits when audio exceeds specified dB threshold
- **Multiple Exit Methods**: Ctrl+C, Escape key, or threshold trigger
- **Smooth Visual Updates**: 10ms refresh rate with exponential smoothing
- **Cross-platform**: Works on macOS, Linux, and Windows
- **Terminal UI**: Beautiful text-based interface using Ratatui

## Installation 🚀

### From Crates.io (when published)

```bash
cargo install standby
```

### From Homebrew (when available)

```bash
brew install standby
```

### From Source

```bash
git clone <repository-url>
cd standby
cargo build --release
# Binary will be at target/release/standby
```

## Release Process 🚀

### Automated Releases

This project uses GitHub Actions for automated building and publishing. Releases are created automatically when you push version tags.

### Creating a Release

1. **Update version in Cargo.toml** (if needed):

   ```toml
   [package]
   version = "1.0.0"
   ```

2. **Create and push a version tag**:

   ```bash
   # Create an annotated tag
   git tag -a v1.0.0 -m "Release version 1.0.0"

   # Push the tag to trigger the release workflow
   git push origin v1.0.0
   ```

3. **Alternative: Manual release trigger**
   - Go to GitHub Actions tab in your repository
   - Select "Release and Homebrew" workflow
   - Click "Run workflow" and enter the tag name

### What Happens During Release

The GitHub Actions workflow automatically:

#### **🔨 Build Phase**

- Builds binaries for multiple platforms:
  - **macOS** (Intel and Apple Silicon)
  - **Linux** (x86_64)
  - **Windows** (x86_64)
- Runs comprehensive tests and quality checks

#### **📦 Release Phase**

- Creates a GitHub release with all platform binaries
- Generates SHA256 checksums for security
- Includes release notes and download links

#### **🍺 Homebrew Publishing** (Optional)

- Generates a Homebrew formula with correct URLs and hashes
- Updates your personal Homebrew tap (if configured)
- Can submit pull requests to homebrew-core (requires maintainer access)

### Setting Up Homebrew Publishing

#### **Option 1: Personal Homebrew Tap**

1. **Create a tap repository**:

   ```bash
   ./setup-homebrew-tap.sh your-github-username
   ```

2. **Follow the setup instructions** to create the repository on GitHub

3. **Add repository secret**:
   - Go to repository Settings → Secrets and variables → Actions
   - Add `HOMEBREW_TAP_TOKEN` with a Personal Access Token (repo scope)

#### **Option 2: Submit to Homebrew Core**

1. **Get maintainer access** to [homebrew-core](https://github.com/Homebrew/homebrew-core)
2. **Add repository secret**:
   - Add `HOMEBREW_CORE_TOKEN` with maintainer token
3. **Releases will automatically create PRs** to homebrew-core

### Release Artifacts

Each release includes:

- **Binaries**: Pre-compiled executables for all supported platforms
- **Checksums**: SHA256 verification files
- **Homebrew formula**: Ready-to-use formula for easy installation

### Version Numbering

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR.MINOR.PATCH** (e.g., `v1.2.3`)
- **Pre-releases**: `v1.0.0-alpha.1`, `v1.0.0-rc.1`
- **Tag format**: Always prefix with `v` (e.g., `v1.0.0`)

### CI/CD Pipeline

#### **Continuous Integration** (on every push/PR)

- ✅ Code formatting checks (`cargo fmt`)
- ✅ Linting (`cargo clippy`)
- ✅ Unit tests (`cargo test`)
- ✅ Multi-platform builds

#### **Continuous Deployment** (on version tags)

- ✅ Automated releases
- ✅ Cross-platform binaries
- ✅ Homebrew formula generation
- ✅ Optional tap/core publishing

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

### Project Structure

```
src/
├── main.rs          # Application entry point
├── app.rs           # Main application logic
├── audio.rs         # Audio device handling
├── ui.rs            # Terminal user interface
├── config.rs        # Command line parsing
├── state.rs         # Application state
├── error.rs         # Error handling
├── constants.rs     # Application constants
└── smoothing.rs     # Audio level smoothing
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run `cargo test && cargo clippy && cargo fmt`
6. Submit a pull request

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
