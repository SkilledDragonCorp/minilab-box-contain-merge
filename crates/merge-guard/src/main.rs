//! `merge-guard` CLI.

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use merge_guard::check_command;

#[derive(Debug, Parser)]
#[command(
    name = "merge-guard",
    about = "Contain agent merges with Strands Box + Dogwood policy"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Evaluate a command against a Dogwood policy and print ALLOW or DENY.
    Check {
        /// Path to a `.dw` policy file.
        #[arg(long)]
        policy: PathBuf,

        /// Command and arguments after `--`.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
        command: Vec<String>,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("merge-guard: {error:#}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Check { policy, command } => {
            if command.is_empty() {
                bail!("pass a command after `--`");
            }
            // Strip a leading `--` if clap left it in (some clap versions do not).
            let argv: Vec<String> = if command.first().map(String::as_str) == Some("--") {
                command.into_iter().skip(1).collect()
            } else {
                command
            };
            if argv.is_empty() {
                bail!("pass a command after `--`");
            }
            let outcome = check_command(&policy, &argv)?;
            println!("{}", outcome.line());
            Ok(if outcome.allowed {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            })
        }
    }
}
