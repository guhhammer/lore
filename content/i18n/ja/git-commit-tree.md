# git commit-tree

コミットオブジェクトを作成する低レベルユーティリティ。

## Usage
- `git commit-tree ツリー -m "メッセージ"` : 指定したメッセージでコミットオブジェクトを作成する
- `git commit-tree ツリー -F ディレクトリ/サブディレクトリ/ファイル` : ファイルからメッセージを読み込んでコミットオブジェクトを作成する (`stdin` には `-` を使用)
- `git commit-tree ツリー -m "メッセージ" -S` : GPG署名付きのコミットオブジェクトを作成する
- `git commit-tree ツリー -m "メッセージ" -p 親コミットSHA` : 指定した親コミットオブジェクトを持つコミットオブジェクトを作成する

## Links
- More information: https://git-scm.com/docs/git-commit-tree
