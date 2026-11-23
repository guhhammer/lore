# git tag

タグを作成、一覧表示、削除、または検証する。 タグはコミットへの静的な参照である。

## Usage
- `git tag` : すべてのタグを一覧表示する
- `git tag タグ名` : 現在のコミットを指す、指定した名前のタグを作成する
- `git tag タグ名 コミット` : 指定したコミットを指す、指定した名前のタグを作成する
- `git tag タグ名 -m タグメッセージ` : 指定したメッセージを持つ注釈付きタグを作成する
- `git tag -d タグ名` : 指定した名前のタグを削除する
- `git fetch -t` : リモートから更新されたタグを取得する
- `git push origin tag タグ名` : タグをリモートへプッシュする
- `git tag --contains コミット` : 指定したコミットを含むすべてのタグを一覧表示する (省略した場合は `HEAD`)

## Links
- More information: https://git-scm.com/docs/git-tag
