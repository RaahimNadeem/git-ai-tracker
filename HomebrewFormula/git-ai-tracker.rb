class GitAiTracker < Formula
  desc "Track AI-generated code contributions automatically with GitHub Copilot and Cursor"
  homepage "https://github.com/RaahimNadeem/git-ai-tracker"
  version "1.0.0"
  
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/RaahimNadeem/git-ai-tracker/releases/download/v#{version}/git-ai-tracker-macos-arm64"
      sha256 "PLACEHOLDER_ARM64_SHA256" # Will be updated by release script
    else
      url "https://github.com/RaahimNadeem/git-ai-tracker/releases/download/v#{version}/git-ai-tracker-macos-x64"
      sha256 "PLACEHOLDER_X64_SHA256" # Will be updated by release script
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/RaahimNadeem/git-ai-tracker/releases/download/v#{version}/git-ai-tracker-linux-arm64"
      sha256 "PLACEHOLDER_LINUX_ARM64_SHA256" # Will be updated by release script
    else
      url "https://github.com/RaahimNadeem/git-ai-tracker/releases/download/v#{version}/git-ai-tracker-linux-x64"
      sha256 "PLACEHOLDER_LINUX_X64_SHA256" # Will be updated by release script
    end
  end

  def install
    # Install the binary
    if OS.mac?
      if Hardware::CPU.arm?
        bin.install "git-ai-tracker-macos-arm64" => "git-ai-tracker"
      else
        bin.install "git-ai-tracker-macos-x64" => "git-ai-tracker"
      end
    elsif OS.linux?
      if Hardware::CPU.arm?
        bin.install "git-ai-tracker-linux-arm64" => "git-ai-tracker"
      else
        bin.install "git-ai-tracker-linux-x64" => "git-ai-tracker"
      end
    end

    # Create symlink for git integration
    bin.install_symlink bin/"git-ai-tracker" => "git"
  end

  def post_install
    # Run install-hooks to set up IDE integrations
    system bin/"git-ai-tracker", "install-hooks"
  end

  test do
    system bin/"git-ai-tracker", "--version"
  end

  def caveats
    <<~EOS
      🎉 git-ai-tracker has been installed!

      The git-ai-tracker binary is now available at:
        #{bin}/git-ai-tracker

      To use it as a git proxy (recommended):
        1. Add #{bin} to the beginning of your PATH in ~/.zshrc or ~/.bashrc:
           export PATH="#{bin}:$PATH"

        2. Restart your shell or run:
           source ~/.zshrc  # or source ~/.bashrc

      IDE Integration:
        VS Code/Cursor extensions have been automatically configured.
        Restart VS Code/Cursor to activate AI tracking.

      Quick Start:
        git-ai-tracker stats           # Show AI% for last commit
        git-ai-tracker stats-display   # Show prominent AI% display
        git-ai-tracker blame <file>    # Enhanced git blame with AI attribution

      Documentation: https://github.com/RaahimNadeem/git-ai-tracker
    EOS
  end
end

