# podman

ポッド、コンテナ、イメージのシンプルな管理ツールです。 PodmanはDocker-CLIと互換性のあるコマンドラインを提供します。簡潔に言うと: `alias docker=podman`。

## Usage
- `podman ps -a` : 全てのコンテナ(実行中と停止中の両方)を一覧表示する
- `podman run --name コンテナ名 イメージ` : イメージから任意の名前でコンテナを作成する
- `podman start|stop コンテナ名` : 既存のコンテナを起動または停止する
- `podman pull イメージ` : レジストリからイメージをプルする (デフォルトは Docker Hub)
- `podman images` : 既にダウンロードされているイメージのリストを表示する
- `podman exec -it コンテナ名 sh` : 既に起動しているコンテナ内でシェルを開く
- `podman rm コンテナ名` : 停止しているコンテナを削除する
- `podman logs -f コンテナ名 コンテナid ...` : 1つまたは複数のコンテナのログを表示し、ログ出力を追跡する

## Links
- More information: https://docs.podman.io/en/latest/Commands.html
