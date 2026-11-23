# git credential

ユーザー認証情報を取得・保存する。

## Usage
- `echo "url=http://example.com" | git credential fill` : 設定ファイルからユーザー名とパスワードを取得して認証情報を表示する
- `echo "url=http://example.com" | git credential approve` : 後で使うため、設定済みのすべての認証情報ヘルパーに認証情報を送って保存する
- `echo "url=http://example.com" | git credential reject` : 指定した認証情報を、設定済みのすべての認証情報ヘルパーから削除する

## Links
- More information: https://git-scm.com/docs/git-credential
