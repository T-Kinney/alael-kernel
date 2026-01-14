//! Vehuel - Behavioral Prediction Agent (Vol III).
//! Risk scoring for grooming patterns.

/// Score grooming evidence (0.0-1.0).
pub fn score_grooming(evidence: &str) -> f32 {
    let mut score = 0.9;  // Base high for demo
    if evidence.contains("dont tell") || evidence.contains("pic") {
        score = 0.98;
    }
    score
}

