use herdr_agent_quota::model::{ResetAt, WindowKind};
use herdr_agent_quota::providers::{agy, codex};
use serde_json::Value;

fn fixture(value: &str) -> Value {
    serde_json::from_str(value).expect("fixture is valid JSON")
}

#[test]
fn codex_fixture_exposes_the_five_hour_and_weekly_contracts() {
    let value = fixture(include_str!("fixtures/codex/rate-limits-weekly.json"));
    let snapshot = codex::parse_rate_limits(&value, 1).unwrap();
    assert_eq!(snapshot.windows.len(), 2);
    assert_eq!(
        snapshot
            .window(WindowKind::FiveHour)
            .unwrap()
            .remaining_percent,
        80.0
    );
    assert_eq!(
        snapshot
            .window(WindowKind::Weekly)
            .unwrap()
            .remaining_percent,
        39.0
    );
    assert_eq!(
        snapshot.window(WindowKind::Weekly).unwrap().resets_at,
        Some(ResetAt::from_unix_seconds(1_787_400_000))
    );
}

#[test]
fn agy_fixture_requires_an_identifiable_pool() {
    let mut value = fixture(include_str!("fixtures/agy/statusline-both.json"));
    assert!(agy::parse_statusline(&value, 1).unwrap().windows.is_empty());
    value["model"] = serde_json::json!({"display_name": "Gemini Flash"});
    let snapshot = agy::parse_statusline(&value, 1).unwrap();
    assert_eq!(snapshot.windows.len(), 3);
    assert!(
        (snapshot
            .window(WindowKind::Weekly)
            .unwrap()
            .remaining_percent
            - 99.69)
            .abs()
            < 1e-9
    );
    assert_eq!(
        snapshot
            .window(WindowKind::Monthly)
            .unwrap()
            .display_label(),
        "api"
    );
}
