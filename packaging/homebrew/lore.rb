# lore for Homebrew, from the release's binaries (macOS and Linux, Intel
# and ARM). Lives in a tap: github.com/guhhammer/homebrew-tap, as
# Formula/lore.rb; then `brew install guhhammer/tap/lore`. Homebrew does
# not quarantine what it installs, so the unsigned macOS binary runs.
# packaging/update.sh fills in the version and the checksums for a release.
class Lore < Formula
  desc "Every command you can type, A to Z, with an offline page for each"
  homepage "https://github.com/guhhammer/lore"
  version "0.3.3"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/guhhammer/lore/releases/download/v0.3.3/lore-v0.3.3-aarch64-apple-darwin.tar.gz"
      sha256 "5626f6400e76e24068836cd2e49fe4a87d4e8334bb4639745112a6693bdddeba"
    end
    on_intel do
      url "https://github.com/guhhammer/lore/releases/download/v0.3.3/lore-v0.3.3-x86_64-apple-darwin.tar.gz"
      sha256 "3f7ecb43a135eed236b254d988d443306693c1d55d1d877e3c30e1bde9e2383f"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/guhhammer/lore/releases/download/v0.3.3/lore-v0.3.3-aarch64-unknown-linux-musl.tar.gz"
      sha256 "cab920e62489f119e1e2f1f5554741cec44cbc435c219ed2a47f48f086207733"
    end
    on_intel do
      url "https://github.com/guhhammer/lore/releases/download/v0.3.3/lore-v0.3.3-x86_64-unknown-linux-musl.tar.gz"
      sha256 "d467c771c0f96e54b79b9b0f81cb2a4b7fe3eb498cd4604c17eb1964e3ff5977"
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
