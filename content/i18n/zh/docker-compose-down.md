# docker compose down

停止并移除由 `docker compose up` 创建的容器、网络、镜像和卷。

## Usage
- `docker compose down` : 停止并移除所有容器和网络
- `docker compose down --rmi all` : 停止并移除容器、网络以及服务使用的所有镜像
- `docker compose down --rmi local` : 停止并移除容器、网络以及没有自定义标签的镜像
- `docker compose down -v` : 停止并移除容器、网络和所有卷
- `docker compose down --remove-orphans` : 停止并移除所有资源，包括孤立容器
- `docker compose -f 路径/到/配置文件 down` : 使用另一个 Compose 文件停止并移除容器
- `docker compose down -t 秒数` : 使用以秒为单位的自定义超时停止并移除容器
- `docker compose down --remove-orphans -v` : 移除 Compose 文件中未定义的服务所对应的容器

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/down/
