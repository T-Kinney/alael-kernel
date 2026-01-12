//! Alael Kernel v0.3.0 - Ethics + Token. Wasm next.

mod ethics;
use ethics::{EthicalConstants, PermissionToken};
use log::info;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("🚀 Alael Kernel v0.3.0 Booting...");

    let ethics = EthicalConstants::default();
    info!("📜 Ethical Constants: {:?}", ethics);

    if ethics.no_innocent_civilians {
        info!("✅ RULE: Innocents protected. Predators OK post-scoring.");
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let test_token = PermissionToken {
        action: "passive_scrape".to_string(),
        target: "discord.threat_channel".to_string(),
        expires_at: now + 3600,
        signature: "sha256_stub".to_string(),
    };
    info!("🔑 Token test: {:?}", test_token);
    if test_token.is_valid(&ethics, now) {
        info!("✅ TOKEN APPROVED for threat channel.");
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    info!("🛑 v0.3.0 ready: Deploy to DO next.");

    Ok(())
}
