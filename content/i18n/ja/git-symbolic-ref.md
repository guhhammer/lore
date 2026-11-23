# git symbolic-ref

参照を保存するファイルを読み取り、変更、削除する。

## Usage
- `git symbolic-ref refs/名前 参照` : 名前を指定して参照を保存する
- `git symbolic-ref -m "メッセージ" refs/名前 refs/heads/ブランチ名` : 更新理由のメッセージを含め、名前を指定して参照を保存する
- `git symbolic-ref refs/名前` : 名前を指定して参照を読み取る
- `git symbolic-ref -d refs/名前` : 名前を指定して参照を削除する
- `git symbolic-ref -q --short refs/名前` : スクリプト用に `--quiet` でエラーを隠し、`--short` で簡潔にする ("refs/heads/X" は "X" と表示される)

## Links
- More information: https://git-scm.com/docs/git-symbolic-ref
