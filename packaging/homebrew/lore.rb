# lore for Homebrew, from the release's binaries (macOS and Linux, Intel
# and ARM). Lives in a tap: github.com/guhhammer/homebrew-tap, as
# Formula/lore.rb; then `brew install guhhammer/tap/lore`. Homebrew does
# not quarantine what it installs, so the unsigned macOS binary runs.
# packaging/update.sh fills in the version and the checksums for a release.
class Lore < Formula
  desc "Every command you can type, A to Z, with an offline page for each"
  homepage "https://github.com/guhhammer/lore"
  version "0.4.4"
  license :cannot_represent

  on_macos do
    on_arm do
      url "https://github.com/guhhammer/lore/releases/download/v0.4.4/lore-v0.4.4-aarch64-apple-darwin.tar.gz"
      sha256 "4ba14eb90afdae2f50eb143406a78babc4391423662ea0e4466fb577368adb4e"
    end
    on_intel do
      url "https://github.com/guhhammer/lore/releases/download/v0.4.4/lore-v0.4.4-x86_64-apple-darwin.tar.gz"
      sha256 "6da7421d9a69fb30e973f7eb77728076005404031ae1cca2210c1bdd2fb3a8e9"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/guhhammer/lore/releases/download/v0.4.4/lore-v0.4.4-aarch64-unknown-linux-musl.tar.gz"
      sha256 "b9d39549725ab61b578db8dadc44f030e87a711c4323fa869fa9985a7fadd456"
    end
    on_intel do
      url "https://github.com/guhhammer/lore/releases/download/v0.4.4/lore-v0.4.4-x86_64-unknown-linux-musl.tar.gz"
      sha256 "4b830c017b702ad40c6b8beadabd2764536fe21cea731f4279b0dd9fa05a8201"
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
