# git ls-tree

ツリーオブジェクトの内容を一覧表示する。

## Usage
- `git ls-tree ブランチ名` : ブランチ上のツリーの内容を一覧表示する
- `git ls-tree -r コミットハッシュ` : コミット上のツリーの内容を、サブツリーまで再帰的に一覧表示する
- `git ls-tree --name-only コミットハッシュ` : コミット上のツリーのファイル名だけを一覧表示する
- `git ls-tree -r --name-only HEAD | tree --fromfile` : 現在のブランチheadのファイル名をツリー構造で出力する (注: `tree --fromfile` はWindowsではサポートされていない)

## Links
- More information: https://git-scm.com/docs/git-ls-tree
