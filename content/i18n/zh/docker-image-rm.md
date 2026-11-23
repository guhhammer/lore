# docker image rm

移除 Docker 镜像。

## Usage
- `docker rmi 镜像1 镜像2 ...` : 根据名称移除一个或多个镜像
- `docker rmi -f 镜像` : 强制移除镜像
- `docker rmi --no-prune 镜像` : 移除镜像但不删除未标记的父镜像
- `docker rmi --help` : 显示帮助

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/rm/
