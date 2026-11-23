# traceroute

ネットワークホストへの経路パケット追跡を表示します。

## Usage
- `traceroute example.com` : ホストへの経路追跡
- `traceroute -n example.com` : IPアドレスとホスト名のマッピングを無効化する
- `traceroute -w 0.5 example.com` : 応答までの待機時間を秒単位で指定する
- `traceroute -q 5 example.com` : ホップごとのクエリ回数を指定する
- `traceroute example.com 42` : プローブパケットのサイズをバイト単位で指定する
- `traceroute --mtu example.com` : 宛先までのMTUを特定する
- `traceroute -I example.com` : UDPの代わりにICMPを使ってトレースルートする

## Links
- More information: https://manned.org/traceroute
