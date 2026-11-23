# git remote

追跡対象のリポジトリ (「リモート」) を管理する。

## Usage
- `git remote -v` : 既存のリモートを名前およびURLと共に一覧表示する
- `git remote show リモート名` : リモートの情報を表示する
- `git remote add リモート名 リモートURL` : リモートを追加する
- `git remote set-url リモート名 新しいURL` : リモートのURLを変更する (既存のURLを残すには `--add` を使用する)
- `git remote set-url リモート名 新しいURL --push` : プッシュ先のリモートURLをフェッチ元とは別に設定する
- `git remote get-url リモート名` : リモートのURLを表示する
- `git remote remove リモート名` : リモートを削除する
- `git remote rename 古い名前 新しい名前` : リモートの名前を変更する

## Links
- More information: https://git-scm.com/docs/git-remote
