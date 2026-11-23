# tracert

PCとターゲット間の経路の各ステップに関する情報を受信します。

## Usage
- `tracert ip_address` : ルートを追跡する
- `tracert /d ip_address` : `tracert`がIPアドレスをホスト名に解決しないようにする
- `tracert /4 ip_address` : `tracert`にIPv4のみの利用を強制する
- `tracert /6 ip_address` : `tracert`にIpv6のみの利用を強制する
- `tracert /h 最大ホップ数 ip_address` : ターゲットの検索における最大ホップ数を指定する
- `tracert /?` : ヘルプを表示する
