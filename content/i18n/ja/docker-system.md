# docker system

Dockerのデータ管理とシステム全体に関わる情報の表示をします。

## Usage
- `docker system` : ヘルプの表示をする
- `docker system df` : dockerディスクの使用量を表示する
- `docker system df -v` : ディスクの使用量に関して詳細な情報を表示する
- `docker system prune` : 不使用データを削除する
- `docker system prune --filter "until=時h分m"` : 不使用データのうち指定時間より前に作成されたものを削除する
- `docker system events` : dockerデーモンからのリアルタイムイベントを表示する
- `docker system events -f 'type=container' --format '\{\{json .\}\}'` : コンテナからのリアルタイムイベントを適正なJSON行としてストリーム表示する
- `docker system info` : システム全体に関わる情報の表示をする

## Links
- More information: https://docs.docker.com/reference/cli/docker/system/
