# git instaweb

GitWebサーバーを起動するヘルパー。

## Usage
- `git instaweb --start` : 現在のGitリポジトリ用にGitWebサーバーを起動する
- `git instaweb --start -l` : localhostだけで待ち受ける
- `git instaweb --start -p 1234` : 指定したポートで待ち受ける
- `git instaweb --start -d lighttpd|apache2|mongoose|plackup|webrick` : 指定したHTTP daemonを使用する
- `git instaweb --start -b` : webブラウザも自動起動する
- `git instaweb --stop` : 現在実行中のGitWebサーバーを停止する
- `git instaweb --restart` : 現在実行中のGitWebサーバーを再起動する

## Links
- More information: https://git-scm.com/docs/git-instaweb
