use crate::model::{CacheUsage, ContextUsage, ResetAt};
use crate::providers::ProviderError;
use serde_json::Value;
use std::hash::{Hash, Hasher};

/// Read the active model label from a statusLine payload. The display name
/// is preferred so the sidebar stays readable at a glance; a payload that
/// reports only an id (a model released after this build, or a new alias)
/// shows that id rather than an empty identity row.
pub fn parse_model(value: &Value) -> Option<String> {
    let model = value.get("model")?.as_object()?;
    ["display_name", "displayName", "id"]
        .into_iter()
        .filter_map(|key| model.get(key).and_then(Value::as_str))
        .map(str::trim)
        .find(|model| !model.is_empty())
        .map(str::to_string)
}

pub fn parse_context(
    value: Option<&Value>,
) -> std::result::Result<Option<ContextUsage>, ProviderError> {
    let Some(value) = value.filter(|value| !value.is_null()) else {
        return Ok(None);
    };
    let Some(object) = value.as_object() else {
        return Err(ProviderError::UnsupportedResponse(
            "context_window is not an object".to_string(),
        ));
    };
    let used = object
        .get("used_percentage")
        .or_else(|| object.get("usedPercentage"))
        .and_then(Value::as_f64);
    let remaining = object
        .get("remaining_percentage")
        .or_else(|| object.get("remainingPercentage"))
        .and_then(Value::as_f64);
    let Some(percent) = used.or_else(|| remaining.map(|value| 100.0 - value)) else {
        return Ok(None);
    };
    let cache = parse_cache_usage(object.get("current_usage"));
    ContextUsage::new(percent)
        .map(|context| context.with_cache(cache))
        .map(Some)
        .map_err(|error| ProviderError::UnsupportedResponse(error.to_string()))
}

pub fn enrich_prompt_cache(context: &mut Option<ContextUsage>, value: &Value) {
    let Some(context) = context.as_mut() else {
        return;
    };
    let Some(cache) = context.cache.as_mut() else {
        return;
    };
    let Some(object) = value
        .get("prompt_cache")
        .or_else(|| value.get("promptCache"))
        .and_then(Value::as_object)
    else {
        return;
    };
    let warm = object.get("warm").and_then(Value::as_bool).unwrap_or(true);
    let expires_at = object
        .get("expires_at")
        .or_else(|| object.get("expiresAt"))
        .and_then(parse_expires_at);
    match (warm, expires_at) {
        (true, Some(expires_at)) => {
            cache.expires_at_unix = Some(expires_at);
            cache.ttl_seconds = object
                .get("ttl")
                .and_then(Value::as_str)
                .and_then(parse_prompt_cache_ttl);
            if let Some(ttl_seconds) = cache.ttl_seconds {
                cache.last_activity_unix = Some(expires_at.saturating_sub(ttl_seconds));
            }
        }
        _ => {
            cache.expires_at_unix = Some(0);
            cache.ttl_seconds = None;
            cache.last_activity_unix = None;
        }
    }
}

fn parse_prompt_cache_ttl(value: &str) -> Option<u64> {
    match value.trim() {
        "5m" => Some(5 * 60),
        "1h" => Some(60 * 60),
        _ => None,
    }
}

fn parse_expires_at(value: &Value) -> Option<u64> {
    if value.is_null() {
        return None;
    }
    value
        .as_u64()
        .or_else(|| {
            let number = value.as_f64()?;
            (number.is_finite() && number >= 0.0).then_some(number.round() as u64)
        })
        .or_else(|| {
            value
                .as_str()
                .and_then(ResetAt::parse)
                .map(ResetAt::unix_seconds)
        })
}

fn parse_cache_usage(value: Option<&Value>) -> Option<CacheUsage> {
    let object = value?.as_object()?;
    let has_cache_counters = [
        "cache_read_input_tokens",
        "cacheReadInputTokens",
        "cache_creation_input_tokens",
        "cacheCreationInputTokens",
    ]
    .into_iter()
    .any(|name| object.contains_key(name));
    if !has_cache_counters {
        return None;
    }
    let fresh = token_count(object, "input_tokens", "inputTokens");
    let read = token_count(object, "cache_read_input_tokens", "cacheReadInputTokens");
    let creation = token_count(
        object,
        "cache_creation_input_tokens",
        "cacheCreationInputTokens",
    );
    CacheUsage::from_token_counts(fresh, read, creation)
}

fn token_count(object: &serde_json::Map<String, Value>, snake: &str, camel: &str) -> u64 {
    object
        .get(snake)
        .or_else(|| object.get(camel))
        .and_then(Value::as_u64)
        .unwrap_or_default()
}

/// Digest statusLine fields whose values are derived from provider responses.
///
/// Claude Code can run the statusLine command again on `refreshInterval`
/// without another API response. This fingerprint lets the cache distinguish
/// that redraw from a newly completed response without treating prompt ids,
/// transcript mtimes, or hook arrival time as freshness evidence.
///
/// The digest is intentionally *not* an account identity. Missing evidence is
/// `None`, so callers fail closed and require an actual quota change before
/// advancing freshness.
pub fn api_generation(value: &Value) -> Option<String> {
    let cost = value.get("cost").and_then(Value::as_object);
    let context = value
        .get("context_window")
        .or_else(|| value.get("contextWindow"))
        .and_then(Value::as_object);
    let current = context
        .and_then(|context| {
            context
                .get("current_usage")
                .or_else(|| context.get("currentUsage"))
        })
        .and_then(Value::as_object);

    // Keep the fingerprint canonical: hash a fixed-order tuple of documented
    // scalar fields rather than serializing the current_usage object itself.
    // Object key order is not part of Claude's statusLine contract.
    let field = |object: Option<&serde_json::Map<String, Value>>, snake: &str, camel: &str| {
        object
            .and_then(|object| object.get(snake).or_else(|| object.get(camel)))
            .cloned()
    };
    let evidence = serde_json::json!([
        field(cost, "total_api_duration_ms", "totalApiDurationMs"),
        field(context, "total_input_tokens", "totalInputTokens"),
        field(context, "total_output_tokens", "totalOutputTokens"),
        field(current, "input_tokens", "inputTokens"),
        field(current, "output_tokens", "outputTokens"),
        field(
            current,
            "cache_creation_input_tokens",
            "cacheCreationInputTokens"
        ),
        field(current, "cache_read_input_tokens", "cacheReadInputTokens"),
    ]);
    let has_evidence = evidence
        .as_array()
        .is_some_and(|values| values.iter().any(|value| !value.is_null()));
    has_evidence.then(|| {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        evidence.to_string().hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn model_parser_prefers_the_display_name_and_never_drops_a_reported_id() {
        assert_eq!(
            parse_model(&json!({
                "model": {"id": "claude-opus-5-5", "display_name": "Opus 5.5"}
            })),
            Some("Opus 5.5".to_string())
        );
        assert_eq!(
            parse_model(&json!({"model": {"displayName": "Sonnet"}})),
            Some("Sonnet".to_string())
        );
        for id in ["claude-opus-5-5", "zz-unreleased-model-9"] {
            assert_eq!(
                parse_model(&json!({"model": {"id": id, "display_name": " "}})),
                Some(id.to_string())
            );
        }
        assert_eq!(parse_model(&json!({"model": {"id": ""}})), None);
    }

    #[test]
    fn api_generation_changes_only_with_api_derived_evidence() {
        let base = json!({
            "session_id": "session-1",
            "prompt_id": "prompt-a",
            "cost": {"total_api_duration_ms": 1200},
            "context_window": {
                "total_input_tokens": 100,
                "total_output_tokens": 20,
                "current_usage": {"input_tokens": 80}
            }
        });
        let first = api_generation(&base).unwrap();

        let mut prompt_only = base.clone();
        prompt_only["prompt_id"] = json!("prompt-b");
        assert_eq!(
            api_generation(&prompt_only).as_deref(),
            Some(first.as_str())
        );

        let mut unrelated_current_usage_field = base.clone();
        unrelated_current_usage_field["context_window"]["current_usage"]["future_field"] =
            json!("ignored");
        assert_eq!(
            api_generation(&unrelated_current_usage_field).as_deref(),
            Some(first.as_str())
        );

        let mut next_response = base.clone();
        next_response["cost"]["total_api_duration_ms"] = json!(1800);
        assert_ne!(
            api_generation(&next_response).as_deref(),
            Some(first.as_str())
        );

        assert_eq!(api_generation(&json!({"prompt_id":"only-a-prompt"})), None);
    }

    #[test]
    fn zero_cache_reads_remain_a_real_zero_percent_hit() {
        let value = json!({
            "used_percentage": 3.4,
            "current_usage": {
                "input_tokens": 25943,
                "cache_read_input_tokens": 0,
                "cache_creation_input_tokens": 0
            }
        });
        let context = parse_context(Some(&value)).unwrap().unwrap();
        let cache = context.cache.unwrap();
        assert_eq!(cache.fresh_input_tokens, 25943);
        assert_eq!(cache.read_tokens, 0);
        assert_eq!(cache.creation_tokens, 0);
        assert_eq!(cache.hit_percent, 0.0);
    }
}
