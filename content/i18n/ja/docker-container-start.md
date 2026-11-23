# docker container start

1つまたは複数の停止中コンテナを起動します。

## Usage
- `docker start コンテナ` : dockerコンテナを起動する
- `docker start -a コンテナ` : コンテナを起動し、`stdout`(標準出力) と `stderr`(標準エラー出力) をアタッチし、シグナルを転送する
- `docker start コンテナ1 コンテナ2 ...` : スペースで区切られた1つまたは複数の停止中コンテナを起動する
- `docker start --help` : ヘルプを表示する

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/start/
