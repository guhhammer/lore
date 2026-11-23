# ps

実行中のプロセスに関する情報。

## Usage
- `ps aux` : 実行中のプロセスをすべてリストアップ
- `ps auxww` : 実行中のすべてのプロセスを、完全なコマンド文字列を含めて一覧表示する
- `ps aux | grep 文字列` : 文字列にマッチするプロセスを検索する
- `ps -u $(id -u) -F` : 現在のユーザーのすべてのプロセスを完全なフォーマットで表示する
- `ps -u $(id -u) f` : カレントユーザーの全プロセスをツリー状にリストアップ
- `ps -o ppid= -p pid` : プロセスの親 pid を取得する
- `ps --sort size` : プロセスをメモリ消費量でソート

## Links
- More information: https://manned.org/ps
