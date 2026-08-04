# Homebrew Formula for zerum
#
# Copy to your tap as Formula/zerum.rb and fill sha256 from Release SHA256SUMS.
class Zerum < Formula
  desc "Deterministic Python code governance: ~75 checks, default/strict profiles, explain mode, optional Ruff orchestration"
  homepage "https://github.com/latentmeta/zerum"
  version "0.4.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/latentmeta/zerum/releases/download/v#{version}/zerum-v#{version}-macos-aarch64.tar.gz"
      # sha256 "REPLACE_AFTER_RELEASE"
    end
    on_intel do
      url "https://github.com/latentmeta/zerum/releases/download/v#{version}/zerum-v#{version}-macos-x86_64.tar.gz"
      # sha256 "REPLACE_AFTER_RELEASE"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/latentmeta/zerum/releases/download/v#{version}/zerum-v#{version}-linux-aarch64.tar.gz"
      # sha256 "REPLACE_AFTER_RELEASE"
    end
    on_intel do
      url "https://github.com/latentmeta/zerum/releases/download/v#{version}/zerum-v#{version}-linux-x86_64.tar.gz"
      # sha256 "REPLACE_AFTER_RELEASE"
    end
  end

  def install
    bin.install "zerum"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/zerum --version")
  end
end
