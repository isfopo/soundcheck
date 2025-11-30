# Release Process 🚀

This document describes how to create and publish releases for the Standby audio monitoring application.

## Automated Releases

This project uses GitHub Actions for automated building and publishing. Releases are created automatically when you push version tags.

## Creating a Release

### 1. Update version in Cargo.toml (if needed)

```toml
[package]
version = "1.0.0"
```

### 2. Create and push a version tag

```bash
# Create an annotated tag
git tag -a v1.0.0 -m "Release version 1.0.0"

# Push the tag to trigger the release workflow
git push origin v1.0.0
```

### 3. Alternative: Manual release trigger

- Go to GitHub Actions tab in your repository
- Select "Release Core" workflow
- Click "Run workflow" and enter the tag name

## What Happens During Release

The GitHub Actions workflow automatically:

### 🔨 Build Phase

- Builds binaries for multiple platforms:
  - **macOS** (Intel and Apple Silicon)
  - **Linux** (x86_64)
  - **Windows** (x86_64)
- Runs comprehensive tests and quality checks

### 📦 Release Phase

- Creates a GitHub release with all platform binaries
- Generates SHA256 checksums for security
- Includes release notes and download links

### 📦 Crates.io Publishing

- Publishes the Rust crate to [crates.io](https://crates.io/)
- Makes the library available for other Rust projects
- Enables `cargo install standby` for users

### 🍨 Scoop Publishing

- Creates Scoop manifest with automatic updates
- Publishes to personal Scoop bucket
- Enables `scoop install standby` on Windows

### 🍫 Chocolatey Publishing

- Creates Chocolatey package (.nupkg)
- Publishes to Chocolatey community repository
- Enables `choco install standby` on Windows

### 🐧 Linux Distribution Packaging

- **AppImage**: Universal Linux package that runs on any distribution
- **AUR Package**: Arch Linux package installable via `yay`
- **DEB Package**: Debian/Ubuntu package for apt-based systems

### 🍺 Homebrew Publishing

- Generates a Homebrew formula with correct URLs and hashes in the main repository
- Triggers workflow in dedicated tap repository (`isfopo/Tap`)
- Updates the shared Homebrew tap with the new formula
- Formula available via `brew tap isfopo/Tap && brew install standby`

## Setting Up Publishing

### Crates.io Token

1. **Create API token**:
   - Go to [crates.io/me](https://crates.io/me) → API Tokens
   - Click "New Token"
   - Give it a name like "GitHub Actions Release"

2. **Add repository secret**:
   - Go to repository Settings → Secrets and variables → Actions
   - Add `CRATES_IO_TOKEN` with your API token

### Scoop Bucket Token

1. **Create a Scoop bucket repository**:

   ```bash
   ./.github/scripts/setup-scoop-bucket.sh your-github-username
   ```

2. **Add repository secret**:
   - Go to repository Settings → Secrets and variables → Actions
   - Add `SCOOP_BUCKET_TOKEN` with a Personal Access Token (repo scope)

### Chocolatey API Key

1. **Get Chocolatey API key**:
   - Sign up at [Chocolatey.org](https://chocolatey.org/)
   - Go to Account → API Keys

2. **Add repository secret**:
   - Go to repository Settings → Secrets and variables → Actions
   - Add `CHOCOLATEY_API_KEY` with your API key

### Linux Packaging

No additional setup required - Linux packages are automatically generated and attached to releases.

## Setting Up Homebrew Publishing

### Option 1: Personal Homebrew Tap

The Homebrew tap is maintained in a separate repository (`isfopo/Tap`) for use across multiple projects.

1. **Add repository secret** (in the main repository):
   - Go to repository Settings → Secrets and variables → Actions
   - Add `TAP_REPO_TOKEN` with a Personal Access Token that has `repo` scope
   - This token allows the main repo to trigger workflows in the tap repository

2. **Add repository secret** (in the tap repository `isfopo/Tap`):
   - Add `HOMEBREW_TAP_TOKEN` with a Personal Access Token (repo scope)
   - This allows the tap repository to push changes to itself

## Release Artifacts

Each release includes:

  - **Binaries**: Pre-compiled executables for all supported platforms
    - **Checksums**: SHA256 verification files
    - **AppImage**: Universal Linux package
    - **AUR Package**: Arch Linux package files (PKGBUILD, .SRCINFO)
    - **DEB Package**: Debian/Ubuntu package
  - **Homebrew formula**: Ready-to-use formula for easy installation

## Version Numbering

This project follows [Semantic Versioning](https://semver.org/):

  - **MAJOR.MINOR.PATCH** (e.g., `v1.2.3`)
  - **Pre-releases**: `v1.0.0-alpha.1`, `v1.0.0-rc.1`
  - **Tag format**: Always prefix with `v` (e.g., `v1.0.0`)

## CI/CD Pipeline

### Continuous Integration (on every push/PR)

- ✅ Code formatting checks (`cargo fmt`)
- ✅ Linting (`cargo clippy`)
- ✅ Unit tests (`cargo test`)
- ✅ Multi-platform builds

### Continuous Deployment (on version tags)

The release process is split into separate workflows:

#### **Release Core** (`release-core.yml`)

- ✅ Automated GitHub releases
- ✅ Cross-platform binaries (macOS, Linux, Windows)
- ✅ Crates.io publishing
- ✅ Triggers Homebrew tap updates

#### **Release Homebrew** (`release-homebrew.yml` in `isfopo/Tap`)

- ✅ Homebrew formula generation (triggered by main repo)
- ✅ Shared tap updates
- ✅ Formula committed to tap repository

#### **Release Linux** (`release-linux.yml`)

- ✅ AppImage creation (universal Linux)
- ✅ AUR package generation (Arch Linux)
- ✅ DEB package creation (Debian/Ubuntu)

#### **Release Windows** (`release-windows.yml`)

- ✅ Scoop manifest generation
- ✅ Chocolatey package creation
- ✅ Windows package publishing

## Manual Release Process

If you need to create a release without using GitHub Actions:

### 1. Build binaries locally

```bash
# Build for current platform
cargo build --release

# Build for multiple targets (requires cross-compilation setup)
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
# ... etc for other platforms
```

### 2. Create GitHub release manually

- Go to GitHub Releases page
- Click "Create a new release"
- Tag with version (e.g., `v1.0.0`)
- Upload built binaries
- Publish release

### 3. Publish to Crates.io manually

```bash
# Verify the crate can be published
cargo publish --dry-run

# Publish to crates.io
cargo publish
```

### 4. Update Scoop bucket manually

- Generate SHA256 hash for Windows binary
- Update manifest JSON with new version and hash
- Commit changes to your bucket repository

### 5. Update Chocolatey package manually

```bash
# Create .nupkg file
choco pack standby.nuspec

# Push to Chocolatey
choco push standby.x.x.x.nupkg --api-key your-api-key
```

### 6. Create Linux packages manually

#### AppImage

```bash
# Download appimagetool
wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
chmod +x appimagetool

# Create AppDir structure and build AppImage
# (See workflow for detailed steps)
```

#### AUR Package

```bash
# Create PKGBUILD and .SRCINFO files
# Upload to AUR or personal repository
```

#### DEB Package

```bash
# Create Debian package structure
mkdir -p package/usr/bin
# Add control file, copy binary, build with dpkg-deb
```

### 7. Update Homebrew formula manually

- Generate SHA256 hashes for binaries
- Update formula URLs and checksums
- Commit changes to your tap repository

## Troubleshooting

### Release workflow fails

- Check that the tag format is correct (`v*`)
- Verify that all tests pass locally
- Check GitHub Actions logs for specific errors

### Homebrew tap not updating

- Verify `HOMEBREW_TAP_TOKEN` secret is set
- Check that the tap repository exists and is accessible
- Ensure the token has `repo` scope

### Crates.io publishing fails

- Verify `CRATES_IO_TOKEN` secret is set correctly
- Check that the crate name doesn't conflict with existing crates
- Ensure all dependencies are published or available
- Run `cargo publish --dry-run` locally to test

### Scoop bucket update fails

- Verify `SCOOP_BUCKET_TOKEN` secret is set
- Check that the bucket repository exists and is accessible
- Ensure the manifest follows Scoop JSON schema

### Chocolatey publishing fails

- Verify `CHOCOLATEY_API_KEY` secret is set correctly
- Check that the package doesn't conflict with existing packages
- Ensure the .nupkg file was created successfully
- Review Chocolatey community guidelines

## Contributing to Releases

When contributing changes that affect releases:

1. Update `CHANGELOG.md` with your changes
2. Test locally: `cargo build --release && cargo test`
3. Ensure CI passes on your branch
4. Follow semantic versioning for version bumps

## Security Considerations

  - All release binaries include SHA256 checksums
  - GitHub releases are signed by GitHub's infrastructure
  - Crates.io verifies package integrity and ownership
  - Homebrew formulas verify checksums during installation
  - Consider code signing for enhanced security (future enhancement)</content>
  <parameter name="filePath">RELEASE.md
