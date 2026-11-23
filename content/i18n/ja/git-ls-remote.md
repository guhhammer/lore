# git ls-remote

名前またはURLを基にリモートリポジトリ内の参照を一覧表示するGitコマンド。 名前もURLも指定しない場合は設定済みの上流ブランチを使用し、未設定ならリモートの `origin` を使用する。

## Usage
- `git ls-remote` : 既定のリモートリポジトリ内のすべての参照を表示する
- `git ls-remote --heads` : 既定のリモートリポジトリ内のブランチ参照 (heads) だけを表示する
- `git ls-remote -t` : 既定のリモートリポジトリ内のタグ参照だけを表示する
- `git ls-remote リポジトリURL` : 名前またはURLを基にリモートリポジトリ内のすべての参照を表示する
- `git ls-remote リポジトリ名 "パターン"` : パターンで絞り込んだリモートリポジトリの参照を表示する

## Links
- More information: https://git-scm.com/docs/git-ls-remote
