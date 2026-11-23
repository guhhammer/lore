# jhat

Java 堆分析工具。

## Usage
- `jhat 路径/堆转储文件` : 分析堆转储文件（来自 jmap），通过 HTTP 端口 7000 进行查看
- `jhat -p 端口 路径/堆转储文件` : 分析堆转储文件，为 HTTP 服务指定备用端口
- `jhat -J-mx8G 路径/堆转储文件` : 通过 jhat 分析转储文件，指定使用 8 GB RAM（建议使用 2-4 倍的转储大小）

## Links
- More information: https://docs.oracle.com/javase/8/docs/technotes/tools/unix/jhat.html
