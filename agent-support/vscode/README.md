# git-ai-tracker for VS Code

<div align="center">
  <img src="git-ai.png" width="100" height="100" />
  <h3>Track AI-Generated Code from GitHub Copilot</h3>
  <p>See what percentage of your codebase was written by AI vs humans</p>
</div>

---

## 🎯 What is git-ai-tracker?

**git-ai-tracker** automatically tracks AI code contributions from GitHub Copilot. Every commit includes AI% metadata in git notes - completely transparent and under your control.

## ✨ Features

- ✅ **Automatic Detection** - Tracks GitHub Copilot inline suggestions and chat edits
- 📊 **Detailed Statistics** - See AI% for any commit
- 🔒 **Privacy First** - All data stored locally in git notes
- 🚀 **Zero Configuration** - Install and forget
- ⚡ **Fast & Lightweight** - Minimal performance impact

## 🚀 Quick Start

### 1. Install the Extension

**Option A: Automatic (Recommended)**
```bash
# Run from terminal to install everything
curl -sSL https://raw.githubusercontent.com/RaahimNadeem/git-ai-tracker/main/install.sh | bash
```

**Option B: Manual**
1. Install this extension from VS Code Marketplace
2. Install the git-ai-tracker CLI:
   ```bash
   brew install git-ai-tracker
   # or
   curl -sSL https://raw.githubusercontent.com/RaahimNadeem/git-ai-tracker/main/install.sh | bash
   ```

### 2. Start Coding!

That's it! Just code normally with GitHub Copilot, and AI contributions are tracked automatically.

## 📊 View AI Statistics

After committing code:

```bash
# Show beautiful AI% visualization
git-ai-tracker stats-display

# View detailed stats
git-ai-tracker stats --json

# Analyze entire repository
git-ai-tracker stats-repo
```

**Example Output:**
```
╔══════════════════════════════════════════════════════════╗
║           🤖 AI CONTRIBUTION TRACKER 🤖                  ║
╠══════════════════════════════════════════════════════════╣
║           AI Contribution: 67%                           ║
╠══════════════════════════════════════════════════════════╣
║  AI Lines:       45 / 67   ( 67%)                        ║
║  Human Lines:    22 / 67   ( 33%)                        ║
╚══════════════════════════════════════════════════════════╝
```

## 🔧 How It Works

1. **Extension monitors** your VS Code editor for Copilot suggestions
2. **Detects AI edits** when you accept suggestions or use Copilot chat
3. **Creates checkpoints** to mark AI vs human contributions
4. **On commit**, git-ai-tracker calculates AI% and stores it in git notes

## 📖 Commands

| Command | Description |
|---------|-------------|
| `git-ai-tracker stats-display` | Show AI% for latest commit |
| `git-ai-tracker stats-repo` | Show aggregate AI% across repository |
| `git-ai-tracker blame <file>` | Enhanced git blame with AI attribution |

## ⚙️ Configuration

Open VS Code Settings and search for "git-ai-tracker":

- **Enable Checkpoint Logging**: Show notifications when AI/human edits are detected (useful for debugging)

## 🔍 Troubleshooting

### Extension Not Detecting Copilot

1. **Check extension is active:**
   - Open Command Palette (Cmd+Shift+P)
   - Look for "🤖 AI Code Tracker is now active!" notification

2. **Check git-ai-tracker is installed:**
   ```bash
   git-ai-tracker --version
   ```

3. **View extension logs:**
   - Open Output panel (Cmd+Shift+U)
   - Select "Log (Extension Host)" from dropdown
   - Look for `[git-ai-tracker]` messages

### Still Showing 0% AI

Make sure you're using GitHub Copilot features that are tracked:
- ✅ Inline suggestions (Tab to accept)
- ✅ Multi-line completions
- ✅ Copilot chat code insertions
- ❌ Manual typing (even if inspired by AI)

## 🤝 Contributing

Contributions welcome! Visit [github.com/RaahimNadeem/git-ai-tracker](https://github.com/RaahimNadeem/git-ai-tracker)

## 📄 License

MIT License - Copyright © 2025 Raahim Nadeem

## 🔗 Links

- [GitHub Repository](https://github.com/RaahimNadeem/git-ai-tracker)
- [Documentation](https://github.com/RaahimNadeem/git-ai-tracker#readme)
- [Report Issues](https://github.com/RaahimNadeem/git-ai-tracker/issues)

---

<div align="center">
  <p>Made with ❤️ by <a href="https://github.com/RaahimNadeem">Raahim Nadeem</a></p>
  <p><a href="https://github.com/RaahimNadeem/git-ai-tracker">⭐ Star on GitHub</a></p>
</div>

