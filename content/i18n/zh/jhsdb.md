# jhsdb

附加到一个 Java 进程或启动一个事后调试器来分析崩溃的 Java 虚拟机的核心转储。

## Usage
- `jhsdb jstack --pid 进程id` : 打印 Java 进程的堆栈和锁信息
- `jhsdb clhsdb --core 路径/到/core_dump --exe 路径/到/jdk/bin/java` : 在交互式调试模式下打开一个核心转储
- `jhsdb debugd --pid 进程id --serverid 可选的唯一标识` : 启动远程调试服务器
- `jhsdb clhsdb --pid 进程id` : 在交互式调试模式下连接到一个进程

## Links
- More information: https://manned.org/jhsdb
