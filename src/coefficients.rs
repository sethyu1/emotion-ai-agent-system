
/// Coefficients for emotion and relationship calculations
///
/// Each coefficient pair represents:
/// (positive_behavior_multiplier, negative_behavior_multiplier)
///
/// - Positive behaviors (≥ 0) use the left coefficient
/// - Negative behaviors (< 0) use the right coefficient

// Relationship Coefficients - How relationships affect behavior intensity
pub const RELATIONSHIP_COEFFICIENTS: &[(&str, (f32, f32))] = &[
    ("Romantic Partner", (2.0, 0.2)),      // Strong positive amplification, weak negative
    ("Flirtatious Relationship", (1.5, 0.5)),
    ("Close Friend", (1.3, 0.7)),
    ("Friend", (1.1, 0.9)),
    ("Acquaintance", (1.0, 1.0)),         // Neutral baseline
    ("Dislike", (0.9, 1.1)),              // Weak positive, slight negative amplification
    ("Hostile", (0.7, 1.3)),
    ("Resentment", (0.2, 1.5)),
    ("Arch-nemesis", (0.1, 2.0)),         // Very weak positive, strong negative amplification
];

// Emotion Coefficients - How emotions affect behavior intensity
pub const EMOTION_COEFFICIENTS: &[(&str, (f32, f32))] = &[
    ("Extremely Happy", (1.5, 0.2)),      // Strong positive amplification, weak negative
    ("Very Happy", (1.3, 0.5)),
    ("Happy", (1.2, 0.7)),
    ("Content", (1.1, 0.9)),
    ("Positive Calm", (1.05, 0.95)),
    ("Neutral", (1.0, 1.0)),             // Neutral baseline
    ("Irritated", (0.8, 1.2)),           // Weak positive, slight negative amplification
    ("Frustrated", (0.6, 1.5)),
    ("Angry", (0.3, 1.8)),
    ("Extremely Angry", (0.1, 2.5)),     // Very weak positive, strong negative amplification
];

/// Represents behavior coefficient multipliers
#[derive(Debug, Clone, Copy)]
pub struct BehaviorCoefficients {
    pub positive_multiplier: f32,
    pub negative_multiplier: f32,
}

impl BehaviorCoefficients {
    /// Get the appropriate multiplier based on behavior sign
    pub fn get_multiplier(&self, is_positive_behavior: bool) -> f32 {
        if is_positive_behavior {
            self.positive_multiplier
        } else {
            self.negative_multiplier
        }
    }
}

// Helper functions to get coefficients

/// Get relationship coefficients with better error handling
pub fn get_relationship_coefficient(relationship: &str) -> Option<BehaviorCoefficients> {
    RELATIONSHIP_COEFFICIENTS.iter()
        .find(|(name, _)| *name == relationship)
        .map(|(_, (pos, neg))| BehaviorCoefficients {
            positive_multiplier: *pos,
            negative_multiplier: *neg,
        })
}

/// Get emotion coefficients with better error handling
pub fn get_emotion_coefficient(emotion: &str) -> Option<BehaviorCoefficients> {
    EMOTION_COEFFICIENTS.iter()
        .find(|(name, _)| *name == emotion)
        .map(|(_, (pos, neg))| BehaviorCoefficients {
            positive_multiplier: *pos,
            negative_multiplier: *neg,
        })
}

/// Legacy function for backward compatibility - returns tuple
pub fn get_relationship_coefficient_tuple(relationship: &str) -> Option<(f32, f32)> {
    RELATIONSHIP_COEFFICIENTS.iter()
        .find(|(name, _)| *name == relationship)
        .map(|(_, coeffs)| *coeffs)
}

/// Legacy function for backward compatibility - returns tuple
pub fn get_emotion_coefficient_tuple(emotion: &str) -> Option<(f32, f32)> {
    EMOTION_COEFFICIENTS.iter()
        .find(|(name, _)| *name == emotion)
        .map(|(_, coeffs)| *coeffs)
}

/// Get all available relationship types
pub fn get_relationship_types() -> Vec<&'static str> {
    RELATIONSHIP_COEFFICIENTS.iter().map(|(name, _)| *name).collect()
}

/// Get all available emotion types
pub fn get_emotion_types() -> Vec<&'static str> {
    EMOTION_COEFFICIENTS.iter().map(|(name, _)| *name).collect()
}