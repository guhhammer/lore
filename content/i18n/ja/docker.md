# docker

Dockerコンテナ及びDockerイメージの管理を行います。 `run` のようないくつかのサブコマンドには、使用方法についての独自のドキュメントがあります。

## Usage
- `docker ps -a` : 全てのDockerコンテナを表示する(実行中・停止中、両方のコンテナ)
- `docker run --name コンテナ名 イメージ` : イメージからカスタムのコンテナ名でコンテナを起動する
- `docker container start|stop コンテナ名` : 既存のコンテナの起動もしくは停止を行う
- `docker pull イメージ` : dockerレジストリからイメージをプルする
- `docker images` : 既にダウンロード済のイメージ一覧を表示する
- `docker exec -it コンテナ名 sh` : 実行中のコンテナ内でシェルを開く
- `docker rm コンテナ名` : 停止中のコンテナを削除する
- `docker logs -f コンテナ名` : コンテナのログを取得・追跡する

## Links
- More information: https://docs.docker.com/reference/cli/docker/
