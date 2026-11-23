# net

ネットワーク関連の設定を表示、変更するためのシステムユーティリティ。

## Usage
- `net start|stop サービス` : windowsサービスを同期的に開始または停止する
- `net use \\smb共有フォルダ /USER:ユーザ名` : 現在のコンソールでSMB共有が利用可能であることを確認する
- `net share` : 現在SMBで共有されているフォルダを表示する
- `net session` : SMB共有の使用者を表示する(管理者特権コンソールで実行)
- `net localgroup "Administrators"` : ローカルセキュリティグループ内のユーザーを表示する
- `net localgroup "Administrators" ユーザ名 /add` : ローカルセキュリティグループにユーザーを追加する(管理者特権コンソールで実行)
- `net help サブコマンド` : サブコマンドのヘルプを表示する
- `net help` : ヘルプを表示する
