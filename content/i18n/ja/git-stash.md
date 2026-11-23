# git stash

ローカルのGit変更を一時領域に退避する。

## Usage
- `git stash` : 現在の未コミットの変更を退避する
- `git stash -u` : 新しい未追跡ファイルを含め、現在の変更を退避する
- `git stash -p` : 変更されたファイルから退避する部分を対話形式で選択する
- `git stash list` : すべてのスタッシュを一覧表示する
- `git stash show -p` : スタッシュと、そのスタッシュエントリが作成された時点のコミットとの差分をパッチとして表示する
- `git stash pop` : スタッシュを適用し、コンフリクトが発生しなければスタッシュ一覧から削除する
- `git stash drop` : 最新のスタッシュを削除する
- `git stash clear` : すべてのスタッシュを削除する

## Links
- More information: https://git-scm.com/docs/git-stash
