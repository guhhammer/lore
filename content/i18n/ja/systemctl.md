# systemctl

systemd システムとサービスマネージャーを制御します。

## Usage
- `systemctl status` : 実行中のサービスを全て表示する
- `systemctl --failed` : 失敗状態のユニット一覧
- `systemctl start|stop|restart|reload|status ユニット` : サービスを Start/Stop/Restart/Reload/Show 状態にする
- `systemctl enable|disable ユニット` : 起動時に起動するユニットを Enable/Disable に設定する
- `systemctl daemon-reload` : systemdを再読み込みし、新規または変更されたユニットをスキャンする
- `systemctl is-active|is-enabled|is-failed ユニット` : ユニットが active/enabled/failed かをチェックする
- `systemctl list-units -t service|socket|automount --state failed|running` : 全ての service/socket/automount ユニットを running/failed 状態でフィルタリングして一覧表示する
- `systemctl cat ユニット` : ユニットファイルの内容と絶対パスを表示する

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
