# bash

Bourne-Again SHell，兼容 `sh` 的命令行解释器。

## Usage
- `bash` : 启动交互式 shell
- `bash --norc` : 启动一个不加载启动配置的交互式的 shell 会话
- `bash -c "echo '已执行bash命令'"` : 执行命令
- `bash 路径/到/脚本文件.sh` : 执行脚本文件
- `bash -x 路径/到/脚本文件.sh` : 执行脚本文件，并将所有执行过的命令输出到终端
- `bash -e 路径/到/脚本文件.sh` : 执行脚本文件，并在第一个错误处终止
- `echo "echo '已执行bash命令'" | bash` : 从 `stdin` 执行指定的命令
- `bash -r` : 启动一个限制的 shell 会话

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Invoking-Bash
