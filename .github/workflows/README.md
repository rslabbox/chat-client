# GitHub Actions Workflows

This directory contains GitHub Actions workflows for building, testing, and releasing the Chat Client application.

## Workflows

### 1. CI (`ci.yml`)
- **Trigger**: Push to `main`/`develop` branches, Pull Requests
- **Purpose**: Continuous Integration - runs tests and builds on all platforms
- **Platforms**: macOS, Ubuntu, Windows
- **Actions**:
  - Install dependencies
  - Run frontend build
  - Check Rust formatting
  - Run Rust tests
  - Build workspace

### 2. Release (`release.yml`)
- **Trigger**: Git tags starting with `v*` (e.g., `v1.0.0`)
- **Purpose**: Create releases with compiled binaries
- **Platforms**: macOS (Universal), Ubuntu, Windows
- **Actions**:
  - Build frontend
  - Build Rust workspace (including plugins)
  - Create Tauri app bundles
  - Upload to GitHub Releases as draft

### 3. Build and Release (`build-and-release.yml`)
- **Trigger**: Git tags or manual workflow dispatch
- **Purpose**: Advanced release workflow with better control
- **Features**:
  - Manual version input
  - Pre-release option
  - Separate build and publish steps
  - Better artifact management

### 4. Nightly Build (`nightly.yml`)
- **Trigger**: Daily at 2 AM UTC or manual dispatch
- **Purpose**: Automated nightly builds for development
- **Note**: Update the repository owner check before using

## Usage

### Creating a Release

#### Method 1: Using Git Tags (Recommended)
```bash
# Create and push a tag
git tag v1.0.0
git push origin v1.0.0
```

#### Method 2: Manual Workflow Dispatch
1. Go to Actions tab in GitHub
2. Select "Build and Release" workflow
3. Click "Run workflow"
4. Enter version and pre-release options

### Platform-Specific Builds

The workflows build for:
- **macOS**: Universal binary (Intel + Apple Silicon)
- **Linux**: x86_64 AppImage and deb packages
- **Windows**: x86_64 MSI installer and portable exe

### Artifacts

Each release includes:
- **macOS**: `.dmg` and `.app.tar.gz`
- **Linux**: `.AppImage` and `.deb`
- **Windows**: `.msi` and `.exe`
- **Updater**: JSON files for auto-updates (if enabled)

## Configuration

### Required Secrets
- `GITHUB_TOKEN`: Automatically provided by GitHub Actions

### Optional Configuration
- Update repository owner in `nightly.yml`
- Modify build targets in workflow matrices
- Adjust caching strategies
- Customize release notes

### Tauri Configuration
The workflows respect your `tauri.conf.json` settings:
- Bundle targets
- App metadata
- Build commands
- Icon paths

## Troubleshooting

### Common Issues

1. **Build Failures on macOS**
   - Ensure universal target is properly configured
   - Check Xcode command line tools

2. **Linux Dependencies**
   - The workflow installs required system packages
   - Add additional packages if needed

3. **Windows Build Issues**
   - Ensure proper MSVC toolchain
   - Check for Windows-specific dependencies

4. **Plugin Build Failures**
   - Verify workspace configuration in `Cargo.toml`
   - Check plugin dependencies

### Debug Tips
- Check workflow logs in GitHub Actions tab
- Use `workflow_dispatch` for manual testing
- Enable debug logging with `ACTIONS_STEP_DEBUG=true`

## Customization

### Adding New Platforms
Add new entries to the matrix strategy:
```yaml
- platform: 'ubuntu-20.04'
  args: '--target x86_64-unknown-linux-gnu'
  target: 'x86_64-unknown-linux-gnu'
```

### Custom Build Arguments
Modify the `args` field in the matrix:
```yaml
args: '--target universal-apple-darwin --features custom-feature'
```

### Different Release Channels
Create separate workflows for:
- Stable releases
- Beta releases
- Alpha/development releases

## Security

- Workflows use pinned action versions
- Minimal required permissions
- Secure token handling
- No secrets in logs
