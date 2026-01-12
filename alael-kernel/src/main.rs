//! Alael Kernel v0.2.0 - Zion Citadel Orchestrator.
//! Boots, enforces ethics, validates tokens.

mod ethics;
use ethics::{EthicalConstants, PermissionToken};
use log::{info, error};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("🚀 Alael Kernel Booting...");

    // Load ethics
    let ethics = EthicalConstants::default();
    info!("📜 Ethical Constants: {:?}", ethics);

    // Test core rule
    if ethics.no_innocent_civilians {
        info!("✅ RULE: Innocents protected. Verified predators OK post-scoring.");
    } else {
        error!("❌ ETHICS BREACH: Innocents targetable—SHUTDOWN!");
        return Err("Ethical failure".into());
    }

    // Token test
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let test_token = PermissionToken {
        action: "passive_scrape".to_string(),
        target: "discord.threat_channel".to_string(),
        expires_at: now + ethics.token_lifespan_seconds,
        signature: "sha256(ethics+action)".to_string(),
    };
    info!("🔑 Testing token: {:?}", test_token);
    if test_token.is_valid(&ethics, now) {
        info!("✅ TOKEN APPROVED: Passive action on verified threat.");
    } else {
        error!("❌ TOKEN REJECTED: Ethics/expiry fail.");
    }

    info!("🏗️ Wasm sandbox stub ready.");

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    info!("🛑 Kernel v0.2.0 ready.");

    Ok(())
}
