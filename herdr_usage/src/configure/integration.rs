//! Herdr agent integration setup and diagnostics for Codex and Agy.

use crate::model::Harness;
use std::process::Command;

/// Herdr's integration id for a harness, when it has one.
///
/// Agy quota comes from the statusLine hook, not Herdr's session id.
fn integration_id(harness: Harness) -> Option<&'static str> {
    match harness {
        Harness::Codex => Some("codex"),
        Harness::Agy => None,
    }
}

pub fn report_missing(agents: &[Harness]) {
    let Some(status) = read_status() else {
        return;
    };
    for harness in agents {
        let Some(id) = integration_id(*harness) else {
            continue;
        };
        if !is_missing(&status, id) {
            continue;
        }
        println!(
            "Herdr's {id} integration is not installed, so Herdr reports no session id for {id} panes and their quota cannot be attributed. Install it with `herdr integration install {id}`, then restart that agent pane."
        );
    }
}

fn read_status() -> Option<String> {
    let executable = std::env::var_os("HERDR_BIN_PATH").unwrap_or_else(|| "herdr".into());
    let output = Command::new(executable)
        .args(["integration", "status"])
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

fn is_missing(status: &str, id: &str) -> bool {
    status.lines().any(|line| {
        line.trim()
            .strip_prefix(id)
            .and_then(|rest| rest.strip_prefix(':'))
            .is_some_and(|state| state.trim_start().starts_with("not installed"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const STATUS: &str = "\
codex: current (v7) (/home/u/.codex/herdr-agent-state.sh)
other: not installed
";

    #[test]
    fn only_an_explicit_not_installed_line_is_reported() {
        assert!(is_missing(STATUS, "other"));
        assert!(!is_missing(STATUS, "codex"));
        assert!(!is_missing("", "codex"));
    }

    #[test]
    fn session_backed_harnesses_report_their_integration_id() {
        assert_eq!(integration_id(Harness::Agy), None);
        assert_eq!(integration_id(Harness::Codex), Some("codex"));
    }
}
