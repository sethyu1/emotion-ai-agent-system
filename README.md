# Emotion AI Agent System

A high-performance Rust API that analyzes user interactions in role-playing games using Grok AI, classifying behaviors and calculating emotional/relational impacts with configurable coefficients.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Build Status](https://img.shields.io/badge/build-passing-brightgreen)

## ✨ Features

- 🤖 **AI-Powered Analysis**: Grok 4 integration for sophisticated behavior classification
- ⚡ **High Performance**: Async Rust with optimized HTTP handling
- 🎯 **Precise Calculations**: Configurable behavior ranges and coefficient systems
- 🔒 **Secure**: Environment-based API key management
- 📊 **Observable**: Health checks and debug logging
- 🏗️ **Modular**: Clean separation of concerns across multiple modules
- 🎮 **Gaming Focused**: Designed specifically for role-playing game emotional dynamics

## 🚀 Quick Start

1. **Install Rust**: Make sure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/)

2. **Clone/Download the project**

3. **Environment Variables**:
   Create a `.env` file in the root directory with your xAI API key:
   ```
   XAI_API_KEY=your_api_key_here
   ```

4. **Build and Run**:
   ```bash
   cargo build --release
   cargo run --release
   ```

The server will start on `http://127.0.0.1:9527`

## API Endpoint

### POST `/analyze-emotion`

Analyzes user input and returns emotion/relationship changes.

**Request Body:**
```json
{
  "character_history": "Previous interaction text...",
  "character_personality": "Character personality description...",
  "current_relationship": 75,
  "current_emotion": 50,
  "user_input": "User's message/input..."
}
```

**Response:**
```json
{
  "emotion_change": "emotion + 25",
  "relationship_change": "relationship - 10"
}
```

## Behavior Categories

The system categorizes behavior into:

1. **Light Behavior** (Mild Positive/Negative)
2. **Moderate Behavior** (Moderate Positive/Negative)
3. **Strong Behavior** (Strong Positive/Negative)
4. **Extreme Behavior** (Extreme Positive/Negative)
5. **Sexual Behavior**
6. **Neutral Behavior**

## Deployment on RunPod

1. **Create a RunPod account** and set up a GPU instance (CPU should be sufficient for this use case)

2. **Choose a base image** with Rust installed, or use a Ubuntu base and install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Upload your project files** to the RunPod instance

4. **Install dependencies and build**:
   ```bash
   apt update && apt install -y pkg-config libssl-dev
   cargo build --release
   ```

5. **Set environment variable**:
   ```bash
   export XAI_API_KEY=your_api_key_here
   ```

6. **Run the server**:
   ```bash
   ./target/release/emotion-system
   ```

7. **Expose the port** (9527) in RunPod networking settings

8. **Test the endpoint** using curl or any HTTP client:
   ```bash
   curl -X POST http://your-runpod-ip:9527/analyze-emotion \
     -H "Content-Type: application/json" \
     -d '{
       "character_history": "Hello, how are you?",
       "character_personality": "Friendly and caring",
       "current_relationship": 75,
       "current_emotion": 50,
       "user_input": "You look amazing today!"
     }'
   ```

## Current Emotion/Relationship Calculation Rules

The current implementation uses these placeholder rules:

- **Light Positive**: emotion +10, relationship +5
- **Light Negative**: emotion -10, relationship -5
- **Moderate Positive**: emotion +25, relationship +15
- **Moderate Negative**: emotion -25, relationship -15
- **Strong Positive**: emotion +50, relationship +30
- **Strong Negative**: emotion -50, relationship -30
- **Extreme Positive**: emotion +100, relationship +50
- **Extreme Negative**: emotion -100, relationship -50
- **Sexual Behavior**: emotion +75, relationship +25
- **Neutral**: emotion +0, relationship +0

You can modify the `calculate_changes` function in `src/main.rs` to implement your custom rules.

## 🤝 Contributing

We welcome contributions! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Fork the repository
2. Clone your fork: `git clone https://github.com/sethyu1/emotion-ai-agent-system.git`
3. Create a feature branch: `git checkout -b feature/amazing-feature`
4. Make your changes and add tests
5. Run tests: `cargo test`
6. Commit your changes: `git commit -am 'Add amazing feature'`
7. Push to the branch: `git push origin feature/amazing-feature`
8. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **xAI** for providing the Grok AI API
- **Rust Community** for the excellent async runtime and ecosystem
- **Contributors** who help improve this project

## 📞 Support

If you have any questions or need help:

- Open an issue on GitHub
- Check the documentation in this README
- Review the code comments for implementation details

---

**Made with ❤️ for the gaming community**
