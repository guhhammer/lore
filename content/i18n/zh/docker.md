# docker

管理 Docker 容器和镜像。

## Usage
- `docker ps -a` : 列出所有 Docker 容器（包括停止的容器）
- `docker run --name 容器名称 镜像` : 透过镜像启动容器，并为容器命名
- `docker container start|stop 容器名称` : 启动或停止现有容器
- `docker pull 镜像` : 从 Docker registry 中拉取镜像
- `docker images` : 显示已下载的镜像清单
- `docker exec -it 容器名称 sh` : 从正在运行的容器内打开一个交互式 (interactive) 终端 (tty) shell (`sh`)
- `docker rm 容器名称` : 删除一个停止的容器
- `docker logs -f 容器名称` : 获取并查看容器的日志

## Links
- More information: https://docs.docker.com/reference/cli/docker/
