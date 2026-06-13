# CI/CD Documentation - buttre

> **Version**: 1.0.0  
> **Last Updated**: 2025-12-25

---

## 📋 Overview

buttre uses GitHub Actions for continuous integration and deployment. All workflows are automatically triggered on code changes.

---

## 🔄 Workflows

### 1. **CI Workflow** (`.github/workflows/ci.yml`)

**Triggers:**
- Push to `main`, `develop`, or `backup-before-fix-*` branches
- Pull requests to `main` or `develop`

**Jobs:**

#### a) Quick Checks
- ✅ Code formatting (`cargo fmt`)
- ✅ Linting (`cargo clippy` with `-D warnings`)
- ⚡ Runs first, fails fast if code style issues

#### b) Test on Windows
- ✅ Build workspace with all features
- ✅ Run unit tests
- ✅ Run doc tests
- 🎯 Primary platform validation

#### c) Code Coverage
- ✅ Generate coverage report with `cargo-tarpaulin`
- ✅ Upload to Codecov
- 📊 Track coverage trends
- **Requirement**: Set `CODECOV_TOKEN` secret

#### d) Security Audit
- ✅ Run `cargo-audit` via rustsec/audit-check
- 🔒 Check for known vulnerabilities in dependencies
- ⚠️ Warns on security advisories

#### e) Dependency Review
- ✅ Run `cargo-deny`
- 📦 Check license compliance
- 🚫 Detect banned dependencies
- ⚠️ Warn on multiple versions

#### f) Build Release (main branch only)
- ✅ Build release binaries
- 📦 Upload artifacts (30 days retention)
- 🎯 Ensures release builds work

**Status Badge:**
```markdown
![CI](https://github.com/YOUR_USERNAME/buttre/workflows/CI/badge.svg)
```

---

### 2. **Release Workflow** (`.github/workflows/release.yml`)

**Triggers:**
- Push tags matching `v*.*.*` (e.g., `v0.6.0`)
- Manual workflow dispatch

**Jobs:**

#### a) Create Release
- 📝 Create GitHub release (draft mode)
- 🏷️ Extract version from tag

#### b) Build Windows
- ✅ Build release binary
- ✅ Run tests on release build
- 📦 Package with README, LICENSE, keyboards/
- 📤 Upload ZIP to release
- 💾 Store as artifact

**Usage:**
```bash
# Create and push a version tag
git tag v0.6.0
git push origin v0.6.0

# Or create release via GitHub UI
```

---

### 3. **Benchmark Workflow** (`.github/workflows/bench.yml`)

**Triggers:**
- Push to `main`
- Pull requests to `main`
- Manual workflow dispatch

**Jobs:**

#### a) Run Benchmarks
- ⚡ Execute `cargo bench`
- 📊 Track performance over time
- 🚨 Alert on >150% regression
- 💬 Comment on PRs if performance degrades

**Usage:**
```bash
# Run benchmarks locally
cargo bench --workspace

# View results
open target/criterion/report/index.html
```

---

## 🔧 Setup Instructions

### Required Secrets

Set these in **Settings → Secrets and variables → Actions**:

| Secret | Purpose | How to Get |
|--------|---------|-----------|
| `CODECOV_TOKEN` | Upload coverage | 1. Go to [codecov.io](https://codecov.io)<br>2. Sign in with GitHub<br>3. Add repository<br>4. Copy token |

### Optional Secrets

| Secret | Purpose |
|--------|---------|
| `DEPLOY_KEY` | Future: Automated deployment |
| `SIGNING_KEY` | Future: Code signing |

---

## 📊 Badges

Add these to your `README.md`:

```markdown
[![CI](https://github.com/YOUR_USERNAME/buttre/workflows/CI/badge.svg)](https://github.com/YOUR_USERNAME/buttre/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/YOUR_USERNAME/buttre/branch/main/graph/badge.svg)](https://codecov.io/gh/YOUR_USERNAME/buttre)
[![Security audit](https://github.com/YOUR_USERNAME/buttre/workflows/Security%20audit/badge.svg)](https://github.com/YOUR_USERNAME/buttre/actions)
```

---

## 🛠️ Local Development

### Run CI checks locally before pushing:

```bash
# 1. Format check
cargo fmt --all -- --check

# 2. Lint check
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 3. Build
cargo build --workspace --all-features

# 4. Test
cargo test --workspace --all-features

# 5. Doc tests
cargo test --workspace --doc

# 6. Security audit (install first: cargo install cargo-audit)
cargo audit

# 7. Dependency check (install first: cargo install cargo-deny)
cargo deny check

# All in one
./scripts/ci-local.sh  # (create this script)
```

### Create `scripts/ci-local.sh`:

```bash
#!/bin/bash
set -e

echo "🔍 Running local CI checks..."

echo "📝 Checking formatting..."
cargo fmt --all -- --check

echo "🔎 Running clippy..."
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "🏗️ Building..."
cargo build --workspace --all-features

echo "🧪 Running tests..."
cargo test --workspace --all-features

echo "📚 Running doc tests..."
cargo test --workspace --doc

echo "🔒 Security audit..."
cargo audit || echo "⚠️ Security issues found"

echo "📦 Dependency check..."
cargo deny check || echo "⚠️ Dependency issues found"

echo "✅ All local CI checks passed!"
```

---

## 📈 Coverage Goals

| Crate | Current | Target | Status |
|-------|---------|--------|--------|
| buttre-engine | ~40% | 90% | 🟡 In Progress |
| buttre-core | ~30% | 80% | 🟡 In Progress |
| buttre-platform | ~10% | 60% | 🔴 Needs Work |
| **Overall** | ~30% | **80%** | 🟡 In Progress |

---

## 🚀 Deployment Process

### Manual Release (Current)

1. Update version in `Cargo.toml`:
   ```toml
   [package]
   version = "0.6.0"
   ```

2. Update `CHANGELOG.md`:
   ```markdown
   ## [0.6.0] - 2025-12-25
   ### Added
   - Feature X
   ### Fixed
   - Bug Y
   ```

3. Commit changes:
   ```bash
   git commit -am "chore: bump version to 0.6.0"
   ```

4. Create and push tag:
   ```bash
   git tag -a v0.6.0 -m "Release v0.6.0"
   git push origin v0.6.0
   ```

5. GitHub Actions will:
   - Create draft release
   - Build Windows binary
   - Upload artifacts

6. Edit release notes in GitHub UI and publish

### Automated Release (Future)

- [ ] Use [release-please](https://github.com/googleapis/release-please)
- [ ] Auto-generate changelog from commits
- [ ] Auto-bump version based on conventional commits
- [ ] Auto-publish releases

---

## 🔍 Monitoring

### GitHub Actions Dashboard

View workflow runs:
```
https://github.com/YOUR_USERNAME/buttre/actions
```

### Code Coverage Dashboard

View coverage reports:
```
https://codecov.io/gh/YOUR_USERNAME/buttre
```

### Security Advisories

View security alerts:
```
https://github.com/YOUR_USERNAME/buttre/security
```

---

## 🐛 Troubleshooting

### CI Failing on Warnings

**Problem**: `cargo clippy` fails with warnings
**Solution**: 
```bash
cargo clippy --workspace --fix --allow-dirty
cargo fmt --all
```

### Coverage Upload Fails

**Problem**: Codecov upload fails
**Solution**: 
1. Check `CODECOV_TOKEN` secret is set
2. Verify repository is added to Codecov
3. Check network/firewall issues

### Benchmark Alerts Noisy

**Problem**: Too many performance regression alerts
**Solution**: Adjust threshold in `bench.yml`:
```yaml
alert-threshold: '200%'  # More lenient
```

### Release Build Fails

**Problem**: Release workflow fails
**Solution**:
1. Test release build locally: `cargo build --release`
2. Check all tests pass: `cargo test --release`
3. Verify tag format: `v0.6.0` (not `0.6.0`)

---

## 📚 Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
- [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)
- [Codecov](https://docs.codecov.io/)

---

## 🔄 Continuous Improvement

### Planned Enhancements

- [ ] Add macOS build to CI
- [ ] Add Linux build to CI
- [ ] Add automated changelog generation
- [ ] Add automated version bumping
- [ ] Add integration tests in CI
- [ ] Add E2E tests in CI
- [ ] Add performance regression tests
- [ ] Add fuzz testing in CI
- [ ] Add dependency update automation (Dependabot)
- [ ] Add automated security scanning (Snyk)

---

*Last updated: 2025-12-25*
