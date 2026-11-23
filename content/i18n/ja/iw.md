# iw

ワイヤレスデバイスの表示と操作を行います。

## Usage
- `iw dev wlanX scan` : 利用可能なワイヤレスネットワークをスキャンする
- `iw dev wlanX connect ssid` : 開いているワイヤレスネットワークに接続する
- `iw dev wlanX disconnect` : 現在の接続を切断する
- `iw dev wlanX link` : 現在の接続に関する情報を表示
- `iw dev` : 全ての物理・論理ワイヤレスネットワークインターフェースを一覧表示
- `iw phy` : 全ての物理ハードウェアインターフェースの、全てのワイヤレス機能を一覧表示
- `iw reg get` : カーネルの現在のワイヤレス規制ドメイン情報を一覧表示
- `iw help` : ヘルプを表示する

## Links
- More information: https://wireless.docs.kernel.org/en/latest/en/users/documentation/iw.html
