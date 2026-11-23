# git grep

リポジトリ内の追跡対象ファイルから文字列を検索する。 通常の `grep` と同じフラグの多くを受け付ける。

## Usage
- `git grep "検索文字列"` : 現在の `HEAD` のファイルから文字列を検索する
- `git grep "検索文字列" -- "*.ext"` : 現在の `HEAD` で、`glob`パターンに一致するファイルから文字列を検索する
- `git grep --recurse-submodules "検索文字列"` : サブモジュールを含めて文字列を検索する
- `git grep "検索文字列" HEAD~2` : 履歴上の指定した時点で文字列を検索する
- `git grep "検索文字列" $(git rev-list --all)` : すべてのブランチとすべての履歴から文字列を検索する

## Links
- More information: https://git-scm.com/docs/git-grep
