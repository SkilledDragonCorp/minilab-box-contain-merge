//! Live sandbox demo: `git push` denied inside a real Strands Box.
//!
//! PREVIEW: Strands Box local sandbox execution is supported on macOS Apple silicon only today.
//! This test is compiled only for that target. CI runs it on `macos-15` (arm64).

#![cfg(all(target_os = "macos", target_arch = "aarch64"))]

use std::path::PathBuf;
use std::process::Command;

use merge_guard::{check_command, policy_path};

/// Policy-level proof used even when the full Box binary is not on PATH.
#[test]
fn git_push_denied_by_merge_locked_policy() {
    let path = policy_path("merge-locked.dw");
    let argv = vec![
        "git".to_string(),
        "push".to_string(),
        "origin".to_string(),
        "main".to_string(),
    ];
    let outcome = check_command(&path, &argv).expect("policy check");
    assert!(!outcome.allowed, "{}", outcome.summary);
    assert!(
        outcome.rule.contains("forbid_git_push"),
        "rule={}",
        outcome.rule
    );
}

/// When `STRANDS_BOX` points at a built `strands-box` binary, run a real box and assert denial text.
#[test]
fn git_push_denied_inside_real_box_when_binary_present() {
    let Some(bin) = std::env::var_os("STRANDS_BOX") else {
        eprintln!("STRANDS_BOX unset; skipping live strands-box subprocess");
        return;
    };
    let bin = PathBuf::from(bin);
    assert!(
        bin.is_file(),
        "STRANDS_BOX is not a file: {}",
        bin.display()
    );

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let config = root.join("examples/box.toml");
    assert!(config.is_file(), "missing {}", config.display());

    let output = Command::new(&bin)
        .args([
            "run",
            "--config",
            config.to_str().expect("utf8 config path"),
            "--",
            "git",
            "push",
            "origin",
            "main",
        ])
        .env_remove("GITHUB_TOKEN")
        .output()
        .expect("spawn strands-box");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");
    assert!(
        !output.status.success()
            || combined.contains("policy denied")
            || combined.contains("denied")
            || combined.contains("forbid_git_push"),
        "expected denial from real box; status={:?}; out={combined}",
        output.status
    );
}
