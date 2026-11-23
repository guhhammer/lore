# docker container ls

Dockerコンテナ一覧を表示します。

## Usage
- `docker ps` : 現在実行中のdockerコンテナ一覧を表示する
- `docker ps -a` : 全てのdockerコンテナを表示する(実行中・停止中、両方のコンテナ)
- `docker ps -l` : 最後に作成したコンテナを表示する(全ての状態を含む)
- `docker ps -f "name=コンテナ名"` : コンテナ名に指定の部分文字列を含むコンテナのみになるようにフィルタリングする
- `docker ps -f "ancestor=イメージ名:タグ"` : 指定したイメージを原型(ancestor)として共有するコンテナのみになるようにフィルタリングする
- `docker ps -f "exited=コード" -a` : 終了コードでコンテナをフィルタリングする
- `docker ps -f "status=ステータス"` : 以下のいずれかのステータスでフィルタリングする(created, running, removing, paused, exited, dead)
- `docker ps -f "volume=ディレクトリパス" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : 特定のボリュームをマウントしている、または特定のパスにボリュームがマウントされているコンテナをフィルタリングする

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/ls/
