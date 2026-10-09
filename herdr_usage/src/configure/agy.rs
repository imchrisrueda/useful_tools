use super::statusline::{settings_path, Adapter};
use crate::cache::CacheStore;
use crate::model::Provider;
use crate::providers::agy::parse_statusline;
use anyhow::{Context, Result};
use serde_json::Value;
use std::io::Read;
use std::path::Path;

const CONFIG: Adapter = Adapter {
    label: "Agy",
    subcommand: "agy-statusline",
    backup_file: "agy-statusline.original.json",
};

pub fn check() -> Result<()> {
    let cache = CacheStore::from_env()?;
    let executable = std::env::current_exe().context("resolve plugin executable")?;
    CONFIG.check(
        &settings_path("AGY_SETTINGS_FILE", ".gemini/antigravity-cli/settings.json")?,
        cache.root(),
        &executable,
    )
}

pub fn apply() -> Result<()> {
    let cache = CacheStore::from_env()?;
    let executable = std::env::current_exe().context("resolve plugin executable")?;
    apply_at(
        &settings_path("AGY_SETTINGS_FILE", ".gemini/antigravity-cli/settings.json")?,
        cache.root(),
        &executable,
    )
}

pub fn uninstall() -> Result<()> {
    let cache = CacheStore::from_env()?;
    uninstall_at(
        &settings_path("AGY_SETTINGS_FILE", ".gemini/antigravity-cli/settings.json")?,
        cache.root(),
    )
}

pub fn apply_at(settings: &Path, state: &Path, executable: &Path) -> Result<()> {
    CONFIG.apply(settings, state, executable)
}

pub fn uninstall_at(settings: &Path, state: &Path) -> Result<()> {
    CONFIG.uninstall(settings, state)
}

/// Keep only typed metric fields in the mailbox, keyed by the Herdr pane
/// that owns the
/// Agy process. Antigravity's PreInvocation hook can report a spawned
/// subagent conversation while statusLine still describes the parent; the
/// pane id is inherited by both and stays stable across that mismatch.
fn observation_for_cache(value: &Value, pane_id: Option<&str>) -> Value {
    let clean = crate::cache::sanitize_statusline_payload(value);
    let Some(pane_id) = pane_id.filter(|pane_id| !pane_id.is_empty()) else {
        return clean;
    };
    let mut observation = clean;
    if let Some(object) = observation.as_object_mut() {
        object.insert("session_id".to_string(), Value::String(pane_id.to_string()));
    }
    observation
}

/// Consume one bounded Agy payload and store only allowlisted local metrics.
/// The previous statusLine is backed up for uninstall, never executed here.
pub fn run_statusline_hook() -> Result<()> {
    const MAX_STATUSLINE_BYTES: u64 = 1024 * 1024;
    let mut input = Vec::new();
    std::io::stdin()
        .take(MAX_STATUSLINE_BYTES + 1)
        .read_to_end(&mut input)?;
    if input.len() as u64 > MAX_STATUSLINE_BYTES {
        return Ok(());
    }
    if let Ok(value) = serde_json::from_slice::<Value>(&input) {
        let pane_id = std::env::var("HERDR_PANE_ID").ok();
        let observation = observation_for_cache(&value, pane_id.as_deref());
        if let Ok(snapshot) = parse_statusline(&observation, CacheStore::now_unix()) {
            if let Ok(cache) = CacheStore::from_env() {
                let _ = cache.save_statusline_observation(Provider::Agy, snapshot, &observation);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn cache_observation_uses_the_herdr_pane_without_mutating_provider_identity() {
        let original = json!({
            "session_id": "parent-conversation",
            "conversation_id": "parent-conversation",
            "model": {"display_name": "Gemini Flash"}
        });
        let cached = observation_for_cache(&original, Some("w1:p7"));

        assert_eq!(
            cached.get("session_id").and_then(Value::as_str),
            Some("w1:p7")
        );
        assert_eq!(
            cached.get("conversation_id").and_then(Value::as_str),
            Some("parent-conversation")
        );
        assert_eq!(
            original.get("session_id").and_then(Value::as_str),
            Some("parent-conversation")
        );
    }

    #[test]
    fn cache_observation_keeps_native_session_outside_herdr() {
        let original = json!({
            "session_id": "parent-conversation",
            "conversation_id": "parent-conversation"
        });
        assert_eq!(observation_for_cache(&original, None), original);
        assert_eq!(observation_for_cache(&original, Some("")), original);
    }

    #[test]
    fn cache_observation_strips_sensitive_prompts_and_tokens() {
        let raw = json!({
            "session_id": "conv-1",
            "conversation_id": "conv-1",
            "prompt": "confidential user prompt",
            "messages": [{"role": "user", "content": "secret password"}],
            "api_key": "secret-key-12345",
            "unknown_metadata": {"internal_env": "production"},
            "model": {"display_name": "Gemini 2.5 Pro"},
            "quota": {
                "gemini-5h": {"remaining_fraction": 0.85, "reset_in_seconds": 3600},
                "secret_pool": {"secret": "val"}
            },
            "context_window": {
                "used_percentage": 25.0,
                "secret_detail": "should be stripped"
            }
        });

        let cached = observation_for_cache(&raw, Some("w1:p2"));

        // Must preserve whitelisted fields
        assert_eq!(
            cached.get("session_id").and_then(Value::as_str),
            Some("w1:p2")
        );
        assert_eq!(
            cached.get("conversation_id").and_then(Value::as_str),
            Some("conv-1")
        );
        assert_eq!(
            cached
                .get("model")
                .and_then(|m| m.get("display_name"))
                .and_then(Value::as_str),
            Some("Gemini 2.5 Pro")
        );
        assert!(cached
            .get("quota")
            .and_then(|q| q.get("gemini-5h"))
            .is_some());
        assert_eq!(
            cached
                .get("context_window")
                .and_then(|c| c.get("used_percentage"))
                .and_then(Value::as_f64),
            Some(25.0)
        );

        // Must strictly strip sensitive non-whitelisted fields
        assert!(cached.get("prompt").is_none());
        assert!(cached.get("messages").is_none());
        assert!(cached.get("api_key").is_none());
        assert!(cached.get("unknown_metadata").is_none());
        assert!(cached
            .get("quota")
            .and_then(|q| q.get("secret_pool"))
            .is_none());
        assert!(cached
            .get("context_window")
            .and_then(|c| c.get("secret_detail"))
            .is_none());
    }
}
