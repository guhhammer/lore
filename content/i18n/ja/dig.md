# dig

DNS 情報を調べるユーティリティーです。

## Usage
- `dig +short example.com` : ホスト名に関連する IP を検索（A レコード）
- `dig +noall +answer example.com` : 指定したドメインの詳細な回答を得る（A レコード）
- `dig +short example.com A|MX|TXT|CNAME|NS` : 指定されたドメイン名に関連する特定の DNS レコードタイプを取得する
- `dig @8.8.8.8 example.com` : 問い合わせる別の DNS サーバーを指定する
- `dig -x 8.8.8.8` : IP アドレスの DNS 逆引きの実行（PTR レコード）
- `dig +nssearch example.com` : ゾーンの権威ネームサーバーの検索と SOA レコードの表示
- `dig +trace example.com` : ドメイン名を解決するための反復的なクエリの実行と、そのトレースパス全体を表示する

## Links
- More information: https://manned.org/dig
