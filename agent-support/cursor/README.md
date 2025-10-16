# git-ai-tracker for Cursor

<div align="center">
  <img src="git-ai.png" width="100" height="100" />
  <h3>Track AI-Generated Code in Cursor IDE</h3>
  <p>See what percentage of your codebase was written by AI vs humans</p>
</div>

---

## 🎯 What is git-ai-tracker?

**git-ai-tracker** automatically tracks AI code contributions in Cursor IDE. Every commit includes AI% metadata in git notes - completely transparent and under your control.

## ✨ Features

- ✅ **Automatic Detection** - Tracks Cursor AI suggestions and commands
- 📊 **Detailed Statistics** - See AI% for any commit
- 🔒 **Privacy First** - All data stored locally in git notes
- 🚀 **Zero Configuration** - Install and forget
- ⚡ **Fast & Lightweight** - Minimal performance impact

## 🚀 Quick Start

### 1. Install git-ai-tracker

```bash
# macOS / Linux
curl -sSL https://raw.githubusercontent.com/RaahimNadeem/git-ai-tracker/main/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/RaahimNadeem/git-ai-tracker/main/install.ps1 | iex

# Homebrew
brew tap RaahimNadeem/tap
brew install git-ai-tracker
```

### 2. Install Hooks

```bash
git-ai-tracker install-hooks
```

This automatically configures Cursor hooks in `~/.cursor/hooks.json`.

### 3. Restart Cursor

Quit Cursor completely (Cmd+Q) and reopen it.

### 4. Start Coding!

Use Cursor's AI features normally - everything is tracked automatically! ✨

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
║           AI Contribution: 78%                           ║
╠══════════════════════════════════════════════════════════╣
║  AI Lines:       52 / 67   ( 78%)                        ║
║  Human Lines:    15 / 67   ( 22%)                        ║
╚══════════════════════════════════════════════════════════╝
```

## 🔧 How It Works

1. **Cursor hooks** are configured in `~/.cursor/hooks.json`
2. **Before/after AI edits**, hooks call git-ai-tracker
3. **Checkpoints track** AI vs human contributions
4. **On commit**, git-ai-tracker calculates AI% and stores it in git notes

### What Gets Tracked

- ✅ Cursor AI commands (Cmd+K)
- ✅ Cursor Composer edits
- ✅ AI-generated code insertions
- ✅ Chat-based code changes
- ❌ Manual typing

## 📖 Commands

| Command | Description |
|---------|-------------|
| `git-ai-tracker stats-display` | Show AI% for latest commit |
| `git-ai-tracker stats-repo` | Show aggregate AI% across repository |
| `git-ai-tracker blame <file>` | Enhanced git blame with AI attribution |
| `git-ai-tracker install-hooks` | (Re)install Cursor hooks |

## ⚙️ Configuration

Hooks are stored in `~/.cursor/hooks.json`:

```json
{
  "hooks": {
    "afterFileEdit": [
      {
        "command": "git-ai-tracker checkpoint cursor --hook-input \"$(cat)\""
      }
    ],
    "beforeSubmitPrompt": [
      {
        "command": "git-ai-tracker checkpoint cursor --hook-input \"$(cat)\""
      }
    ]
  },
  "version": 1
}
```

## 🔍 Troubleshooting

### Hooks Not Working

1. **Check hooks file exists:**
   ```bash
   cat ~/.cursor/hooks.json
   ```

2. **Reinstall hooks:**
   ```bash
   git-ai-tracker install-hooks
   ```

3. **Restart Cursor** (Cmd+Q, then reopen)

### Still Showing 0% AI

1. **Check working log:**
   ```bash
   git-ai-tracker checkpoint --show-working-log
   ```

2. **Verify hooks are firing:**
   - Use Cursor AI features (Cmd+K)
   - Save the file
   - Check if checkpoints were created

3. **Make sure you committed with git-ai-tracker active**

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

