# git clean

Gitで追跡されていないファイルを作業ツリーから削除する。

## Usage
- `git clean -i` : 未追跡ファイルを対話的に削除する
- `git clean -n` : 実際には削除せず、削除対象のファイルを表示する
- `git clean -f` : すべての未追跡ファイルをただちに強制削除する
- `git clean -f -d` : 未追跡のディレクトリを削除する
- `git clean -f -- ディレクトリ/サブディレクトリ '*.ext'` : 指定したパスまたは`glob`パターンに一致する未追跡ファイルだけを削除する
- `git clean -f -e '*.ext' -e ディレクトリ/サブディレクトリ/` : 指定したパターンに一致するものを除き、未追跡ファイルを削除する
- `git clean -f -x` : 未追跡ファイルと無視されたファイル (`.gitignore` と `.git/info/exclude` に記載されたもの) を削除する

## Links
- More information: https://git-scm.com/docs/git-clean
