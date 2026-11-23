# docker compose

複数コンテナを持つDockerアプリケーションの実行と管理をします。

## Usage
- `docker compose ps` : 実行中のコンテナ全てをリスト表示する
- `docker compose up -d` : カレントディレクトリにある `docker-compose.yml`ファイルを使用してバックグラウンドで全てのコンテナを作成・起動する
- `docker compose up --build` : 全てのコンテナを起動し、必要に応じて再ビルドする
- `docker compose stop` : 実行中の全てのコンテナを停止する
- `docker compose down --rmi all -v` : 全てのコンテナ、ネットワーク、イメージ、ボリュームを停止して削除する
- `docker compose logs -f` : 全てのコンテナのログをフォローする(表示し続ける)
- `docker compose logs -f コンテナ名` : 特定コンテナのログをフォローする(表示し続ける)

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/
