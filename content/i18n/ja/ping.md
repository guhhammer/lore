# ping

ICMP ECHO_REQUEST パケットをネットワークホストに送信します。

## Usage
- `ping ホスト` : ホストをPingする
- `ping -c 回数 ホスト` : ホストに特定の回数だけpingする
- `ping -i 秒数 ホスト` : リクエストの間隔を秒単位で指定して、ホストにpingする (デフォルトは1秒)
- `ping -n ホスト` : アドレスのシンボリックネームを調べずに、ホストにpingする
- `ping -a ホスト` : pingしてパケットを受信したら、ベルを鳴らす (端末がサポートしている場合)
- `ping -O ホスト` : 応答がない場合は、メッセージも表示する
- `ping -c 回数 -W 秒数 -w 秒数 ホスト` : 特定のping回数、パケットごとの応答タイムアウト(`-W`)、ping実行全体の制限時間(`-w`)を指定して、ホストにpingする

## Links
- More information: https://manned.org/ping
