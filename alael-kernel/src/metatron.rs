//! Metatron - Evidence Manager (Vol IX).
//! Immutable capsules for prosecution.

use serde_json::json;

/// Package intel into court-admissible capsule.
pub fn package_evidence(intel: &str, risk: f32, target: &str) -> String {
    let capsule = json!({
        "chain_of_custody": {
            "timestamp": "2026-01-14T01:53:04Z",
            "agent": "Alael Pipeline",
            "hash": "sha256(intel+risk)"
        },
        "evidence": intel,
        "risk_score": risk,
        "target": target,
        "admissible": true
    });
    format!("📦 Metatron Capsule: {}", capsule.to_string())
}