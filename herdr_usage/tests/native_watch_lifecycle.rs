#![cfg(windows)]

use std::fs;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[test]
fn detached_watcher_does_not_keep_parent_capture_pipes_open() {
    let directory = tempfile::tempdir().unwrap();
    let state = directory.path().join("state");
    fs::create_dir(&state).unwrap();
    let cleanup_state = state.clone();
    let cleanup = thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        let stopped = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        fs::write(cleanup_state.join("turn-watch.stop"), stopped.to_string()).unwrap();
        // The watcher exits when its owned state directory disappears too.
        fs::remove_dir_all(cleanup_state).unwrap();
    });
    let started = Instant::now();
    let output = Command::new(env!("CARGO_BIN_EXE_herdr-agent-usage"))
        .args(["startup", "--provider", "agy"])
        .env_clear()
        .env("HERDR_PLUGIN_STATE_DIR", &state)
        .env("HERDR_PLUGIN_CONFIG_DIR", directory.path().join("prefs"))
        .env("HERDR_AGENT_QUOTA_AGENT_ORDER", "default")
        .env("HERDR_BIN_PATH", directory.path().join("missing-herdr.exe"))
        .env("USERPROFILE", directory.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    let elapsed = started.elapsed();
    cleanup.join().unwrap();
    assert!(output.status.success());
    assert!(
        elapsed < Duration::from_secs(2),
        "capture waited for watcher: {elapsed:?}"
    );
}
