use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalConstants {
    pub no_innocent_civilians: bool,
    pub no_agent_initiation: bool,
    pub no_persistent_malware: bool,
    pub max_operation_duration_seconds: u64,
    pub min_risk_confidence_threshold: f32,
    pub token_lifespan_seconds: u64,
}

impl Default for EthicalConstants {
    fn default() -> Self {
        EthicalConstants {
            no_innocent_civilians: true,
            no_agent_initiation: true,
            no_persistent_malware: true,
            max_operation_duration_seconds: 300,
            min_risk_confidence_threshold: 0.95,
            token_lifespan_seconds: 3600,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionToken {
    pub action: String,
    pub target: String,
    pub expires_at: u64,
    pub signature: String,
}

impl PermissionToken {
    pub fn is_valid(&self, _ethics: &EthicalConstants, now: u64) -> bool {
        self.expires_at > now &&
        self.action != "target_innocents"
    }
}
