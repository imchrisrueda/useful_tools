use herdr_agent_quota::cache::{sanitize_statusline_payload, CacheStore};
use herdr_agent_quota::model::{Provider, ProviderSnapshot};
use herdr_agent_quota::providers::agy::parse_statusline;
use serde_json::json;
use std::fs;
use tempfile::tempdir;

#[test]
fn nested_secrets_and_transcript_paths_never_reach_the_mailbox() {
    let raw = json!({
        "session_id": "synthetic-session",
        "model": {"display_name": "Gemini Flash", "api_key": "FAKE_SECRET"},
        "prompt": "FAKE_SECRET",
        "transcript_path": "/private/FAKE_SECRET.jsonl",
        "quota": {
            "gemini-5h": {"remaining_fraction": 0.8, "reset_in_seconds": 3600},
            "3p-5h": {"remaining_fraction": "FAKE_SECRET", "reset_time": "FAKE_SECRET"}
        },
        "context_window": {
            "remaining_percentage": 75.0,
            "current_usage": {
                "input_tokens": 50,
                "cache_read_input_tokens": 50,
                "cache_creation_input_tokens": 0,
                "api_key": "FAKE_SECRET"
            }
        },
        "prompt_cache": {
            "warm": true,
            "ttl": "5m",
            "expires_at": 9999,
            "api_key": "FAKE_SECRET",
            "messages": [{"content": "FAKE_SECRET"}]
        },
        "promptCache": {"ttl": "FAKE_SECRET", "expiresAt": "FAKE_SECRET"}
    });
    let clean = sanitize_statusline_payload(&raw);
    assert!(!clean.to_string().contains("FAKE_SECRET"));
    assert_eq!(clean["prompt_cache"]["ttl"], "5m");
    assert!(clean.get("transcript_path").is_none());
    let snapshot = parse_statusline(&clean, 100).unwrap();
    assert_eq!(snapshot.context.as_ref().unwrap().used_percent, 25.0);
    let directory = tempdir().unwrap();
    let cache = CacheStore::new(directory.path());
    cache
        .save_statusline_observation(Provider::Agy, snapshot, &raw)
        .unwrap();
    let path = directory
        .path()
        .join(format!("{}.observation.json", Provider::Agy.source()));
    assert!(!fs::read_to_string(path).unwrap().contains("FAKE_SECRET"));
}

#[test]
fn legacy_mailboxes_are_sanitized_on_read() {
    let directory = tempdir().unwrap();
    let path = directory
        .path()
        .join(format!("{}.observation.json", Provider::Agy.source()));
    fs::write(
        &path,
        serde_json::to_vec(&json!({
            "snapshot": ProviderSnapshot::new(Provider::Agy, vec![], 1),
            "payload": {
                "session_id": "synthetic-session",
                "transcript_path": "/private/FAKE_SECRET",
                "prompt_cache": {"messages": ["FAKE_SECRET"]}
            }
        }))
        .unwrap(),
    )
    .unwrap();
    let loaded = CacheStore::new(directory.path())
        .load_statusline_observation(Provider::Agy)
        .unwrap()
        .unwrap();
    assert!(!loaded.payload.to_string().contains("FAKE_SECRET"));
}

#[cfg(unix)]
#[test]
fn state_is_private_and_existing_temp_symlinks_are_refused() {
    use std::os::unix::fs::{symlink, PermissionsExt};
    let directory = tempdir().unwrap();
    let state = directory.path().join("state");
    let cache = CacheStore::new(&state);
    let snapshot = ProviderSnapshot::new(Provider::Agy, vec![], 1);
    cache.save(&snapshot).unwrap();
    assert_eq!(
        fs::metadata(&state).unwrap().permissions().mode() & 0o777,
        0o700
    );
    let destination = state.join(format!("{}.json", Provider::Agy.source()));
    assert_eq!(
        fs::metadata(destination).unwrap().permissions().mode() & 0o777,
        0o600
    );
    let victim = directory.path().join("private-file");
    fs::write(&victim, "unchanged").unwrap();
    let scratch = state.join(format!(
        ".{}.{}.tmp",
        Provider::Agy.source(),
        std::process::id()
    ));
    symlink(&victim, scratch).unwrap();
    assert!(cache.save(&snapshot).is_err());
    assert_eq!(fs::read_to_string(victim).unwrap(), "unchanged");
}

#[test]
fn oversized_statusline_is_ignored_without_output_or_state() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let directory = tempdir().unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_herdr-agent-usage"))
        .arg("agy-statusline")
        .env_clear()
        .env("HERDR_PLUGIN_STATE_DIR", directory.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&vec![b'x'; 1024 * 1024 + 1])
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(fs::read_dir(directory.path()).unwrap().next().is_none());
}
