# ip

ルーティング、デバイス、ポリシールーティング、トンネルの表示/操作。 `address` のようないくつかのサブコマンドには、使用方法についての独自のドキュメントがあります。

## Usage
- `ip a` : インターフェースの詳細情報を表示する
- `ip -br a` : 簡単なネットワークレイヤの情報を持つインターフェースを一覧表示する
- `ip -br l` : リンク層の簡単な情報を持つインターフェースを一覧表示する
- `ip r` : ルーティングテーブルを表示する
- `ip n` : ネイバー(ARP テーブル)を表示する
- `sudo ip l s インターフェース up|down` : インターフェースを up/down する
- `sudo ip a add|delete ip_address/mask dev インターフェース` : インターフェースに IP アドレスを追加/削除する
- `sudo ip r a default via ip_address dev インターフェース` : デフォルトルートを追加する

## Links
- More information: https://manned.org/ip.8
