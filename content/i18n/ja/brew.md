# brew

Homebrew - macOS と Linux 用のパッケージマネージャ。 `install` のようないくつかのサブコマンドには、使用方法についての独自のドキュメントがあります。

## Usage
- `brew install formula|cask` : formulae や cask の最新の安定版をインストールする
- `brew list` : 全てのインストールされている formulae や cask を一覧表示
- `brew upgrade formula|cask` : インストールされている formula または cask をアップグレード (何も指定されない場合、インストールされている全ての formulae/cask がアップグレードされる)
- `brew update` : homebrew のソースリポジトリから、Homebrew の最新バージョン、全ての formulae と cask を取得
- `brew outdated` : より新しいバージョンが利用可能な formulae と cask を表示
- `brew search 検索文字列` : 利用可能な formulae (パッケージ) と cask (ネイティブ macOS の `.app` パッケージ) を検索
- `brew info formula|cask` : formula または cask の情報を表示 (バージョン, インストールパス, 依存関係, etc.)
- `brew doctor` : ローカルの Homebrew インストールに潜在的な問題がないかチェック

## Links
- More information: https://docs.brew.sh/Manpage
