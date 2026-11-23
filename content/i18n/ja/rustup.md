# rustup

Rust ツールチェーンのインストール、管理、更新を行います。 `toolchain`, `target`, `update` のようないくつかのサブコマンドには、使用方法についての独自のドキュメントがあります。

## Usage
- `rustup install nightly` : nightly ツールチェーンをシステムにインストール
- `rustup default nightly` : デフォルトのツールチェーンを nightly に切り替えて、`cargo` コマンドと `rustc` コマンドがそれを使用するようにする
- `rustup override set nightly` : 現在のプロジェクト内では nightly ツールチェーンを使用するが、グローバル設定は変更しない
- `rustup update` : 全てのツールチェーンを更新
- `rustup show` : インストール済ツールチェーンを一覧表示
- `rustup run ツールチェーン名 cargo build` : 特定のツールチェーンで `cargo build` を実行
- `rustup doc` : デフォルトのウェブブラウザでローカルの Rust ドキュメントを開く

## Links
- More information: https://rust-lang.github.io/rustup/
