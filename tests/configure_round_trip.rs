use herdr_agent_quota::cache::CacheStore;
use herdr_agent_quota::cli::AgentSelection;
use herdr_agent_quota::configure::herdr::{add_quota_row, remove_quota_row};
use herdr_agent_quota::model::Harness;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tempfile::tempdir;

fn isolated_plugin_command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_herdr-agent-usage"));
    command.env_remove("HERDR_SOCKET_PATH");
    command
}

fn sidebar_has_status_icon_rules(sidebar: &str) -> bool {
    sidebar.contains("$quota_icon")
        && sidebar.contains("fg = \"#f9e2af\"")
        && sidebar.contains("fg = \"#94e2d5\"")
        && !sidebar.contains("$quota_icon_working")
        && !sidebar.contains("$quota_icon_done")
}

fn report_sets_done_icon(text: &str) -> bool {
    (text.contains("--token quota_icon=") || text.contains(" quota_icon="))
        && text.contains('\u{2060}')
        && !text.contains("quota_icon_done=")
}

fn install_herdr_stub(state: &Path, agent_list: &str) -> (PathBuf, PathBuf) {
    let log = state.join("herdr.log");
    let executable = state.join("herdr");
    fs::write(
        &executable,
        format!(
            "#!/bin/sh\nif [ \"$1 $2\" = \"agent list\" ]; then\n  printf '%s\\n' '{}'\nelif [ \"$1 $2\" = \"pane read\" ]; then\n  printf '%s\\n' \"$*\" >> '{}'\nelif [ \"$1 $2\" = \"pane report-metadata\" ]; then\n  printf '%s\\n' \"$*\" >> '{}'\nelif [ \"$1 $2\" = \"notification show\" ]; then\n  printf '%s\\n' \"$*\" >> '{}'\nfi\n",
            agent_list,
            log.display(),
            log.display(),
            log.display()
        ),
    )
    .unwrap();
    let mut permissions = fs::metadata(&executable).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&executable, permissions).unwrap();
    (executable, log)
}

fn _future_reset_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is after unix epoch")
        .as_secs()
        + 3_600
}

fn _agy_statusline_windows(
    session_id: &str,
    five_hour_used: f64,
    seven_day_used: Option<f64>,
    reset: u64,
) -> String {
    match seven_day_used {
        Some(week_used) => format!(
            r#"{{"session_id":"{session_id}","rate_limits":{{"five_hour":{{"used_percentage":{five_hour_used},"resets_at":{reset}}},"seven_day":{{"used_percentage":{week_used},"resets_at":{reset}}}}}}}"#
        ),
        None => format!(
            r#"{{"session_id":"{session_id}","rate_limits":{{"five_hour":{{"used_percentage":{five_hour_used},"resets_at":{reset}}}}}}}"#
        ),
    }
}

fn run_agy_collector(state: &Path, herdr: &Path, input: &[u8]) {
    let mut command = isolated_plugin_command();
    command
        .arg("agy-statusline")
        .env("HERDR_PLUGIN_STATE_DIR", state)
        .env("HERDR_BIN_PATH", herdr)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());
    let mut child = command.spawn().unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    assert!(child.wait_with_output().unwrap().status.success());
}

fn run_agy_collector_with_timeout(state: &Path, input: &[u8], timeout: Duration) -> bool {
    let mut child = isolated_plugin_command()
        .arg("agy-statusline")
        .env("HERDR_PLUGIN_STATE_DIR", state)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            return status.success();
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return false;
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn hold_refresh_lock_in_child(state: &Path) -> std::process::Child {
    let lock_path = state.join("refresh.lock");
    let ready_path = state.join("refresh.lock.ready");
    let locker = Command::new("perl")
        .args([
            "-e",
            r#"use Fcntl qw(:flock); open my $f, '+>', $ARGV[0] or die $!; flock($f, LOCK_EX) or die $!; open my $r, '>', $ARGV[1] or die $!; print $r 'locked'; close $r; sleep 20"#,
            lock_path.to_str().unwrap(),
            ready_path.to_str().unwrap(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    while !ready_path.exists() {
        assert!(
            Instant::now() < deadline,
            "refresh lock helper did not start"
        );
        thread::sleep(Duration::from_millis(10));
    }
    locker
}

fn pin_compact_layout(state: &Path) {
    CacheStore::new(state)
        .set_sidebar_layout(herdr_agent_quota::cli::SidebarLayout::Packed)
        .unwrap();
}

fn run_agy_refresh(state: &Path, herdr: &Path) {
    pin_compact_layout(state);
    let output = isolated_plugin_command()
        .args(["refresh", "--provider", "agy", "--force"])
        .env("HERDR_PLUGIN_STATE_DIR", state)
        .env("HERDR_BIN_PATH", herdr)
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[test]
fn sidebar_configuration_is_idempotent_and_removes_plugin_rows() {
    let original = "[ui.sidebar.agents]\nrows = [[\"state_icon\", \"agent\"]]\n";
    let applied = add_quota_row(original).unwrap();
    assert!(applied.contains("key = \"prefix+shift+r\""));
    assert!(applied.contains("type = \"plugin_action\""));
    assert!(applied.contains("command = \"herdr-agent-usage.refresh\""));
    assert!(applied.contains("key = \"prefix+shift+q\""));
    assert!(applied.contains("command = \"herdr-agent-usage.open-settings\""));
    assert!(applied.contains("agent_panel_sort = \"spaces\" # herdr-agent-usage"));
    assert_eq!(add_quota_row(&applied).unwrap(), applied);
    assert_eq!(
        remove_quota_row(&applied).unwrap(),
        "[ui]\n[ui.sidebar.agents]\nrows = [[\"state_icon\", \"machine\", \"workspace\", \"tab\"], [\"agent\"]]\n"
    );
}

#[test]
fn sidebar_configuration_preserves_a_conflicting_refresh_key() {
    let original = concat!(
        "[[keys.command]]\n",
        "key = \"prefix+shift+r\"\n",
        "type = \"shell\"\n",
        "command = \"echo user-owned\"\n",
        "description = \"user refresh\"\n\n",
        "[ui.sidebar.agents]\n",
        "rows = [[\"state_icon\", \"agent\"]]\n"
    );
    let applied = add_quota_row(original).unwrap();
    assert_eq!(applied.matches("key = \"prefix+shift+r\"").count(), 1);
    assert!(applied.contains("command = \"echo user-owned\""));
    assert!(!applied.contains("command = \"herdr-agent-usage.refresh\""));
    assert_eq!(
        remove_quota_row(&applied).unwrap(),
        "[[keys.command]]\nkey = \"prefix+shift+r\"\ntype = \"shell\"\ncommand = \"echo user-owned\"\ndescription = \"user refresh\"\n\n[ui]\n\n[ui.sidebar.agents]\nrows = [[\"state_icon\", \"machine\", \"workspace\", \"tab\"], [\"agent\"]]\n"
    );
}

#[test]
fn sidebar_configuration_preserves_a_conflicting_settings_key() {
    let original = concat!(
        "[[keys.command]]\n",
        "key = \"prefix+shift+q\"\n",
        "type = \"shell\"\n",
        "command = \"echo user-owned\"\n",
        "description = \"user settings\"\n\n",
        "[ui.sidebar.agents]\n",
        "rows = [[\"state_icon\", \"agent\"]]\n"
    );
    let applied = add_quota_row(original).unwrap();
    assert_eq!(applied.matches("key = \"prefix+shift+q\"").count(), 1);
    assert!(applied.contains("command = \"echo user-owned\""));
    assert!(!applied.contains("command = \"herdr-agent-usage.open-settings\""));
    assert!(applied.contains("command = \"herdr-agent-usage.refresh\""));
    let removed = remove_quota_row(&applied).unwrap();
    assert!(removed.contains("command = \"echo user-owned\""));
    assert!(!removed.contains("herdr-agent-usage.refresh"));
}

#[test]
fn default_herdr_rows_become_plane_provider_usage_and_topic_lines() {
    let original = concat!(
        "[ui.sidebar.agents]\n",
        "rows = [[\"state_icon\", \"workspace\", \"tab\"], [\"agent\"]]\n"
    );
    let applied = add_quota_row(original).unwrap();
    assert!(applied.contains("$quota_provider_model"));
    assert!(applied.contains("bold = true"));
    for base in ["$quota_5h", "$quota_week", "$quota_week_inline"] {
        for band in ["normal", "warning", "danger", "unknown"] {
            assert!(applied.contains(&format!("{base}_{band}")), "{base}_{band}");
        }
        // `Severity` has no caution band, so no token can ever fill this row.
        assert!(
            !applied.contains(&format!("{base}_caution")),
            "{base}_caution is unreachable and must not be configured"
        );
    }
    assert!(!applied.contains("[\"$quota_summary\"]"));
    assert!(applied.contains("$quota_topic"));
    assert!(applied.contains("$quota_context"));
    assert!(applied.contains("$quota_provider_model"));
    assert!(!applied.contains("fg = \"#969eae\""));
    assert!(applied.find("$quota_provider_model").unwrap() < applied.find("$quota_topic").unwrap());
    assert!(applied.contains("$quota_cache"));
    assert!(applied.contains("$quota_cache_ttl"));
    assert!(applied.contains("$quota_cache_state"));
    assert!(!applied.contains("$quota_5h_label"));
    assert!(!applied.contains("$quota_5h_eta"));
    assert!(!applied.contains("fg = \"#c8cdd6\""));
    assert!(applied.contains("row_gap = 0 # herdr-agent-usage"));
    assert!(applied.find("$quota_topic").unwrap() < applied.find("$quota_5h_normal").unwrap());
    assert!(applied.contains("fg = \"#82d978\""));
    assert!(applied.contains("fg = \"#e4b957\""));
    assert!(!applied.contains("fg = \"#c6d768\""));
    assert!(!applied.contains("fg = \"#e2bd58\""));
    assert!(applied.contains("fg = \"#f16f7e\""));
    assert!(!applied.contains("fg = \"#eceef2\""));
    assert!(!applied.contains("selection_bg"));
    assert!(!applied.contains("active_row_bg"));
    assert!(!applied.contains("[ui.sidebar.agents.rows_by_agent]"));
    assert!(sidebar_has_status_icon_rules(&applied), "{applied}");
    assert!(applied.contains("fg = \"#e9e9f0\""));
    assert!(applied.contains("fg = \"#f9e2af\""));
    assert!(applied.contains("fg = \"#94e2d5\""));
}

#[test]
fn non_semantic_text_inherits_the_active_herdr_theme() {
    let applied =
        add_quota_row("[ui.sidebar.agents]\nrows = [[\"state_icon\", \"tab\", \"agent\"]]\n")
            .unwrap();
    let document = applied.parse::<toml_edit::DocumentMut>().unwrap();
    let rows = document["ui"]["sidebar"]["agents"]["rows"]
        .as_array()
        .unwrap();
    let inherited = [
        "$quota_topic",
        "$quota_cache",
        "$quota_cache_ttl",
        "$quota_context",
        "$quota_5h_unknown",
        "$quota_week_unknown",
        "$quota_week_inline_unknown",
    ];

    for token in inherited {
        let style = rows
            .iter()
            .filter_map(toml_edit::Value::as_array)
            .flat_map(|row| row.iter())
            .find(|item| configured_token(item) == Some(token))
            .and_then(toml_edit::Value::as_inline_table)
            .unwrap_or_else(|| panic!("missing styled token {token}"));
        assert!(
            !style.contains_key("fg"),
            "{token} must inherit Herdr's foreground: {style}"
        );
    }
}

#[test]
fn context_is_the_penultimate_row_and_model_shares_provider_style() {
    let applied =
        add_quota_row("[ui.sidebar.agents]\nrows = [[\"state_icon\", \"agent\"]]\n").unwrap();
    let document = applied.parse::<toml_edit::DocumentMut>().unwrap();
    let agents = &document["ui"]["sidebar"]["agents"];
    let rows = agents["rows"].as_array().unwrap();
    let context_index = rows
        .iter()
        .position(|row| {
            row.as_array().is_some_and(|items| {
                items.iter().any(|item| {
                    item.as_inline_table()
                        .and_then(|table| table.get("token"))
                        .and_then(toml_edit::Value::as_str)
                        .is_some_and(|token| token == "$quota_context")
                })
            })
        })
        .unwrap();
    let limit_index = rows
        .iter()
        .position(|row| {
            row.as_array().is_some_and(|items| {
                items.iter().any(|item| {
                    item.as_inline_table()
                        .and_then(|table| table.get("token"))
                        .and_then(toml_edit::Value::as_str)
                        .is_some_and(|token| token == "$quota_5h_normal")
                })
            })
        })
        .unwrap();
    assert_eq!(context_index + 1, limit_index);
    let nest_gap_index = rows
        .iter()
        .position(|row| {
            row.as_array().is_some_and(|items| {
                items.iter().any(|item| {
                    item.as_inline_table()
                        .and_then(|table| table.get("token"))
                        .and_then(toml_edit::Value::as_str)
                        .is_some_and(|token| token == "$quota_nest_gap")
                })
            })
        })
        .unwrap();
    assert_eq!(limit_index + 1, nest_gap_index);
    assert_eq!(nest_gap_index + 1, rows.len());

    let identity = rows
        .iter()
        .find(|row| row_contains_token(row, "$quota_icon"))
        .and_then(toml_edit::Value::as_array)
        .unwrap();
    assert!(!identity
        .iter()
        .any(|item| item.as_str() == Some("state_icon")));
    assert!(!identity
        .iter()
        .any(|item| configured_token(item) == Some("$quota_icon_working")));
    assert!(!identity
        .iter()
        .any(|item| configured_token(item) == Some("$quota_icon_done")));
    for token in ["$quota_icon", "$quota_provider_model"] {
        let fg = identity
            .iter()
            .find(|item| configured_token(item) == Some(token))
            .and_then(toml_edit::Value::as_inline_table)
            .and_then(|table| table.get("fg"))
            .and_then(toml_edit::Value::as_str);
        assert_eq!(fg, Some("#e9e9f0"), "{token}");
    }
    assert!(agents.get("rows_by_agent").is_none(), "{applied}");
}

#[test]
fn provider_model_is_compact_and_every_provider_can_fold_week_without_five_hour() {
    let applied =
        add_quota_row("[ui.sidebar.agents]\nrows = [[\"state_icon\", \"agent\"]]\n").unwrap();
    let document = applied.parse::<toml_edit::DocumentMut>().unwrap();
    let agents = &document["ui"]["sidebar"]["agents"];
    let rows = agents["rows"].as_array().unwrap();
    let identity_row = rows
        .iter()
        .find(|row| row_contains_token(row, "$quota_provider_model"))
        .unwrap();
    let identity_tokens = identity_row.as_array().unwrap();
    assert_eq!(
        configured_token(identity_tokens.get(0).unwrap()),
        Some("$quota_icon")
    );
    assert!(
        identity_tokens
            .iter()
            .any(|item| configured_token(item) == Some("$quota_icon")),
        "identity row must carry the vendor mark: {identity_row}"
    );
    assert!(!identity_tokens
        .iter()
        .any(|item| configured_token(item) == Some("$quota_icon_working")));
    assert!(!identity_tokens
        .iter()
        .any(|item| configured_token(item) == Some("$quota_icon_done")));
    assert!(!identity_tokens.iter().any(|item| {
        matches!(
            configured_token(item),
            Some("$quota_provider") | Some("$quota_model")
        ) || item.as_str() == Some("state_icon")
    }));

    // Week fold lives on the shared packed rows — there are no per-agent copies.
    assert!(agents.get("rows_by_agent").is_none(), "{applied}");
    let context_row = rows
        .iter()
        .find(|row| row_contains_token(row, "$quota_context"))
        .unwrap()
        .as_array()
        .unwrap();
    assert!(
        context_row
            .iter()
            .any(|item| configured_token(item) == Some("$quota_week_inline_normal")),
        "shared rows should fold 7d onto context when 5h is empty"
    );
    assert!(
        context_row
            .iter()
            .all(|item| configured_token(item) != Some("$quota_5h_normal")),
        "5h must not sit on the context row"
    );
    assert!(
        rows.iter().any(|row| {
            row_contains_token(row, "$quota_week_normal")
                && row_contains_token(row, "$quota_5h_normal")
                && !row_contains_token(row, "$quota_context")
        }),
        "shared rows should keep 5h/7d on a dedicated limits row"
    );
}

#[test]
fn existing_provider_and_model_tokens_are_migrated_to_one_identity_token() {
    let applied = add_quota_row(
        r#"[ui.sidebar.agents]
rows = [["state_icon", "tab", { token = "$quota_provider" }, { token = "$quota_model" }]]
"#,
    )
    .unwrap();
    let document = applied.parse::<toml_edit::DocumentMut>().unwrap();
    let rows = document["ui"]["sidebar"]["agents"]["rows"]
        .as_array()
        .unwrap();
    let identity_row = rows
        .iter()
        .find(|row| row_contains_token(row, "$quota_provider_model"))
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(
        identity_row
            .iter()
            .filter(|item| configured_token(item) == Some("$quota_provider_model"))
            .count(),
        1
    );
    assert!(!identity_row.iter().any(|item| {
        matches!(
            configured_token(item),
            Some("$quota_provider") | Some("$quota_model")
        )
    }));
}

fn configured_token(value: &toml_edit::Value) -> Option<&str> {
    value
        .as_str()
        .or_else(|| value.as_inline_table()?.get("token")?.as_str())
}

fn row_contains_token(row: &toml_edit::Value, token: &str) -> bool {
    row.as_array().is_some_and(|items| {
        items
            .iter()
            .any(|item| configured_token(item) == Some(token))
    })
}

#[test]
fn configuration_removes_obsolete_session_summary_rows() {
    let original = concat!(
        "[ui.sidebar.agents]\n",
        "rows = [[\"state_icon\", \"agent\"], [\"$quota_topic\"], [\"$quota_session\"]]\n"
    );
    let applied = add_quota_row(original).unwrap();
    assert!(applied.contains("$quota_context"));
    assert!(!applied.contains("$quota_session"));
}

#[test]
fn sidebar_configuration_preserves_an_explicit_row_gap() {
    let original = concat!(
        "[ui.sidebar.agents]\n",
        "row_gap = 2\n",
        "rows = [[\"state_icon\", \"agent\"]]\n"
    );
    let applied = add_quota_row(original).unwrap();
    assert!(applied.contains("row_gap = 2"));
    assert!(!applied.contains("row_gap = 1"));
    assert_eq!(
        remove_quota_row(&applied).unwrap(),
        "[ui]\n[ui.sidebar.agents]\nrow_gap = 2\nrows = [[\"state_icon\", \"machine\", \"workspace\", \"tab\"], [\"agent\"]]\n"
    );
}

#[test]
fn sidebar_configuration_keeps_plugin_owned_gap_packed() {
    let original = concat!(
        "[ui.sidebar.agents]\n",
        "row_gap = 1 # herdr-agent-usage\n",
        "rows = [[\"state_icon\", \"agent\"]]\n"
    );
    let applied = add_quota_row(original).unwrap();
    assert!(applied.contains("row_gap = 0 # herdr-agent-usage"));
    assert!(!applied.contains("row_gap = 1"));
}

#[test]
fn agy_collector_is_silent_without_a_previous_statusline() {
    let state = tempdir().unwrap();
    let mut child = isolated_plugin_command()
        .arg("agy-statusline")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(include_bytes!("fixtures/agy/statusline-both.json"))
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn agy_collector_does_not_wait_for_a_refresh_lock() {
    let state = tempdir().unwrap();
    let mut locker = hold_refresh_lock_in_child(state.path());

    assert!(run_agy_collector_with_timeout(
        state.path(),
        include_bytes!("fixtures/agy/statusline-both.json"),
        Duration::from_secs(2),
    ));
    assert!(state
        .path()
        .join("agy-statusline.observation.json")
        .exists());
    let _ = locker.kill();
    let _ = locker.wait();
}

#[test]
fn agy_collector_bounds_a_hanging_previous_statusline() {
    let state = tempdir().unwrap();
    fs::write(
        state.path().join("agy-statusline.original.json"),
        r#"{"type":"command","command":"sleep 20"}"#,
    )
    .unwrap();

    let started = Instant::now();
    assert!(run_agy_collector_with_timeout(
        state.path(),
        include_bytes!("fixtures/agy/statusline-both.json"),
        Duration::from_secs(4),
    ));
    assert!(started.elapsed() < Duration::from_secs(4));
}

#[test]
fn agy_cache_is_published_by_refresh_event() {
    let state = tempdir().unwrap();
    let (herdr_stub, herdr_log) = install_herdr_stub(
        state.path(),
        r#"{"result":{"agents":[{"agent":"agy","pane_id":"w1:p1","agent_session":{"value":"test-session"}}]}}"#,
    );
    run_agy_collector(
        state.path(),
        &herdr_stub,
        br#"{
            "session_id":"test-session",
            "quota": {
                "gemini-5h": {"remaining_percent": 42.0},
                "gemini-weekly": {"remaining_percent": 73.0}
            }
        }"#,
    );
    assert!(!herdr_log.exists());

    run_agy_refresh(state.path(), &herdr_stub);
    let report = fs::read_to_string(herdr_log).unwrap();
    assert!(!report.contains("pane read"));
    assert!(report.contains("quota_5h_warning=5h 42%"));
    assert!(report.contains("quota_week_normal=7d 73%"));
    assert!(!report.contains("quota_5h_label="));
    assert!(!report.contains("quota_week_label="));
}

#[test]
fn agy_statusline_without_rate_limits_clears_stale_quota_windows() {
    let state = tempdir().unwrap();
    let (herdr_stub, _herdr_log) = install_herdr_stub(
        state.path(),
        r#"{"result":{"agents":[{"agent":"agy","pane_id":"w1:p1"}]}}"#,
    );
    run_agy_collector(
        state.path(),
        &herdr_stub,
        br#"{
            "quota": {
                "gemini-5h": {"remaining_fraction": 0.5},
                "gemini-weekly": {"remaining_fraction": 0.5}
            }
        }"#,
    );
    run_agy_collector(
        state.path(),
        &herdr_stub,
        br#"{
            "context_window": {"used_percentage": 43.0},
            "quota": {
                "gemini-weekly": {"remaining_fraction": 0.8},
                "3p-weekly": {"remaining_fraction": 0.7}
            }
        }"#,
    );
    run_agy_refresh(state.path(), &herdr_stub);

    let snapshot: serde_json::Value =
        serde_json::from_slice(&fs::read(state.path().join("agy-statusline.json")).unwrap())
            .unwrap();
    assert_eq!(snapshot["windows"].as_array().unwrap().len(), 0);
    assert_eq!(snapshot["context"]["used_percent"], 43.0);
}

#[test]
fn statusline_without_context_keeps_the_last_context_snapshot() {
    let state = tempdir().unwrap();
    let (herdr_stub, herdr_log) = install_herdr_stub(
        state.path(),
        r#"{"result":{"agents":[{"agent":"agy","pane_id":"w1:p1","agent_session":{"value":"session-1"}}]}}"#,
    );
    run_agy_collector(
        state.path(),
        &herdr_stub,
        br#"{
            "session_id": "session-1",
            "context_window": {"used_percentage": 23.5},
            "quota": {"gemini-weekly": {"remaining_fraction": 0.72}}
        }"#,
    );
    run_agy_collector(
        state.path(),
        &herdr_stub,
        br#"{"session_id":"session-1","quota":{"gemini-weekly":{"remaining_fraction":0.72}}}"#,
    );

    run_agy_refresh(state.path(), &herdr_stub);
    let report = fs::read_to_string(herdr_log).unwrap();
    assert!(report.contains("quota_context=context 24%"));
    assert!(report.contains("quota_week_inline_normal=7d 72%"));
    assert!(!report.contains("quota_week_label="));
    assert!(!report.contains("quota_week_normal="));
}
#[test]
fn quota_refresh_does_not_report_metadata_to_a_scrolled_pane() {
    let state = tempdir().unwrap();
    let log = state.path().join("herdr.log");
    let herdr = state.path().join("herdr");
    fs::write(
        &herdr,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = \"agent list\" ]; then\n  printf '%s\\n' '{}'\nelif [ \"$1 $2\" = \"pane get\" ]; then\n  printf '%s\\n' '{}'\nfi\n",
            log.display(),
            r#"{"result":{"agents":[{"agent":"agy","pane_id":"w1:p1"}]}}"#,
            r#"{"result":{"pane":{"scroll":{"offset_from_bottom":12}}}}"#,
        ),
    )
    .unwrap();
    let mut permissions = fs::metadata(&herdr).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&herdr, permissions).unwrap();

    run_agy_collector(
        state.path(),
        &herdr,
        include_bytes!("fixtures/agy/statusline-both.json"),
    );
    run_agy_refresh(state.path(), &herdr);
    let calls = fs::read_to_string(log).unwrap();
    assert!(calls.contains("pane get w1:p1"));
    assert!(!calls.contains("pane report-metadata"));
}

#[test]
fn a_scrolled_pane_completion_reports_only_icon_tokens() {
    let state = tempdir().unwrap();
    let log = state.path().join("herdr.log");
    let herdr = state.path().join("herdr");
    fs::write(
        &herdr,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = \"agent list\" ]; then\n  printf '%s\\n' '{}'\nelif [ \"$1 $2\" = \"pane get\" ]; then\n  printf '%s\\n' '{}'\nfi\n",
            log.display(),
            r#"{"result":{"agents":[{"agent":"agy","pane_id":"w1:p1","agent_status":"idle","tokens":{"quota_icon_working":"YELLOW","quota_5h_normal":"5h 80%"}}]}}"#,
            r#"{"result":{"pane":{"scroll":{"offset_from_bottom":12}}}}"#,
        ),
    )
    .unwrap();
    chmod_exec(&herdr);
    let output = isolated_plugin_command()
        .arg("event")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env(
            "HERDR_PLUGIN_EVENT_JSON",
            r#"{"event":"pane_agent_status_changed","data":{"pane_id":"w1:p1","agent":"agy","status":"idle"}}"#,
        )
        .output()
        .unwrap();
    assert!(output.status.success());
    let calls = fs::read_to_string(log).unwrap();
    let reports = calls
        .lines()
        .filter(|line| line.contains("pane report-metadata w1:p1"))
        .collect::<Vec<_>>();
    assert_eq!(reports.len(), 1, "{calls}");
    assert!(report_sets_done_icon(reports[0]), "{calls}");
    assert!(!reports[0].contains("quota_5h"), "{calls}");
}

#[test]
fn focus_paints_icons_without_reading_the_pane_or_collectors() {
    let state = tempdir().unwrap();
    let log = state.path().join("herdr.log");
    let herdr = state.path().join("herdr");
    fs::write(
        &herdr,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = \"pane current\" ]; then\n  printf '%s\\n' '{}'\nelif [ \"$1 $2\" = \"agent list\" ]; then\n  printf '%s\\n' '{}'\nelif [ \"$1 $2\" = \"pane get\" ]; then\n  printf '%s\\n' '{}'\nfi\n",
            log.display(),
            r#"{"result":{"pane":{"agent":"codex","pane_id":"w1:p1"}}}"#,
            r#"{"result":{"agents":[{"agent":"codex","pane_id":"w1:p1","agent_status":"idle","tokens":{"quota_icon_done":"TEAL","quota_provider":"Codex","quota_provider_model":"Codex"}}]}}"#,
            r#"{"result":{"pane":{"scroll":{"offset_from_bottom":12}}}}"#,
        ),
    )
    .unwrap();
    let mut permissions = fs::metadata(&herdr).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&herdr, permissions).unwrap();

    let output = isolated_plugin_command()
        .arg("focus")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env_remove("HERDR_PLUGIN_EVENT_JSON")
        .output()
        .unwrap();
    assert!(output.status.success());
    let calls = fs::read_to_string(log).unwrap();
    assert!(calls.contains("pane current"), "{calls}");
    assert!(!calls.contains("pane read"), "{calls}");
    let paint = calls
        .lines()
        .filter(|line| line.contains("pane report-metadata w1:p1"))
        .collect::<Vec<_>>();
    assert!(!paint.is_empty(), "{calls}");
    let joined = paint.join("\n");
    assert!(
        joined.contains("quota_icon=") || joined.contains("--token quota_icon="),
        "{joined}"
    );
    assert!(
        joined.contains("clear-token quota_icon_done")
            || joined.contains("--clear-token quota_icon_done"),
        "{joined}"
    );
    assert!(!joined.contains("quota_provider"), "{joined}");
    assert!(!joined.contains("quota_group"), "{joined}");
}

#[test]
fn agent_event_refreshes_and_reads_topics_only_for_its_provider() {
    let state = tempdir().unwrap();
    let log = state.path().join("herdr.log");
    let codex_log = state.path().join("codex.log");
    let herdr = state.path().join("herdr");
    let codex = state.path().join("codex");
    fs::write(
        &herdr,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = \"agent list\" ]; then\n  printf '%s\\n' '{}'\nelif [ \"$1 $2\" = \"pane get\" ]; then\n  printf '%s\\n' '{}'\nfi\n",
            log.display(),
            r#"{"result":{"agents":[{"agent":"agy","pane_id":"w1:p1"},{"agent":"codex","pane_id":"w1:p2"}]}}"#,
            r#"{"result":{"pane":{"scroll":{"offset_from_bottom":0}}}}"#,
        ),
    )
    .unwrap();
    fs::write(
        &codex,
        format!("#!/bin/sh\nprintf called > '{}'\n", codex_log.display()),
    )
    .unwrap();
    for executable in [&herdr, &codex] {
        let mut permissions = fs::metadata(executable).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(executable, permissions).unwrap();
    }
    run_agy_collector(
        state.path(),
        &herdr,
        include_bytes!("fixtures/agy/statusline-both.json"),
    );

    let output = isolated_plugin_command()
        .arg("event")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .env(
            "HERDR_PLUGIN_EVENT_JSON",
            r#"{"event":{"pane":{"agent":"agy","pane_id":"w1:p1"}}}"#,
        )
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(!codex_log.exists());
    let calls = fs::read_to_string(log).unwrap();
    assert!(calls.contains("pane read w1:p1"));
    assert!(!calls.contains("pane read w1:p2"));
}

fn chmod_exec(path: &Path) {
    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

fn two_agent_inventory_with_working_codex() -> &'static str {
    r#"{"result":{"agents":[{"agent":"agy","pane_id":"w1:p1","agent_status":"idle"},{"agent":"codex","pane_id":"w1:p2","agent_status":"working"}]}}"#
}

fn install_logged_herdr_and_codex(
    state: &Path,
    agent_list: &str,
    pane_current: Option<&str>,
) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let herdr_log = state.join("herdr.log");
    let codex_log = state.join("codex.log");
    let herdr = state.join("herdr");
    let codex = state.join("codex");
    let pane_current_branch = pane_current
        .map(|json| {
            format!("elif [ \"$1 $2\" = \"pane current\" ]; then\n  printf '%s\\n' '{json}'\n")
        })
        .unwrap_or_default();
    fs::write(
        &herdr,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{log}'\nif [ \"$1 $2\" = \"agent list\" ]; then\n  printf '%s\\n' '{agents}'\n{pane_current}elif [ \"$1 $2\" = \"pane get\" ]; then\n  printf '%s\\n' '{scroll}'\nfi\n",
            log = herdr_log.display(),
            agents = agent_list,
            pane_current = pane_current_branch,
            scroll = r#"{"result":{"pane":{"scroll":{"offset_from_bottom":0}}}}"#,
        ),
    )
    .unwrap();
    fs::write(
        &codex,
        format!("#!/bin/sh\nprintf called > '{}'\n", codex_log.display()),
    )
    .unwrap();
    chmod_exec(&herdr);
    chmod_exec(&codex);
    (herdr, herdr_log, codex, codex_log)
}

fn run_event_binary(
    state: &Path,
    herdr: &Path,
    codex: &Path,
    event_json: &str,
) -> std::process::Output {
    pin_compact_layout(state);
    isolated_plugin_command()
        .arg("event")
        .env("HERDR_PLUGIN_STATE_DIR", state)
        .env("HERDR_BIN_PATH", herdr)
        .env("CODEX_BIN_PATH", codex)
        .env("HERDR_PLUGIN_EVENT_JSON", event_json)
        .output()
        .unwrap()
}

fn assert_no_agent_collection(state: &Path, herdr_log: &Path, codex_log: &Path) {
    assert!(
        !codex_log.exists(),
        "Codex stub was invoked: {}",
        fs::read_to_string(codex_log).unwrap_or_default()
    );
    let calls = fs::read_to_string(herdr_log).unwrap_or_default();
    assert!(
        !calls.contains("pane read"),
        "unexpected pane read: {calls}"
    );
    assert!(
        !calls.lines().any(|line| {
            line.contains("pane report-metadata")
                && (line.contains("quota_5h")
                    || line.contains("quota_week")
                    || line.contains("quota_month")
                    || line.contains("quota_context"))
        }),
        "unexpected quota window write: {calls}"
    );
    for marker in ["codex-app-server.refresh", "agy-statusline.refresh"] {
        assert!(!state.join(marker).exists(), "{marker} was written");
    }
}

#[test]
fn unknown_agent_working_event_does_not_refresh_any_collector() {
    let state = tempdir().unwrap();
    let (herdr, herdr_log, codex, codex_log) = install_logged_herdr_and_codex(
        state.path(),
        two_agent_inventory_with_working_codex(),
        None,
    );

    let output = run_event_binary(
        state.path(),
        &herdr,
        &codex,
        r#"{"event":"pane_agent_status_changed","data":{"pane_id":"w1:p8","agent":"amp","status":"working"}}"#,
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    thread::sleep(Duration::from_millis(200));
    assert_no_agent_collection(state.path(), &herdr_log, &codex_log);
}

#[test]
fn focus_event_uses_its_pane_even_when_the_current_focus_differs() {
    for pane_id in ["w1:p9", "w1:p99"] {
        let state = tempdir().unwrap();
        let (herdr, herdr_log, codex, codex_log) = install_logged_herdr_and_codex(
            state.path(),
            two_agent_inventory_with_working_codex(),
            Some(r#"{"result":{"pane":{"agent":"codex","pane_id":"w1:p2"}}}"#),
        );
        let output = isolated_plugin_command()
            .arg("focus")
            .env("HERDR_PLUGIN_STATE_DIR", state.path())
            .env("HERDR_BIN_PATH", &herdr)
            .env("CODEX_BIN_PATH", &codex)
            .env("HERDR_PLUGIN_EVENT_JSON", format!(
                r#"{{"event":"pane_focused","data":{{"type":"pane_focused","pane_id":"{pane_id}","workspace_id":"w1"}}}}"#
            ))
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let calls = fs::read_to_string(&herdr_log).unwrap_or_default();
        assert!(!calls.contains("pane current"), "{calls}");
        assert!(!calls.contains("pane read"), "{calls}");
        assert!(
            (1..=6).contains(&calls.matches("agent list").count()),
            "{calls}"
        );
        assert_no_agent_collection(state.path(), &herdr_log, &codex_log);
    }
}

#[test]
fn workspace_focus_uses_that_workspaces_layout_and_keeps_other_green_panes() {
    let state = tempdir().unwrap();
    let initial_attention =
        r#"{"working":["w5:pB"],"unseen":["w9:p1","w9:p6"],"last_focused":"w5:pB"}"#;
    fs::write(state.path().join("icon-attention.json"), initial_attention).unwrap();
    let inventory = r#"{"result":{"agents":[
        {"agent":"codex","pane_id":"w5:pB","agent_status":"working","focused":true,
         "tokens":{"quota_icon_working":"YELLOW","quota_provider":"Codex","quota_provider_model":"Codex"}},
        {"agent":"agy","pane_id":"w9:p1","agent_status":"idle","focused":false,
         "tokens":{"quota_icon_done":"GREEN","quota_provider":"Agy","quota_provider_model":"Agy"}},
        {"agent":"agy","pane_id":"w9:p6","agent_status":"idle","focused":false,
         "tokens":{"quota_icon_done":"OTHER_GREEN","quota_provider":"Agy","quota_provider_model":"Agy"}}
    ]}}"#;
    let (herdr, herdr_log, codex, _) = install_logged_herdr_and_codex(
        state.path(),
        inventory,
        Some(r#"{"result":{"pane":{"agent":"codex","pane_id":"w5:pB"}}}"#),
    );
    let snapshot = r#"{"result":{"snapshot":{"focused_workspace_id":"w9","focused_tab_id":"w9:t1","focused_pane_id":"w9:p1","workspaces":[{"workspace_id":"w9","active_tab_id":"w9:t1"}],"layouts":[{"workspace_id":"w9","tab_id":"w9:t1","focused_pane_id":"w9:p1"}]}}}"#;
    let mut script = fs::OpenOptions::new().append(true).open(&herdr).unwrap();
    writeln!(
        script,
        "if [ \"$1 $2\" = \"api snapshot\" ]; then printf \'%s\n\' \'{snapshot}\'; fi"
    )
    .unwrap();
    drop(script);

    let output = isolated_plugin_command()
        .arg("focus")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .env(
            "HERDR_PLUGIN_EVENT_JSON",
            r#"{"event":"workspace_focused","data":{"type":"workspace_focused","workspace_id":"w9"}}"#,
        )
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let calls = fs::read_to_string(&herdr_log).unwrap_or_default();
    assert!(
        calls.contains("pane report-metadata w9:p1")
            && calls.contains("--clear-token quota_icon_done"),
        "focused green pane must become white: {calls}"
    );
    assert!(
        !calls.contains("pane report-metadata w9:p6"),
        "other unseen green pane must stay green: {calls}"
    );
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(
            &fs::read(state.path().join("icon-attention.json")).unwrap()
        )
        .unwrap()["last_focused"],
        "w9:p1"
    );

    fs::write(state.path().join("icon-attention.json"), initial_attention).unwrap();
    fs::remove_file(&herdr_log).unwrap();
    let tab_output = isolated_plugin_command()
        .arg("focus")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .env(
            "HERDR_PLUGIN_EVENT_JSON",
            r#"{"event":"tab_focused","data":{"type":"tab_focused","tab_id":"w9:t1","workspace_id":"w9"}}"#,
        )
        .output()
        .unwrap();
    assert!(
        tab_output.status.success(),
        "{}",
        String::from_utf8_lossy(&tab_output.stderr)
    );
    let tab_calls = fs::read_to_string(&herdr_log).unwrap_or_default();
    assert!(
        tab_calls.contains("pane report-metadata w9:p1")
            && !tab_calls.contains("pane report-metadata w9:p6"),
        "tab focus must clear only its focused pane: {tab_calls}"
    );
}

#[test]
fn delayed_workspace_focus_does_not_repaint_a_green_pane() {
    let state = tempdir().unwrap();
    fs::write(
        state.path().join("icon-attention.json"),
        r#"{"unseen":["w9:p1"],"last_focused":"w5:pB"}"#,
    )
    .unwrap();
    let inventory = r#"{"result":{"agents":[
        {"agent":"agy","pane_id":"w9:p1","agent_status":"idle","focused":false,
         "tokens":{"quota_icon_done":"GREEN","quota_provider":"Agy","quota_provider_model":"Agy"}}
    ]}}"#;
    let (herdr, herdr_log, codex, _) = install_logged_herdr_and_codex(
        state.path(),
        inventory,
        Some(r#"{"result":{"pane":{"agent":"agy","pane_id":"w9:p1"}}}"#),
    );
    let snapshot = r#"{"result":{"snapshot":{"focused_workspace_id":"w5","focused_tab_id":"w5:t1","workspaces":[{"workspace_id":"w9","active_tab_id":"w9:t1"}],"layouts":[{"tab_id":"w9:t1","focused_pane_id":"w9:p1"}]}}}"#;
    let mut script = fs::OpenOptions::new().append(true).open(&herdr).unwrap();
    writeln!(
        script,
        "if [ \"$1 $2\" = \"api snapshot\" ]; then printf \'%s\n\' \'{snapshot}\'; fi"
    )
    .unwrap();
    drop(script);
    let output = isolated_plugin_command()
        .arg("focus")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .env(
            "HERDR_PLUGIN_EVENT_JSON",
            r#"{"event":"workspace_focused","data":{"workspace_id":"w9"}}"#,
        )
        .output()
        .unwrap();
    assert!(output.status.success());
    let calls = fs::read_to_string(&herdr_log).unwrap_or_default();
    assert!(!calls.contains("pane report-metadata"), "{calls}");
    let attention: serde_json::Value =
        serde_json::from_slice(&fs::read(state.path().join("icon-attention.json")).unwrap())
            .unwrap();
    assert_eq!(attention["last_focused"], "w5:pB");
    assert_eq!(attention["unseen"], serde_json::json!(["w9:p1"]));
}

#[test]
fn watcher_acknowledges_focus_change_even_without_a_focus_event() {
    let state = tempdir().unwrap();
    fs::write(
        state.path().join("icon-attention.json"),
        r#"{"unseen":["w1:p1","w1:p3"],"last_focused":"w1:p1"}"#,
    )
    .unwrap();
    let inventory = r#"{"result":{"agents":[
        {"agent":"agy","pane_id":"w1:p1","agent_status":"idle","focused":false,
         "tokens":{"quota_icon_done":"GREEN","quota_provider":"Agy","quota_provider_model":"Agy"}},
        {"agent":"codex","pane_id":"w1:p2","agent_status":"idle","focused":true,
         "tokens":{"quota_icon":"WHITE","quota_provider":"Codex","quota_provider_model":"Codex"}},
        {"agent":"agy","pane_id":"w1:p3","agent_status":"idle","focused":false,
         "tokens":{"quota_icon_done":"OTHER_GREEN","quota_provider":"Agy","quota_provider_model":"Agy"}}
    ]}}"#;
    let (herdr, herdr_log, codex, _) =
        install_logged_herdr_and_codex(state.path(), inventory, None);
    let snapshot = r#"{"result":{"snapshot":{"focused_workspace_id":"w1","focused_tab_id":"w1:t1","focused_pane_id":"w1:p2"}}}"#;
    let mut script = fs::OpenOptions::new().append(true).open(&herdr).unwrap();
    writeln!(
        script,
        "if [ \"$1 $2\" = \"api snapshot\" ]; then printf \'%s\n\' \'{snapshot}\'; fi"
    )
    .unwrap();
    drop(script);
    let mut watcher = isolated_plugin_command()
        .args([
            "watch",
            "--provider",
            "all",
            "--interval-seconds",
            "30",
            "--defer",
        ])
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut calls = String::new();
    while Instant::now() < deadline {
        calls = fs::read_to_string(&herdr_log).unwrap_or_default();
        if calls.contains("pane report-metadata w1:p1") {
            break;
        }
        if watcher.try_wait().unwrap().is_some() {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    let _ = watcher.kill();
    let watcher_output = watcher.wait_with_output().unwrap();
    assert!(
        calls.contains("pane report-metadata w1:p1")
            && calls.contains("--clear-token quota_icon_done"),
        "missed focus must turn the previous pane white: {calls}; watcher: {}",
        String::from_utf8_lossy(&watcher_output.stderr)
    );
    assert!(
        !calls.contains("pane report-metadata w1:p3"),
        "unrelated green pane must remain green: {calls}"
    );
}

#[test]
fn watcher_keeps_a_focused_completion_green_until_focus_moves() {
    let state = tempdir().unwrap();
    fs::write(
        state.path().join("icon-attention.json"),
        r#"{"unseen":["w1:p1"],"last_focused":"w1:p1"}"#,
    )
    .unwrap();
    let inventory = r#"{"result":{"agents":[
        {"agent":"agy","pane_id":"w1:p1","agent_status":"idle","focused":true,
         "tokens":{"quota_icon_done":"GREEN","quota_provider":"Agy","quota_provider_model":"Agy"}},
        {"agent":"codex","pane_id":"w1:p2","agent_status":"idle","focused":false,
         "tokens":{"quota_icon":"WHITE","quota_provider":"Codex","quota_provider_model":"Codex"}}
    ]}}"#;
    let (herdr, herdr_log, codex, _) =
        install_logged_herdr_and_codex(state.path(), inventory, None);
    let snapshot_path = state.path().join("snapshot.json");
    fs::write(
        &snapshot_path,
        r#"{"result":{"snapshot":{"focused_pane_id":"w1:p1"}}}"#,
    )
    .unwrap();
    let mut script = fs::OpenOptions::new().append(true).open(&herdr).unwrap();
    writeln!(
        script,
        "if [ \"$1 $2\" = \"api snapshot\" ]; then cat \'{}\'; fi",
        snapshot_path.display()
    )
    .unwrap();
    drop(script);
    let mut watcher = isolated_plugin_command()
        .args([
            "watch",
            "--provider",
            "all",
            "--interval-seconds",
            "30",
            "--defer",
        ])
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let first_poll_deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < first_poll_deadline {
        if fs::read_to_string(&herdr_log)
            .unwrap_or_default()
            .contains("api snapshot")
        {
            break;
        }
        if watcher.try_wait().unwrap().is_some() {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    thread::sleep(Duration::from_millis(1100));
    let before = fs::read_to_string(&herdr_log).unwrap_or_default();
    fs::write(
        &snapshot_path,
        r#"{"result":{"snapshot":{"focused_pane_id":"w1:p2"}}}"#,
    )
    .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut after = String::new();
    while Instant::now() < deadline {
        after = fs::read_to_string(&herdr_log).unwrap_or_default();
        if after.contains("pane report-metadata w1:p1") {
            break;
        }
        if watcher.try_wait().unwrap().is_some() {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    let _ = watcher.kill();
    let watcher_output = watcher.wait_with_output().unwrap();
    assert!(
        !before.contains("pane report-metadata w1:p1"),
        "completion must remain green while still focused: {before}"
    );
    assert!(
        after.contains("pane report-metadata w1:p1")
            && after.contains("--clear-token quota_icon_done"),
        "focus loss must clear green: {after}; watcher: {}",
        String::from_utf8_lossy(&watcher_output.stderr)
    );
}

#[test]
fn focusing_a_done_pane_clears_the_teal_icon_immediately() {
    let state = tempdir().unwrap();
    let inventory = r#"{"result":{"agents":[
        {"agent":"agy","pane_id":"w1:p1","agent_status":"done","focused":false,
         "tokens":{"quota_icon_done":"TEAL","quota_provider":"Agy","quota_provider_model":"Agy"}},
        {"agent":"codex","pane_id":"w1:p2","agent_status":"idle","focused":false,
         "tokens":{"quota_icon":"x","quota_provider":"Codex","quota_provider_model":"Codex"}}
    ]}}"#;
    let (herdr, herdr_log, codex, codex_log) = install_logged_herdr_and_codex(
        state.path(),
        inventory,
        Some(r#"{"result":{"pane":{"agent":"codex","pane_id":"w1:p2"}}}"#),
    );
    let output = isolated_plugin_command()
        .arg("focus")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .env(
            "HERDR_PLUGIN_EVENT_JSON",
            r#"{"event":"pane_focused","data":{"type":"pane_focused","pane_id":"w1:p1","workspace_id":"w1"}}"#,
        )
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let calls = fs::read_to_string(&herdr_log).unwrap_or_default();
    let paint = calls
        .lines()
        .filter(|line| line.contains("pane report-metadata w1:p1"))
        .collect::<Vec<_>>();
    assert!(
        !paint.is_empty(),
        "focus must republish the focused pane: {calls}"
    );
    let joined = paint.join("\n");
    assert!(
        joined.contains("quota_icon=") || joined.contains("--token quota_icon="),
        "must publish idle brand icon: {joined}"
    );
    assert!(
        joined.contains("clear-token quota_icon_done")
            || joined.contains("--clear-token quota_icon_done"),
        "must clear teal done twin: {joined}"
    );
    assert!(!calls.contains("pane read"), "{calls}");
    assert_no_agent_collection(state.path(), &herdr_log, &codex_log);
}

#[test]
fn focusing_a_green_pane_clears_it_even_if_inventory_still_says_working() {
    let state = tempdir().unwrap();
    fs::write(
        state.path().join("icon-attention.json"),
        r#"{"unseen":["w1:p1"]}"#,
    )
    .unwrap();
    let inventory = r#"{"result":{"agents":[
        {"agent":"agy","pane_id":"w1:p1","agent_status":"working","focused":true,
         "tokens":{"quota_icon_done":"GREEN","quota_provider":"Agy","quota_provider_model":"Agy"}}
    ]}}"#;
    let (herdr, herdr_log, codex, _) = install_logged_herdr_and_codex(
        state.path(),
        inventory,
        Some(r#"{"result":{"pane":{"agent":"agy","pane_id":"w1:p1"}}}"#),
    );
    let output = isolated_plugin_command()
        .arg("focus")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .env(
            "HERDR_PLUGIN_EVENT_JSON",
            r#"{"event":"pane_focused","data":{"type":"pane_focused","pane_id":"w1:p1","workspace_id":"w1"}}"#,
        )
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let calls = fs::read_to_string(&herdr_log).unwrap_or_default();
    let paint = calls
        .lines()
        .filter(|line| line.contains("pane report-metadata w1:p1"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        paint.contains("--token quota_icon="),
        "green pane must become white: {paint}"
    );
    assert!(
        paint.contains("--clear-token quota_icon_done"),
        "green token must clear: {paint}"
    );
}

#[test]
fn focus_change_clears_only_the_previous_green_pane() {
    let state = tempdir().unwrap();
    fs::write(
        state.path().join("icon-attention.json"),
        r#"{"last_focused":"w1:p1"}"#,
    )
    .unwrap();
    let inventory = r#"{"result":{"agents":[
        {"agent":"agy","pane_id":"w1:p1","tab_id":"w1:t1","agent_status":"done","focused":false,
         "tokens":{"quota_icon_done":"GREEN","quota_provider":"Agy","quota_provider_model":"Agy"}},
        {"agent":"codex","pane_id":"w1:p2","tab_id":"w1:t1","agent_status":"idle","focused":true,
         "tokens":{"quota_icon":"WHITE","quota_provider":"Codex","quota_provider_model":"Codex"}},
        {"agent":"codex","pane_id":"w1:p3","tab_id":"w1:t1","agent_status":"done","focused":false,
         "tokens":{"quota_icon_done":"OTHER","quota_provider":"Codex","quota_provider_model":"Codex"}},
        {"agent":"codex","pane_id":"w1:p4","tab_id":"w1:t1","agent_status":"working","focused":false,
         "tokens":{"quota_icon_working":"YELLOW","quota_provider":"Codex","quota_provider_model":"Codex"}},
        {"agent":"agy","pane_id":"w1:p5","tab_id":"w1:t2","agent_status":"done","focused":false,
         "tokens":{"quota_icon_done":"OTHER","quota_provider":"Agy","quota_provider_model":"Agy"}}
    ]}}"#;
    let (herdr, herdr_log, codex, _) = install_logged_herdr_and_codex(
        state.path(),
        inventory,
        Some(r#"{"result":{"pane":{"agent":"codex","pane_id":"w1:p2"}}}"#),
    );
    let output = isolated_plugin_command()
        .arg("focus")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .env(
            "HERDR_PLUGIN_EVENT_JSON",
            r#"{"event":"pane_focused","data":{"type":"pane_focused","pane_id":"w1:p2","workspace_id":"w1"}}"#,
        )
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let calls = fs::read_to_string(&herdr_log).unwrap_or_default();
    let sibling = calls
        .lines()
        .filter(|line| line.contains("pane report-metadata w1:p1"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        sibling.contains("--token quota_icon="),
        "previous green pane must become white: {calls}"
    );
    assert!(
        sibling.contains("--clear-token quota_icon_done"),
        "green token must clear: {calls}"
    );
    assert!(
        !calls.contains("pane report-metadata w1:p3"),
        "unseen green sibling in the same tab must stay green: {calls}"
    );
    assert!(
        !calls.contains("pane report-metadata w1:p5"),
        "other tab must stay green: {calls}"
    );
    let working_sibling = calls
        .lines()
        .filter(|line| line.contains("pane report-metadata w1:p4"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !working_sibling.contains("--token quota_icon="),
        "working sibling must stay yellow: {calls}"
    );
    assert!(
        !calls.contains("pane read"),
        "focus must read no terminal: {calls}"
    );
}

#[test]
fn focus_to_a_non_agent_pane_acknowledges_the_previous_agent() {
    let state = tempdir().unwrap();
    fs::write(
        state.path().join("icon-attention.json"),
        r#"{"unseen":["w1:p1"],"last_focused":"w1:p1"}"#,
    )
    .unwrap();
    let inventory = r#"{"result":{"agents":[
        {"agent":"agy","pane_id":"w1:p1","agent_status":"idle","focused":false,
         "tokens":{"quota_icon_done":"GREEN"}}
    ]}}"#;
    let (herdr, herdr_log, _, _) = install_logged_herdr_and_codex(state.path(), inventory, None);
    let output = isolated_plugin_command()
        .arg("focus")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env(
            "HERDR_PLUGIN_EVENT_JSON",
            r#"{"event":"pane_focused","data":{"pane_id":"w1:p9","workspace_id":"w1"}}"#,
        )
        .output()
        .unwrap();
    assert!(output.status.success());
    let calls = fs::read_to_string(herdr_log).unwrap_or_default();
    assert!(
        calls.contains("pane report-metadata w1:p1")
            && calls.contains("--clear-token quota_icon_done"),
        "{calls}"
    );
    assert!(!calls.contains("pane report-metadata w1:p9"), "{calls}");
}

#[test]
fn completion_stays_teal_even_when_the_pane_was_already_focused() {
    let state = tempdir().unwrap();
    let inventory = r#"{"result":{"agents":[
        {"agent":"agy","pane_id":"w1:p1","agent_status":"idle","focused":true,
         "tokens":{"quota_icon":"x","quota_provider":"Agy","quota_provider_model":"Agy"}},
        {"agent":"agy","pane_id":"w1:p2","agent_status":"working","focused":false,
         "tokens":{"quota_icon_working":"Y","quota_provider":"Agy","quota_provider_model":"Agy"}}
    ]}}"#;
    let (herdr, herdr_log, codex, codex_log) = install_logged_herdr_and_codex(
        state.path(),
        inventory,
        Some(r#"{"result":{"pane":{"agent":"agy","pane_id":"w1:p2"}}}"#),
    );
    let output = isolated_plugin_command()
        .arg("event")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .env("HERDR_PANE_ID", "w1:p2")
        .env(
            "HERDR_PLUGIN_EVENT_JSON",
            r#"{"event":"pane_agent_status_changed","data":{"pane_id":"w1:p2","agent":"agy","agent_status":"idle"}}"#,
        )
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let calls = fs::read_to_string(&herdr_log).unwrap_or_default();
    assert!(
        !calls.contains("pane current"),
        "event must not call pane current (HERDR_PANE_ID trap): {calls}"
    );
    let paint = calls
        .lines()
        .filter(|line| line.contains("pane report-metadata w1:p2"))
        .collect::<Vec<_>>();
    assert!(!paint.is_empty(), "completion must publish: {calls}");
    let joined = paint.join("\n");
    assert!(
        !joined.contains("quota_icon_done=") && !joined.contains("--token quota_icon_done="),
        "nested vendor children omit the brand icon, so teal cannot land there: {joined}"
    );
    assert!(
        !calls
            .lines()
            .any(|line| line.contains("pane report-metadata w1:p1")
                && (line.contains("quota_icon_done=")
                    || line.contains("--token quota_icon_done="))),
        "shared vendor header stays idle, not teal: {calls}"
    );
    assert!(
        calls.contains("pane read w1:p2") && !calls.contains("pane read w1:p1"),
        "{calls}"
    );
    assert!(!calls.contains("recent"), "{calls}");
    assert!(!codex_log.exists(), "codex stub must stay idle");

    // A finish in the already focused pane also paints teal.
    let state = tempdir().unwrap();
    let inventory = r#"{"result":{"agents":[
        {"agent":"agy","pane_id":"w1:p1","agent_status":"working","focused":true,
         "tokens":{"quota_icon_working":"Y","quota_provider":"Agy","quota_provider_model":"Agy"}}
    ]}}"#;
    let (herdr, herdr_log, codex, codex_log) = install_logged_herdr_and_codex(
        state.path(),
        inventory,
        Some(r#"{"result":{"pane":{"agent":"agy","pane_id":"w1:p1"}}}"#),
    );
    let output = isolated_plugin_command()
        .arg("event")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .env("HERDR_PANE_ID", "w1:p1")
        .env(
            "HERDR_PLUGIN_EVENT_JSON",
            r#"{"event":"pane_agent_status_changed","data":{"pane_id":"w1:p1","agent":"agy","agent_status":"idle"}}"#,
        )
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let calls = fs::read_to_string(&herdr_log).unwrap_or_default();
    assert!(
        !calls.contains("pane current"),
        "event must not call pane current: {calls}"
    );
    let paint = calls
        .lines()
        .filter(|line| line.contains("pane report-metadata w1:p1"))
        .collect::<Vec<_>>();
    assert!(
        !paint.is_empty(),
        "focused completion must publish: {calls}"
    );
    let joined = paint.join("\n");
    assert!(
        report_sets_done_icon(&joined),
        "focused completion must stay teal: {joined}"
    );
    assert!(calls.contains("pane read w1:p1"), "{calls}");
    assert!(!calls.contains("recent"), "{calls}");
    assert!(!codex_log.exists(), "codex stub must stay idle");
}

#[test]
fn unfocused_idle_uses_working_set_when_the_yellow_icon_is_gone() {
    let state = tempdir().unwrap();
    fs::write(
        state.path().join("icon-attention.json"),
        r#"{"working":["w1:p2"]}"#,
    )
    .unwrap();
    let inventory = r#"{"result":{"agents":[
        {"agent":"agy","pane_id":"w1:p1","agent_status":"idle","focused":true,
         "tokens":{"quota_icon":"x","quota_provider":"Agy","quota_provider_model":"Agy"}},
        {"agent":"agy","pane_id":"w1:p2","agent_status":"idle","focused":false,
         "tokens":{"quota_icon":"x","quota_provider":"Agy","quota_provider_model":"Agy"}}
    ]}}"#;
    let (herdr, herdr_log, codex, codex_log) = install_logged_herdr_and_codex(
        state.path(),
        inventory,
        Some(r#"{"result":{"pane":{"agent":"agy","pane_id":"w1:p2"}}}"#),
    );
    let output = isolated_plugin_command()
        .arg("event")
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .env("HERDR_PANE_ID", "w1:p2")
        .env(
            "HERDR_PLUGIN_EVENT_JSON",
            r#"{"event":"pane_agent_status_changed","data":{"pane_id":"w1:p2","agent":"agy","agent_status":"idle"}}"#,
        )
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let calls = fs::read_to_string(&herdr_log).unwrap_or_default();
    assert!(
        !calls.contains("pane current"),
        "event must not call pane current: {calls}"
    );
    let paint = calls
        .lines()
        .filter(|line| line.contains("pane report-metadata w1:p2"))
        .collect::<Vec<_>>();
    assert!(!paint.is_empty(), "completion must publish: {calls}");
    let joined = paint.join("\n");
    assert!(
        !joined.contains("quota_icon_done=") && !joined.contains("--token quota_icon_done="),
        "nested extra Agy tab has no brand icon to keep teal: {joined}"
    );
    assert!(
        calls.contains("pane read w1:p2") && !calls.contains("pane read w1:p1"),
        "{calls}"
    );
    assert!(!calls.contains("recent"), "{calls}");
    assert!(!codex_log.exists(), "codex stub must stay idle");
}

#[test]
fn a_low_quota_notifies_once_and_re_arms_only_after_recovering() {
    let state = tempdir().unwrap();
    let (herdr_stub, herdr_log) = install_herdr_stub(
        state.path(),
        r#"{"result":{"agents":[{"agent":"agy","pane_id":"w1:p1","agent_session":{"value":"test-session"},"tokens":{}}]}}"#,
    );
    fs::create_dir_all(state.path()).unwrap();
    fs::write(state.path().join("low-quota-alert"), "20%").unwrap();

    let notifications = || {
        fs::read_to_string(&herdr_log)
            .unwrap_or_default()
            .lines()
            .filter(|line| line.starts_with("notification show"))
            .count()
    };
    let quota = |five_hour_rem: f64, seven_day_rem: f64| {
        format!(
            r#"{{"session_id":"test-session","quota":{{"gemini-5h":{{"remaining_percent":{five_hour_rem}}},"gemini-weekly":{{"remaining_percent":{seven_day_rem}}}}}}}"#
        )
    };

    let observe = |rem_five_hour: f64, rem_seven_day: f64| {
        run_agy_collector(
            state.path(),
            &herdr_stub,
            quota(rem_five_hour, rem_seven_day).as_bytes(),
        );
        run_agy_refresh(state.path(), &herdr_stub);
    };

    observe(90.0, 12.0);
    assert_eq!(notifications(), 1, "{:?}", fs::read_to_string(&herdr_log));

    observe(80.0, 9.0);
    assert_eq!(notifications(), 1);

    observe(90.0, 70.0);
    assert_eq!(notifications(), 1);
    observe(90.0, 5.0);
    assert_eq!(notifications(), 2);
}

#[test]
fn no_alert_threshold_means_no_notification_however_low_the_quota_is() {
    let state = tempdir().unwrap();
    let (herdr_stub, herdr_log) = install_herdr_stub(
        state.path(),
        r#"{"result":{"agents":[{"agent":"agy","pane_id":"w1:p1","agent_session":{"value":"test-session"},"tokens":{}}]}}"#,
    );
    run_agy_collector(
        state.path(),
        &herdr_stub,
        br#"{"session_id":"test-session","rate_limits":{"five_hour":{"used_percentage":100.0},"seven_day":{"used_percentage":100.0}}}"#,
    );
    run_agy_refresh(state.path(), &herdr_stub);
    let log = fs::read_to_string(&herdr_log).unwrap_or_default();
    assert!(!log.contains("notification show"), "{log}");
}

#[test]
fn agy_collector_does_not_republish_unchanged_quota() {
    let state = tempdir().unwrap();
    let (herdr_stub, herdr_log) = install_herdr_stub(
        state.path(),
        &format!(
            r#"{{"result":{{"agents":[{{"agent":"agy","pane_id":"w1:p1","agent_session":{{"value":"test-session"}},"tokens":{{"quota_group":"w1","quota_icon":"{}","quota_provider":"Agy","quota_provider_model":"Agy","quota_5h_warning":"5h 42%","quota_week_normal":"7d 73%","quota_headroom":"042","quota_stack":"042990042","quota_nest_gap":"{}"}}}}]}}}}"#,
            "\u{e1b2}", "\u{200b}\u{2800}"
        ),
    );

    let input = br#"{
        "session_id":"test-session",
        "quota": {
            "gemini-5h": {"remaining_percent": 42.0},
            "gemini-weekly": {"remaining_percent": 73.0}
        }
    }"#;
    run_agy_collector(state.path(), &herdr_stub, input);
    assert!(
        !herdr_log.exists()
            || !fs::read_to_string(&herdr_log)
                .unwrap()
                .contains("report-metadata"),
        "collector must not republish: {}",
        fs::read_to_string(&herdr_log).unwrap_or_default()
    );

    run_agy_refresh(state.path(), &herdr_stub);
    assert!(
        !herdr_log.exists()
            || !fs::read_to_string(&herdr_log)
                .unwrap()
                .contains("report-metadata"),
        "refresh must not republish unchanged quota: {}",
        fs::read_to_string(&herdr_log).unwrap_or_default()
    );
}

struct AgentHomes {
    state: PathBuf,
    herdr_config: PathBuf,
    agy_settings: PathBuf,
}

impl AgentHomes {
    fn new(root: &Path) -> Self {
        Self {
            state: root.join("state"),
            herdr_config: root.join("herdr/config.toml"),
            agy_settings: root.join("agy/settings.json"),
        }
    }

    fn configure(&self, args: &[&str]) -> std::process::Output {
        self.configure_with_env(args, &[])
    }

    fn configure_with_env(&self, args: &[&str], env: &[(&str, &str)]) -> std::process::Output {
        fs::create_dir_all(&self.state).unwrap();
        let mut command = isolated_plugin_command();
        for (key, value) in env {
            command.env(key, value);
        }
        command.env_remove("HERDR_SOCKET_PATH");
        command
            .arg("configure")
            .args(args)
            .env("HOME", self.state.parent().unwrap())
            .env(
                "XDG_CONFIG_HOME",
                self.state.parent().unwrap().join(".config"),
            )
            .env(
                "XDG_DATA_HOME",
                self.state.parent().unwrap().join(".local/share"),
            )
            .env("HERDR_PLUGIN_STATE_DIR", &self.state)
            .env("HERDR_CONFIG_FILE", &self.herdr_config)
            .env("AGY_SETTINGS_FILE", &self.agy_settings)
            .env("HERDR_BIN_PATH", self.state.join("herdr-absent"))
            .output()
            .unwrap()
    }

    fn sidebar(&self) -> String {
        fs::read_to_string(&self.herdr_config).unwrap_or_default()
    }
}

#[test]
fn configure_tests_do_not_reach_the_callers_live_herdr_socket() {
    let root = tempdir().unwrap();
    let socket = root.path().join("live-herdr.sock");
    let listener = UnixListener::bind(&socket).unwrap();
    listener.set_nonblocking(true).unwrap();
    let stop = Arc::new(AtomicBool::new(false));
    let server_stop = Arc::clone(&stop);
    let server = thread::spawn(move || loop {
        match listener.accept() {
            Ok((mut stream, _)) => {
                let mut request = String::new();
                BufReader::new(stream.try_clone().unwrap())
                    .read_line(&mut request)
                    .unwrap();
                writeln!(
                    stream,
                    r#"{{"result":{{"type":"agent_view","active":true}}}}"#
                )
                .unwrap();
                return Some(request);
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                if server_stop.load(Ordering::Relaxed) {
                    return None;
                }
                thread::sleep(Duration::from_millis(5));
            }
            Err(error) => panic!("accept fake Herdr socket: {error}"),
        }
    });

    let homes = AgentHomes::new(root.path());
    let output = homes.configure_with_env(
        &["--apply", "--agent", "agy"],
        &[("HERDR_SOCKET_PATH", socket.to_str().unwrap())],
    );
    stop.store(true, Ordering::Relaxed);
    let request = server.join().unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        request, None,
        "an integration test changed the live Agent view: {request:?}"
    );
}

#[test]
fn installing_one_agent_leaves_every_other_agent_untouched() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());

    let output = homes.configure(&["--apply", "--agent", "codex"]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let sidebar = homes.sidebar();
    assert!(sidebar_has_status_icon_rules(&sidebar), "{sidebar}");
    assert!(!sidebar.contains("state_icon"), "{sidebar}");
    assert!(!sidebar.contains("rows_by_agent"), "{sidebar}");
    for harness in AgentSelection::SUPPORTED {
        let other = style_row(harness);
        assert!(!sidebar.contains(&other), "{other} was written: {sidebar}");
    }

    assert!(
        !homes.agy_settings.exists(),
        "an unselected Agy settings file was created"
    );
}

#[test]
fn uninstalling_one_agent_keeps_the_rest_working() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    assert!(homes.configure(&["--apply"]).status.success());
    assert!(homes.agy_settings.exists());

    let output = homes.configure(&["--uninstall", "--agent", "codex"]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let sidebar = homes.sidebar();
    assert!(
        sidebar_has_status_icon_rules(&sidebar) && !sidebar.contains("state_icon"),
        "shared quota rows were lost: {sidebar}"
    );
    assert!(homes.agy_settings.exists(), "removing Codex tore out Agy");
}

#[test]
fn uninstall_without_an_agent_still_removes_everything() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    assert!(homes.configure(&["--apply"]).status.success());
    assert!(!homes.sidebar().is_empty());

    assert!(homes.configure(&["--uninstall"]).status.success());
    let sidebar = homes.sidebar();
    assert!(
        !sidebar.contains("rows_by_agent"),
        "a managed row survived a full uninstall: {sidebar}"
    );
}

#[test]
fn a_partial_uninstall_can_be_repeated_and_then_completed() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    assert!(homes.configure(&["--apply"]).status.success());

    for _ in 0..2 {
        assert!(homes
            .configure(&["--uninstall", "--agent", "codex"])
            .status
            .success());
        assert!(homes.agy_settings.exists());
    }

    assert!(homes.configure(&["--uninstall"]).status.success());
    assert!(!homes.sidebar().contains("rows_by_agent"));
}

#[test]
fn an_installer_can_narrow_the_selection_through_the_environment() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    let output = homes.configure_with_env(&["--apply"], &[("HERDR_AGENT_QUOTA_AGENTS", "codex")]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let sidebar = homes.sidebar();
    assert!(sidebar_has_status_icon_rules(&sidebar), "{sidebar}");
    assert!(!sidebar.contains("state_icon"), "{sidebar}");
    assert!(!sidebar.contains("rows_by_agent"), "{sidebar}");
    assert!(!homes.agy_settings.exists());
}

#[test]
fn an_unusable_environment_selection_still_installs_everything() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    assert!(homes
        .configure_with_env(&["--apply"], &[("HERDR_AGENT_QUOTA_AGENTS", "nonsense")])
        .status
        .success());
    let sidebar = homes.sidebar();
    assert!(sidebar_has_status_icon_rules(&sidebar), "{sidebar}");
    assert!(!sidebar.contains("state_icon"), "{sidebar}");
    assert!(!sidebar.contains("rows_by_agent"), "{sidebar}");
    assert!(homes.agy_settings.exists());
}

fn style_row(harness: Harness) -> String {
    format!("{} =", AgentSelection::harness_name(harness))
}

#[test]
fn stacked_sidebar_layout_is_persisted_across_a_repair() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    let output = homes.configure(&["--apply", "--sidebar-layout", "stacked"]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        sidebar_is_stacked(&homes.sidebar()),
        "first apply was not stacked: {}",
        homes.sidebar()
    );

    assert!(homes.configure(&["--apply"]).status.success());
    assert!(
        sidebar_is_stacked(&homes.sidebar()),
        "repair dropped stacked: {}",
        homes.sidebar()
    );

    assert!(homes
        .configure(&["--apply", "--sidebar-layout", "packed"])
        .status
        .success());
    assert!(
        sidebar_is_packed(&homes.sidebar()),
        "explicit packed did not switch: {}",
        homes.sidebar()
    );
}

#[test]
fn sidebar_pacing_is_opt_in_and_persisted_across_a_repair() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());

    assert!(homes.configure(&["--apply"]).status.success());
    assert_eq!(
        fs::read_to_string(homes.state.join("sidebar-pacing")).unwrap(),
        "off"
    );

    assert!(homes
        .configure(&["--apply", "--sidebar-pacing", "on"])
        .status
        .success());
    assert_eq!(
        fs::read_to_string(homes.state.join("sidebar-pacing")).unwrap(),
        "on"
    );

    assert!(homes.configure(&["--apply"]).status.success());
    assert_eq!(
        fs::read_to_string(homes.state.join("sidebar-pacing")).unwrap(),
        "on"
    );
}

#[test]
fn sidebar_pacing_can_be_selected_through_the_installer_environment() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    let output =
        homes.configure_with_env(&["--apply"], &[("HERDR_AGENT_QUOTA_SIDEBAR_PACING", "on")]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(homes.state.join("sidebar-pacing")).unwrap(),
        "on"
    );
}

#[test]
fn an_installer_can_select_stacked_layout_through_the_environment() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    let output = homes.configure_with_env(
        &["--apply"],
        &[("HERDR_AGENT_QUOTA_SIDEBAR_LAYOUT", "stacked")],
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(sidebar_is_stacked(&homes.sidebar()), "{}", homes.sidebar());
}

#[test]
fn flush_row_gap_is_persisted_across_a_repair() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    let output = homes.configure(&["--apply", "--row-gap", "0"]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(homes.sidebar().contains("row_gap = 0 # herdr-agent-usage"));

    assert!(homes.configure(&["--apply"]).status.success());
    assert!(
        homes.sidebar().contains("row_gap = 0 # herdr-agent-usage"),
        "repair dropped flush gap: {}",
        homes.sidebar()
    );

    assert!(homes
        .configure(&["--apply", "--row-gap", "1"])
        .status
        .success());
    assert!(homes.sidebar().contains("row_gap = 0 # herdr-agent-usage"));
    assert!(!homes.sidebar().contains("row_gap = 1"));
}

#[test]
fn an_installer_can_select_flush_gap_through_the_plugin_config_dir() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    let config_dir = root.path().join("plugin-config");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(config_dir.join("row-gap"), "0\n").unwrap();
    fs::write(config_dir.join("sidebar-layout"), "stacked\n").unwrap();
    let output = homes.configure_with_env(
        &["--apply"],
        &[("HERDR_PLUGIN_CONFIG_DIR", config_dir.to_str().unwrap())],
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let sidebar = homes.sidebar();
    assert!(sidebar_is_stacked(&sidebar), "{sidebar}");
    assert!(
        sidebar.contains("row_gap = 0 # herdr-agent-usage"),
        "{sidebar}"
    );
}

#[test]
fn gauges_sidebar_layout_is_persisted_across_a_repair() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    let output = homes.configure(&["--apply", "--sidebar-layout", "gauges"]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        sidebar_is_gauges(&homes.sidebar()),
        "first apply was not gauges: {}",
        homes.sidebar()
    );

    assert!(homes.configure(&["--apply"]).status.success());
    assert!(
        sidebar_is_gauges(&homes.sidebar()),
        "repair dropped gauges: {}",
        homes.sidebar()
    );

    assert!(homes
        .configure(&["--apply", "--sidebar-layout", "packed"])
        .status
        .success());
    assert!(
        sidebar_is_packed(&homes.sidebar()),
        "explicit packed did not switch: {}",
        homes.sidebar()
    );
}

#[test]
fn an_installer_can_select_gauges_layout_through_the_environment() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    let output = homes.configure_with_env(
        &["--apply"],
        &[("HERDR_AGENT_QUOTA_SIDEBAR_LAYOUT", "gauges")],
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(sidebar_is_gauges(&homes.sidebar()), "{}", homes.sidebar());
}

#[test]
fn an_installer_can_select_gauges_layout_through_the_plugin_config_dir() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    let config_dir = root.path().join("plugin-config");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(config_dir.join("sidebar-layout"), "gauges\n").unwrap();
    let output = homes.configure_with_env(
        &["--apply"],
        &[("HERDR_PLUGIN_CONFIG_DIR", config_dir.to_str().unwrap())],
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(sidebar_is_gauges(&homes.sidebar()), "{}", homes.sidebar());
}

/// A gauges install is only reversible if uninstall recognises the rows it
/// wrote; otherwise it falls back to stripping tokens and the user never gets
/// their own file back.
#[test]
fn a_full_uninstall_of_a_gauges_install_restores_the_original_config() {
    let root = tempdir().unwrap();
    let homes = AgentHomes::new(root.path());
    let terminal_config = root.path().join(".config/ghostty/config");
    fs::create_dir_all(terminal_config.parent().unwrap()).unwrap();
    fs::write(&terminal_config, "# my terminal\n").unwrap();
    let font_dir = if cfg!(target_os = "macos") {
        root.path().join("Library/Fonts")
    } else {
        root.path().join(".local/share/fonts")
    };
    let original = concat!(
        "# hand-written\n",
        "[ui]\nagent_panel_sort = \"spaces\"\n\n",
        "[ui.sidebar.agents]\n",
        "row_gap = 2\n",
        "rows = [\n    [\"state_icon\", \"machine\"],\n    [\"agent\"],\n]\n",
    );
    fs::create_dir_all(homes.herdr_config.parent().unwrap()).unwrap();
    fs::write(&homes.herdr_config, original).unwrap();

    assert!(homes
        .configure(&["--apply", "--sidebar-layout", "gauges"])
        .status
        .success());
    assert!(sidebar_is_gauges(&homes.sidebar()), "{}", homes.sidebar());
    assert!(fs::read_to_string(&terminal_config)
        .unwrap()
        .contains("# BEGIN herdr-agent-usage font"));
    assert!(fs::read_dir(&font_dir).unwrap().next().is_some());

    assert!(homes.configure(&["--uninstall"]).status.success());
    assert_eq!(homes.sidebar(), original);
    assert_eq!(
        fs::read_to_string(&terminal_config).unwrap(),
        "# my terminal\n"
    );
    assert!(fs::read_dir(&font_dir).unwrap().next().is_none());
}

fn sidebar_is_gauges(sidebar: &str) -> bool {
    // Default fields omit cache/TTL; gauges is identified by severity-split
    // context tokens and windows that do not share a packed row.
    !quota_tokens_share_a_row(sidebar, "$quota_context", "$quota_week_inline_normal")
        && !quota_tokens_share_a_row(sidebar, "$quota_5h_normal", "$quota_week_normal")
        && !tab_shares_row_with_provider_model(sidebar)
        && sidebar_has_token(sidebar, "$quota_provider_model")
        && !sidebar_has_token(sidebar, "$quota_provider")
        && sidebar_has_token(sidebar, "$quota_model")
        && sidebar_has_token(sidebar, "$quota_nest_gap")
        && sidebar_has_token(sidebar, "$quota_share_week_normal")
        && sidebar.contains("$quota_context_normal")
        && sidebar.contains("$quota_week_normal")
}

fn sidebar_is_packed(sidebar: &str) -> bool {
    quota_tokens_share_a_row(sidebar, "$quota_5h_normal", "$quota_week_normal")
        && sidebar_has_token(sidebar, "$quota_provider_model")
        && !tab_shares_row_with_provider_model(sidebar)
        && !sidebar.contains("$quota_context_normal")
}

fn sidebar_is_stacked(sidebar: &str) -> bool {
    !quota_tokens_share_a_row(sidebar, "$quota_context", "$quota_week_inline_normal")
        && !quota_tokens_share_a_row(sidebar, "$quota_5h_normal", "$quota_week_normal")
        && !quota_tokens_share_a_row(sidebar, "$quota_provider", "$quota_model")
        && !tab_shares_row_with_provider_model(sidebar)
        && sidebar_has_token(sidebar, "$quota_provider")
        && sidebar_has_token(sidebar, "$quota_model")
        && !sidebar_has_token(sidebar, "$quota_provider_model")
        && !sidebar.contains("$quota_context_normal")
        && sidebar.contains("$quota_week_normal")
}

fn sidebar_has_token(sidebar: &str, token: &str) -> bool {
    let document = sidebar.parse::<toml_edit::DocumentMut>().unwrap();
    let Some(rows) = document["ui"]["sidebar"]["agents"]["rows"].as_array() else {
        return false;
    };
    let present = rows.iter().any(|row| row_contains_token(row, token));
    present
}

fn tab_shares_row_with_provider_model(sidebar: &str) -> bool {
    let document = sidebar.parse::<toml_edit::DocumentMut>().unwrap();
    let Some(rows) = document["ui"]["sidebar"]["agents"]["rows"].as_array() else {
        return false;
    };
    let shares = rows.iter().any(|row| {
        let Some(items) = row.as_array() else {
            return false;
        };
        items
            .iter()
            .any(|item| configured_token(item) == Some("tab"))
            && items
                .iter()
                .any(|item| configured_token(item) == Some("$quota_provider_model"))
    });
    shares
}

fn quota_tokens_share_a_row(sidebar: &str, left: &str, right: &str) -> bool {
    let document = sidebar.parse::<toml_edit::DocumentMut>().unwrap();
    let Some(rows) = document["ui"]["sidebar"]["agents"]["rows"].as_array() else {
        return false;
    };
    let shares = rows
        .iter()
        .any(|row| row_contains_token(row, left) && row_contains_token(row, right));
    shares
}

#[test]
fn a_manual_refresh_reads_no_pane_at_all() {
    let state = tempdir().unwrap();
    let inventory = r#"{"result":{"agents":[{"agent":"codex","pane_id":"w1:p1"},{"agent":"agy","pane_id":"w1:p2"}]}}"#;
    let (herdr, herdr_log, codex, _codex_log) =
        install_logged_herdr_and_codex(state.path(), inventory, None);

    let output = isolated_plugin_command()
        .args(["refresh", "--provider", "all"])
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .env("CODEX_BIN_PATH", &codex)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let calls = fs::read_to_string(&herdr_log).unwrap_or_default();
    assert!(
        !calls.contains("pane read"),
        "manual refresh read a pane: {calls}"
    );
    assert!(
        !calls.contains("recent"),
        "manual refresh used a repainting source: {calls}"
    );
}

#[test]
fn a_quota_less_pane_still_gets_its_brand_icon_on_refresh() {
    let state = tempdir().unwrap();
    let inventory = r#"{"result":{"agents":[{"agent":"agy","pane_id":"w1:p1","agent_status":"idle","tokens":{}}]}}"#;
    let (herdr, herdr_log, _, _) = install_logged_herdr_and_codex(state.path(), inventory, None);
    let output = isolated_plugin_command()
        .args(["refresh", "--provider", "agy"])
        .env("HERDR_PLUGIN_STATE_DIR", state.path())
        .env("HERDR_BIN_PATH", &herdr)
        .output()
        .unwrap();
    assert!(output.status.success());
    let calls = fs::read_to_string(herdr_log).unwrap_or_default();
    assert!(calls.contains("pane report-metadata w1:p1"), "{calls}");
    assert!(calls.contains("--token quota_icon="), "{calls}");
    assert!(!calls.contains("pane read"), "{calls}");
}
