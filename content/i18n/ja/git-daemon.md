# git daemon

Gitリポジトリ用の非常に単純なサーバー。

## Usage
- `git daemon --export-all ディレクトリ/サブディレクトリ1 ディレクトリ/サブディレクトリ2 ...` : 許可されたディレクトリ群でGit daemonを起動する
- `git daemon --base-path=ディレクトリ/サブディレクトリ --export-all --reuseaddr` : 指定したベースディレクトリでGit daemonを起動し、Gitリポジトリのように見えるすべてのサブディレクトリからのpullを許可する
- `git daemon ディレクトリ/サブディレクトリ --enable=receive-pack --informative-errors --verbose` : 指定したディレクトリ用にGit daemonを起動し、ログメッセージを詳細に表示してGitクライアントからの書き込みを許可する

## Links
- More information: https://git-scm.com/docs/git-daemon
