//! Module: Hard-coded Ethical Constants for Zion Citadel.
//! This is the immutable moral foundation from the dossier.
//! Changes require full recompile and audit.

use serde::{Deserialize, Serialize};

/// Core ethical rules—non-negotiable laws of the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalConstants {
    /// Civilian targets are absolutely forbidden.
    pub no_civilian_targets: bool,
    /// Avatars/agents never initiate contact (no entrapment).
    pub no_initiation_of_contact: bool,
    /// No persistent malware—everything ephemeral and zero-trace.
    pub no_persistent_malware: bool,
    /// Max duration for any operation without re-approval (seconds).
    pub max_operation_duration_seconds: u64,
    /// Minimum confidence threshold for acting on risks (0.0-1.0).
    pub min_risk_confidence_threshold: f32,
}

impl Default for EthicalConstants {
    fn default() -> Self {
        // Default to maximum restriction—your constitutional baseline.
        EthicalConstants {
            no_civilian_targets: true,
            no_initiation_of_contact: true,
            no_persistent_malware: true,
            max_operation_duration_seconds: 300, // 5 minutes max
            min_risk_confidence_threshold: 0.95, // 95% confidence required
        }
    }
}
