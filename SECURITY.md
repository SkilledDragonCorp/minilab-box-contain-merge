# Security Policy

## Supported versions

This educational minilab tracks a single `main` branch. Only the latest commit on `main` receives fixes.

## Reporting a vulnerability

Email **hello@skilleddragon.com**. This repository is an educational minilab, not a vulnerability bounty program, and is not regularly maintained.

## Secret handling

- Secrets come from process environment variables or a local gitignored `.env` only.
- Never commit `.env`, tokens, private keys, or credential files.
- `GITHUB_TOKEN` (optional) is injected by Strands Box from the host into egress destinations named in `examples/box.toml`. It must not appear in `[agent.env]` or in the environment `merge-guard` would hand a guarded child (`guarded_env()` strips it).
- CI non-secret env: `CARGO_TERM_COLOR=always`, `RUSTFLAGS=-C debuginfo=0`.

## Gitleaks

- Pre-commit hook and CI use **gitleaks v8.30.1**.
- CI downloads `gitleaks_8.30.1_linux_x64.tar.gz` and verifies SHA256 `551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb` from the release checksums file, then runs `gitleaks git --redact --verbose .`.

## Dependency audit

```sh
cargo install cargo-audit --locked --version 0.22.2
cargo audit --deny warnings --ignore RUSTSEC-2026-0249
```

`RUSTSEC-2026-0249` (`smartstring` unmaintained) is transitive through `strands-box-policy` → cedar/dogwood at pin `223e3959d539110eaacfacb6f973c03928dc39b0`. The lab does not vendor a replacement for that pin.

## Upstream

- Repository: https://github.com/strands-agents/box
- Pin: `223e3959d539110eaacfacb6f973c03928dc39b0` (v0.1.0)
- License: Apache-2.0
