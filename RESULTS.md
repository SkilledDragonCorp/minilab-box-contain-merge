# RESULTS

Upstream pin: `223e3959d539110eaacfacb6f973c03928dc39b0`  
Verified by CI: `git -C upstream rev-parse HEAD` == `223e3959d539110eaacfacb6f973c03928dc39b0`

| Suite | Result | Pass count | Duration | Notes |
|---|---|---|---|---|
| Lab `cargo test --workspace --locked` | pass | 3 integration | ~23s (CI test job) | Table (≥25 cmds), parse, GITHUB_TOKEN strip |
| Lab lint / clippy / banned phrases | pass | — | ~22s | |
| Lab macos-live | pass | — | ~37s | `macos-15` aarch64 cfg |
| Lab secrets (gitleaks v8.30.1) | pass | — | ~6s | checksum-verified |
| Lab audit | pass | — | ~2m15s | ignores RUSTSEC-2026-0249 (transitive smartstring) |
| Lab CI Gate | pass | — | ~3s | |
| Upstream unit-test (ubuntu-latest) | pass | full workspace minus namespace skips | see Actions | Skips as upstream `Run tests (linux)` |
| Upstream unit-test (macos-latest) | pass | full workspace | see Actions | Advisory `continue-on-error` kept; job succeeded |
| Upstream lint | pass | — | ~55s | fmt + clippy owned crates |
| Upstream parity + pin | pass | — | ~19s / ~6s | |
| containment (upstream ci.yml) | not run (out of lab scope) | — | — | Calls containment-deterministic.yml |
| verdict (upstream ci.yml) | not run (out of lab scope) | — | — | Calls verdict-hermetic.yml |
| upstream ci-gate | not run (out of lab scope) | — | — | alls-green over containment/verdict |

Local smoke:

```text
ALLOW rule=git_status command=git status
DENY rule=forbid_git_push command=git push origin main
```

Lab CI: https://github.com/SkilledDragonCorp/minilab-box-contain-merge/actions/runs/38525497616  
Upstream CI: https://github.com/SkilledDragonCorp/minilab-box-contain-merge/actions/runs/38525497805
