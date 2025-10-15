<div align="center">
  <img src="assets/docs/git-ai.png" width="100" height="100" />
  <h1>git-ai-tracker</h1>
  <p><strong>Automatically track AI code contributions with GitHub Copilot & Cursor</strong></p>
  
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![GitHub Release](https://img.shields.io/github/v/release/RaahimNadeem/git-ai-tracker)](https://github.com/RaahimNadeem/git-ai-tracker/releases)
  [![Build Status](https://github.com/RaahimNadeem/git-ai-tracker/workflows/Release/badge.svg)](https://github.com/RaahimNadeem/git-ai-tracker/actions)
</div>

---

## 🎯 What is git-ai-tracker?

**git-ai-tracker** is a transparent Git wrapper that automatically tracks AI-generated code contributions. Every commit automatically includes AI% metadata in git notes - no manual work required!

### ✨ Key Features

- ✅ **Automatic AI% Tracking** - Every commit automatically tagged with AI contribution percentage
- 🤖 **GitHub Copilot & Cursor Support** - Works seamlessly with your favorite AI coding assistants  
- 📊 **Beautiful Visualizations** - See AI% with gorgeous terminal displays
- 🔄 **GitHub PR Integration** - Automatic AI stats comments on pull requests
- 🚀 **Zero Configuration** - Install once, works everywhere
- ⚡ **Blazing Fast** - Written in Rust for maximum performance
- 🔒 **Privacy First** - All data stays in git notes, fully under your control

---

## 🚀 Quick Start

### Installation

**macOS / Linux:**
```bash
curl -sSL https://raw.githubusercontent.com/RaahimNadeem/git-ai-tracker/main/install.sh | bash
```

**Homebrew (macOS):**
```bash
brew tap RaahimNadeem/tap
brew install git-ai-tracker
```

**Windows (PowerShell):**
```powershell
powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/RaahimNadeem/git-ai-tracker/main/install.ps1 | iex"
```

### Usage

After installation, git-ai-tracker works transparently. Just use git normally:

```bash
# Make changes with GitHub Copilot or Cursor
# ... code with AI assistance ...

# Commit as usual
git commit -m "Add new feature"

# ✨ AI% is automatically added to git notes!

# View AI contribution stats
git-ai-tracker stats-display

# Output:
# ╔══════════════════════════════════════════════════════════╗
# ║                                                          ║
# ║           🤖 AI CONTRIBUTION TRACKER 🤖                  ║
# ║                                                          ║
# ╠══════════════════════════════════════════════════════════╣
# ║                                                          ║
# ║           AI Contribution: 67%                           ║
# ║                                                          ║
# ╚══════════════════════════════════════════════════════════╝
```

---

## 📊 Features in Detail

### 1. Automatic AI% in Git Notes

Every commit automatically includes AI contribution metadata:

```bash
git notes show HEAD

# Output includes:
# --- AI Contribution Metadata ---
# AI-Percentage: 67%
# AI-Lines: 45/67
# Human-Lines: 22/67
# Mixed-Lines: 5
# Generated: 2025-10-15T12:34:56Z
```

### 2. Beautiful Terminal Display

```bash
git-ai-tracker stats-display
# or the shorter alias:
git-ai-tracker show-ai
```

Shows a gorgeous visual breakdown of AI vs human contributions with color-coded bars and detailed statistics.

### 3. Enhanced Git Blame

```bash
git-ai-tracker blame <file>
```

See which lines were written by AI vs humans, with full attribution.

### 4. GitHub PR Integration

Add this to your repository's `.github/workflows/` to get automatic AI stats comments on every pull request:

```yaml
# Already included in git-ai-tracker!
# Just enable GitHub Actions in your repo
```

Pull requests automatically get comments like:

> ## 🤖 AI Contribution Statistics
>
> This PR contains **45%** AI-generated code.
>
> ```
> Human  ███████░░░░░░  AI
>         55%         45%
> ```

---

## 🏢 Organization-Wide Deployment

### Option 1: Install Script (Recommended)

Distribute via your internal tools:

```bash
# Add to company dotfiles or onboarding scripts
curl -sSL https://raw.githubusercontent.com/RaahimNadeem/git-ai-tracker/main/install.sh | bash
```

### Option 2: Homebrew Tap

```bash
# Create internal Homebrew tap
brew tap mycompany/internal
# Add git-ai-tracker formula
brew install git-ai-tracker
```

### Option 3: MDM Deployment

Package the binary and deploy via Jamf/Intune/etc:
- Binary location: `~/.git-ai-tracker/bin/git-ai-tracker`
- PATH update: Add to `~/.zshrc` or `~/.bashrc`

### Option 4: GitHub Action

Use as a GitHub Action to enforce AI tracking across all repos:

```yaml
- name: Check AI contribution
  uses: RaahimNadeem/git-ai-tracker@v1
  with:
    max-ai-percentage: 80  # Fail if AI% exceeds threshold
```

---

## 🤖 Supported AI Tools

| Tool | Status | Notes |
|------|--------|-------|
| GitHub Copilot | ✅ Full Support | VS Code extension included |
| Cursor | ✅ Full Support | Native integration |
| Claude Code | ⚠️ Not Supported | Removed in v1.0 |
| Other AI Tools | 🔄 Coming Soon | Extensible architecture |

---

## 📖 Commands

### Core Commands

```bash
# Show AI stats for a commit
git-ai-tracker stats [commit]
git-ai-tracker stats --json  # JSON output

# Show prominent AI% display
git-ai-tracker stats-display [commit]
git-ai-tracker show-ai [commit]  # Alias

# Enhanced blame with AI attribution
git-ai-tracker blame <file>

# Create checkpoints (used internally by IDE extensions)
git-ai-tracker checkpoint github-copilot
git-ai-tracker checkpoint cursor

# Install IDE hooks
git-ai-tracker install-hooks

# Show help
git-ai-tracker --help

# Show version
git-ai-tracker --version
```

---

## 🔧 How It Works

1. **IDE Integration**: VS Code/Cursor extensions detect when AI writes code
2. **Checkpointing**: Before/after AI edits, checkpoints are created
3. **Commit Hook**: On commit, AI% is calculated and stored in git notes
4. **Transparent**: Works automatically, no manual tagging needed

### Architecture

```
┌─────────────────────┐
│   VS Code/Cursor    │
│  (Extension)        │
└──────────┬──────────┘
           │ Detects AI edits
           ▼
┌─────────────────────┐
│  git-ai-tracker     │
│  (Rust Binary)      │
└──────────┬──────────┘
           │ Creates checkpoints
           ▼
┌─────────────────────┐
│   Git Commit        │
│  (Post-commit hook) │
└──────────┬──────────┘
           │ Calculates AI%
           ▼
┌─────────────────────┐
│   Git Notes         │
│  (refs/notes/ai)    │
└─────────────────────┘
```

---

## 🎓 Best Practices

### For Developers

1. **Install Once**: Set it up once, it works for all your repos
2. **Review AI%**: Use `git-ai-tracker stats-display` to see your contribution ratio
3. **Share Stats**: Use `--json` flag to integrate with dashboards

### For Teams

1. **PR Comments**: Enable the GitHub Action for automatic PR comments
2. **Metrics**: Track AI% trends across your codebase
3. **Transparency**: Know what code was AI-generated for better code reviews

### For Organizations

1. **Standardize**: Deploy via Homebrew or install script
2. **Enforce**: Use GitHub Actions to fail builds above certain AI% thresholds
3. **Audit**: Analyze AI adoption across teams with git notes data

---

## 📦 What Gets Installed

```
~/.git-ai-tracker/
├── bin/
│   ├── git-ai-tracker          # Main binary
│   ├── git -> git-ai-tracker   # Git proxy symlink
│   └── git-og                  # Original git binary
└── config.json                 # Configuration

VS Code Extension:
~/.vscode/extensions/git-ai-tracker-vscode-*/

Cursor Extension:
~/.cursor/extensions/git-ai-tracker-cursor-*/
```

---

## 🔒 Privacy & Security

- ✅ **All data stays local** - Stored in git notes, under your control
- ✅ **No external API calls** - Everything runs on your machine
- ✅ **Open source** - Full transparency, audit the code
- ✅ **Optional sync** - Git notes are synced only if you push them

---

## 🛠️ Development

### Building from Source

```bash
git clone https://github.com/RaahimNadeem/git-ai-tracker.git
cd git-ai-tracker
cargo build --release
cargo test
```

### Running Tests

```bash
cargo test
```

### Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new features
4. Submit a pull request

---

## 📊 Example Output

### Terminal Stats Display

```
╔══════════════════════════════════════════════════════════╗
║                                                          ║
║           🤖 AI CONTRIBUTION TRACKER 🤖                  ║
║                                                          ║
╠══════════════════════════════════════════════════════════╣
║                                                          ║
║           AI Contribution: 67%                           ║
║                                                          ║
╠══════════════════════════════════════════════════════════╣
║  Commit: a1b2c3d4                                        ║
║  Ref: main                                               ║
╠══════════════════════════════════════════════════════════╣
║                                                          ║
║  📊 Breakdown:                                           ║
║                                                          ║
║    AI Lines:        45 / 67   ( 67%)                     ║
║    Human Lines:     22 / 67   ( 33%)                     ║
║    Mixed Lines:      5                                   ║
║                                                          ║
╚══════════════════════════════════════════════════════════╝
```

### Git Notes Example

```bash
$ git notes show HEAD

{
  "version": "3.0",
  "attestations": [...],
  "metadata": {
    "prompts": {...}
  }
}

--- AI Contribution Metadata ---
AI-Percentage: 67%
AI-Lines: 45/67
Human-Lines: 22/67
Mixed-Lines: 5
Generated: 2025-10-15T12:34:56Z
```

---

## 🐛 Known Limitations

- ❌ **Tab completions**: Currently counted as human edits (IDE limitation)
- ⚠️ **Rebase**: Authorship logs preserved only when using git-ai-tracker for rebase
- ℹ️ **Deletions**: Only additions tracked, deletions not measured

---

## 📚 Documentation

- [Installation Guide](docs/installation.md)
- [VS Code Setup](docs/agent-support/vs-code-github-copilot.mdx)
- [Cursor Setup](docs/agent-support/cursor.mdx)
- [API Reference](docs/api.md)
- [Troubleshooting](docs/troubleshooting.md)

---

## 🆘 Support

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/RaahimNadeem/git-ai-tracker/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/RaahimNadeem/git-ai-tracker/discussions)
- 📧 **Email**: raahim.nadeem@example.com

---

## 📜 License

MIT License - see [LICENSE.md](LICENSE.md)

---

## 🙏 Credits

This project is a fork of the original [git-ai-tracker](https://github.com/RaahimNadeem/git-ai) by [@acunniffe](https://github.com/RaahimNadeem).

**git-ai-tracker** focuses specifically on automatic AI% tracking for GitHub Copilot and Cursor, with streamlined enterprise deployment and automatic AI percentage metadata in Git notes.

---

<div align="center">
  <p><strong>Made with ❤️ for transparent AI coding</strong></p>
  <p>
    <a href="https://github.com/RaahimNadeem/git-ai-tracker">⭐ Star on GitHub</a> •
    <a href="https://github.com/RaahimNadeem/git-ai-tracker/issues">🐛 Report Bug</a> •
    <a href="https://github.com/RaahimNadeem/git-ai-tracker/issues">💡 Request Feature</a>
  </p>
</div>
