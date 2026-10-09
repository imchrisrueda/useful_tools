pub mod agy;
pub mod codex;
pub mod statusline;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("provider credentials are unavailable")]
    MissingCredentials,
    #[error("provider quota is unavailable: {0}")]
    Unavailable(String),
    #[error("provider response is not supported: {0}")]
    UnsupportedResponse(String),
    #[error("provider request failed: {0}")]
    Request(String),
}

#[cfg(test)]
mod tests {
    use crate::model::{Provider, ProviderSnapshot};
    #[test]
    fn all_direct_collectors_reject_unknown_or_other_account_snapshots() {
        {
            let provider = Provider::Codex;
            let mut cached = ProviderSnapshot::new(provider, vec![], 100);
            assert!(!cached.usable_for_account(Some("new"), Some(1)));
            cached.account_id = Some("old".into());
            assert!(!cached.usable_for_account(Some("new"), Some(1)));
            assert!(cached.usable_for_account(Some("old"), Some(200)));
        }
    }
}
