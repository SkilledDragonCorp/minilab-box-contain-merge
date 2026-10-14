//! Table-driven policy checks across merge-locked and merge-unlocked.

use merge_guard::{guarded_env, open_engine, policy_path};

struct Case {
    policy: &'static str,
    argv: &'static [&'static str],
    allow: bool,
    rule_contains: &'static str,
}

const CASES: &[Case] = &[
    // --- merge-locked allows ---
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "status"],
        allow: true,
        rule_contains: "git_status",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "diff"],
        allow: true,
        rule_contains: "git_diff",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "log", "--oneline", "-n", "5"],
        allow: true,
        rule_contains: "git_log",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "fetch", "origin"],
        allow: true,
        rule_contains: "git_fetch",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "show", "HEAD"],
        allow: true,
        rule_contains: "git_show",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "rev-parse", "HEAD"],
        allow: true,
        rule_contains: "git_rev_parse",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "branch"],
        allow: true,
        rule_contains: "git_branch_list",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["gh", "pr", "view", "1"],
        allow: true,
        rule_contains: "gh_pr_readonly",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["gh", "pr", "list"],
        allow: true,
        rule_contains: "gh_pr_readonly",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["gh", "pr", "checks", "1"],
        allow: true,
        rule_contains: "gh_pr_readonly",
    },
    // --- merge-locked denies ---
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "push"],
        allow: false,
        rule_contains: "forbid_git_push",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "push", "origin", "main"],
        allow: false,
        rule_contains: "forbid_git_push",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "push", "origin", "feature/x"],
        allow: false,
        rule_contains: "forbid_git_push",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "merge", "main"],
        allow: false,
        rule_contains: "forbid_git_merge",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "rebase", "main"],
        allow: false,
        rule_contains: "forbid_git_rebase_main",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "rebase", "origin/main"],
        allow: false,
        rule_contains: "forbid_git_rebase_main",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["gh", "pr", "merge", "42"],
        allow: false,
        rule_contains: "forbid_gh_pr_merge",
    },
    Case {
        policy: "merge-locked.dw",
        argv: &["git", "commit", "-m", "x"],
        allow: false,
        rule_contains: "<default-deny>",
    },
    // --- merge-unlocked ---
    Case {
        policy: "merge-unlocked.dw",
        argv: &["git", "status"],
        allow: true,
        rule_contains: "git_status",
    },
    Case {
        policy: "merge-unlocked.dw",
        argv: &["git", "push", "origin", "feature/x"],
        allow: true,
        rule_contains: "git_push_non_main",
    },
    Case {
        policy: "merge-unlocked.dw",
        argv: &["git", "push", "origin", "main"],
        allow: false,
        rule_contains: "forbid_git_push_main",
    },
    Case {
        policy: "merge-unlocked.dw",
        argv: &["git", "push"],
        allow: false,
        rule_contains: "forbid_git_push_bare",
    },
    Case {
        policy: "merge-unlocked.dw",
        argv: &["git", "merge", "feature/x"],
        allow: false,
        rule_contains: "forbid_git_merge",
    },
    Case {
        policy: "merge-unlocked.dw",
        argv: &["gh", "pr", "merge", "7"],
        allow: false,
        rule_contains: "forbid_gh_pr_merge",
    },
    Case {
        policy: "merge-unlocked.dw",
        argv: &["git", "diff", "HEAD~1"],
        allow: true,
        rule_contains: "git_diff",
    },
];

#[test]
fn table_driven_commands_match_expected_verdicts() {
    use merge_guard::{decide_spawn, open_engine, rule_label};
    use std::collections::HashMap;

    assert!(
        CASES.len() >= 20,
        "expected at least 20 table cases, got {}",
        CASES.len()
    );

    let cwd = std::env::current_dir()
        .expect("cwd")
        .to_string_lossy()
        .into_owned();
    let mut engines = HashMap::new();
    for case in CASES {
        if !engines.contains_key(case.policy) {
            let path = policy_path(case.policy);
            let opened = open_engine(&path)
                .unwrap_or_else(|error| panic!("open {}: {error:#}", case.policy));
            engines.insert(case.policy, opened);
        }
        let (engine, _) = engines.get(case.policy).expect("engine");
        let argv: Vec<String> = case.argv.iter().map(|s| (*s).to_string()).collect();
        let decision = decide_spawn(engine, &argv, &cwd).unwrap_or_else(|error| {
            panic!(
                "decide failed for {} {:?}: {error:#}",
                case.policy, case.argv
            )
        });
        let allowed = decision.is_allow();
        let rule = rule_label(&decision);
        assert_eq!(
            allowed, case.allow,
            "policy={} argv={:?} rule={rule}",
            case.policy, case.argv
        );
        assert!(
            rule.contains(case.rule_contains),
            "policy={} argv={:?} rule={rule} expected to contain {}",
            case.policy,
            case.argv,
            case.rule_contains
        );
    }
}

#[test]
fn policy_files_parse_with_pinned_crate() {
    for name in ["merge-locked.dw", "merge-unlocked.dw"] {
        let path = policy_path(name);
        let (_engine, _history) =
            open_engine(&path).unwrap_or_else(|error| panic!("open {name}: {error:#}"));
    }
}

#[test]
fn github_token_absent_from_guarded_env() {
    // SAFETY: test process owns GITHUB_TOKEN for this assertion.
    unsafe {
        std::env::set_var("GITHUB_TOKEN", "should-never-reach-agent");
    }
    let env = guarded_env();
    assert!(
        env.iter().all(|(key, _)| key != "GITHUB_TOKEN"),
        "GITHUB_TOKEN must not appear in the guarded environment"
    );
    unsafe {
        std::env::remove_var("GITHUB_TOKEN");
    }
}
