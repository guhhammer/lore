# netstat

開いている接続、開いているソケットポートなどのネットワーク関連情報を表示します。

## Usage
- `netstat -a` : 全てのポートを一覧表示する
- `netstat -l` : 全てのリスニングポートを一覧表示する
- `netstat -t` : リッスン中のTCPポートを一覧表示する
- `netstat -p` : PIDとプログラム名を表示する
- `netstat -c` : 情報を連続的に一覧表示する
- `netstat -rn` : ルートを一覧表示し、IPアドレスをホスト名に解決しない
- `netstat -tulpne` : リッスンしているTCPポートとUDPポートを一覧表示する (+ rootの場合はユーザーとプロセス)

## Links
- More information: https://manned.org/netstat
