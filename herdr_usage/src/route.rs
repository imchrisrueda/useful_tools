use crate::herdr::{AgentPane, PaneIdentity};
use crate::model::{BillingTarget, ContextUsage, Harness, Resolution};

/// Attribute a pane to a subscription from local evidence only.
pub fn resolve(pane: &AgentPane) -> Resolution {
    resolve_with_identity(pane).resolution
}

pub struct ResolvedPane {
    pub resolution: Resolution,
    pub identity: Option<PaneIdentity>,
    pub context: Option<ContextUsage>,
}

pub fn resolve_with_identity(pane: &AgentPane) -> ResolvedPane {
    let resolution = match pane.harness {
        Harness::Codex | Harness::Agy => pane
            .harness
            .billing()
            .map(BillingTarget::original_four)
            .map(Resolution::Subscription)
            .unwrap_or(Resolution::Indeterminate),
    };
    ResolvedPane {
        resolution,
        identity: None,
        context: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Provider;
    use std::collections::BTreeMap;

    fn test_pane(harness: Harness) -> AgentPane {
        AgentPane {
            pane_id: "w1:p1".to_string(),
            workspace_id: "w1".to_string(),
            cwd: String::new(),
            title: String::new(),
            harness,
            session: None,
            session_summary: String::new(),
            topic: String::new(),
            tokens: BTreeMap::new(),
            status: crate::herdr::AgentStatus::Idle,
            focused: false,
        }
    }

    #[test]
    fn resolve_codex_and_agy() {
        assert_eq!(
            resolve(&test_pane(Harness::Codex)),
            Resolution::Subscription(BillingTarget::original_four(Provider::Codex))
        );
        assert_eq!(
            resolve(&test_pane(Harness::Agy)),
            Resolution::Subscription(BillingTarget::original_four(Provider::Agy))
        );
    }
}
