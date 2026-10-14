---
name: authoring-merge-policy
description: Author or extend a Dogwood merge policy that keeps git push/merge and gh pr merge locked until gates pass; verify each change with merge-guard check.
---

# Authoring a merge-lock Dogwood policy

Use this skill when an operator wants an agent able to inspect a repo but unable to land merges until CI and review gates pass.

## Intent

- Default deny.
- Permit read-only git: `status`, `diff`, `log`, `fetch`, `show`, `rev-parse`, `branch` (list).
- Permit read-only `gh pr view|list|status|checks|diff`.
- Forbid `git push`, `git merge`, `git rebase` onto main/master, and `gh pr merge` while locked.
- After unlock (`policies/merge-unlocked.dw`), permit `git push` only to non-main refs.

## Files

- Locked pack: `policies/merge-locked.dw`
- Unlocked pack: `policies/merge-unlocked.dw`
- Example box config: `examples/box.toml` (loads merge-locked; injects `GITHUB_TOKEN` via egress only)

## Verify every change

From the repo root:

```sh
cargo run -p merge-guard --locked -- check --policy policies/merge-locked.dw -- git status
cargo run -p merge-guard --locked -- check --policy policies/merge-locked.dw -- git push origin main
cargo run -p merge-guard --locked -- check --policy policies/merge-locked.dw -- gh pr merge 1
cargo run -p merge-guard --locked -- check --policy policies/merge-unlocked.dw -- git push origin feature/x
cargo run -p merge-guard --locked -- check --policy policies/merge-unlocked.dw -- git push origin main
```

Expected: `ALLOW` for read-only commands under both policies; `DENY` for push/merge under locked; `ALLOW` for non-main push under unlocked; `DENY` for main push under unlocked.

## Authoring rules

1. Prefer `shell:spawn` for host binaries (`git`, `gh`).
2. Match `context.input.program` and `arg1`/`arg2`; use `context.input.command like "..."` when the ref sits past arg2 (`git push origin main`).
3. Give every `forbid` an `@id` and `@description` the agent can act on.
4. Re-run the `merge-guard check` commands above after every edit.
5. Do not put `GITHUB_TOKEN` in `[agent.env]`.

## Preview caveat

Strands Box local sandbox execution is a preview supported on macOS Apple silicon only today; Linux support is planned.
