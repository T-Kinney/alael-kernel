//! Hard-coded Ethical Constants + Token Logic for Zion Citadel.
//! Immutable moral foundation. Predators targetable post-verification.

use serde::{Deserialize, Serialize};

/// Core ethical rules—non-negotiable constitutional law.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalConstants {
    /// Innocents/non-threats forbidden. Verified predators OK (Raziel/Vehuel >0.95).
    pub no_innocent_civilians: bool,
    /// Avatars passive: Predators initiate, self-incriminate.
    pub no_agent_initiation: bool,
    /// Ephemeral/zero-trace only—no persistent malware.
    pub no_persistent_malware: bool,
    /// Max op duration without re-approval (seconds).
    pub max_operation_duration_seconds: u64,
    /// Min confidence for risk actions (0.0-1.0).
    pub min_risk_confidence_threshold: f32,
    /// Max token lifespan (seconds).
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
            token_lifespan_seconds: 3600, // 1 hour
        }
    }
}

/// Permission token for agent actions (crypto-signed later).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionToken {
    pub action: String,        // e.g., "passive_scrape"
    pub target: String,        // e.g., "discord.predator_channel"
    pub expires_at: u64,       // Unix timestamp
    pub signature: String,     // Hash stub
}

impl PermissionToken {
    /// Validate against ethics + expiry.
    pub fn is_valid(&self, ethics: &EthicalConstants, now: u64) -> bool {
        self.expires_at > now &&
        self.action != "target_innocents" &&  // Ethics check
        true  // Stub (expand later)
    }
}
