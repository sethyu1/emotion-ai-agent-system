#!/bin/bash

# Emotion AI Agent System - GitHub Setup Script

echo "🚀 Setting up Emotion AI Agent System for GitHub..."

# Initialize git repository
echo "📝 Initializing Git repository..."
git init

# Add all files
echo "📦 Adding files to Git..."
git add .

# Create initial commit
echo "💾 Creating initial commit..."
git commit -m "Initial commit: Emotion AI Agent System

- AI-powered behavior classification using Grok 4
- Configurable emotion and relationship coefficients
- 18 behavior categories with precise value ranges
- High-performance Rust async API
- Comprehensive test suite and documentation"

# Instructions for GitHub setup
echo ""
echo "✅ Git repository initialized!"
echo ""
echo "📋 Next steps for GitHub:"
echo "1. Create a new repository on GitHub named 'emotion-ai-agent-system'"
echo "2. Copy the repository URL"
echo "3. Run these commands:"
echo "   git branch -M main"
echo "   git remote add origin https://github.com/YOUR_USERNAME/emotion-ai-agent-system.git"
echo "   git push -u origin main"
echo ""
echo "4. Update the repository URL in Cargo.toml with your actual GitHub URL"
echo "5. Enable GitHub Actions for CI/CD"
echo ""
echo "🎉 Your Emotion AI Agent System is ready for GitHub!"
