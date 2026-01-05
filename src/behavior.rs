// Behavior definitions and ranges for emotion system

/// Positive behaviors - always use positive_multiplier
pub mod positive_behaviors {
    pub const RANGES: &[(&str, (i32, i32))] = &[
        ("Neutral Behavior", (0, 0)),
        ("LightPositiveBehavior", (1, 4)),
        ("ModeratePositiveBehavior", (5, 9)),
        ("StrongPositiveBehavior", (10, 16)),
        ("ExtremePositiveBehavior", (17, 25)),
        // Sexual positive behaviors
        ("Sexual_Neutral", (-1, 1)),
        ("Sexual_Light", (2, 5)),
        ("Sexual_Moderate", (6, 10)),
        ("Sexual_Strong", (11, 19)),
        ("Sexual_Extreme", (20, 40)),
    ];
}

/// Negative behaviors - always use negative_multiplier
pub mod negative_behaviors {
    pub const RANGES: &[(&str, (i32, i32))] = &[
        ("LightNegativeBehavior", (-5, -1)),      // From -5 to -1
        ("ModerateNegativeBehavior", (-15, -6)),   // From -15 to -6
        ("StrongNegativeBehavior", (-30, -16)),    // From -30 to -16
        ("ExtremeNegativeBehavior", (-50, -31)),   // From -50 to -31
        // Sexual negative behaviors
        ("Sexual_Neg_Light", (-10, -1)),           // From -10 to -1
        ("Sexual_Neg_Moderate", (-20, -11)),       // From -20 to -11
        ("Sexual_Neg_Strong", (-80, -21)),         // From -80 to -21
        ("Sexual_Neg_Extreme", (-100, -81)),       // From -100 to -81
    ];
}


// Helper functions

/// Get behavior range by name (used in calculate_changes)
pub fn get_behavior_range(behavior: &str) -> Option<(i32, i32)> {
    // Check positive behaviors first
    if let Some(range) = get_positive_behavior_range(behavior) {
        return Some(range);
    }
    // Then check negative behaviors
    get_negative_behavior_range(behavior)
}

/// Determine behavior type from a value (used in parse_behavior_from_response)
pub fn get_behavior_from_value(value: i32) -> Option<&'static str> {
    // Check positive behaviors first
    for (name, (min, max)) in positive_behaviors::RANGES.iter() {
        if value >= *min && value <= *max {
            return Some(name);
        }
    }
    // Then check negative behaviors
    for (name, (min, max)) in negative_behaviors::RANGES.iter() {
        if value >= *min && value <= *max {
            return Some(name);
        }
    }
    None
}

/// Check if a behavior is in the positive behaviors list
pub fn is_positive_behavior(behavior_category: &str) -> bool {
    positive_behaviors::RANGES.iter().any(|(name, _)| *name == behavior_category)
}

/// Check if a behavior is in the negative behaviors list
pub fn is_negative_behavior(behavior_category: &str) -> bool {
    negative_behaviors::RANGES.iter().any(|(name, _)| *name == behavior_category)
}

/// Get behavior range from positive behaviors
pub fn get_positive_behavior_range(behavior: &str) -> Option<(i32, i32)> {
    positive_behaviors::RANGES.iter()
        .find(|(name, _)| *name == behavior)
        .map(|(_, range)| *range)
}

/// Get behavior range from negative behaviors
pub fn get_negative_behavior_range(behavior: &str) -> Option<(i32, i32)> {
    negative_behaviors::RANGES.iter()
        .find(|(name, _)| *name == behavior)
        .map(|(_, range)| *range)
}
