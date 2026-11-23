# distcc

与 `distccd` 搭配使用的分布式 C/C++/ObjC 编译客户端。

## Usage
- `distcc gcc -c 路径/到/源文件.c -o 路径/到/输出文件.o` : 使用类似 `gcc` 的编译器编译源文件
- `export DISTCC_HOSTS="localhost 主机IP1 主机IP2 ..."` : 设置用于分布式编译的远程主机
- `make -j 并行任务数 CC="distcc gcc"` : 使用 `distcc` 和 `make` 并行编译项目
- `distcc --show-hosts` : 显示当前配置的 `distcc` 主机列表
- `distcc --help` : 显示帮助
- `distcc --version` : 显示版本

## Links
- More information: https://manned.org/distcc
