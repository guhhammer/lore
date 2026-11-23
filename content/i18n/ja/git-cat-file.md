# git cat-file

Gitリポジトリオブジェクトの内容・種類・サイズ情報を表示する。

## Usage
- `git cat-file -s HEAD` : `HEAD` コミットのサイズをバイト単位で取得する
- `git cat-file -t 8c442dc3` : 指定したGitオブジェクトの種類 (blob・tree・commit・tag) を取得する
- `git cat-file -p HEAD~2` : 指定したGitオブジェクトの内容を種類に応じて整形表示する

## Links
- More information: https://git-scm.com/docs/git-cat-file
