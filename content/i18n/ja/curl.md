# curl

データをサーバーから転送、もしくはサーバーへ転送します。 HTTP、FTP、POP3を含むほとんどのプロトコルをサポートしています。

## Usage
- `curl http://example.com -o ファイルパス` : URLのコンテンツをファイルにダウンロードする
- `curl -O http://example.com/filename` : ファイルをダウンロードし、URLにより指定されたファイル名で出力を保存する
- `curl -f -O -L -C - http://example.com/filename` : ファイルをダウンロードし、ロケーションレスポンスヘッダーのリダイレクトに追従しつつ自動的に以前のファイル転送を継続(再開)する。サーバーエラー時にはエラーを返す
- `curl -d 'name=bob' http://example.com/form` : フォームエンコードされたデータを送信する（`application/x-www-form-urlencoded`型のPOSTリクエスト）。`stdin`(標準入力) から読み込むには、`--data @file_name` または `--data @'-'` を使用する
- `curl -H 'X-My-Header: 123' -X PUT http://example.com` : カスタムHTTPメソッドを用いて追加のヘッダーを持つリクエストを送信する
- `curl -d '{"name":"bob"}' -H 'Content-Type: application/json' http://example.com/users/1234` : 適切なContent-Type 表現ヘッダーを指定することで、JSONフォーマットでデータを送信する
- `curl -u ユーザー名 http://example.com` : サーバー認証のためユーザー名とパスワードを送る
- `curl -E client.pem --key key.pem -k https://example.com` : リソースのクライアント証明書とキーを送り、証明書の検証をスキップする

## Links
- More information: https://curl.se/docs/manpage.html
