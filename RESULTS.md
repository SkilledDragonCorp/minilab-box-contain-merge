# RESULTS

Upstream pin: `223e3959d539110eaacfacb6f973c03928dc39b0`

| Suite | Result | Pass count | Duration | Notes |
|---|---|---|---|---|
| Lab `cargo test --workspace --locked` | pass (local) | 3 integration | ~63s after engine reuse | CI URL filled after green push |
| Upstream unit-test (ubuntu-latest) | pending CI | — | — | Namespace skips as upstream `Run tests (linux)` |
| Upstream unit-test (macos-latest) | pending CI | — | — | Advisory (`continue-on-error`) |
| Upstream lint | pending CI | — | — | fmt + clippy owned crates |
| containment (upstream ci.yml) | not run (out of lab scope) | — | — | Calls containment-deterministic.yml |
| verdict (upstream ci.yml) | not run (out of lab scope) | — | — | Calls verdict-hermetic.yml |
| upstream ci-gate | not run (out of lab scope) | — | — | alls-green over containment/verdict |
| Lab CI Gate | pending CI | — | — | lint, test, macos-live, skills-layout, secrets, audit |

Local smoke:

```text
ALLOW rule=git_status command=git status
DENY rule=forbid_git_push command=git push origin main
```

CI run URL: _pending first green push_
