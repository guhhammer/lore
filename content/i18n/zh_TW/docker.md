# docker

管理 Docker 容器和映像檔。

## Usage
- `docker ps -a` : 列出所有 Docker 容器（包括停止的容器）
- `docker run --name 容器名稱 映像檔` : 透過映像檔啟動容器，並為容器命名
- `docker container start|stop 容器名稱` : 啟動或停止現有容器
- `docker pull 映像檔` : 從 Docker registry 中拉取映像檔
- `docker images` : 顯示已下載的映像檔清單
- `docker exec -it 容器名稱 sh` : 從正在運行的容器內打開一個互動式 (interactive) 終端 shell (`sh`)
- `docker rm 容器名稱` : 刪除一個停止的容器
- `docker logs -f 容器名稱` : 獲取並查看容器的日誌

## Links
- More information: https://docs.docker.com/reference/cli/docker/
