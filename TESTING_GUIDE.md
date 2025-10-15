# Git-AI-Tracker Testing Guide

## 🔍 All "git-ai-tracker" References Fixed

### ✅ Files Updated (Final Pass)

1. **docs/limitations.mdx** - Updated `git-ai squash-authorship` → `git-ai-tracker squash-authorship`
2. **install.sh** - Updated path checks and config paths from `.git-ai` → `.git-ai-tracker`
3. **src/config.rs** - Updated config paths from `~/.git-ai/config.json` → `~/.git-ai-tracker/config.json`
4. **src/utils.rs** - Updated debug log prefix from `[git-ai-tracker]` → `[git-ai-tracker]`
5. **tests/repos/test_repo.rs** - Updated binary compilation and references
6. **tests/repos/test_file.rs** - Updated error messages
7. **tests/blame_flags.rs** - Updated comments and debug output
8. **Cargo.lock** - Needs regeneration (auto-generated file)

### ⚠️ Files That Are Fine As-Is

- **Snapshot files** (`.snap`) - Test data, doesn't affect functionality
- **Test fixtures** - Example data like `copilot_session_simple.json`
- **REBRANDING_SUMMARY.md** - Documentation about the old project
- **`use git_ai::` imports** - These are Rust module paths, correct as-is
- **`git-ai.png`** - Asset filename, works for both projects

---

## 🧪 Testing Before Publishing

### Phase 1: Build & Compilation

```bash
cd /Users/muhammad.nadeem2/Documents/Codes/Superportal/git-ai-tracker

# 1. Clean previous builds
cargo clean

# 2. Update Cargo.lock with new package name
cargo update --workspace

# 3. Build the project
cargo build --release

# 4. Verify binary was created
ls -lh target/release/git-ai-tracker

# 5. Test version output
./target/release/git-ai-tracker --version
```

**Expected output:**
```
git-ai-tracker 1.0.0
```

---

### Phase 2: Run Test Suite

```bash
# Run all tests
cargo test

# Run with output (if you want to see what's happening)
cargo test -- --nocapture

# Run specific test suites
cargo test --test simple_additions
cargo test --test blame_flags
cargo test --test cursor
cargo test --test github_copilot
```

**What to check:**
- ✅ All tests should pass
- ✅ No compilation errors
- ✅ No "git-ai-tracker" references in test output (except in test data)

---

### Phase 3: Manual Installation Test

```bash
# Test the installation script locally
cd /Users/muhammad.nadeem2/Documents/Codes/Superportal/git-ai-tracker

# Run install script (it will install from your local build)
bash install.sh

# Verify installation
which git-ai-tracker
git-ai-tracker --version

# Check symlinks
ls -la ~/.git-ai-tracker/bin/

# Expected output:
# git -> git-ai-tracker
# git-ai-tracker (the main binary)
# git-og -> /usr/bin/git (or wherever your real git is)
```

---

### Phase 4: Functional Testing

#### Test 1: Basic Git Operations
```bash
# Create a test repository
mkdir ~/test-git-ai-tracker
cd ~/test-git-ai-tracker
git init

# Create a test file
echo "Hello World" > test.txt
git add test.txt
git commit -m "Initial commit"

# Check that git-ai-tracker intercepted it
git notes show HEAD
# Should see authorship log data
```

#### Test 2: AI Checkpoint Test
```bash
# Trigger a manual checkpoint
git-ai-tracker checkpoint github-copilot

# Check working log
git-ai-tracker checkpoint --show-working-log
```

#### Test 3: Stats Display (Showcase Feature!)
```bash
# Make a commit with some changes
echo "Line 2" >> test.txt
git add test.txt
git commit -m "Add line 2"

# Test the prominent AI% display
git-ai-tracker stats-display

# Should show:
# ╔══════════════════════════════════════════════════════════╗
# ║           🤖 AI CONTRIBUTION TRACKER 🤖                  ║
# ╠══════════════════════════════════════════════════════════╣
# ║           AI Contribution: XX%                           ║
# ╚══════════════════════════════════════════════════════════╝
```

#### Test 4: Enhanced Blame
```bash
# Test AI blame
git-ai-tracker blame test.txt

# Should show line-by-line attribution
```

#### Test 5: Config Path Test
```bash
# Check config file location
ls ~/.git-ai-tracker/config.json

# Should exist and contain git_path
cat ~/.git-ai-tracker/config.json
```

---

### Phase 5: IDE Extensions Testing

#### VS Code Extension
```bash
cd agent-support/vscode

# Install dependencies
yarn install

# Compile
yarn compile

# Run tests
yarn test

# Package (optional, for distribution)
yarn package
# Should create: git-ai-tracker-vscode-1.0.0.vsix
```

**Manual Test:**
1. Open VS Code
2. Install extension: `code --install-extension git-ai-tracker-vscode-1.0.0.vsix`
3. Reload VS Code
4. Check that it activates: Look for "🤖 AI Code Tracker is now active!" notification
5. Use GitHub Copilot, make some changes, and commit
6. Verify git notes are populated

#### Cursor Extension
```bash
cd agent-support/cursor

# Install dependencies
yarn install

# Compile
yarn compile

# Run tests
yarn test
```

**Manual Test:**
1. Open Cursor
2. Install extension from VSIX
3. Check Cursor logs (Cmd+Shift+P → "Output: Show Output Channels" → "Hooks")
4. Should see hooks loaded
5. Use Cursor AI, make changes, commit
6. Verify git notes

---

### Phase 6: Cross-Platform Testing

#### macOS (Current Platform)
```bash
# Already tested above ✅
```

#### Linux (If Available)
```bash
# On Linux machine:
curl -sSL https://raw.githubusercontent.com/RaahimNadeem/git-ai-tracker/main/install.sh | bash
git-ai-tracker --version
```

#### Windows (If Available)
```powershell
# On Windows machine:
powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/RaahimNadeem/git-ai-tracker/main/install.ps1 | iex"
git-ai-tracker --version
```

---

### Phase 7: Uninstall Test

```bash
# Remove installation
rm -rf ~/.git-ai-tracker

# Remove from PATH (edit ~/.zshrc or ~/.bashrc)
# Remove the line: export PATH="$HOME/.git-ai-tracker/bin:$PATH"

# Reload shell
source ~/.zshrc  # or source ~/.bashrc

# Verify removed
which git-ai-tracker  # Should show nothing

# Verify git still works
which git  # Should show /usr/bin/git or similar
git --version  # Should work
```

---

## 🚨 Common Issues & Solutions

### Issue 1: Cargo.lock Still Shows "git-ai-tracker"

**Solution:**
```bash
cargo clean
rm Cargo.lock
cargo build --release
```

### Issue 2: Tests Fail with "binary not found"

**Solution:**
```bash
# Ensure binary is built
cargo build --bin git-ai-tracker

# Check it exists
ls target/debug/git-ai-tracker
```

### Issue 3: "git-ai not found" in tests

**Cause:** Test code still references old binary name

**Solution:**
- Already fixed in tests/repos/test_repo.rs
- Rerun: `cargo test`

### Issue 4: Config not found

**Cause:** Old config at ~/.git-ai/config.json

**Solution:**
```bash
# Migrate old config
mv ~/.git-ai ~/.git-ai-tracker
```

---

## ✅ Pre-Publish Checklist

Before pushing to GitHub and publishing:

- [ ] All tests pass: `cargo test`
- [ ] Binary builds successfully: `cargo build --release`
- [ ] Version command works: `./target/release/git-ai-tracker --version`
- [ ] Installation script works: `bash install.sh`
- [ ] Symlinks created correctly in `~/.git-ai-tracker/bin/`
- [ ] Config file created at `~/.git-ai-tracker/config.json`
- [ ] `git-ai-tracker stats-display` shows formatted output
- [ ] VS Code extension compiles: `cd agent-support/vscode && yarn compile`
- [ ] Cursor extension compiles: `cd agent-support/cursor && yarn compile`
- [ ] No "git-ai-tracker" references in code (except snapshots/docs about old project)
- [ ] All GitHub URLs point to `RaahimNadeem/git-ai-tracker`
- [ ] README.md is comprehensive and accurate
- [ ] Cargo.lock is regenerated

---

## 🚀 Publishing Steps

### 1. Tag Release
```bash
git add .
git commit -m "Complete rebranding to git-ai-tracker"
git tag -a v1.0.0 -m "Initial release of git-ai-tracker"
git push origin main --tags
```

### 2. GitHub Release
- Go to https://github.com/RaahimNadeem/git-ai-tracker/releases/new
- Select tag: v1.0.0
- Title: "Git-AI-Tracker v1.0.0"
- Description: Copy from README.md features section
- The GitHub Action will automatically build binaries and upload them

### 3. Homebrew Tap
```bash
# Create homebrew tap repository
# https://github.com/RaahimNadeem/homebrew-tap

# Add formula
cp HomebrewFormula/git-ai-tracker.rb <homebrew-tap-repo>/
cd <homebrew-tap-repo>
git add git-ai-tracker.rb
git commit -m "Add git-ai-tracker formula"
git push
```

### 4. VS Code Marketplace (Optional)
```bash
cd agent-support/vscode
yarn run package
# Upload .vsix to VS Code marketplace
# Or use: vsce publish
```

### 5. Cursor Extension (Optional)
```bash
cd agent-support/cursor
yarn run package
# Distribute via GitHub releases or Cursor extension marketplace
```

---

## 📊 Performance Benchmarks

Test that the overhead is acceptable:

```bash
# Benchmark native git
time git status  # ~5-10ms

# Benchmark git-ai-tracker
time ~/.git-ai-tracker/bin/git status  # Should be ~6-15ms

# The overhead should be < 10ms
```

---

## 🎯 Success Criteria

Your project is ready to publish when:

1. ✅ All tests pass
2. ✅ Installation script works on all platforms
3. ✅ `stats-display` command shows beautiful AI% output
4. ✅ IDE extensions install and activate without errors
5. ✅ No "git-ai-tracker" references remaining (except in docs/snapshots)
6. ✅ Git operations are transparent (no noticeable slowdown)
7. ✅ README.md is comprehensive
8. ✅ You're proud to show it off! 🎉

---

## 💡 Demo Script

When showcasing your project:

```bash
# 1. Install
curl -sSL https://raw.githubusercontent.com/RaahimNadeem/git-ai-tracker/main/install.sh | bash

# 2. Create test repo
mkdir ~/demo && cd ~/demo
git init

# 3. Make a commit
echo "Hello AI" > demo.txt
git add . && git commit -m "Initial commit"

# 4. Show the magic! ✨
git-ai-tracker stats-display

# 5. Show enhanced blame
git-ai-tracker blame demo.txt

# 6. Show git notes
git notes show HEAD
```

**🎉 Congratulations! Your project is ready to publish!**

