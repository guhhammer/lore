# git stripspace

`stdin` からテキスト (コミットメッセージ、ノート、タグ、ブランチ説明など) を読み取り、Gitで使われる形式に整える。

## Usage
- `cat ディレクトリ/サブディレクトリ/ファイル | git stripspace` : ファイルから空白を取り除く
- `cat ディレクトリ/サブディレクトリ/ファイル | git stripspace -s` : ファイルから空白とGitコメントを取り除く
- `git <ディレクトリ/サブディレクトリ/ファイル stripspace -c` : ファイル内のすべての行をGitコメントに変換する

## Links
- More information: https://git-scm.com/docs/git-stripspace
