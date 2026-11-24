# lore for Homebrew, from the release's binaries (macOS and Linux, Intel
# and ARM). Lives in a tap: github.com/guhhammer/homebrew-tap, as
# Formula/lore.rb; then `brew install guhhammer/tap/lore`. Homebrew does
# not quarantine what it installs, so the unsigned macOS binary runs.
# packaging/update.sh fills in the version and the checksums for a release.
class Lore < Formula
  desc "Every command you can type, A to Z, with an offline page for each"
  homepage "https://github.com/guhhammer/lore"
  version "0.1.6"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/guhhammer/lore/releases/download/v0.1.6/lore-v0.1.6-aarch64-apple-darwin.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
    on_intel do
      url "https://github.com/guhhammer/lore/releases/download/v0.1.6/lore-v0.1.6-x86_64-apple-darwin.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/guhhammer/lore/releases/download/v0.1.6/lore-v0.1.6-aarch64-unknown-linux-musl.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
    on_intel do
      url "https://github.com/guhhammer/lore/releases/download/v0.1.6/lore-v0.1.6-x86_64-unknown-linux-musl.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
  end

  def install
    bin.install "lore"
    man1.install "lore.1"
    bash_completion.install "completions/lore.bash" => "lore"
    zsh_completion.install "completions/_lore"
    fish_completion.install "completions/lore.fish"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/lore --version")
    assert_match "HOW TO USE", shell_output("#{bin}/lore --print tar")
  end
end
