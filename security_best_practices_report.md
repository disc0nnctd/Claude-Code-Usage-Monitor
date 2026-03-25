# Security Best Practices Report

## Executive Summary
A vulnerability review was completed and all previously identified findings were addressed in code and CI workflow.

- `cargo audit` reported no known RustSec advisories for the current lockfile.
- Self-update and release-pipeline trust controls were hardened.
- Local execution/token exfiltration risk paths were constrained with explicit allowlists.

## Remediated Findings

### SBP-001 (Previously Critical): Unverified update artifact execution
- Status: Fixed
- Changes:
  - Update asset must match exact release filename.
  - Update URL must match expected repository release download path.
  - Download size guard added.
  - PE header validation added.
  - Authenticode publisher verification added (expected subject token: `Code Zeno`).
  - Optional override env var for development/testing: `CCUM_ALLOW_UNSIGNED_UPDATE`.
- Evidence:
  - `src/updater.rs`

### SBP-002 (Previously High): PATH hijack of `claude`/`codex` command resolution
- Status: Fixed
- Changes:
  - Resolvers now require absolute existing executables under trusted roots.
  - No trusted path -> fail closed.
- Evidence:
  - `src/poller.rs`
  - `src/codex_poller.rs`

### SBP-003 (Previously Medium): Unquoted startup command in Run key
- Status: Fixed
- Changes:
  - Startup command written in quoted form.
  - Startup detection supports legacy unquoted and new quoted forms.
- Evidence:
  - `src/window.rs`

### SBP-004 (Previously Medium): Bearer token can be sent to arbitrary `chatgpt_base_url`
- Status: Fixed
- Changes:
  - Base URL now restricted to `https://chatgpt.com` and subdomains by default.
  - Unsafe configured value is ignored with fallback to safe default.
  - Explicit override env var available: `CCUM_ALLOW_UNTRUSTED_CODEX_BASE_URL`.
- Evidence:
  - `src/codex_poller.rs`

### SBP-005 (Previously Low): CI supply-chain pinning gaps
- Status: Fixed
- Changes:
  - GitHub Actions pinned to immutable SHAs.
  - `wingetcreate` download pinned to explicit version.
  - SHA256 verification added for downloaded `wingetcreate.exe`.
- Evidence:
  - `.github/workflows/release.yml`

## Dependency Scan
- Command: `cargo audit`
- Result: No known RustSec advisories in current dependency graph.

## Residual Risk
- Authenticode verification depends on expected signer token and local Windows trust store behavior.
- Threat modeling and runtime fuzzing were out of scope for this pass.
