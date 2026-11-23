# git fetch

リモートリポジトリからオブジェクトと参照をダウンロードする。

## Usage
- `git fetch` : 既定のリモートリポジトリ (設定されている場合) から最新の変更を取得する
- `git fetch リモート名` : 指定したリモートリポジトリから新しいブランチを取得する
- `git fetch --all` : すべてのリモートリポジトリから最新の変更を取得する
- `git fetch -t` : リモートリポジトリからタグも取得する
- `git fetch -p` : リモートリポジトリで削除されたリモートブランチへのローカル参照を削除する
- `git fetch --deepen 2` : 現在の shallow ブランチの履歴を2コミット分深くする
- `git fetch origin main:main` : `main` ブランチを、切り替えずに更新する (`git pull` と同等)

## Links
- More information: https://git-scm.com/docs/git-fetch
