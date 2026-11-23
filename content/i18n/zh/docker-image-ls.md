# docker image ls

列出 Docker 镜像。

## Usage
- `docker images` : 列出所有 Docker 镜像
- `docker images -a` : 列出包括中间镜像在内的所有 Docker 镜像
- `docker images -q` : 以静默模式列出结果（仅显示数字 ID）
- `docker images -f dangling=true` : 列出所有未被任何容器使用的 Docker 镜像
- `docker images "*名称*"` : 列出名称中包含指定子字符串的镜像
- `docker images --format "\{\{.ID\}\}\t\{\{.Size\}\}\t\{\{.Repository\}\}:\{\{.Tag\}\}" | sort -k 2 -h` : 按大小对镜像排序

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/ls/
