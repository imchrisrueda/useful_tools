use herdr_agent_quota::configure::agy;
use serde_json::Value;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn agy_setup_is_idempotent_and_restores_the_previous_statusline() {
    let directory = tempdir().unwrap();
    let settings = directory.path().join("settings.json");
    let state = directory.path().join("state");
    let executable = directory.path().join("herdr-agent-quota");
    fs::write(
        &settings,
        r#"{"theme":"dark","statusLine":{"type":"command","command":"echo old","refreshInterval":5}}"#,
    )
    .unwrap();

    agy::apply_at(&settings, &state, &executable).unwrap();
    let once = fs::read(&settings).unwrap();
    agy::apply_at(&settings, &state, &executable).unwrap();
    assert_eq!(fs::read(&settings).unwrap(), once);

    let installed: Value = serde_json::from_slice(&once).unwrap();
    assert_eq!(installed["theme"], "dark");
    assert_eq!(installed["statusLine"]["refreshInterval"], 5);
    let command = installed["statusLine"]["command"].as_str().unwrap();
    assert!(command.contains("agy-statusline"));
    assert!(command.contains(state.to_str().unwrap()));

    agy::uninstall_at(&settings, &state).unwrap();
    let restored: Value = serde_json::from_slice(&fs::read(&settings).unwrap()).unwrap();
    assert_eq!(restored["statusLine"]["command"], "echo old");
    assert_eq!(restored["statusLine"]["refreshInterval"], 5);
}

#[test]
fn old_plugin_wrappers_are_repaired_without_becoming_the_backup() {
    let directory = tempdir().unwrap();
    let settings = directory.path().join("settings.json");
    let state = directory.path().join("state");
    let executable = directory.path().join("herdr-agent-quota");
    fs::write(
        &settings,
        r#"{"statusLine":{"type":"command","command":"HERDR_PLUGIN_STATE_DIR='/wrong' '/old/herdr-agent-quota' agy-statusline"}}"#,
    )
    .unwrap();

    agy::apply_at(&settings, &state, &executable).unwrap();
    let installed: Value = serde_json::from_slice(&fs::read(&settings).unwrap()).unwrap();
    let command = installed["statusLine"]["command"].as_str().unwrap();
    assert!(command.contains(state.to_str().unwrap()));
    assert!(!command.contains("/wrong"));

    agy::uninstall_at(&settings, &state).unwrap();
    let restored: Value = serde_json::from_slice(&fs::read(&settings).unwrap()).unwrap();
    assert!(restored.get("statusLine").is_none());
}

#[test]
fn direct_configuration_write_refuses_an_ambiguous_cache_directory() {
    let output = Command::new(env!("CARGO_BIN_EXE_herdr-agent-usage"))
        .args(["configure", "--apply"])
        .env_remove("HERDR_SOCKET_PATH")
        .env_remove("HERDR_PLUGIN_STATE_DIR")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("must run through Herdr"));
}
