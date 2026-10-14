//! Library API for `merge-guard`: load a Dogwood policy and decide shell:spawn requests.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use policy::{Decision, GovernedBox, Operator, Policy, PolicyEngine, Principal, Request, RuleId};

/// Result of `merge-guard check`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckOutcome {
    /// Whether the policy allowed the command.
    pub allowed: bool,
    /// Matching rule `@id` or default-deny sentinel.
    pub rule: String,
    /// One-line CLI output.
    pub summary: String,
}

impl CheckOutcome {
    /// Printable ALLOW/DENY line.
    pub fn line(&self) -> &str {
        &self.summary
    }
}

/// Environment passed to a guarded child: host env minus `GITHUB_TOKEN`.
pub fn guarded_env() -> Vec<(String, String)> {
    std::env::vars()
        .filter(|(key, _)| key != "GITHUB_TOKEN")
        .collect()
}

/// Resolve a program name to a path spelling for `shell:spawn` context.
pub fn resolve_program_path(program: &str) -> String {
    if program.contains('/') {
        return program.to_string();
    }
    if let Ok(path) = which(program) {
        return path;
    }
    format!("/usr/bin/{program}")
}

fn which(program: &str) -> Result<String, ()> {
    let path = std::env::var_os("PATH").ok_or(())?;
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join(program);
        if candidate.is_file() {
            return Ok(candidate.to_string_lossy().into_owned());
        }
    }
    Err(())
}

/// Open a policy engine from a `.dw` file. The `TempDir` must outlive the engine.
pub fn open_engine(policy_path: &Path) -> Result<(PolicyEngine, tempfile::TempDir)> {
    let text = std::fs::read_to_string(policy_path)
        .with_context(|| format!("read policy {}", policy_path.display()))?;
    let sources = vec![Policy {
        origin: policy_path.to_path_buf(),
        text,
    }];
    PolicyEngine::validate(&Operator::unanchored(), &sources)
        .map_err(|error| anyhow::anyhow!("policy validation failed: {error}"))?;
    let history = tempfile::tempdir().context("create policy history directory")?;
    let engine = PolicyEngine::open(sources, &history.path().join("dogwood.redb"))
        .map_err(|error| anyhow::anyhow!("policy open failed: {error}"))?;
    Ok((engine, history))
}

/// Decide a shell:spawn request for `argv`.
pub fn decide_spawn(engine: &PolicyEngine, argv: &[String], cwd: &str) -> Result<Decision> {
    if argv.is_empty() {
        bail!("empty command");
    }
    let program = argv[0].as_str();
    let args: Vec<String> = argv[1..].to_vec();
    let command = argv.join(" ");
    let program_path = resolve_program_path(program);
    let credential_reads: &[String] = &[];
    let governed = GovernedBox::assigned("merge-guard");
    let principal = Principal::agent();
    let request = Request::ShellSpawn {
        command: &command,
        program,
        program_path: &program_path,
        credential_reads,
        args: &args,
        cwd,
    };
    Ok(engine.decide(&governed, &principal, &request))
}

/// Human rule label for a decision.
pub fn rule_label(decision: &Decision) -> String {
    if let Some(attr) = decision.attribution().first() {
        if let Some(id) = attr.annotation_id.as_deref().filter(|s| !s.is_empty()) {
            return id.to_string();
        }
        return attr.rule.as_str().to_string();
    }
    if decision.is_allow() {
        "permit".to_string()
    } else {
        RuleId::DEFAULT_DENY.to_string()
    }
}

/// Evaluate `argv` against `policy_path`.
pub fn check_command(policy_path: &Path, argv: &[String]) -> Result<CheckOutcome> {
    let cwd = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .to_string_lossy()
        .into_owned();
    let (engine, _history) = open_engine(policy_path)?;
    let decision = decide_spawn(&engine, argv, &cwd)?;
    let allowed = decision.is_allow();
    let rule = rule_label(&decision);
    let verb = if allowed { "ALLOW" } else { "DENY" };
    let summary = format!("{verb} rule={rule} command={}", argv.join(" "));
    Ok(CheckOutcome {
        allowed,
        rule,
        summary,
    })
}

/// Workspace-relative path to a policy file (tests and demos).
pub fn policy_path(name: &str) -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../policies")
        .join(name)
        .canonicalize()
        .unwrap_or_else(|_| manifest.join("../../policies").join(name))
}
