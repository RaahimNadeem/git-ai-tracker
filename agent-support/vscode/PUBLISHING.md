# Publishing git-ai-tracker VS Code Extension

This guide explains how to publish the extension to the VS Code Marketplace so end-users can install it easily.

## 📋 Prerequisites

1. **Microsoft Account** for Visual Studio Marketplace
2. **Personal Access Token (PAT)** from Azure DevOps
3. **Publisher account** on VS Code Marketplace

## 🚀 Step-by-Step Publishing Guide

### 1. Create a Publisher Account

1. Go to https://marketplace.visualstudio.com/manage
2. Sign in with your Microsoft account
3. Click "Create publisher"
4. Fill in details:
   - **Publisher ID**: `RaahimNadeem` (must match package.json)
   - **Display Name**: `Raahim Nadeem`
   - **Description**: `Developer tools for AI code tracking`

### 2. Generate Personal Access Token

1. Go to https://dev.azure.com/RaahimNadeem
2. Click on "User Settings" → "Personal Access Tokens"
3. Click "New Token"
4. Configure:
   - **Name**: `vscode-marketplace`
   - **Organization**: `All accessible organizations`
   - **Expiration**: 90 days (or custom)
   - **Scopes**: Select "Custom defined" → Check "Marketplace" → "Manage"
5. Click "Create" and **SAVE THE TOKEN** (you won't see it again!)

### 3. Login to vsce

```bash
cd /Users/muhammad.nadeem2/Documents/Codes/Superportal/git-ai-tracker/agent-support/vscode

# Login with your publisher
npx vsce login RaahimNadeem

# Paste your Personal Access Token when prompted
```

### 4. Build and Package

```bash
# Install dependencies
yarn install

# Copy icon
yarn run copy-static

# Compile TypeScript
yarn run compile

# Package the extension
npx vsce package

# This creates: git-ai-tracker-vscode-1.0.0.vsix
```

### 5. Publish to Marketplace

```bash
# Publish the extension
npx vsce publish

# Or specify version bump
npx vsce publish patch  # 1.0.0 → 1.0.1
npx vsce publish minor  # 1.0.0 → 1.1.0
npx vsce publish major  # 1.0.0 → 2.0.0
```

### 6. Verify Publication

1. Go to https://marketplace.visualstudio.com/manage/publishers/RaahimNadeem
2. Check your extension is listed
3. Visit the extension page:
   https://marketplace.visualstudio.com/items?itemName=RaahimNadeem.git-ai-tracker-vscode

### 7. Test Installation

```bash
# Install from marketplace
code --install-extension RaahimNadeem.git-ai-tracker-vscode

# Verify it works
```

## 🔄 Updating the Extension

### Make Changes

1. Edit code in `src/extension.ts`
2. Update version in `package.json`
3. Add changelog entry in `CHANGELOG.md`

### Publish Update

```bash
# Compile changes
yarn run compile

# Publish with version bump
npx vsce publish patch

# Or publish specific version
npx vsce publish 1.0.1
```

## 📝 Publishing Checklist

Before publishing, ensure:

- [ ] ✅ Version number updated in `package.json`
- [ ] ✅ CHANGELOG.md updated with new features/fixes
- [ ] ✅ README.md is accurate and up-to-date
- [ ] ✅ Icon (`git-ai.png`) is present
- [ ] ✅ Extension tested locally (`yarn run compile` → symlink)
- [ ] ✅ No linter errors (`yarn run lint`)
- [ ] ✅ Keywords in `package.json` are relevant
- [ ] ✅ Publisher set to `RaahimNadeem`
- [ ] ✅ License is MIT

## 🎨 Marketplace Assets

The marketplace will use:
- **Icon**: `git-ai.png` (100x100px, displayed on marketplace)
- **README**: `README.md` (displayed on extension page)
- **Publisher**: Raahim Nadeem
- **Keywords**: Used for search ranking

## 🔐 Security

**NEVER** commit your Personal Access Token (PAT)!

Store it securely:
```bash
# macOS Keychain
security add-generic-password -a "vscode-pat" -s "vscode-marketplace" -w "YOUR_PAT"

# Retrieve when needed
security find-generic-password -a "vscode-pat" -s "vscode-marketplace" -w
```

## 📊 Analytics

After publishing, view analytics at:
https://marketplace.visualstudio.com/manage/publishers/RaahimNadeem

Track:
- **Installs**: Total installations
- **Downloads**: .vsix downloads
- **Ratings**: User reviews
- **Page Views**: Marketplace traffic

## 🚨 Troubleshooting

### Error: "Publisher not found"

```bash
# Make sure publisher ID matches package.json
grep "publisher" package.json
# Should show: "publisher": "RaahimNadeem"

# Create publisher if needed (see Step 1)
```

### Error: "Extension with ID already exists"

```bash
# Unpublish old version first
npx vsce unpublish RaahimNadeem.git-ai-tracker-vscode

# Then republish
npx vsce publish
```

### Error: "Package validation failed"

```bash
# Check package.json has all required fields
cat package.json

# Ensure these exist:
# - name
# - publisher
# - version
# - displayName
# - description
# - engines.vscode
```

## 🎯 Publishing to Open VSX (Optional)

For users who can't access VS Code Marketplace (e.g., VSCodium):

```bash
# Install ovsx
npm install -g ovsx

# Login
ovsx login

# Publish
ovsx publish git-ai-tracker-vscode-1.0.0.vsix
```

Registry: https://open-vsx.org/

## 📚 Resources

- [VS Code Publishing Guide](https://code.visualstudio.com/api/working-with-extensions/publishing-extension)
- [vsce Documentation](https://github.com/microsoft/vsce)
- [Marketplace Management](https://marketplace.visualstudio.com/manage)
- [Extension Guidelines](https://code.visualstudio.com/api/references/extension-guidelines)

---

## 🎉 After Publishing

1. **Update install instructions** in main README
2. **Add marketplace badge**:
   ```markdown
   [![VS Code Marketplace](https://img.shields.io/vscode-marketplace/v/RaahimNadeem.git-ai-tracker-vscode)](https://marketplace.visualstudio.com/items?itemName=RaahimNadeem.git-ai-tracker-vscode)
   ```
3. **Update `install-hooks` command** to reference marketplace ID
4. **Announce** on GitHub, social media, etc.

---

<div align="center">
  <p>Happy Publishing! 🚀</p>
</div>

