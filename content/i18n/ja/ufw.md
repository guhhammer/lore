# ufw

シンプルなファイアウォール。 ファイアウォールの設定を簡単にすることを目的とした `iptables` のフロントエンドです。

## Usage
- `sudo ufw enable` : ufw を有効化
- `sudo ufw disable` : ufw を無効化
- `sudo ufw status numbered` : ufw ルールを、番号と共に表示する
- `sudo ufw allow 5432 comment "Service"` : このホストのポート 5432 へのトラフィックを、サービスを識別するコメント付きで許可
- `sudo ufw allow proto tcp from 192.168.0.4 to any port 22` : 192.168.0.4 からこのホストの任意のアドレスへのポート 22 の TCP トラフィックのみを許可
- `sudo ufw deny 80` : このホストのポート 80 のトラフィックを拒否
- `sudo ufw deny proto udp from any to any port 8412:8500` : 8412:8500 の範囲のポートへの全ての UDP トラフィックを拒否
- `sudo ufw delete ルール番号` : 特定のルールを削除する。ルール番号は `ufw status numbered` コマンドで取得できる

## Links
- More information: https://manned.org/ufw
