// Emotion and Relationship Ranges

// Relationship Ranges
// Format: (min, max)
pub const RELATIONSHIP_RANGES: &[(&str, (i32, i32))] = &[
    ("Romantic Partner", (2000, 5000)),
    ("Ambiguous", (1001, 2000)),
    ("Close Friend", (501, 1000)),
    ("Friend", (151, 500)),
    ("Acquaintance", (0, 150)),
    ("Dislike", (-1, -150)),
    ("Hostile", (-151, -500)),
    ("Resentment", (-501, -1000)),
    ("Arch-nemesis", (-1001, -4000)),
];

// Emotion Ranges
// Format: (min, max) - Note: Neutral is a single value (0)
pub const EMOTION_RANGES: &[(&str, (i32, i32))] = &[
    ("Extremely Happy", (151, 200)),
    ("Very Happy", (101, 150)),
    ("Happy", (61, 100)),
    ("Content", (31, 60)),
    ("Positive Calm", (1, 30)),
    ("Neutral", (0, 0)),
    ("Irritated", (-31, -60)),
    ("Frustrated", (-61, -100)),
    ("Angry", (-101, -150)),
    ("Extremely Angry", (-151, -200)),
];

// Helper functions

/// Determine relationship level from value (used in main.rs for Grok payload)
pub fn get_relationship_from_value(value: i32) -> Option<&'static str> {
    for (name, (min, max)) in RELATIONSHIP_RANGES.iter() {
        if value >= *min && value <= *max {
            return Some(name);
        }
    }
    None
}

/// Determine emotion level from value (used in main.rs for Grok payload)
pub fn get_emotion_from_value(value: i32) -> Option<&'static str> {
    for (name, (min, max)) in EMOTION_RANGES.iter() {
        if value >= *min && value <= *max {
            return Some(name);
        }
    }
    None
}
