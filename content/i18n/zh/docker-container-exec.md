# docker container exec

在一个正在运行的 Docker 容器内部，执行一个命令。

## Usage
- `docker exec -it 容器_名称 /bin/bash` : 在正在运行的容器内，启动交互式 shell 会话
- `docker exec -d 容器_名称 命令` : 在一个运行的容器内部，后台（分离）执行一个命令
- `docker exec -it -w 路径/到/文件夹 容器_名称 命令` : 指定要运行命令的运行目录
- `docker exec -i -d 容器_名称 命令` : 在现存容器内部，后台执行一个命令，但是保持 `stdin` 开启
- `docker exec -it -e 变量_名称=值 容器_名称 /bin/bash` : 在运行中容器的 Bash 内，设置环境变量
- `docker exec -u 用户名 容器_名称 命令` : 作为特定用户执行命令

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/exec/
