# docker compose

运行并管理多个 Docker 容器应用。

## Usage
- `docker compose ps` : 列出所有正在运行的容器
- `docker compose up -d` : 根据当前目录下的 `docker-compose.yml` 文件，创建并后台启动所有的容器
- `docker compose up --build` : 启动所有容器。如果必要，重新构建
- `docker compose -p 项目_名称 -f 路径/到/文件 up` : 指定一个项目名称和特定 compose 文件，启动所有容器
- `docker compose stop` : 停止所有运行中的容器
- `docker compose down --rmi all -v` : 停止，然后移除对应所有的容器、网络、镜像文件和文件卷
- `docker compose logs -f` : 跟踪所有容器的日志
- `docker compose logs -f 容器_名称` : 跟踪特定容器的日志

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/
