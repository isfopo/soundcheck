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
- Select "Release and Homebrew" workflow
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

### 🍺 Homebrew Publishing (Optional)

- Generates a Homebrew formula with correct URLs and hashes
- Updates your personal Homebrew tap (if configured)
- Can submit pull requests to homebrew-core (requires maintainer access)

## Setting Up Homebrew Publishing

### Option 1: Personal Homebrew Tap

1. **Create a tap repository**:

   ```bash
   ./setup-homebrew-tap.sh your-github-username
   ```

2. **Follow the setup instructions** to create the repository on GitHub

3. **Add repository secret**:
   - Go to repository Settings → Secrets and variables → Actions
   - Add `HOMEBREW_TAP_TOKEN` with a Personal Access Token (repo scope)

### Option 2: Submit to Homebrew Core

1. **Get maintainer access** to [homebrew-core](https://github.com/Homebrew/homebrew-core)
2. **Add repository secret**:
   - Add `HOMEBREW_CORE_TOKEN` with maintainer token
3. **Releases will automatically create PRs** to homebrew-core

## Release Artifacts

Each release includes:

- **Binaries**: Pre-compiled executables for all supported platforms
- **Checksums**: SHA256 verification files
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

- ✅ Automated releases
- ✅ Cross-platform binaries
- ✅ Homebrew formula generation
- ✅ Optional tap/core publishing

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

### 3. Update Homebrew formula manually

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

### Formula submission to homebrew-core fails

- Verify `HOMEBREW_CORE_TOKEN` has maintainer access
- Check that the formula follows Homebrew guidelines
- Review the PR that was created for any issues

## Contributing to Releases

When contributing changes that affect releases:

1. Update `CHANGELOG.md` with your changes
2. Test locally: `cargo build --release && cargo test`
3. Ensure CI passes on your branch
4. Follow semantic versioning for version bumps

## Security Considerations

- All release binaries include SHA256 checksums
- GitHub releases are signed by GitHub's infrastructure
- Homebrew formulas verify checksums during installation
- Consider code signing for enhanced security (future enhancement)</content>
<parameter name="filePath">RELEASE.md