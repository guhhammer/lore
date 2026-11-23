# cargo

Rust プロジェクトとそのモジュールの依存関係(クレート)を管理します。 `build` のようないくつかのサブコマンドには、使用方法についての独自のドキュメントがあります。

## Usage
- `cargo search 検索文字列` : クレートを検索
- `cargo install クレート名` : バイナリのクレートをインストール
- `cargo install --list` : インストールされたバイナリクレートを一覧表示
- `cargo init --bin|lib path/to/directory` : 指定したディレクトリ（デフォルトでは現在の作業ディレクトリ）に新しいバイナリまたはライブラリ Rust プロジェクトを作成
- `cargo add dependency` : カレントディレクトリの `Cargo.toml` に依存関係を追加
- `cargo b -r` : カレントディレクトリの Rust プロジェクトを release プロファイルを使ってビルド
- `cargo +nightly b` : nightly コンパイラを使ってカレントディレクトリに Rust プロジェクトをビルド (`rustup` が必要)
- `cargo b -j スレッド数` : 特定のスレッド数を使用してビルド (デフォルトは論理 CPU 数)

## Links
- More information: https://doc.rust-lang.org/stable/cargo/
