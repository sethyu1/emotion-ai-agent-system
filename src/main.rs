use axum::{
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use rand::Rng;
use chrono;
use std::env;

mod behavior;
mod coefficients;
mod ranges;
mod system_prompt;

#[derive(Deserialize)]
struct EmotionRequest {
    character_history: String,
    character_personality: String,
    current_relationship: i32,
    current_emotion: i32,
    user_input: String,
}

#[derive(Serialize)]
struct EmotionResponse {
    emotion_change: i32,
    relationship_change: i32,
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get the API key from environment
    let api_key = env::var("XAI_API_KEY")
        .expect("XAI_API_KEY environment variable must be set. Create a .env file with your xAI API key.");

    // Build the application
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/analyze-emotion", post(analyze_emotion))
        .layer(CorsLayer::permissive())
        .with_state(AppState { api_key });

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9527").await.unwrap();
    println!("Server running on http://0.0.0.0:9527");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    api_key: String,
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "emotion-system",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn analyze_emotion(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(payload): Json<EmotionRequest>,
) -> Result<Json<EmotionResponse>, StatusCode> {

    // Convert emotion and relationship i32 values to category names for Grok
    let emotion_str = ranges::get_emotion_from_value(payload.current_emotion)
        .unwrap_or("Neutral");
    let relationship_str = ranges::get_relationship_from_value(payload.current_relationship)
        .unwrap_or("Acquaintance");

    // Construct the user prompt with dynamic data
    let final_prompt = format!(
        "{}\n\nCharacter's History Output: {}\n{}'s Personality = {}\nCurrent Relationship: {}\nCurrent Emotion: {}\nUser Input: {}",
        system_prompt::SYSTEM_PROMPT,
        payload.character_history,
        payload.character_history.split_whitespace().next().unwrap_or("Character"),
        payload.character_personality,
        relationship_str,
        emotion_str,
        payload.user_input
    );

    // Make request to Grok API
    let client = reqwest::Client::new();
    let grok_response = client
        .post("https://api.x.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", state.api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": "grok-4-1-fast-non-reasoning",
            "messages": [
                {
                    "role": "user",
                    "content": final_prompt
                }
            ]
        }))
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !grok_response.status().is_success() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let grok_data: serde_json::Value = grok_response
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extract the category from Grok's response
    let grok_response = grok_data["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Neutral Behavior")
        .trim();

    // Debug: Log Grok's response
    println!("🤖 Grok Response: '{}'", grok_response);

    // Parse the behavior category from Grok's response
    let behavior_category = parse_behavior_from_response(grok_response);

    // Debug: Log parsed behavior category
    println!("🎭 Parsed Behavior: '{}'", behavior_category);

    // Calculate emotion and relationship changes based on behavior category and current state
    let (emotion_change, relationship_change) = calculate_changes(
        &behavior_category,
        payload.current_emotion,
        payload.current_relationship
    );

    Ok(Json(EmotionResponse {
        emotion_change,
        relationship_change,
    }))
}


fn calculate_changes(behavior_category: &str, current_emotion: i32, current_relationship: i32) -> (i32, i32) {

    // Get the range for this behavior category
    let behavior_range = match behavior::get_behavior_range(&behavior_category) {
        Some(range) => range,
        None => {
            // If behavior not recognized, return neutral changes
            return (0, 0);
        }
    };

    // Generate random value within the behavior range
    let mut rng = rand::thread_rng();
    let random_value = if behavior_range.0 == behavior_range.1 {
        // Single value (like Neutral)
        behavior_range.0 as f32
    } else {
        // Random value within range
        rng.gen_range(behavior_range.0..=behavior_range.1) as f32
    };

    // Debug: Log the random value generated
    println!("🎲 Random value: {:.2} (range: [{}, {}])", random_value, behavior_range.0, behavior_range.1);

    // Convert current emotion and relationship values to their corresponding names
    let emotion_name = ranges::get_emotion_from_value(current_emotion)
        .unwrap_or("Neutral");

    let relationship_name = ranges::get_relationship_from_value(current_relationship)
        .unwrap_or("Acquaintance");

    // Get emotion coefficient for current emotion state
    let emotion_coeff = coefficients::get_emotion_coefficient(emotion_name)
        .unwrap_or(coefficients::BehaviorCoefficients {
            positive_multiplier: 1.0,
            negative_multiplier: 1.0,
        }); // Default to neutral if not found

    // Get relationship coefficient for current relationship state
    let relationship_coeff = coefficients::get_relationship_coefficient(relationship_name)
        .unwrap_or(coefficients::BehaviorCoefficients {
            positive_multiplier: 1.0,
            negative_multiplier: 1.0,
        }); // Default to neutral if not found

    // Determine if this is positive or negative behavior
    let is_positive_behavior = if behavior::is_positive_behavior(&behavior_category) {
        true   // Behavior is in positive list - always use positive multiplier
    } else if behavior::is_negative_behavior(&behavior_category) {
        false  // Behavior is in negative list - always use negative multiplier
    } else {
        // Unknown behavior - fall back to mathematical sign
        random_value >= 0.0
    };

    // Debug: Log behavior classification and coefficients
    println!("📊 Behavior: '{}' | Positive: {} | Random value: {:.2}",
             behavior_category, is_positive_behavior, random_value);
    println!("👥 Emotion: '{}' -> Coeff: ({:.2}, {:.2})",
             emotion_name, emotion_coeff.positive_multiplier, emotion_coeff.negative_multiplier);
    println!("🤝 Relationship: '{}' -> Coeff: ({:.2}, {:.2})",
             relationship_name, relationship_coeff.positive_multiplier, relationship_coeff.negative_multiplier);

    // Get the appropriate multipliers based on behavior classification
    let emotion_multiplier = emotion_coeff.get_multiplier(is_positive_behavior);
    let relationship_multiplier = relationship_coeff.get_multiplier(is_positive_behavior);

    // Apply coefficients to the behavior value
    let final_emotion_change = random_value * emotion_multiplier;
    let final_relationship_change = random_value * relationship_multiplier;

    // Debug: Log final calculations
    println!("🧮 Emotion: {:.2} × {:.2} = {:.2} → {}",
             random_value, emotion_multiplier, final_emotion_change, final_emotion_change.round() as i32);
    println!("🧮 Relationship: {:.2} × {:.2} = {:.2} → {}",
             random_value, relationship_multiplier, final_relationship_change, final_relationship_change.round() as i32);

    // Return the numeric changes
    (final_emotion_change.round() as i32, final_relationship_change.round() as i32)
}

/// Parse behavior category from Grok's response
fn parse_behavior_from_response(response: &str) -> String {
    // Clean up the response and extract the behavior category
    let response = response.trim();

    // Look for the behavior categories in the response
    // Grok should return one of the behavior names
    let behavior_names = [
        "Neutral Behavior",
        "LightPositiveBehavior",
        "LightNegativeBehavior",
        "ModeratePositiveBehavior",
        "ModerateNegativeBehavior",
        "StrongPositiveBehavior",
        "StrongNegativeBehavior",
        "ExtremePositiveBehavior",
        "ExtremeNegativeBehavior",
        "Sexual_Neutral",
        "Sexual_Light",
        "Sexual_Moderate",
        "Sexual_Strong",
        "Sexual_Extreme",
        "Sexual_Neg_Light",
        "Sexual_Neg_Moderate",
        "Sexual_Neg_Strong",
        "Sexual_Neg_Extreme",
    ];

    // Find the behavior name in the response
    for behavior in &behavior_names {
        if response.contains(behavior) {
            return behavior.to_string();
        }
    }

    // If no specific behavior found, try to match partial patterns
    if response.contains("Sexual Behavior") || response.contains("Sexual") {
        return "Sexual_Neutral".to_string(); // Default to neutral sexual
    }
    if response.contains("Positive") && response.contains("Extreme") {
        return "ExtremePositiveBehavior".to_string();
    }
    if response.contains("Positive") && response.contains("Strong") {
        return "StrongPositiveBehavior".to_string();
    }
    if response.contains("Positive") && response.contains("Moderate") {
        return "ModeratePositiveBehavior".to_string();
    }
    if response.contains("Positive") && response.contains("Light") {
        return "LightPositiveBehavior".to_string();
    }
    if response.contains("Negative") && response.contains("Extreme") {
        return "ExtremeNegativeBehavior".to_string();
    }
    if response.contains("Negative") && response.contains("Strong") {
        return "StrongNegativeBehavior".to_string();
    }
    if response.contains("Negative") && response.contains("Moderate") {
        return "ModerateNegativeBehavior".to_string();
    }
    if response.contains("Negative") && response.contains("Light") {
        return "LightNegativeBehavior".to_string();
    }

    // Default fallback
    "Neutral Behavior".to_string()
}
