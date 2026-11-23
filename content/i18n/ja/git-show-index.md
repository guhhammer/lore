# git show-index

Gitリポジトリのパック済みアーカイブインデックスを表示する。

## Usage
- `git show-index ディレクトリ/サブディレクトリ/ファイル.idx` : git packfileのIDXファイルを読み取り、その内容を `stdout` に出力する
- `git show-index --object-format sha1|sha256 ディレクトリ/サブディレクトリ/ファイル` : インデックスファイル用のハッシュアルゴリズムを指定する (実験的)

## Links
- More information: https://git-scm.com/docs/git-show-index
